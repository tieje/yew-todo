use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: String,
    pub onclick: Callback<MouseEvent>
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let onclick = props.onclick.clone();
    html! {
        <li>
            <button onclick={move |e: MouseEvent| onclick.emit(e)}>
                {props.todo.clone()}
            </button>
        </li>
    }
}
