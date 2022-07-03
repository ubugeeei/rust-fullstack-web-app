use crate::graphql::get_todos::get_todos_query::GetTodosQueryGetTodos;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub todo: GetTodosQueryGetTodos,
}

#[function_component(TodoCard)]
pub fn todo_card(TodoCardProps { todo }: &TodoCardProps) -> Html {
    html! {
        <li>
            <h2>{ todo.title.to_string()}</h2>
            <p>{ todo.description.to_string() }</p>
            <p>{ todo.is_done.to_string() }</p>
        </li>
    }
}
