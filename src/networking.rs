use {
    log::error,
    rand::{seq::SliceRandom, thread_rng as rng},
    std::net::{IpAddr, Ipv4Addr},
    trust_dns_resolver::{
        config::{NameServerConfigGroup, ResolverConfig, ResolverOpts},
        Resolver,
    },
};

pub fn get_records(resolver: &Resolver, domain: &str) -> String {
    if let Ok(ips) = resolver.ipv4_lookup(domain) {
        ips.iter()
            .map(|x| x.to_string())
            .next()
            .expect("Failed to get IPV4.")
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
                    resolvers_ips
                        .choose(&mut rng())
                        .expect("failed to read ipv4 string")
                        .to_owned(),
                )],
                53,
                false,
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
