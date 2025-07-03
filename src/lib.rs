pub struct RootData {
    pub source: Option<bool>, // network or local
    pub hostname: Option<String>,
    pub locale: Option<String>,
    pub keyboard: Option<String>,
    pub timezone: Option<String>,
    pub root_password: Option<String>,
    pub additional_users: Option<Box<User>>,
    pub partition: Option<bool>, // true = automatic partitioning, false = let user partition using cfdisk
    pub setup_bootloader: Option<Bootloader>,
    pub additional_repositories: Option<Box<String>>
}
pub struct User {
    pub name: String,
    pub pass: String,
    pub sudoer: bool
}

pub enum Bootloader {
    Grub,
    Refind,
    Systemd,
    Efistub,
    None
}