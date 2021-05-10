use {
    crate::nmap::Port,
    std::{collections::HashSet, time::Instant},
};

#[derive(Clone, Debug)]
pub struct Args {
    pub target: String,
    pub file_name: String,
    pub version: String,
    pub logs_dir: String,
    pub threads: usize,
    pub ports: String,
    pub with_output: bool,
    pub unique_output_flag: bool,
    pub min_rate: String,
    pub from_file_flag: bool,
    pub quiet_flag: bool,
    pub custom_resolvers: bool,
    pub custom_ports_range: bool,
    pub no_keep_nmap_logs: bool,
    pub raw_output: bool,
    pub fast_scan: bool,
    pub files: Vec<String>,
    pub resolvers: Vec<String>,
    pub targets: HashSet<String>,
    pub time_wasted: Instant,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResolvData {
    pub ip: String,
    pub ports_data: Vec<Port>,
}
impl ResolvData {
    pub fn default() -> ResolvData {
        ResolvData {
            ip: String::new(),
            ports_data: Vec::new(),
        }
    }
}
