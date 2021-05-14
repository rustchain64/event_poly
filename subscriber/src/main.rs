use futures::StreamExt;

use tmq::{subscribe, Context, Result};

use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let mut socket = subscribe(&Context::new())
        .connect("tcp://127.0.0.1:7899")?
        .subscribe(b"topic")?;

    while let Some(msg) = socket.next().await {
        println!(
            "Subscribe: {:?}",
            msg?.iter()
                .map(|item| item.as_str().unwrap_or("invalid text"))
                .collect::<Vec<&str>>()
        );
    }
    Ok(())
}
