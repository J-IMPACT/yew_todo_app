use yew::prelude::*;
use crate::redux::{Action, State};

pub struct TodoList {
    state: Option<UseReducerHandle<State>>,
    _context_handle: Option<ContextHandle<UseReducerHandle<State>>>,
}

impl Component for TodoList {
    type Message = UseReducerHandle<State>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (state, handle) = ctx
            .link()
            .context(ctx.link().callback(|state: UseReducerHandle<State>| state))
            .expect("No context found");

        Self {
            state: Some(state),
            _context_handle: Some(handle),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = Some(msg);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(state) = &self.state {
            html! {
                <ul>
                {
                    for state.tasks.iter().map(|task| {
                        let task_id = task.id;
                        let toggle_callback = {
                            let state = state.clone();
                            Callback::from(move |_| state.dispatch(Action::Toggle(task_id)))
                        };
                        let remove_callback = {
                            let state = state.clone();
                            Callback::from(move |_| state.dispatch(Action::Remove(task_id)))
                        };

                        html! {
                            <li>
                                <span style={ if task.completed { "text-decoration: line-through;" } else { "" } }>
                                    { &task.text }
                                </span>
                                <button onclick={toggle_callback}>{ if task.completed { "Restart" } else { "Finish" } }</button>
                                <button onclick={remove_callback}>{ "Remove" }</button>
                            </li> }})
                }
                </ul>
            }
        } else {
            html! { <p>{ "Loading..." }</p> }
        }
    }
}