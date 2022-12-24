use adventure_rust::menu::menu;
use adventure_rust::map::map;
use crate::menu::Menu;
use crate::menu::MenuChoice;
use crate::map::Map;
use std::fs::{File, read_dir};
use std::ffi::OsString;
use std::io::Read;


fn main() {
    let mut main_menu: Menu = Menu::new("Adventure Rust Main Menu", "> ");

    main_menu
        .new_menu_item("New Game", MenuChoice::New)
        .new_menu_item("Help", MenuChoice::Help)
        .new_menu_item("Exit", MenuChoice::Exit);

    let mut choice = MenuChoice::Invalid;

    while choice != MenuChoice::Exit {
         choice = main_menu.show();
         match choice {
            MenuChoice::Help => show_help(),
            MenuChoice::New => new_game(),
            _ => ()
         }
    }
    println!("Thanks for playing!");
}

fn show_help() {
    println!("\nAdventure Rust v0.1 by Gibson");
    println!("This is a simple adventure game. Start a new game and then follow the menu commands. Escape the maze to win.\n");
    press_btn_continue::wait("Press any key to continue...").expect("Failed to wait for any key.");
}

fn new_game() {
    let mut choice = MenuChoice::Invalid;
    while choice != MenuChoice::Back {
        let games = get_maps();
        if games.len() < 1 {
            panic!("No map files found.")
        }
        let mut menu = Menu::new("Choose A Game", "> ");
        for (i, game) in games.iter().enumerate() {
            menu.new_menu_item(get_map_name(game.to_str().unwrap()).as_str(), MenuChoice::NewGame{index: i as u32});
        }
        menu.new_menu_item("Back", MenuChoice::Back);
        choice = menu.show();
        match choice {
            MenuChoice::NewGame { index } => start_game(games[index as usize].to_str().unwrap()),
            MenuChoice::Back => println!("back"),
            _ => ()
        }
    }
}

fn get_maps() -> Vec<OsString> {
    let mut games = vec![];
    let paths = read_dir("./maps").unwrap();
    for path in paths {
        games.push(path.unwrap().file_name());
    }
    if games.len() < 1 {
        panic!("No map files found.");
    }
    games
}

fn get_map_name(filename: &str) -> String {
    let map: Map = load_map(filename);
    map.get_name().to_string()
}

fn start_game(file: &str) {
    let mut choice = MenuChoice::Invalid;
    let map = load_map(file);
    let mut location = map.find_location(map.get_start_location());
    while location.get_id() != map.get_end_location() && choice != MenuChoice::Back {
        let description = format!("{}\n\n{}\n", location.get_name(), location.get_descritpion());
        let mut menu = Menu::new("Paths", "> ");
        menu.set_header(description.as_str());
        for (i, path) in location.get_paths().iter().enumerate() {
            menu.new_menu_item(path.get_name(), MenuChoice::Move { index: i as u32});
        }
        menu.new_menu_item("Back", MenuChoice::Back);
        match menu.show() {
            MenuChoice::Move { index } => location = map.find_location(location.get_paths()[index as usize].get_id()),
            MenuChoice::Back => choice = MenuChoice::Back,
            _ => ()
        }
    }
    if location.get_id() == map.get_end_location() {
        println!("{}\n\n{}", location.get_name(), location.get_descritpion());
        println!("\nYou won!\n");
        press_btn_continue::wait("Press any key to continue...").expect("Failed to wait for any key.");
    }
}

fn load_map(filename: &str) -> Map {
    let mut file = File::open(format!("maps/{filename}")).expect("Could not open map file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Could not read text from map file.");
    serde_json::from_str(&data).expect("JSON failed to parse")
}