mod controllers;

use controllers::Menu;

fn main() {
    let menu = Menu::new();
    menu.menu_section();
}
