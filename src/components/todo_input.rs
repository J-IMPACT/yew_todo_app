use yew::prelude::*;
use crate::redux::{Action, State};

#[function_component(TodoInput)]
pub fn todo_input() -> Html {
    let state = use_context::<UseReducerHandle<State>>().expect("No context found");
    
    let input_ref = use_node_ref();

    let on_submit = {
        let state = state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    state.dispatch(Action::Add(value.clone()));
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input type="text" ref={input_ref} placeholder="Add a task..." />
            <button type="submit">{ "Add" }</button>
        </form>
    }
}