mod app;
mod components;
mod redux;
mod storage;

fn main() {
    yew::Renderer::<app::TodoApp>::new().render();
}
