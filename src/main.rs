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
    // TODO: create database and navigator
    let db = Rc::new(JiraDatabase::new(DB_FILE_PATH.to_string()));
    let mut nav = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = nav.get_current_page();
        if current_page.is_none() {
            break;
        }

        // 2. render page
        let page = current_page.unwrap();
        page.draw_page().expect("Page render fail");

        // 3. get user input
        let user_input = get_user_input();

        // 4. pass input to page's input handler
        let action = page.handle_input(&user_input).expect("User input fail");

        // 5. if the page's input handler returns an action let the navigator process the action
        if let Some(action) = action {
            nav.handle_action(action).expect("Navigator failed to handle action");
        }

        break;
    }
}