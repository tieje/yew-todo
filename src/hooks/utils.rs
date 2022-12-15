use web_sys::{MouseEvent, HtmlElement, console};
use yew::{UseStateHandle, TargetCast};

pub fn delete_todo_v4(state: UseStateHandle<Vec<&str>>, e: MouseEvent) -> () {
    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        let value = target.inner_text();
        console::log_1(&format!("{:?}", value).into());
        let mut current_todos = state.clone().to_vec();
        current_todos.retain(|&x| x != value);
        state.set(current_todos);
    }
}
