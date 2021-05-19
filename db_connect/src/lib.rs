use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

// #[async_std::main]
#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    //1) create connection
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://root:rootpassword@localhost:5432")
    //     .await?;
    //2) create a table if none exists
    //     sqlx::query(
    //         r#"
    //    CREATE TABLE IF NOT EXISTS event (
    //        id bigserial,
    //        name text
    //    );"#,
    //     )
    //     .exeute(&pool)
    //     .await?;

    // 3) Insert a new ticket
    // let row: (i64,) = sqlx::query_as("insert into event (event_data) values ($1) returning id")
    //     .bind("mint_coin_event")
    //     .fetch_one(&pool)
    //     .await?;

    //4) Select all tickets

    // // Postgres
    // sqlx::query!(
    //     r#"INSERT INTO event (event_data)
    //      VALUES (my_int);"#
    // ) // MySQL: use "select 1 as `id: MyInt4`" instead
    // .fetch_all(&pool)
    // .await?;

    // let value1: &str = "test";
    // let stream = sqlx::query(
    //     "INSERT INTO event (event_data)
    //     VALUES (&value1);",
    // )
    // .fetch_all(&pool)
    // .await?;

    //println!("Stream {:?}", stream);

    // let stream = sqlx::query("SELECT * FROM event")
    //     .map(|row: PgRow| {
    //         //println!("DATA {:?}", row)
    //         // map the row into a user-defined domain type
    //     })
    //     .fetch_all(&pool)
    //     .await?;

    //assert_eq!(row.0, 150);

    Ok(())
}
