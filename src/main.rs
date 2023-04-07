mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

use std::{io, process::exit};

use error::Error;
use model::Subdomain;

use crate::subdomains::enumerate;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut subdomain = Subdomain {
        domain: String::new(),
        open_ports: vec![],
    };
    println!("Enter Domain Name: ");
    match io::stdin().read_line(&mut subdomain.domain) {
        Ok(_v) => subdomain.domain.trim(),
        Err(e) => return Err(e.into()),
    };

    subdomain.domain = subdomain.domain.trim().into();

    let client = reqwest::Client::new();
    let mut subdomains = match enumerate(&client, &subdomain.domain).await{
        Ok(subdomains) => subdomains,
        Err(err) => {
            eprintln!("Error in Enumerating Subdomain {}: {}", subdomain.domain, err);
            exit(-1);
        },
    };

    let _ = subdomains
        .iter_mut()
        .map(|subdomain| {
            ports::scan_ports(subdomain)
        }).collect::<Vec<&Subdomain>>();

    println!("{:#?}", subdomains);
    Ok(())
    // Err(Error::CliUsage.into())
}
