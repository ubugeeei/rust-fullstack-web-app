use yew::prelude::*;

use crate::root::todo::repository::get_todos_query::GetTodosQueryGetTodos;

#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub todo: GetTodosQueryGetTodos,
    pub oncheck: Callback<GetTodosQueryGetTodos>,
}

#[function_component(TodoCard)]
pub fn todo_card(TodoCardProps { todo, oncheck }: &TodoCardProps) -> Html {
    let _oncheck = {
        let oncheck = oncheck.clone();
        let todo = todo.clone();
        Callback::from(move |_| {
            oncheck.emit(todo.clone());
        })
    };

    html! {
        <div class="todo-card" style="border: 1px solid #efefef; border-radius: 5px; padding: 12px; margin-bottom: 8px;">
            <div style="display: flex;">
                <input type="checkbox" checked={todo.is_done} onchange={_oncheck}/>
                <span>{ todo.title.to_string()}</span>
            </div>

            <hr />

            <p>
                <span>{"description: "}</span>
                {
                    if todo.description.len() == 0 {
                        "no description"
                    } else {
                        &todo.description
                    }
                }
            </p>
        </div>
    }
}
