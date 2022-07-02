use yew::prelude::*;
pub mod root;
use crate::root::todo::components::TodoCard::TodoCard;
use root::todo::entities::Todo;

#[function_component(App)]
fn app() -> Html {
    // TODO: fetching
    let todos: UseStateHandle<Vec<Todo>> = use_state(|| {
        vec![
            Todo {
                id: 1,
                title: "Learn Rust".to_string(),
                description: "Learn Rust".to_string(),
                is_done: false,
            },
            Todo {
                id: 2,
                title: "Learn Yew".to_string(),
                description: "Learn Yew".to_string(),
                is_done: true,
            },
            Todo {
                id: 3,
                title: "Learn Diesel".to_string(),
                description: "Learn Diesel".to_string(),
                is_done: false,
            },
        ]
    });

    html! {
        <div>
            <h1>{ "Todo App !" }</h1>
            <ul>
                {
                    todos.iter()
                    .map(|todo| html! {
                        <TodoCard todo={todo.clone()} />
                    })
                    .collect::<Html>()
                }
            </ul>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
