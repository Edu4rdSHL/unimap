use {
    crate::{files, structs::Args},
    std::{collections::HashSet, net::SocketAddr},
    trust_dns_resolver::{
        config::{NameServerConfig, NameServerConfigGroup, Protocol, ResolverConfig, ResolverOpts},
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

pub fn get_resolver(nameserver_ips: HashSet<SocketAddr>, opts: ResolverOpts) -> Resolver {
    let mut name_servers = NameServerConfigGroup::with_capacity(nameserver_ips.len() * 2);
    name_servers.extend(nameserver_ips.into_iter().flat_map(|socket_addr| {
        std::iter::once(NameServerConfig {
            socket_addr,
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_nx_responses: false,
        })
        .chain(std::iter::once(NameServerConfig {
            socket_addr,
            protocol: Protocol::Tcp,
            tls_dns_name: None,
            trust_nx_responses: false,
        }))
    }));
    Resolver::new(ResolverConfig::from_parts(None, vec![], name_servers), opts).unwrap()
}

pub fn return_socket_address(args: &Args) -> HashSet<SocketAddr> {
    let mut resolver_ips = HashSet::new();
    if args.custom_resolvers {
        for r in &files::return_file_targets(args, args.resolvers.clone()) {
            let server = r.to_owned() + ":53";
            let socket_addr = SocketAddr::V4(match server.parse() {
                Ok(a) => a,
                Err(e) => unreachable!(
                    "Error parsing the server {}, only IPv4 are allowed. Error: {}",
                    r, e
                ),
            });
            resolver_ips.insert(socket_addr);
        }
    } else {
        for r in &args.resolvers {
            let server = r.to_owned() + ":53";
            let socket_addr = SocketAddr::V4(match server.parse() {
                Ok(a) => a,
                Err(e) => unreachable!(
                    "Error parsing the server {}, only IPv4 are allowed. Error: {}",
                    r, e
                ),
            });
            resolver_ips.insert(socket_addr);
        }
    }
    resolver_ips
}
