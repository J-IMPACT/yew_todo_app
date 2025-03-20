use yew::prelude::*;
use std::rc::Rc;
use crate::redux::{Action, State};

pub struct TodoList {
    state: Option<Rc<State>>,
    _context_handle: Option<ContextHandle<Rc<State>>>,
}

impl Component for TodoList {
    type Message = Rc<State>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (state, handle) = ctx
            .link()
            .context(ctx.link().callback(|state: Rc<State>| state))
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
                    { for state.tasks.iter().map(|task| html! { <li>{ &task.text }</li> }) }
                </ul>
            }
        } else { html! {} }
    }
}