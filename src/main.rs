use chimera_install::RootData;
use cursive::align::HAlign;
use cursive::views::{Dialog, LinearLayout, RadioGroup, SelectView};
use cursive::{Cursive, CursiveExt};

// generic type so rust doesnt complain :(
type MenuFn = fn(&mut Cursive);

fn main() {
    let root_data = RootData {
        source: None,                  // Network = true / Local = false
        hostname: None,
        locale: None,                  // en_US.UTF-8/en_GB.UTF-8 etc. etc.
        keyboard: None,                // US/UK etc. etc.
        timezone: None,                // Europe/London etc. etc.
        root_password: None,           // root password
        additional_users: None,        // see lib.rs
        partition: None,               // see lib.rs
        setup_bootloader: None,        // grub, refind, systemd-boot, efistub, none
        additional_repositories: None, // see lib.rs
    };
    let mut root = Cursive::new();
    root.set_user_data(root_data); // store root_data in the root

    root.add_global_callback('q', |s| s.quit()); // quit with 'q', duh

    // Use the generic MenuFn type for all menu functions
    let main_select = SelectView::<MenuFn>::new()
        .h_align(HAlign::Center)
        .item("Source", source_menu as MenuFn)
        .item("Hostname", hostname_menu as MenuFn)
        .item("Locale", locale_menu as MenuFn)
        .item("Keyboard", keyboard_menu as MenuFn)
        .item("Timezone", timezone_menu as MenuFn)
        .item("Root Password", root_password_menu as MenuFn)
        .item("Additional Users", additional_users_menu as MenuFn)
        .item("Partitioning", partition_menu as MenuFn)
        .item("Setup Bootloader", setup_bootloader_menu as MenuFn)
        .item(
            "Additional Repositories",
            additional_repositories_menu as MenuFn,
        )
        .item("Install", install_menu as MenuFn)
        .on_submit(|s, val| {
            val(s);
        });

    let main_view = Dialog::around(main_select).title("Chimera Linux Installer");

    root.add_layer(main_view);
    root.run();
}

fn source_menu(s: &mut Cursive) {
    let mut group: RadioGroup<&bool> = RadioGroup::new();

    s.add_layer(
        Dialog::new()
            .title("Source of bootstrapped packages")
            .content(
                LinearLayout::vertical()
                    .child(group.button(&true, "Network"))
                    .child(group.button(&false, "Local")),
            )
            .button("Ok", move |siv| {
                let selected = group.selection();
                siv.with_user_data(|data: &mut RootData| {
                    data.source = Some(**selected);
                });
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    );
}

fn hostname_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Hostname menu not implemented."));
}
fn locale_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Locale menu not implemented."));
}
fn keyboard_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Keyboard menu not implemented."));
}
fn timezone_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Timezone menu not implemented."));
}
fn root_password_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Root password menu not implemented."));
}
fn additional_users_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Additional users menu not implemented."));
}
fn partition_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Partition menu not implemented."));
}
fn setup_bootloader_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Setup bootloader menu not implemented."));
}
fn additional_repositories_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info(
        "Additional repositories menu not implemented.",
    ));
}
fn install_menu(s: &mut Cursive) {
    s.add_layer(Dialog::info("Install menu not implemented."));
}
