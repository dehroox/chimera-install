#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod enums;
mod helpers;
mod menus;

use crate::menus::main_app;
use cursive::CursiveExt;

fn main() {
    let mut app = main_app();

    app.run();
}
