mod character;

use std::str::FromStr;

use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::character::Character;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = connect_db().await?;

    create_character_table(&db).await?;

    let character = &Character {
        name: String::from("Alex"),
        class: String::from("Fighter"),
    };
    create_character(&db, character).await?;

    let characters = list_characters(&db).await?;
    println!("{:?}", characters);

    Ok(())
}

async fn connect_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite://app.db")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

async fn create_character_table(db: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                class TEXT NOT NULL
            )
        "#,
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn create_character(db: &sqlx::SqlitePool, character: &Character) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            INSERT INTO characters (name, class) VALUES (?, ?)
        "#,
    )
    .bind(&character.name)
    .bind(&character.class)
    .execute(db)
    .await?;

    Ok(())
}

async fn list_characters(db: &sqlx::SqlitePool) -> Result<Vec<Character>, sqlx::Error> {
    sqlx::query_as::<_, Character>(
        r#"
            SELECT * FROM characters
        "#,
    )
    .fetch_all(db)
    .await
}
