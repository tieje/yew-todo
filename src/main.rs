mod components;
use crate::components::{todo_input::TodoInput, todo_item::TodoItem};
use std::vec;
use web_sys::{console, HtmlElement};
use yew::{html::onclick::Event, prelude::*, Html};


#[function_component]
fn App() -> Html {
    let initial_state = vec!["take out the doge", "smell the flowers", "study"];
    let todo_items = use_state_eq(|| initial_state);
    let delete_todo_v1: Callback<MouseEvent> = {
        let todo_items = todo_items.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                let value = target.inner_text();
                console::log_1(&format!("{:?}", value).into());
                let mut current_todos = todo_items.clone().to_vec();
                current_todos.retain(|&x| x != value);
                todo_items.set(current_todos);
            }
        })
    };
    fn delete_todo_v2(e: MouseEvent) {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let value = target.inner_text();
            console::log_1(&format!("{:?}", value).into());
            let mut current_todos = || todo_items.clone().to_vec();
            current_todos.retain(|&x| x != value);
            todo_items.set(current_todos);
        }
    }
    
    let delete_todo_v2: Callback<Event> = {
        Callback::from(delete_todo_v2)
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
                        html!{<TodoItem onclick={delete_todo_v2.clone()} key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
