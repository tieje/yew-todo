mod components;
mod hooks;
use crate::components::{todo_input::TodoInput, todo_item::TodoItem};
use crate::hooks::utils::delete_todo_v4;
use std::vec;
use web_sys::{console, HtmlElement};
use yew::{prelude::*, Html};

fn delete_todo_v2(state: UseStateHandle<Vec<&str>>, e: MouseEvent) {
    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        let value = target.inner_text();
        console::log_1(&format!("{:?}", value).into());
        let mut current_todos = state.clone().to_vec();
        current_todos.retain(|&x| x != value);
        state.set(current_todos);
    }
}

fn delete_todo_v3(state: UseStateHandle<Vec<&str>>, e: MouseEvent) -> () {
    delete_todo_v2(state.clone(), e)
}

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

    // Abstracting delete_todo_v1 further
    let delete_todo_v2: Callback<MouseEvent> = {
        let todo_items = todo_items.clone();
        Callback::from(move |e: MouseEvent| delete_todo_v2(todo_items.clone(), e))
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
                        html!{<TodoItem onclick={delete_todo_v1.clone()} key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
            <ul>
                {
                    (*todo_items).clone().into_iter().map(|item| {
                        html!{<TodoItem onclick={delete_todo_v2.clone()} key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
            // Abstracting delete_todo_v2 further
            <ul>
                {
                    (*todo_items).clone().into_iter().map(|item| {
                        let todo_items = todo_items.clone();
                        html!{<TodoItem onclick={move |e: MouseEvent| delete_todo_v3(todo_items.clone(), e)} key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
            // Import delete_todo_v4 from a mod, which combines v3 and v2
            <ul>
                {
                    (*todo_items).clone().into_iter().map(|item| {
                        let todo_items = todo_items.clone();
                        html!{<TodoItem onclick={move |e: MouseEvent| delete_todo_v4(todo_items.clone(), e)} key={item} todo={item} />}
                    }).collect::<Html>()
                }
            </ul>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
