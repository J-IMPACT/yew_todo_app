use yew::prelude::*;
use crate::redux::State;

#[derive(Properties, PartialEq)]
pub struct FilteredTodoListProps {
    pub show_completed: bool,
}

pub struct  FilteredTodoList {
    state: Option<UseReducerHandle<State>>,
    _context_handle: Option<ContextHandle<UseReducerHandle<State>>>,
}

impl Component for FilteredTodoList {
    type Message = UseReducerHandle<State>;
    type Properties = FilteredTodoListProps;

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

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(state) = &self.state {
            let show_completed = ctx.props().show_completed;

            html! {
                <div>
                    <h3>{ if show_completed { "Completed Tasks" } else { "Pending Tasks" } }</h3>
                    <ul>
                        {
                            for state.tasks.iter()
                                .filter(|task| task.completed == show_completed)
                                .map(|task| html! { <li>{ &task.text }</li> })
                        }
                    </ul>
                </div>
            }
        } else {
            html! { <p>{ "Loading..." }</p> }
        }
    }
}