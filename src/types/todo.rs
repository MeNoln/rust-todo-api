#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub datecreated: String,
    pub completed: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateTodoCommand {
    pub title: String,
    pub description: String,
    pub datecreated: String,
    pub completed: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub datecreated: String,
    pub completed: bool
}