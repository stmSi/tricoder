use reqwest::Client;
use std::{error::Error, collections::HashSet, time::Duration};
use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::config::ResolverConfig;
use trust_dns_resolver::Resolver;
use crate::{model::{CrtShEntry, Subdomain}, subdomains};

pub async fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Box<dyn Error>> {
// pub async fn enumerate(http_client: &Client, target: &str) -> Result<(), Box<dyn Error>> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()
        .await?
        .json()
        .await?;

    // cleanup and remove duplicate results
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
                entry.name_value
                    .split("\n")
                    .map(|subdomain| subdomain.trim().to_string())
                    .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains("*"))
        .collect();

    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        // .filter(resolves)
        .collect();

    Ok(subdomains)
}


pub fn resolves(domain: &Subdomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(
        ResolverConfig::default(),
        opts,
    )
    .expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
