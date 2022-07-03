use yew::prelude::*;

use crate::root::todo::repository::get_todos_query::GetTodosQueryGetTodos;

#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub todo: GetTodosQueryGetTodos,
}

#[function_component(TodoCard)]
pub fn todo_card(TodoCardProps { todo }: &TodoCardProps) -> Html {
    html! {
        <div class="todo-card">
            <h2>{ todo.title.to_string()}</h2>
            <p>{ todo.description.to_string() }</p>
            <p>{ if todo.is_done  { "done!" } else { "yet!"}}</p>
        </div>
    }
}
