use crate::helpers::{
    get_locales, get_timezones, input_dialog, single_select_menu, titled_input_row,
    wrap_view_with_shortcuts,
};

use crate::enums::{Bootloader, MenuCallback, PartitionType, RootData, Source, User};
use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, RadioGroup, SelectView, TextView};
use cursive::Cursive;

// prepare for the worst code of all time
// cursive makes me insaneeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee

pub fn main_app(install_callback: fn(&mut Cursive)) -> Cursive {
    let initial_root_data = RootData {
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
    let mut cursive_app = Cursive::new();
    cursive_app.set_user_data(initial_root_data);

    cursive_app.add_global_callback('q', |s| s.quit());

    let main_menu_select_view = Dialog::around(
        SelectView::<MenuCallback>::new()
            .h_align(HAlign::Center)
            .item("Source", source_menu as MenuCallback)
            .item("Hostname", hostname_menu as MenuCallback)
            .item("Locale", locale_menu as MenuCallback)
            .item("Timezone", timezone_menu as MenuCallback)
            .item("Root Password", root_password_menu as MenuCallback)
            .item("Additional Users", additional_users_menu as MenuCallback)
            .item("Partitioning", partition_menu as MenuCallback)
            .item("Setup Bootloader", setup_bootloader_menu as MenuCallback)
            .item(
                "Additional Repositories",
                additional_repositories_menu as MenuCallback,
            )
            .on_submit(|cursive, callback| callback(cursive)),
    )
    .button("Install", move |cursive| {
        // tbh idk what move does but it works, this holds true for all the moves
        cursive.pop_layer();
        let installation_summary = cursive
            .with_user_data(|root_data: &mut RootData| {
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
                    root_data.source,
                    root_data.hostname,
                    root_data.locale,
                    root_data.timezone,
                    root_data.root_password,
                    root_data.additional_users,
                    root_data.partition,
                    root_data.setup_bootloader,
                    root_data.additional_repositories
                )
            })
            .unwrap_or_else(|| "No data found.".to_owned());
        cursive.add_layer(
            Dialog::new()
                .content(TextView::new(
                    installation_summary + "\n\n Are you sure you want to proceed?",
                ))
                .title("Installation Summary")
                .button("Start Installation", move |cursive| {
                    cursive.pop_layer();
                    install_callback(cursive);
                })
                .button("Cancel", |cursive| {
                    cursive.pop_layer();
                }),
        );
    });

    cursive_app.add_layer(Dialog::around(main_menu_select_view).title("Chimera Linux Installer"));

    return cursive_app;
}

pub fn source_menu(cursive: &mut Cursive) {
    let mut source_radio_group: RadioGroup<Source> = RadioGroup::new();
    cursive.add_layer(wrap_view_with_shortcuts(
        Dialog::new()
            .title("Source of bootstrapped packages")
            .content(
                LinearLayout::vertical()
                    .child(source_radio_group.button(Source::Network, "Network"))
                    .child(source_radio_group.button(Source::Local, "Local")),
            )
            .button("Ok", move |cursive| {
                let selected_source = source_radio_group.selection();
                cursive.with_user_data(|root_data: &mut RootData| {
                    root_data.source = Some((*selected_source).to_owned());
                });
                cursive.pop_layer();
            })
            .button("Cancel", |cursive| {
                cursive.pop_layer();
            }),
    ));
}

pub fn hostname_menu(cursive: &mut Cursive) {
    input_dialog(
        cursive,
        "Set Hostname",
        "hostname_edit".to_owned(),
        |cursive, hostname_value| {
            cursive.with_user_data(|root_data: &mut RootData| {
                root_data.hostname = Some(hostname_value);
            });
        },
    );
}

pub fn locale_menu(cursive: &mut Cursive) {
    let available_locales = get_locales()
        .lines()
        .map(|line| (line.to_owned(), line.to_owned()))
        .collect::<Vec<_>>();
    single_select_menu(
        cursive,
        "Select Locale",
        available_locales,
        |cursive, selected_locale| {
            cursive.with_user_data(|root_data: &mut RootData| {
                root_data.locale = Some(selected_locale.to_owned())
            });
        },
    );
}

pub fn timezone_menu(cursive: &mut Cursive) {
    let available_timezones = get_timezones();
    single_select_menu(
        cursive,
        "Select Timezone",
        available_timezones,
        |cursive, selected_timezone| {
            cursive.with_user_data(|root_data: &mut RootData| {
                root_data.timezone = Some(selected_timezone.to_owned())
            });
        },
    );
}

