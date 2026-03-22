use serde::Serialize;

 
 #[derive(Serialize)]
pub struct Session {
    pub access: String,
    pub last_access: String,
    pub ip: String 
}

#[derive(Serialize)]
pub struct Response{
    pub sessions: Vec<Session>
}