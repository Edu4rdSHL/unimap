use {
    crate::{
        args,
        errors::*,
        files, logic, networking,
        nmap::{self, Nmaprun},
        structs::{Args, ResolvData},
    },
    log::{error, info},
    prettytable,
    prettytable::Table,
    rayon::prelude::*,
    std::{
        collections::{HashMap, HashSet},
        net::Ipv4Addr,
        time::Duration,
    },
    trust_dns_resolver::{config::ResolverOpts, proto::rr::RecordType},
};

lazy_static! {
    pub static ref RESOLVERS: Vec<Ipv4Addr> = {
        let args = args::get_args();
        let mut resolver_ips = Vec::new();
        if args.custom_resolvers {
            for r in &files::return_file_targets(&args, args.resolvers.clone()) {
                match r.parse::<Ipv4Addr>() {
                    Ok(ip) => resolver_ips.push(ip),
                    Err(e) => {
                        error!("Error parsing the {} IP from resolvers file to IP address. Please check and try again. Error: {}\n", r, e);
                        std::process::exit(1)
                    }
                }
            }
        } else {
            for r in args.resolvers {
                match r.parse::<Ipv4Addr>() {
                    Ok(ip) => resolver_ips.push(ip),
                    Err(e) => {
                        error!("Error parsing the {} IP from resolvers file to IP address. Please check and try again. Error: {}\n", r, e);
                        std::process::exit(1)
                    }
                }
            }
        }
        resolver_ips
    };
}

pub fn async_resolver_all(args: &mut Args) -> Result<()> {
    if args.with_output && !args.unique_output_flag && !files::check_full_path(&args.logs_dir) {
        error!(
            "Error creating the logs folder in the path {}. Leaving...",
            &args.logs_dir
        );
        std::process::exit(1)
    }

    if !args.quiet_flag {
        info!(
            "Performing asynchronous resolution for {} targets with {} threads, it will take a while...\n",
            args.targets.len(), args.threads
        )
    }

    let data = async_resolver_engine(&args, args.targets.clone());

    let mut table = Table::new();
    table.set_titles(row![
        bcFg => "HOST",
       "IP",
       "OPEN PORTS",
       "SERVICES"
    ]);
    for (target, resolv_data) in &data {
        if !resolv_data.ip.is_empty() {
            let mut services_table = Table::new();
            for port_data in &resolv_data.ports_data {
                services_table
                    .add_row(row![bc => &format!("PORT => {}", port_data.portid.clone())]);
                services_table.add_row(
                    row![c => &format!("SERVICE: {}", port_data.service.clone().unwrap_or_default().name)],
                );
                services_table.add_row(row![c => &format!("VERSION: {}" ,port_data
                .service.clone().unwrap_or_default()
                .version
                .clone()
                .unwrap_or_else(|| "NULL".to_string()))]);
                services_table.add_row(row![c => &format!("PRODUCT: {}", port_data
                    .service.clone().unwrap_or_default()
                    .product
                    .clone()
                    .unwrap_or_else(|| "NULL".to_string()))]);
                services_table.add_row(row![c => &format!("OS TYPE: {}", port_data
                    .service.clone().unwrap_or_default()
                    .ostype
                    .clone()
                    .unwrap_or_else(|| "NULL".to_string()))]);
                services_table.add_row(row![c => &format!("EXTRA INFO: {}", port_data
                    .service.clone().unwrap_or_default()
                    .extrainfo
                    .clone()
                    .unwrap_or_else(|| "NULL".to_string()))]);
            }
            table.add_row(row![ d =>
                target,
                logic::null_ip_checker(&resolv_data.ip),
                logic::return_ports_string(
                    &resolv_data
                        .ports_data
                        .iter()
                        .map(|f| f.portid.clone())
                        .collect(),
                ),
                services_table,
            ]);
        }
    }

    if args.with_output && !args.targets.is_empty() {
        if files::table_to_file(&table, files::return_output_file(&args)).is_err()
            && !args.quiet_flag
        {
            error!(
                "An error occurred while writing the output file {}.\n",
                args.file_name
            )
        }
    }
    if !args.quiet_flag {
        table.printstd();
    }

    if ((args.with_output && !args.unique_output_flag) || args.unique_output_flag)
        && !args.quiet_flag
    {
        info!(
            "Job finished in {} seconds.\n",
            args.time_wasted.elapsed().as_secs()
        );
        info!("Logfile saved in {}\n\n", args.file_name);
    }

    Ok(())
}

fn async_resolver_engine(args: &Args, targets: HashSet<String>) -> HashMap<String, ResolvData> {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(2);

    let resolv_data: HashMap<String, ResolvData> = targets
        .par_iter()
        .map(|target| {
            let fqdn_target = format!("{}.", target);
            let mut resolv_data = ResolvData::default();
            resolv_data.ip = networking::get_records(
                &networking::get_resolver(&RESOLVERS, &opts),
                &fqdn_target,
                RecordType::A,
            );
            (target.to_owned(), resolv_data)
        })
        .collect();

    let mut nmap_ips: HashSet<String> = resolv_data
        .iter()
        .map(|(_, resolv_data)| resolv_data.ip.clone())
        .collect();

    nmap_ips.retain(|ip| !ip.is_empty());

    let nmap_data: HashMap<String, Nmaprun> = nmap_ips
        .par_iter()
        .map(|ip| {
            let filename = format!("{}.xml", &ip);
            match nmap::get_nmap_data(
                &filename,
                &ip,
                args.min_rate,
                args.initial_port,
                args.last_port,
                args.fast_scan,
            ) {
                Ok(nmap_data) => {
                    nmap_data
                        .host
                        .clone()
                        .unwrap_or_default()
                        .ports
                        .unwrap_or_default()
                        .port
                        .retain(|f| f.state.state == "open");
                    if !args.keep_nmap_logs && std::fs::remove_file(&filename).is_err() {
                        error!("Error removing filename {}.", &filename)
                    }
                    (ip.clone(), nmap_data)
                }
                Err(e) => {
                    error!("Error scanning the ip {}. Description: {}", &ip, e);
                    (String::new(), Nmaprun::default())
                }
            }
        })
        .collect();

    resolv_data
        .iter()
        .map(|(target, resolv_data)| {
            (
                target.clone(),
                ResolvData {
                    ip: resolv_data.ip.clone(),
                    ports_data: if resolv_data.ip.is_empty() {
                        resolv_data.ports_data.clone()
                    } else {
                        nmap_data
                            .get_key_value(&resolv_data.ip)
                            .unwrap()
                            .1
                            .host
                            .clone()
                            .unwrap_or_default()
                            .ports
                            .unwrap_or_default()
                            .port
                    },
                },
            )
        })
        .collect()
}
