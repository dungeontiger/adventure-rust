use adventure_rust::menu::menu;
use crate::menu::Menu;
use crate::menu::MenuChoice;
use std::io::{stdin, stdout, Write, Read};

fn main() {
    let mut mainMenu: Menu = Menu::new("Main Menu", "> ");
    mainMenu
        .new_menu_item("New Game", MenuChoice::New)
        .new_menu_item("Help", MenuChoice::Help)
        .new_menu_item("Exit", MenuChoice::Exit);
    let mut choice = MenuChoice::Invalid;
    while choice != MenuChoice::Exit {
         choice = mainMenu.show();
         if choice == MenuChoice::Invalid {
            println!("\nYou entered an invalid choice. Please choose again.\n");
            press_btn_continue::wait("Press any key to continue...").expect("Failed to wait for any key.");
         }
    }
}
