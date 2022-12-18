pub mod menu {
    use crate::menu::clear_screen;
    use std::io::{stdin, stdout, Write};

    pub struct Menu {
        menu_items: Vec<MenuItem>,
        title: String,
        prompt: String
    }
    
    impl Menu {
        pub fn new(title: &str, prompt: &str) -> Menu {
            Menu {
                title: title.to_string(),
                prompt: prompt.to_string(),
                menu_items: vec![]
            }
        }

        pub fn new_menu_item(&mut self, text: &str, choice: MenuChoice) -> &mut Menu {
            let mi = MenuItem::new(text, choice);
            self.menu_items.push(mi);
            self
        }

        pub fn show(&self) -> MenuChoice {
            clear_screen();
            println!("{}\n", self.title);
            for (i, menu_item) in self.menu_items.iter().enumerate() {
                println!("{}. {}", i + 1, menu_item.get_text());
            }
            print!("\n{}", self.prompt);
            stdout().flush().expect("Failed to flush output.");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Cannot read from stdio.");
            let trimmed = line.trim();
            match trimmed.parse::<u32>() {
                Ok(choice) => return self.menu_items[(choice - 1) as usize].get_choice(),
                Err(_) => return MenuChoice::Invalid
            }
         }
    }
    
    struct MenuItem {
        text: String,
        choice: MenuChoice
    }
    
    impl MenuItem {
        pub fn new(text: &str, choice: MenuChoice) -> MenuItem {
            MenuItem {
                text: text.to_string(),
                choice
            }
        } 
    
        pub fn get_text(&self)  -> &str {
            return &self.text;
        }

        pub fn get_choice(&self) -> MenuChoice {
            return self.choice;
        }
    }
    
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum MenuChoice {
        New,
        Move {id: u32},
        Help,
        Back,
        Exit,
        Invalid
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}