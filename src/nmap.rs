use {
    crate::resolver_engine,
    log::error,
    std::{path::Path, process::Command},
};

lazy_static! {
    static ref NMAP_DNS_RESOLVERS: String = resolver_engine::RESOLVERS
        .clone()
        .iter()
        .map(ToString::to_string)
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
    min_rate: &str,
    ports: &str,
    fast_scan: bool,
) -> Result<Nmaprun, serde_xml_rs::Error> {
    let min_rate = min_rate.to_string();
    let mut nmap_args = vec![
        "nmap",
        "--dns-servers",
        &NMAP_DNS_RESOLVERS,
        "-Pn",
        "-sS",
        "--open",
        "-dd",
        "-T4",
        "--max-retries",
        "3",
        "-oX",
        filename,
    ];

    if !min_rate.is_empty() {
        nmap_args.append(&mut vec!["--min-rate", &min_rate])
    }

    if fast_scan {
        nmap_args.append(&mut vec!["--host-timeout", "20m"])
    } else {
        nmap_args.append(&mut vec!["-sV"])
    }

    if !ports.is_empty() {
        nmap_args.append(&mut vec!["-p", ports])
    }

    nmap_args.push(host);

    match Command::new("nmap").args(&nmap_args).output() {
        Ok(_) => {
            if Path::new(&filename).exists() && Path::new(&filename).is_file() {
                serde_xml_rs::from_str(&std::fs::read_to_string(filename).unwrap_or_default())
            } else {
                error!("Error executing nmap, possible causes: Nmap is not installed or you need root/administrator permissions. Leaving.\n");
                println!();
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
