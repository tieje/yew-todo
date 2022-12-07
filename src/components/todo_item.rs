use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: String,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    html! {
        <li>
            <button>
                {props.todo.clone()}
            </button>
        </li>
    }
}
