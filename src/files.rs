use {
    crate::{errors::*, structs::Args},
    log::error,
    prettytable::Table,
    std::{
        collections::HashSet,
        fs::{self, File, OpenOptions},
        io::{BufRead, BufReader, Write},
        path::Path,
    },
};

pub fn return_file_targets(args: &Args, mut files: Vec<String>) -> Vec<String> {
    let mut targets: Vec<String> = Vec::new();
    files.sort();
    files.dedup();
    for f in files {
        match File::open(&f) {
            Ok(file) => {
                for target in BufReader::new(file).lines().flatten() {
                    targets.push(target);
                }
            }
            Err(e) => {
                if args.files.len() == 1 {
                    error!("Can not open file {}. Error: {}\n", f, e);
                    std::process::exit(1)
                } else if !args.quiet_flag {
                    error!(
                        "Can not open file {}, working with next file. Error: {}\n",
                        f, e
                    );
                }
            }
        }
    }
    targets.sort();
    targets.dedup();
    targets.iter().map(|target| target.to_lowercase()).collect()
}

pub fn table_to_file(table: &Table, file_name: Option<std::fs::File>) -> Result<()> {
    table.to_csv(file_name.unwrap())?;
    Ok(())
}

pub fn return_output_file(args: &Args) -> Option<File> {
    if args.file_name.is_empty() || !args.with_output {
        None
    } else {
        Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&args.file_name)
                .with_context(|_| format!("Can't create file {}", &args.file_name))
                .unwrap(),
        )
    }
}

pub fn check_full_path(full_path: &str) -> bool {
    (Path::new(full_path).exists() && Path::new(full_path).is_dir())
        || fs::create_dir_all(full_path).is_ok()
}

pub fn delete_files(paths: &HashSet<String>) {
    for file in paths {
        if Path::new(&file).exists() {
            match std::fs::remove_file(&file) {
                Ok(_) => (),
                Err(e) => error!("Error deleting the file {}. Description: {}", &file, e),
            }
        }
    }
}

pub fn string_to_file(data: String, mut file: File) -> Result<()> {
    file.write_all(data.as_bytes())?;
    Ok(())
}
