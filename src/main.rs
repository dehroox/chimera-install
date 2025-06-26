use chimera_install::{self, RootData};
use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, SelectView};
use cursive::align::{HAlign};

type MenuFunc = fn(root: &Cursive);

fn main() {
	let mut root = Cursive::new();
    let mut root_data = RootData {
        source: todo!(),
        hostname: todo!(),
        locale: todo!(),
        keyboard: todo!(),
        timezone: todo!(),
        root_password: todo!(),
        additional_users: todo!(),
        partition: todo!(),
        setup_bootloader: todo!(),
        additional_repositories: todo!()
    };

    root.add_global_callback('q', |s| s.quit());
    
    let main_select = SelectView::new().h_align(HAlign::Center)
        .item("Source", SOURCE_MENU)
        .item("Hostname", SOURCE_MENU)
        .item("Locale", SOURCE_MENU)
        .item("Keyboard", SOURCE_MENU)
        .item("Timezone", SOURCE_MENU)
        .item("Root Password", SOURCE_MENU)
        .item("Additional Users", SOURCE_MENU)
        .item("Partitioning", SOURCE_MENU)
        .item("Setup Bootloader", SOURCE_MENU)
        .item("Additional Repositories", SOURCE_MENU)
        .item("Install", SOURCE_MENU)
        .on_submit(|parent, val| {
            parent.pop_layer();
            val(parent);
        });
    let main_view = Dialog::around(main_select)
        .title("Chimera Linux Installer");


    root.run()
}

const SOURCE_MENU: MenuFunc = |root: &Cursive| {

};