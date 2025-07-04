use chimera_install::{get_locales, get_timezones, Bootloader, RootData, User};
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, OnEventView, RadioGroup, SelectView};
use cursive::{Cursive, CursiveExt, View};

// generic type so rust doesnt complain :(
type MenuFn = fn(&mut Cursive);

fn main() {
    let root_data = RootData {
        source: None,
        hostname: None,
        locale: None,
        timezone: None,
        root_password: None,
        additional_users: None,
        partition: None,
        setup_bootloader: None,
        additional_repositories: None,
    };
    let mut root = Cursive::new();
    root.set_user_data(root_data);

    root.add_global_callback('q', |s| s.quit());

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
        .on_submit(|s, val| val(s));

    root.add_layer(Dialog::around(main_select).title("Chimera Linux Installer"));
    root.run();
}

// HELPERS AND REUSABLES

fn wrap_with_shortcuts<T: View>(f: T) -> OnEventView<T> {
    OnEventView::new(f).on_event(Key::Esc, |s| {
        s.pop_layer();
    })
}

fn single_select_menu<T, I, F>(s: &mut Cursive, title: &str, items: I, on_select: F)
where
    T: Clone + Send + Sync + 'static,
    I: IntoIterator<Item = (String, T)>,
    F: Fn(&mut Cursive, &T) + Send + Sync + 'static,
{
    let mut select = SelectView::<T>::new().h_align(HAlign::Center);
    for (label, value) in items {
        select.add_item(label, value);
    }
    let select = select.on_submit(move |siv: &mut Cursive, val| {
        on_select(siv, val);
        siv.pop_layer();
    });

    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title(title)
            .content(select)
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

fn titled_input(title: &str, name: &str) -> LinearLayout {
    LinearLayout::horizontal()
        .child(Dialog::text(title))
        .child(EditView::new().with_name(name).fixed_width(20))
}

fn input_dialog<F>(s: &mut Cursive, title: &str, name: String, on_ok: F)
where
    F: Fn(&mut Cursive, String) + Send + Sync + 'static,
{
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title(title)
            .content(EditView::new().fixed_width(20).with_name(name.to_owned()))
            .button("Ok", move |siv| {
                if let Some(val) = siv.call_on_name(&name, |view: &mut EditView| view.get_content())
                {
                    on_ok(siv, val.to_string());
                }
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

// MENUS

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
                siv.with_user_data(|data: &mut RootData| data.source = Some(**selected));
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

fn hostname_menu(s: &mut Cursive) {
    input_dialog(s, "Set Hostname", "hostname_edit".to_owned(), |siv, val| {
        siv.with_user_data(|data: &mut RootData| {
            data.hostname = Some(val);
        });
    });
}

fn locale_menu(s: &mut Cursive) {
    let locales = get_locales()
        .lines()
        .map(|line| (line.to_owned(), line.to_owned()))
        .collect::<Vec<_>>();
    single_select_menu(s, "Select Locale", locales, |siv, val| {
        siv.with_user_data(|data: &mut RootData| data.locale = Some(val.to_owned()));
    });
}

fn timezone_menu(s: &mut Cursive) {
    let timezones = get_timezones();
    single_select_menu(s, "Select Timezone", timezones, |siv, val| {
        siv.with_user_data(|data: &mut RootData| data.timezone = Some(val.to_owned()));
    });
}

fn root_password_menu(s: &mut Cursive) {
    input_dialog(
        s,
        "Set Root Password",
        "rootpass_edit".to_owned(),
        |siv, val| {
            siv.with_user_data(|data: &mut RootData| {
                data.root_password = Some(val);
            });
        },
    );
}

fn additional_users_menu(s: &mut Cursive) {
    let content = LinearLayout::vertical()
        .child(titled_input("Username:", "user_name"))
        .child(titled_input("Password:", "user_pass"))
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
        );

    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Add Additional User")
            .content(content)
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
                        view.selection().as_deref().copied().unwrap_or(false)
                    })
                    .unwrap_or(false);

                if name.is_empty() || pass.is_empty() {
                    siv.add_layer(Dialog::info("Username and password cannot be empty."));
                    return;
                }

                siv.with_user_data(|data: &mut RootData| {
                    data.additional_users
                        .get_or_insert_with(Vec::new)
                        .push(User { name, pass, sudoer });
                });
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

fn partition_menu(s: &mut Cursive) {
    let options = vec![
        ("Automatic Partitioning + FS".to_owned(), true),
        (
            "Use current partition scheme and current FS".to_owned(),
            false,
        ),
    ];
    single_select_menu(s, "Partitioning", options, |siv, val| {
        siv.with_user_data(|data: &mut RootData| data.partition = Some(*val));
    });
}

fn setup_bootloader_menu(s: &mut Cursive) {
    let options = vec![
        ("GRUB".to_owned(), Bootloader::Grub),
        ("rEFInd".to_owned(), Bootloader::Refind),
        ("systemd-boot".to_owned(), Bootloader::Systemd),
        ("efistub".to_owned(), Bootloader::Efistub),
        ("None".to_owned(), Bootloader::None),
    ];
    single_select_menu(s, "Setup Bootloader", options, |siv, val| {
        siv.with_user_data(|data: &mut RootData| data.setup_bootloader = Some(val.to_owned()));
    });
}

fn additional_repositories_menu(s: &mut Cursive) {
    let mut group: RadioGroup<String> = RadioGroup::new();
    s.add_layer(wrap_with_shortcuts(
        Dialog::new()
            .title("Additional Repositories")
            .content(
                LinearLayout::vertical()
                    .child(group.button("chimera-repo-user".to_owned(), "Chimera User Repo"))
                    .child(group.button(
                        "chimera-repo-user-debug".to_owned(),
                        "Chimera Debug User Repo",
                    ))
                    .child(group.button(
                        "chimera-repo-main-debug".to_owned(),
                        "Chimera Debug Main Repo",
                    )),
            )
            .button("Ok", move |siv| {
                let selected = group.selection();
                siv.with_user_data(|data: &mut RootData| {
                    data.additional_repositories
                        .get_or_insert_with(Vec::new)
                        .push(selected.to_string());
                });
                siv.pop_layer();
            })
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    ));
}

fn install_menu(s: &mut Cursive) {
    let info = s
        .with_user_data(|data: &mut RootData| {
            format!(
                "Installation started with the following data:\n\n\
                Source: {:?}\n\
                Hostname: {:?}\n\
                Locale: {:?}\n\
                Timezone: {:?}\n\
                Root Password: {:?}\n\
                Additional Users: {:?}\n\
                Partitioning: {:?}\n\
                Setup Bootloader: {:?}\n\
                Additional Repositories: {:?}",
                data.source,
                data.hostname,
                data.locale,
                data.timezone,
                data.root_password,
                data.additional_users,
                data.partition,
                data.setup_bootloader,
                data.additional_repositories
            )
        })
        .unwrap_or_else(|| "No data found.".to_owned());
    s.add_layer(Dialog::info(info));
}
