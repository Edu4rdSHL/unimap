use {
    crate::{
        defaults,
        logic::validate_target,
        misc::{return_matches_vec, sanitize_target_string},
        structs::Args,
    },
    chrono::Utc,
    clap::{load_yaml, value_t, App},
    std::{collections::HashSet, time::Instant},
};

#[allow(clippy::cognitive_complexity)]
pub fn get_args() -> Args {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .version(clap::crate_version!())
        .get_matches();

    Args {
        target: {
            let target = sanitize_target_string(
                value_t!(matches, "target", String).unwrap_or_else(|_| String::new()),
            );
            if validate_target(&target) {
                target
            } else {
                String::new()
            }
        },
        file_name: if matches.is_present("output") {
            value_t!(matches, "logs-dir", String).unwrap_or_else(|_| "unimap_logs".to_string())
                + "/"
                + "unimap"
                + &Utc::now().format("-log-%Y-%m-%d_%H-%M-%S").to_string()
                + ".csv"
        } else if matches.is_present("unique-output") {
            matches.value_of("unique-output").unwrap().to_string()
        } else {
            String::new()
        },
        logs_dir: value_t!(matches, "logs-dir", String)
            .unwrap_or_else(|_| "unimap_logs".to_string()),
        threads: if matches.is_present("ports") && !matches.is_present("threads") {
            30
        } else {
            value_t!(matches, "threads", usize).unwrap_or_else(|_| 50)
        },
        version: clap::crate_version!().to_string(),
        ports: value_t!(matches, "ports", String).unwrap_or_else(|_| String::new()),
        with_output: matches.is_present("output") || matches.is_present("unique-output"),
        unique_output_flag: matches.is_present("unique-output"),
        from_file_flag: matches.is_present("files"),
        quiet_flag: matches.is_present("quiet"),
        custom_resolvers: matches.is_present("custom-resolvers"),
        custom_ports_range: matches.is_present("ports"),
        fast_scan: matches.is_present("fast-scan"),
        no_keep_nmap_logs: matches.is_present("no-keep-nmap-logs"),
        raw_output: matches.is_present("raw-output"),
        url_output: matches.is_present("url-output"),
        from_stdin: matches.is_present("stdin"),
        files: return_matches_vec(&matches, "files"),
        min_rate: value_t!(matches, "min-rate", String).unwrap_or_else(|_| String::new()),
        resolvers: if matches.is_present("custom-resolvers") {
            return_matches_vec(&matches, "custom-resolvers")
        } else {
            defaults::ipv4_resolvers()
        },
        targets: HashSet::new(),
        time_wasted: Instant::now(),
    }
}
