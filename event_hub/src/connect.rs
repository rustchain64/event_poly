use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Event {
    id: i32,
    data: String,
}

// #[async_std::main]
#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    //1) create connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:rootpassword@localhost:5432")
        .await?;
    //2) create a table if none exists
    sqlx::query(
        r#"
   CREATE TABLE IF NOT EXISTS event (
       id bigserial,
       event_data text
   );"#,
    )
    .fetch_one(&pool)
    .await?;

    // 3) Insert a new ticket
    let row: (i64,) = sqlx::query_as("insert into event (event_data) values ($1) returning id")
        .bind("mint_coin_event")
        .fetch_one(&pool)
        .await?;

    //4) Select all tickets
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM event").fetch_all(&pool).await?;
    let str_result = rows
        .iter()
        .map(|r| {
            format!(
                "{} - {}",
                r.get::<i64, _>("id"),
                r.get::<String, _>("event_data")
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    println!("\n== select events with PgRows:\n{}", str_result);

    //5) build manually with struct
    let select_query: sqlx::query::Query<'_, sqlx::Postgres, _> =
        sqlx::query("SELECT id, event_data FROM event");
    let events: Vec<Event> = select_query
        .map(|row: PgRow| Event {
            id: row.get("account_id"),
            data: row.get("event_data"),
        })
        .fetch_all(&pool)
        .await?;
    println!("\n== select events with query.map...::\n{:?}", events);

    //6) Select query_as ( From Row)
    let select_query = sqlx::query_as::<'_, Event>("SELECT id, event_data FROM event");
    let events: Vec<Event> = select_query.fetch_all(&pool).await?;

    Ok(())
}
