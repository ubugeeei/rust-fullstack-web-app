use yew::prelude::*;

mod graphql;

pub mod root;
use crate::root::todo::components::todo_card::TodoCard;
use crate::root::todo::queries::get_todos_query::{self, GetTodosQueryGetTodos};
use root::todo::queries::perform_my_query;

#[function_component(App)]
fn app() -> Html {
    let todos: UseStateHandle<Vec<GetTodosQueryGetTodos>> = use_state(|| vec![]);
    {
        let todos = todos.clone();
        use_effect_with_deps(
            move |_| {
                let todos = todos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let res = perform_my_query(get_todos_query::Variables {})
                        .await
                        .unwrap()
                        .get_todos;
                    todos.set(res);
                });
                || ()
            },
            (),
        );
    }

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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
