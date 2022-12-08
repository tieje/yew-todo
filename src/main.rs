mod components;
use crate::components::todo_item::TodoItem;
use std::vec;
use yew::{prelude::*, Html};

#[function_component]
fn App() -> Html {
    let temp = vec!["take out the doge", "smell the flowers", "study"];
    let todo_items = use_state_eq(|| temp);
    html! {
        // container
        <section>
            // input
            <input />
            <hr/>
            // todo items
            <ul>
                {
                    (*todo_items).clone().into_iter().map(|item| {
                        html!{<TodoItem key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
