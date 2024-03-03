use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

const DB_FILE_PATH: &str = "data/db.json";

fn main() {
    let db = Rc::new(JiraDatabase::new(DB_FILE_PATH.to_string()));
    let mut nav = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        let current_page = nav.get_current_page().unwrap_or_else(|| {
            println!("Page not found\nPress any key to continue...");

            wait_for_key_press();
            std::process::exit(1);
        });

        current_page.draw_page().unwrap_or_else(|err| {
            println!("Page error - {}\nPress any key to continue...", err);

            wait_for_key_press();
            std::process::exit(1);
        });

        let user_input = get_user_input();
        let action = current_page.handle_input(&user_input).unwrap_or_else(|err| {
            println!("User input error - {}\nPress any key to continue...", err);

            wait_for_key_press();
            std::process::exit(1);
        });

        if let Some(action) = action {
            nav.handle_action(action).unwrap_or_else(|err| {
                println!("Navigator error - {}\nPress any key to continue...", err);

                wait_for_key_press();
                std::process::exit(1);
            });
        }
    }
}