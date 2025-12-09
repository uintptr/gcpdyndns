use std::path::PathBuf;

use addr::parse_domain_name;
use clap::Args;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    error::{Error, Result},
    external::ExternalIp,
};

const DO_API_URL: &str = "https://api.digitalocean.com";

#[derive(Args, Clone)]
pub struct DoArgs {
    /// auth JSON file
    #[arg(short, long)]
    pub api_key_file: PathBuf,

    /// hostname
    #[arg(short = 'H', long)]
    pub hostname: String,

    /// ipv6
    #[arg(long)]
    pub ipv6: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DigitalOceanRecord {
    id: u64,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct DigitalOceanRecords {
    domain_records: Vec<DigitalOceanRecord>,
}

struct DomainInfo {
    pub name: String,
    pub sub: String,
}

impl DomainInfo {
    pub fn from_hostname(hostname: &str) -> Result<Self> {
        let domain = match parse_domain_name(hostname) {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to parse {hostname} ({e})");
                return Err(Error::DomainParsingFailure);
            }
        };

        let name = domain
            .root()
            .map(|d| d.to_string())
            .ok_or(Error::DomainParsingFailure)?;

        let sub = domain
            .prefix()
            .map(|s| s.to_string())
            .ok_or(Error::DomainParsingFailure)?;

        Ok(Self { name, sub })
    }
}

impl DoArgs {
    async fn find_record_id(&self, client: &Client, domain: &str, token: &str) -> Result<u64> {
        let url = format!(
            "{DO_API_URL}/v2/domains/{}/records?name={}",
            domain, self.hostname
        );

        let recs: DigitalOceanRecords = client
            .get(url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<DigitalOceanRecords>()
            .await?;

        let first = recs
            .domain_records
            .first()
            .ok_or(Error::DomainRecordNotFound)?;

        Ok(first.id)
    }

    pub async fn update(&self, ip: &ExternalIp) -> Result<()> {
        let token = fs::read_to_string(&self.api_key_file).await?;
        let token = token.trim();

        let client = Client::new();

        let domain = DomainInfo::from_hostname(&self.hostname)?;

        let record_id = self.find_record_id(&client, &domain.name, token).await?;

        let url = format!(
            "{DO_API_URL}/v2/domains/{}/records/{}",
            domain.name, record_id
        );

        let rec_type = if ip.is_ipv4() { "A" } else { "AAAA" };

        let new_rec = DigitalOceanRecord {
            id: record_id,
            name: domain.sub,
            data: ip.address.to_string(),
            record_type: rec_type.to_string(),
        };

        let res = client
            .patch(url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .json(&new_rec)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::UpdateFailure(res.status()))
        }
    }
}
