use std::env;
use anyhow::Result;
use sqlx::PgPool;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    handle: String
}


#[tokio::main]
async fn main() -> Result<()> {
    // Input (default: "")
    let input: String = env::args().last().unwrap_or(String::from(""));
    println!(">>> Input <<<");
    println!(" `{input}`");

    // Create DB connection
    let connection_url: String = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&connection_url).await?;

    println!(">>> Running Query <<<");

    let input_wide: String = format!("%{input}%");

    let users: Vec<User> = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username LIKE $1 OR handle LIKE $2",
            input_wide, input_wide
        )
        .fetch_all(&pool)
        .await?;

    println!("User count :: {}", users.len());
    for user in users {
        println!(" >> {:?}", user);
    }

    Ok(())
}

