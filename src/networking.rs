use {
    log::error,
    rand::Rng,
    std::net::{IpAddr, Ipv4Addr},
    trust_dns_resolver::{
        config::{NameServerConfigGroup, ResolverConfig, ResolverOpts},
        proto::rr::RecordType,
        Resolver,
    },
};

pub fn get_records(resolver: &Resolver, domain: &str, record_type: RecordType) -> String {
    if let Ok(rdata) = resolver.lookup(&domain, record_type) {
        let mut record_data: Vec<String> = Vec::new();
        if record_type == RecordType::AAAA {
            record_data = rdata
                .iter()
                .filter_map(|rdata| rdata.as_aaaa())
                .map(|ipv6| ipv6.to_string())
                .collect();
        } else if record_type == RecordType::A {
            record_data = rdata
                .iter()
                .filter_map(|rdata| rdata.as_a())
                .map(|ipv4| ipv4.to_string())
                .collect();
        } else if record_type == RecordType::CNAME {
            record_data = rdata
                .iter()
                .filter_map(|rdata| rdata.as_cname())
                .map(|name| {
                    let name = name.to_string();
                    name[..name.len() - 1].to_owned()
                })
                .collect();
        }
        record_data
            .iter()
            .next()
            .expect("Failed retrieving records data.")
            .to_owned()
    } else {
        String::new()
    }
}

pub fn get_resolver(resolvers_ips: &[Ipv4Addr], opts: &ResolverOpts) -> Resolver {
    match Resolver::new(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(
                &[IpAddr::V4(
                    resolvers_ips[rand::thread_rng().gen_range(0, resolvers_ips.len())],
                )],
                53,
            ),
        ),
        *opts,
    ) {
        Ok(resolver) => resolver,

        Err(e) => {
            error!("Failed to create the resolver. Error: {}\n", e);
            std::process::exit(1)
        }
    }
}
