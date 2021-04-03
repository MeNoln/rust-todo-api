#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserIdResponse {
    pub id: i32
}