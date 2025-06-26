use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, SelectView};
use cursive::align::{HAlign};

fn main() {
	let mut root = Cursive::new();
    root.add_global_callback('q', |s| s.quit());
    
    let main_view = Dialog::around(
        
        SelectView::new().h_align(HAlign::Center)
        .item("Source", 1)
        .item("Hostname", 2)
        .item("Locale", 3)
        .item("Keyboard", 4)
        .item("Timezone", 5)
        .item("Root Password", 6)
        .item("Additional Users", 7)
        .item("Partitioning", 8)
        .item("Setup Bootloader", 9)
        .item("Additional Repositories", 10)
        .item("Install", 11))
        
        .title("Chimera Linux Installer");

    root.add_layer(main_view);

    root.run()
}
