lazy_static! {
    static ref SPECIAL_CHARS: Vec<char> = vec![
        '[', ']', '{', '}', '(', ')', '*', '|', ':', '<', '>', '/', '\\', '%', '&', '¿', '?', '¡',
        '!', '#', '\'', ' ', ','
    ];
}

pub fn validate_target(target: &str) -> bool {
    !target.starts_with('.')
        && target.contains('.')
        && !target.contains(&SPECIAL_CHARS[..])
        && target.is_ascii()
}

pub fn null_ip_checker(ip: &str) -> String {
    if ip.is_empty() {
        String::from("NULL")
    } else {
        ip.to_string()
    }
}

#[allow(clippy::ptr_arg)]
pub fn return_ports_string(ports: &Vec<String>) -> String {
    if ports.is_empty() {
        String::from("NULL")
    } else {
        ports.join(";")
    }
}
