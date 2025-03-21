use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

use crate::storage::save_to_storage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

#[derive(Clone, PartialEq)]
pub struct State {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Add(String),
    Toggle(usize),
    Remove(usize)
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            Action::Add(text) => {
                let id = new_state.tasks.len() + 1;
                new_state.tasks.push(Task { id, text, completed: false });
            }
            Action::Toggle(id) => {
                if let Some(task) = new_state.tasks.iter_mut().find(|t| t.id == id) {
                    task.completed = !task.completed;
                }
            }
            Action::Remove(id) => {
                new_state.tasks.retain(|task| task.id != id);
            }
        }

        save_to_storage(&new_state.tasks);
        new_state.into()
    }
}