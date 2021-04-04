#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub token: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserIdResponse {
    pub id: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProdileResponse {
    pub name: String
}