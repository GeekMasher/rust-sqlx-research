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
    // Create connection to Postgres DB
    let connection_url: String = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&connection_url).await?;

    // Input (default: "")
    let input: String = std::env::args().last().unwrap_or(String::from(""));

    // SQLx query_as macro (secure)
    let users: Vec<User> = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            input
        )
        .fetch_all(&pool)
        .await?;

    // SQLx query_as function (secure)
    let users: Vec<User> = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(&input)
        .fetch_all(&pool)
        .await?;

    // [1] SQLx query_as function (insecure)
    let users: Vec<User> = sqlx::query_as::<_, User>(
            &format!("SELECT * FROM users WHERE username = '{}'", &input)
        )
        .fetch_all(&pool)
        .await?;
    
    // [2] SQLx query_as function using format string (insecure)
    let query = format!("SELECT * FROM users WHERE username = '{}'", &input);
    let users: Vec<User> = sqlx::query_as::<_, User>(&query)
        .fetch_all(&pool)
        .await?;

    // Wide card input (secure)
    let input_wide: String = format!("%{input}%");
    let users: Vec<User> = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username LIKE $1 OR handle LIKE $2",
            input_wide, input_wide
        )
        .fetch_all(&pool)
        .await?;



    // Print all the users
    for user in users {
        println!(" >> {:?}", user);
    }

    Ok(())
}

