use {
    log::{error, Level},
    std::{collections::HashSet, iter::FromIterator},
    unimap::{args, errors::*, files::return_file_targets, logger, resolver_engine},
};

fn run() -> Result<()> {
    if std::env::var("UNIMAP_LOG_LEVEL").is_ok() {
        logger::init_by_env()
    } else {
        logger::init_with_level(Level::Info).unwrap()
    }
    let mut arguments = args::get_args();
    if !arguments.files.is_empty() {
        arguments.targets =
            HashSet::from_iter(return_file_targets(&arguments, arguments.files.clone()))
    } else {
        arguments.targets.insert(arguments.target.clone());
    }

    if arguments.targets.len() < 50 {
        arguments.threads = arguments.targets.len()
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(arguments.threads)
        .build_global()
        .unwrap();

    if !arguments.target.is_empty() || !arguments.files.is_empty() {
        resolver_engine::async_resolver_all(&mut arguments)
    } else {
        error!("Error: Target is empty or invalid!\n");
        std::process::exit(1)
    }
}

fn main() {
    if let Err(err) = run() {
        error!("Error: {}", err);
        for cause in err.iter_chain().skip(1) {
            error!("Error description: {}\n", cause);
        }
        std::process::exit(1);
    }
}
