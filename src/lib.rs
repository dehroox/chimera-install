use std::fs::read_to_string;

pub struct RootData {
    pub source: Option<bool>, // network or local
    pub hostname: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub root_password: Option<String>,
    pub additional_users: Option<Vec<User>>,
    pub partition: Option<bool>, // true = automatic partitioning, false = let user partition using cfdisk
    pub setup_bootloader: Option<Bootloader>,
    pub additional_repositories: Option<Box<String>>,
}
pub struct User {
    pub name: String,
    pub pass: String,
    pub sudoer: bool,
}

#[derive(Clone)]
pub enum Bootloader {
    Grub,
    Refind,
    Systemd,
    Efistub,
    None,
}

pub fn get_locales() -> String {
    return read_to_string("/usr/share/i18/SUPPORTED").expect("Failed to read locales file");
}

pub fn get_timezones() -> Vec<(String, String)> {
    let content =
        read_to_string("/usr/share/zoneinfo/zone.tab").expect("Failed to read timezones file");

    let mut timezones: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some(tz) = line.split_whitespace().nth(2) {
            let stringed = tz.to_string();
            timezones.push((stringed.clone(), stringed));
        }
    }

    timezones.sort();
    return timezones;
}
