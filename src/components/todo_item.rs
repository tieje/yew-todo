use yew::{prelude::*};

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: String,
    pub onclick: Callback<Vec<&str>>
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    html! {
        <li>
            <button onclick={props.onclick}>
                {props.todo.clone()}
            </button>
        </li>
    }
}
// use wasm_bindgen::{prelude::Closure, JsCast};
// use web_sys::{Event, HtmlElement};
// use yew::{prelude::*, use_effect_with_deps, use_node_ref};

// #[derive(PartialEq, Properties)]
// pub struct TodoItemProps {
//     pub todo: String,
// }

// #[function_component(TodoItem)]
// // pub fn todo_item(props: &TodoItemProps) -> Html {
// pub fn todo_item() -> Html {
//     let item_ref = use_node_ref();
//     {
//         let item_ref = item_ref.clone();
//         use_effect_with_deps(
//             |item_ref| {
//                 let li = item_ref
//                     .cast::<HtmlElement>()
//                     .expect("item_ref not attached to element");
//                 let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|_| {
//                     // web_sys::console::log_1(&"Clicked!".into())

//                 }));
//                 li.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
//                     .unwrap();
//                 move || {
//                     li.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
//                         .unwrap();
//                 }
//             },
//             item_ref,
//         );
//     }
//     //let todo_item_delete = Callback::from();
//     // html! {
//     //     <li ref={item_ref}>
//     //         <button>
//     //             {props.todo.clone()}
//     //         </button>
//     //     </li>
//     // }
//     html! {
//         <li ref={item_ref}>
//             {"Click me and watch the console log"}
//         </li>
//     }
// }
