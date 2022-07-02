#[derive(PartialEq, Clone)]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub description: String,
  pub is_done: bool,
}