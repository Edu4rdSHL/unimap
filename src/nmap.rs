use {
    crate::resolver_engine,
    log::error,
    std::{path::Path, process::Command},
};

lazy_static! {
    static ref NMAP_DNS_RESOLVERS: String = resolver_engine::RESOLVERS
        .clone()
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub nmaprun: Nmaprun,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nmaprun {
    pub scanner: String,
    pub args: String,
    pub start: String,
    pub startstr: String,
    pub version: String,
    pub xmloutputversion: String,
    pub host: Option<Host>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub starttime: String,
    pub endtime: String,
    pub status: Status,
    pub address: Address,
    pub hostnames: Hostnames,
    pub ports: Option<Ports>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub state: String,
    pub reason: String,
    #[serde(rename = "reason_ttl")]
    pub reason_ttl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub addr: Option<String>,
    pub addrtype: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hostnames {
    pub hostname: Option<Hostname>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hostname {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ports {
    pub port: Vec<Port>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub protocol: String,
    pub portid: String,
    pub state: State,
    pub service: Option<Service>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub state: String,
    pub reason: String,
    #[serde(rename = "reason_ttl")]
    pub reason_ttl: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub name: String,
    pub method: String,
    pub conf: String,
    pub product: Option<String>,
    pub ostype: Option<String>,
    pub version: Option<String>,
    pub extrainfo: Option<String>,
}

pub fn get_nmap_data(
    filename: &str,
    host: &str,
    min_rate: usize,
    initial_port: usize,
    last_port: usize,
    fast_scan: bool,
) -> Result<Nmaprun, serde_xml_rs::Error> {
    let _ports_range = format!("{}-{}", initial_port, last_port);
    let min_rate = min_rate.to_string();
    let nmap_args = if fast_scan {
        vec![
            "nmap",
            "-n",
            "--dns-servers",
            &NMAP_DNS_RESOLVERS,
            "-Pn",
            "--host-timeout",
            "5m",
            "--min-rate",
            &min_rate,
            "-sS",
            "--open",
            "-dd",
            "-T4",
            "--max-retries",
            "2",
            "-oX",
            filename,
            host,
        ]
    } else {
        vec![
            "nmap",
            "-n",
            "--dns-servers",
            &NMAP_DNS_RESOLVERS,
            "-Pn",
            "--host-timeout",
            "10m",
            "-sV",
            "--min-rate",
            &min_rate,
            "-sS",
            "-p-",
            "--open",
            "-dd",
            "-T4",
            "--max-retries",
            "2",
            "-oX",
            filename,
            host,
        ]
    };
    match Command::new("sudo").args(&nmap_args).output() {
        Ok(_) => {
            if Path::new(&filename).exists() && Path::new(&filename).is_file() {
                serde_xml_rs::from_str(&std::fs::read_to_string(filename).unwrap_or_default())
            } else {
                error!("Error executing nmap, you need root permissions. Leaving.\n");
                std::process::exit(1)
            }
        }
        Err(e) => {
            error!(
                    "Error waiting command to finish for {}, continuing with remaining hosts. Description: {}",
                    host, e
                );
            Ok(Nmaprun::default())
        }
    }
}
