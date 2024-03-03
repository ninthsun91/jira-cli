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
    let nav = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = nav.get_current_page();
        if current_page.is_none() {
            break;
        }

        // 2. render page
        let render_result = current_page.unwrap().draw_page().unwrap();

        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}