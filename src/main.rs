mod components;
use crate::components::{todo_input::TodoInput, todo_item::TodoItem};
use std::vec;
use yew::{prelude::*, Html};

#[function_component]
fn App() -> Html {
    let initial_state = vec!["take out the doge", "smell the flowers", "study"];
    let todo_items = use_state_eq(|| initial_state);
    let delete_todo = {
        let todo_items = todo_items.clone();
        Callback::from(move |someEvent| {
            
            todo_items.set(Model { value: "" })
    })
    };
    html! {
        // container
        <section>
            // input
            <TodoInput />
            <hr/>
            // todo items
            <ul>
                {
                    (*todo_items).clone().into_iter().map(|item| {
                        html!{<TodoItem onclick={delete_todo} key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
