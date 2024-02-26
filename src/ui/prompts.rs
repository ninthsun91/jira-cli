use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("Epic Name:");
    let epic_name = get_user_input().trim().to_owned();
    println!("Epic Description:");
    let epic_description = get_user_input().trim().to_string();

    Epic::new(epic_name, epic_description)
}

fn create_story_prompt() -> Story {
    println!("Story Name:");
    let story_name = get_user_input().trim().to_string();
    println!("Story Description:");
    let story_description = get_user_input().trim().to_string();

    Story::new(story_name, story_description)
}

fn delete_epic_prompt() -> bool {
    todo!();
}

fn delete_story_prompt() -> bool {
    todo!();
}

fn update_status_prompt() -> Option<Status> {
    todo!();
}