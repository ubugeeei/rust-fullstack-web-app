use root::todo::repository::TodoRepository;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod graphql;

pub mod root;
use crate::root::todo::components::todo_card::TodoCard;
use crate::root::todo::repository::get_todos_query::{self, GetTodosQueryGetTodos};

#[function_component(App)]
fn app() -> Html {
    let todos: UseStateHandle<Vec<GetTodosQueryGetTodos>> = use_state(|| vec![]);

    /*
     * create
     */
    // form state
    let new_title = use_state(|| "".to_string());
    let on_change_new_title = {
        let new_title = new_title.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_title.set(input.value());
        })
    };

    let new_description = use_state(|| "".to_string());
    let on_change_new_description = {
        let new_description = new_description.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_description.set(input.value());
        })
    };

    let create_todo = { Callback::from(move |e: Event| {}) };

    // dialog state
    let is_opened_create_dialog = use_state(|| false);
    let open_create_dialog = {
        let is_opened_create_dialog = is_opened_create_dialog.clone();
        Callback::from(move |_| {
            is_opened_create_dialog.set(true);
        })
    };
    let close_create_dialog = {
        let is_opened_create_dialog = is_opened_create_dialog.clone();
        Callback::from(move |_| {
            is_opened_create_dialog.set(false);
        })
    };

    /*
     * fetching todos
     */
    {
        let todos = todos.clone();
        use_effect_with_deps(
            move |_| {
                let todos = todos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let res = TodoRepository::get(get_todos_query::Variables {})
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

        <button type="button" onclick={open_create_dialog}>{"new"}</button>
        <dialog open={*is_opened_create_dialog}>
        <h2>{"Create New Todos!"}</h2>
            <div>
            <input
            placeholder="title"
                    label="title"
                    onchange={on_change_new_title}
                    value={(*new_title).clone()}
                />
                <input
                    placeholder="description"
                    label="description"
                    onchange={on_change_new_description}
                    value={(*new_description).clone()}
                    />
                </div>
                <button type="button" onclick={close_create_dialog}>{"cancel"}</button>
                <button type="button">{"create"}</button>
            </dialog>

            <ul>
            {
                    todos.iter()
                    .map(|todo| html! {
                    <li><TodoCard todo={todo.clone()} /></li>
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
