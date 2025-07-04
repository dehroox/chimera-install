pub struct RootData {
    pub source: Option<Source>,
    pub hostname: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub root_password: Option<String>,
    pub additional_users: Option<Vec<User>>,
    pub partition: Option<PartitionType>,
    pub setup_bootloader: Option<Bootloader>,
    pub additional_repositories: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum Source {
    Network,
    Local,
}

#[derive(Debug, Clone, Copy)]
pub enum PartitionType {
    Auto,
    Manual,
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub pass: String,
    pub sudoer: bool,
}

#[derive(Clone, Debug)]
pub enum Bootloader {
    Grub,
    Refind,
    Systemd,
    Efistub,
    None,
}

pub type MenuCallback = fn(&mut cursive::Cursive);