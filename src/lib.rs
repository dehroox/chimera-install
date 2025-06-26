pub struct RootData {
    pub source: bool, // true = network install, false = local install
    pub hostname: String,
    pub locale: String,
    pub keyboard: String,
    pub timezone: String,
    pub root_password: String,
    pub additional_users: Box<User>,
    pub partition: bool, // true = automatic partitioning, false = let user partition using cfdisk
    pub setup_bootloader: Bootloader,
    pub additional_repositories: Box<String>
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