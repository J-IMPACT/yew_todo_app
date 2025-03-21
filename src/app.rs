use yew::prelude::*;

use crate::components::filtered_todo_list::FilteredTodoList;
use crate::components::todo_input::TodoInput;
use crate::components::todo_list::TodoList;
use crate::redux::State;
use crate::storage::load_from_storage;

#[function_component(TodoApp)]
pub fn todo_app() -> Html {
    let state = use_reducer(|| {
        let tasks = load_from_storage();
        State { tasks }
    });

    html! {
        <ContextProvider<UseReducerHandle<State>> context={state.clone()}>
            <div class="container">
                <h1>{ "Yew To-Do List" }</h1>
                <TodoInput />
                <TodoList />
                <FilteredTodoList show_completed={false} />
                <FilteredTodoList show_completed={true} />
            </div>
        </ContextProvider<UseReducerHandle<State>>>
    }
}