use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Character {
    pub name: String,
    pub class: String,
}
