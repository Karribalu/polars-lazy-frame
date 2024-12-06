use polars::prelude::*;
use tokio_postgres::Client;
use crate::models::User;

// Initialize the database
pub async fn initialize_db(client: &Client) -> Result<(), tokio_postgres::Error> {
    // Create the table
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name VARCHAR(100),
                age INT
            )
            "#,
            &[],
        )
        .await?;

    // Insert dummy data
    client
        .execute(
            r#"
            INSERT INTO users (name, age)
            VALUES
            ('Alice', 30),
            ('Bob', 25),
            ('Charlie', 35)
            ON CONFLICT DO NOTHING
            "#,
            &[],
        )
        .await?;

    Ok(())
}

// Fetch users and convert to LazyFrame
pub async fn fetch_users_as_lazyframe(client: &Arc<Client>) -> Result<(LazyFrame, Vec<User>), Box<dyn std::error::Error>> {
    let rows = client
        .query("SELECT id, name, age FROM users", &[])
        .await?;
    let users = rows
        .iter()
        .map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
        })
        .collect();
    // Extract data into vectors
    let ids: Vec<i32> = rows.iter().map(|row| row.get("id")).collect();
    let names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
    let ages: Vec<i32> = rows.iter().map(|row| row.get("age")).collect();

    // Build a DataFrame and convert to LazyFrame
    let df = df! {
        "id" => &ids,
        "name" => &names,
        "age" => &ages,
    }?;

    Ok((df.lazy(), users))
}
