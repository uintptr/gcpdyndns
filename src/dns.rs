use reqwest::Client;
use serde::Serialize;

use crate::error::{Error, Result};

const DNS_API_URL: &str = "https://dns.googleapis.com/dns/v1beta2/projects";

#[derive(Debug, Serialize)]
struct DnsPatchRequest<'a> {
    rrdatas: Vec<&'a str>,
}

async fn auth() -> Result<String> {
    let provider = gcp_auth::provider().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = provider.token(scopes).await?;
    Ok(token.as_str().to_string())
}

pub async fn edit_dns_record(
    project: &str,
    zone: &str,
    dns_name: &str,
    ip_addr: &str,
) -> Result<()> {
    let token = auth().await?;

    //
    // make sure the name ends with a dot (.)
    //
    let name = match dns_name.ends_with(".") {
        true => dns_name.to_string(),
        false => format!("{dns_name}."),
    };

    let url = format!(
        "{}/{}/managedZones/{}/rrsets/{name}/A",
        DNS_API_URL, project, zone
    );

    let req_data = DnsPatchRequest {
        rrdatas: vec![ip_addr],
    };

    let client = Client::new();

    let res = client
        .patch(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&req_data)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Error::UpdateFailure(res.status()))
    }
}
