use sqlx::postgres::{PgPoolOptions, PgRow};
//use sqlx::{FromRow, Row};

#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:rootpassword@localhost/polydb")
        .await?;
    println!("CONNECTED TO POSTGRESS: 5432");

    // Insert a new event
    let row: (i32,) = sqlx::query_as(
        "insert into public.events(event)
         values ($1)
         returning id",
    )
    .bind("mint_coin_event")
    .bind("v1.0")
    .fetch_one(&pool)
    .await?;
    println!("INSERTED {:?}", row);

    Ok(())
}
