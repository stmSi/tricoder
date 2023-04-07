mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

use std::io;

use error::Error;
use model::Subdomain;

fn main() -> Result<(), anyhow::Error> {
    let mut subdomain = Subdomain {
        domain: String::new(),
        open_ports: vec![],
    };
    println!("Enter Domain Name: ");
    match io::stdin().read_line(&mut subdomain.domain) {
        Ok(v) => subdomain.domain.trim(),
        Err(e) => return Err(e.into()),
    };

    subdomain.domain = subdomain.domain.trim().into();

    let subdomain = ports::scan_ports(subdomain);
    println!("{:?}", subdomain);

    // Ok(())
    Err(Error::CliUsage.into())
}
