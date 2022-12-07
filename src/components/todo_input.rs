use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct todo_input_props {
}

#[function_component(TodoInput)]
fn todo_input() -> Html {
    html! {
        <input />
    }
}
