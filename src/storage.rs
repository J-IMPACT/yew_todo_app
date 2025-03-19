use web_sys::window;
use serde_json::{to_string, from_str};
use crate::reduce::Task;

pub fn save_to_storage(tasks: &Vec<Task>) {
    if let Some(storage) = window().unwrap().local_storage().unwrap() {
        let json = to_string(tasks).unwrap();
        storage.set_item("tasks", &json).unwrap();
    }
}

pub fn load_from_storage() -> Vec<Task> {
    if let Some(storage) = window().unwrap().local_storage().unwrap() {
        if let Ok(Some(json)) = storage.get_item("tasks") {
            return from_str(&json).unwrap_or_default();
        }
    }
    Vec::new()
}