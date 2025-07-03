use chimera_install::{get_locales, get_timezones, RootData, User};
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Dialog, EditView, LinearLayout, OnEventView, RadioGroup, ScrollView, SelectView,
};
use cursive::{Cursive, CursiveExt, View};

// generic type so rust doesnt complain :(
type MenuFn = fn(&mut Cursive);

fn main() {
    let root_data = RootData {
        source: None, // Network = true / Local = false
        hostname: None,
        locale: None,                  // en_US.UTF-8/en_GB.UTF-8 etc. etc.
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

    let main_select = SelectView::<MenuFn>::new()
        .h_align(HAlign::Center)
        .item("Source", source_menu as MenuFn)
        .item("Hostname", hostname_menu as MenuFn)
        .item("Locale", locale_menu as MenuFn)
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

// common shortcuts for all submenus
fn wrap_with_shortcuts<T: View>(f: T) -> OnEventView<T> {
    return OnEventView::new(f).on_event(Key::Esc, |s| {
        s.pop_layer();
    }); // \1xb is the escape key
}

fn source_menu(s: &mut Cursive) {
    let mut group: RadioGroup<&bool> = RadioGroup::new();

    s.add_layer(wrap_with_shortcuts(
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
    ));
}

fn hostname_menu(s: &mut Cursive) {
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Set Hostname")
            .content(EditView::new().fixed_width(20).with_name("hostname_edit"))
            .button("Ok", |siv| {
                let val = siv
                    .call_on_name("hostname_edit", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap_or_default();
                siv.with_user_data(|data: &mut RootData| {
                    data.hostname = Some(val);
                });
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

fn locale_menu(s: &mut Cursive) {
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Select Locale")
            .content(ScrollView::new(
                SelectView::<String>::new()
                    .h_align(HAlign::Center)
                    .with_all(
                        get_locales()
                            .lines()
                            .map(|line| (line.to_string(), line.to_string())),
                    )
                    .on_submit(|siv, val: &String| {
                        siv.with_user_data(|data: &mut RootData| {
                            data.locale = Some(val.clone());
                        });
                        siv.pop_layer();
                    }),
            ))
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

fn timezone_menu(s: &mut Cursive) {
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Select Timezone")
            .content(ScrollView::new(
                SelectView::<String>::new()
                    .h_align(HAlign::Center)
                    .with_all(get_timezones())
                    .on_submit(|siv, val: &String| {
                        siv.with_user_data(|data: &mut RootData| {
                            data.timezone = Some(val.clone());
                        });
                        siv.pop_layer();
                    }),
            ))
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}
fn root_password_menu(s: &mut Cursive) {
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Set Root Password")
            .content(EditView::new().fixed_width(20).with_name("rootpass_edit"))
            .button("Ok", |siv| {
                let val = siv
                    .call_on_name("rootpass_edit", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap_or_default();
                siv.with_user_data(|data: &mut RootData| {
                    data.root_password = Some(val);
                });
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

// holyyyyyy shittt
fn additional_users_menu(s: &mut Cursive) {
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Add Additional User")
            .content(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(Dialog::text("Username:"))
                            .child(
                                EditView::new()
                                    .secret()
                                    .with_name("user_name")
                                    .fixed_width(20),
                            ),
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(Dialog::text("Password:"))
                            .child(
                                EditView::new()
                                    .secret()
                                    .with_name("user_pass")
                                    .fixed_width(20),
                            ),
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(Dialog::text("Sudo privileges?"))
                            .child(
                                SelectView::new()
                                    .popup()
                                    .item("Yes", true)
                                    .item("No", false)
                                    .with_name("user_sudoer")
                                    .fixed_width(10),
                            ),
                    ),
            )
            .button("Add", |siv| {
                let name = siv
                    .call_on_name("user_name", |view: &mut EditView| view.get_content())
                    .unwrap_or_default()
                    .to_string();

                let pass = siv
                    .call_on_name("user_pass", |view: &mut EditView| view.get_content())
                    .unwrap_or_default()
                    .to_string();

                let sudoer = siv
                    .call_on_name("user_sudoer", |view: &mut SelectView<bool>| {
                        view.selection().map(|arc| *arc).unwrap_or(false) // THIS IS A MESS ?!?!?!
                    })
                    .unwrap_or(false);

                if name.is_empty() || pass.is_empty() {
                    siv.add_layer(Dialog::info("Username and password cannot be empty."));
                    return;
                }

                siv.with_user_data(|data: &mut RootData| {
                    if let Some(users) = &mut data.additional_users {
                        users.push(User { name, pass, sudoer });
                    } else {
                        data.additional_users = Some(vec![User { name, pass, sudoer }]);
                    }
                });
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

// the way we handle partitions rn is very very bad, bleh, no support for dual boot, no support for custom partitioning, etc.
// this should be the first thing to be improved after getting a working prototype.
fn partition_menu(s: &mut Cursive) {
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Partitioning")
            .content(
                SelectView::<bool>::new()
                    .h_align(HAlign::Center)
                    .item("Automatic Partitioning + FS", true)
                    .item("Use current partition scheme and current FS", false)
                    .on_submit(|siv, val: &bool| {
                        siv.with_user_data(|data: &mut RootData| {
                            data.partition = Some(*val);
                        });
                        siv.pop_layer();
                    }),
            )
            .button("Ok", |siv| {
                siv.pop_layer();
            }),
    ));
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
