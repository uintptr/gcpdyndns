use std::{env, fs, path::PathBuf};

use clap::{Parser, Subcommand};
use log::{LevelFilter, warn};
use rstaples::logging::StaplesLogger;
use udyndns::{
    error::{Error, Result},
    external::get_external_ip,
    persistent::Persistance,
    providers::{digital_ocean::DoArgs, google_cloud::GcpArgs},
};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Subcommand)]
enum Providers {
    Gcp(GcpArgs),
    DigitalOcean(DoArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct UserArgs {
    /// force update
    #[arg(short, long)]
    force: bool,

    /// verbose
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    providers: Providers,
}

fn get_data_dir() -> Result<PathBuf> {
    let data_root = dirs::data_dir().ok_or(Error::DataDirNotFound)?;

    let data_dir = data_root.join(CRATE_NAME);

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    Ok(data_dir)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = UserArgs::parse();

    let log_level = match args.verbose {
        true => LevelFilter::Info,
        false => LevelFilter::Warn,
    };

    StaplesLogger::new()
        .with_colors()
        .with_log_level(log_level)
        .start();

    let data_dir = get_data_dir()?;

    let host_name = match &args.providers {
        Providers::Gcp(gcp) => gcp.hostname.to_string(),
        Providers::DigitalOcean(ocean) => ocean.hostname.to_string(),
    };

    let mut persist = Persistance::new(data_dir, &host_name)?;

    let ip_addr = get_external_ip().await?;

    let changed = persist.ip_changed(&ip_addr);

    if changed || args.force {
        warn!("new ip {ip_addr}");

        match args.providers {
            Providers::Gcp(gcp) => gcp.update(&ip_addr).await?,
            Providers::DigitalOcean(ocean) => ocean.update(&ip_addr).await?,
        }

        persist.update(ip_addr)
    } else {
        Ok(())
    }
}
