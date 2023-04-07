use serde::Deserialize;

#[derive(Debug)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug, Deserialize)]
pub struct CrtShEntry {
    pub id: u64,
    pub name_value: String,

}
