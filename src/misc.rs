use std::{
    collections::HashSet,
    io::{self, Read},
};

pub fn sanitize_target_string(target: String) -> String {
    target
        .replace("www.", "")
        .replace("https://", "")
        .replace("http://", "")
        .replace('/', "")
}

pub fn return_matches_vec(matches: &clap::ArgMatches, value: &str) -> Vec<String> {
    if matches.is_present(value) {
        matches
            .values_of(value)
            .unwrap()
            .map(str::to_owned)
            .collect()
    } else {
        Vec::new()
    }
}

#[allow(dead_code)]
pub fn return_matches_hashset(matches: &clap::ArgMatches, value: &str) -> HashSet<String> {
    if matches.is_present(value) {
        matches
            .values_of(value)
            .unwrap()
            .map(str::to_owned)
            .collect()
    } else {
        HashSet::new()
    }
}

pub fn read_stdin() -> HashSet<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Error getting input list.");
    buffer.lines().map(str::to_owned).collect()
}
