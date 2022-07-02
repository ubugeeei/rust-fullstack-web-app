use yew::prelude::*;

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}

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
                is_done: false,
            },
        ]
    });

    html! {
        <div>
            <h1>{ "Todo App !" }</h1>
            <ul>{ todos
                .iter()
                .map(|todo| {
                    html! {
                        <li>
                            <h2>{todo.title.to_string()}</h2>
                            <p>{ todo.description.to_string() }</p>
                            <p>{ todo.is_done.to_string() }</p>
                        </li>
                    }
                })
                .collect::<Html>() }</ul>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