pub fn root_password_menu(cursive: &mut Cursive) {
    input_dialog(
        cursive,
        "Set Root Password",
        "rootpass_edit".to_owned(),
        |cursive, root_password_value| {
            cursive.with_user_data(|root_data: &mut RootData| {
                root_data.root_password = Some(root_password_value);
            });
        },
    );
}

pub fn additional_users_menu(cursive: &mut Cursive) {
    let user_input_content = LinearLayout::vertical()
        .child(titled_input_row("Username:", "user_name"))
        .child(titled_input_row("Password:", "user_pass"))
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

    cursive.add_layer(wrap_view_with_shortcuts(
        Dialog::new()
            .title("Add Additional User")
            .content(user_input_content)
            .button("Add", |cursive| {
                let username = cursive
                    .call_on_name("user_name", |view: &mut EditView| view.get_content())
                    .unwrap_or_default()
                    .to_string();

                let password = cursive
                    .call_on_name("user_pass", |view: &mut EditView| view.get_content())
                    .unwrap_or_default()
                    .to_string();

                let is_sudoer = cursive
                    .call_on_name("user_sudoer", |view: &mut SelectView<bool>| {
                        view.selection().as_deref().copied().unwrap_or(false)
                    })
                    .unwrap_or(false);

                if username.is_empty() || password.is_empty() {
                    cursive.add_layer(Dialog::info("Username and password cannot be empty."));
                    return;
                }

                cursive.with_user_data(|root_data: &mut RootData| {
                    root_data
                        .additional_users
                        .get_or_insert_with(Vec::new)
                        .push(User {
                            name: username,
                            pass: password,
                            sudoer: is_sudoer,
                        });
                });
                cursive.pop_layer();
            })
            .button("Cancel", |cursive| {
                cursive.pop_layer();
            }),
    ));
}

pub fn partition_menu(cursive: &mut Cursive) {
    let partition_options = vec![
        (
            "Automatic Partitioning + FS".to_owned(),
            PartitionType::Auto,
        ),
        (
            "Use current partition scheme and current FS".to_owned(),
            PartitionType::Manual,
        ),
    ];
    single_select_menu(
        cursive,
        "Partitioning",
        partition_options,
        |cursive, selected_partition_type| {
            cursive.with_user_data(|root_data: &mut RootData| {
                root_data.partition = Some(*selected_partition_type)
            });
        },
    );
}

pub fn setup_bootloader_menu(cursive: &mut Cursive) {
    let bootloader_options = vec![
        ("GRUB".to_owned(), Bootloader::Grub),
        ("rEFInd".to_owned(), Bootloader::Refind),
        ("systemd-boot".to_owned(), Bootloader::Systemd),
        ("efistub".to_owned(), Bootloader::Efistub),
        ("None".to_owned(), Bootloader::None),
    ];
    single_select_menu(
        cursive,
        "Setup Bootloader",
        bootloader_options,
        |cursive, selected_bootloader| {
            cursive.with_user_data(|root_data: &mut RootData| {
                root_data.setup_bootloader = Some(selected_bootloader.to_owned())
            });
        },
    );
}

pub fn additional_repositories_menu(cursive: &mut Cursive) {
    let mut repository_radio_group: RadioGroup<String> = RadioGroup::new();
    cursive.add_layer(wrap_view_with_shortcuts(
        Dialog::new()
            .title("Additional Repositories")
            .content(
                LinearLayout::vertical()
                    .child(
                        repository_radio_group
                            .button("chimera-repo-user".to_owned(), "Chimera User Repo"),
                    )
                    .child(repository_radio_group.button(
                        "chimera-repo-user-debug".to_owned(),
                        "Chimera Debug User Repo",
                    ))
                    .child(repository_radio_group.button(
                        "chimera-repo-main-debug".to_owned(),
                        "Chimera Debug Main Repo",
                    )),
            )
            .button("Ok", move |cursive| {
                let selected_repository = repository_radio_group.selection();

                cursive.with_user_data(|root_data: &mut RootData| {
                    let additional_repositories = root_data
                        .additional_repositories
                        .get_or_insert_with(Vec::new);

                    if !additional_repositories.contains(&selected_repository) {
                        additional_repositories.push(selected_repository.to_string());
                    }
                });
                cursive.pop_layer();
            })
            .button("Cancel", |cursive| {
                cursive.pop_layer();
            }),
    ));
}
