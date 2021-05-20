use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;

#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:rootpassword@localhost/polydb")
        .await?;
    println!("CONNECTED TO POSTGRESS: 5432");

    // Select all tickets
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM public.events")
        .fetch_all(&pool)
        .await?;
    let str_result = rows
        .iter()
        .map(|r| {
            format!(
                "{} - {}\n",
                r.get::<i32, _>("id"),
                r.get::<String, _>("event")
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    println!("\n== All Events:\n{}", str_result);

    Ok(())
}
