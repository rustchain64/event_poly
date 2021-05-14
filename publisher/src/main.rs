use tmq::{publish, Context, Result};

use futures::SinkExt;
use log::info;
use std::env;
use std::time::Duration;
use tokio::time::delay_for;

#[tokio::main]
async fn main() -> Result<()> {
    let mut socket = publish(&Context::new()).bind("tcp://127.0.0.1:7899")?;

    let mut i = 0;

    loop {
        i += 1;

        socket
            .send(vec!["topic", &format!("Broadcast #{}", i)])
            .await?;

        delay_for(Duration::from_secs(1)).await;
    }
}
