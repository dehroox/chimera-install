#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod enums;
mod helpers;
mod menus;

use crate::menus::main_app;
use cursive::CursiveExt;

fn begin_installation(cursive: &mut cursive::Cursive) {
    cursive.add_layer(cursive::views::Dialog::info("Installation started!"));
}

fn main() {
    let mut app = main_app(begin_installation);

    app.run();
}
