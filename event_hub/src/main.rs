//! Pub/Sub (topics & broadcast).
//!
//! This pattern is used to allow a single broadcaster to publish messages to many subscribers,
//! which may choose to limit which messages they receive.
//! cargo run publisher inproc://nng/event
//! cargo run subscriber inproc://nng/event
//!
//! cargo run publisher ipc://nng
//! cargo run subscriber ipc://nng
mod connect_fetch_all_events;
mod connect_insert;
//use std::{convert::TryInto, env, process, time::SystemTime};
use rand::prelude::*;

use nng::{
    options::{protocol::pubsub::Subscribe, Options},
    PipeEvent, Protocol, Socket,
};
use std::{
    convert::TryInto,
    env, process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
    time::SystemTime,
};

use chrono::offset::Utc;
use chrono::DateTime;
//use std::time::SystemTime;

use serde::{Deserialize, Serialize};
// mandatory lines to use json in rust
#[derive(Debug, Deserialize, Serialize)]
struct MintCoins {
    time: String,
    amount: f32,
    account_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct TransferCoins {
    time: String,
    amount: f32,
    account_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct BurnCoins {
    time: String,
    amount: f32,
    account_id: i32,
}

/// Entry point of the application.
fn main() -> Result<(), nng::Error> {
    // subscriber or the publisher and what URL to connect with.
    let args: Vec<_> = env::args().take(3).collect();

    match &args[..] {
        [_, t, url] if t == "publisher" => publisher(url),
        [_, t, url] if t == "subscriber" => subscriber(url),
        _ => {
            println!("Usage: pubsub publisher|subscriber <url>");
            process::exit(1);
        }
    }
}

/// Run the publisher portion of the program.
fn publisher(url: &str) -> Result<(), nng::Error> {
    let s = Socket::new(Protocol::Pub0)?;
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();

    s.pipe_notify(move |_, ev| {
        match ev {
            PipeEvent::AddPost => count_clone.fetch_add(1, Ordering::Relaxed),
            PipeEvent::RemovePost => count_clone.fetch_sub(1, Ordering::Relaxed),
            _ => 0,
        };
    })?;

    s.listen(url)?;

    loop {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<u64> = (1..4).collect();
        nums.shuffle(&mut rng);
        println!(">>>> Sleep Duration: {:?}", nums[1]);
        // Sleep before sending the next message.
        thread::sleep(Duration::from_secs(nums[1]));

        let connect_psdb = connect_insert::main();
        // returns the id
        println!("PUBLISH INSERT EVENT RESPONSE ::  OK? {:?}", connect_psdb);

        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        if nums[1] == 1 {
            let mint_coin = MintCoins {
                time: datetime.format("%d/%m/%Y %T").to_string(),
                amount: 11.00,
                account_id: 1234,
            };
            let serialized = serde_json::to_string(&mint_coin).unwrap();
            println!("MESSAGE >> {:?}", serialized);
            // call DB
            //s.send(serialized.as_bytes())?;

            let data = count.load(Ordering::Relaxed) as u64;
            println!("PUBLISHER: SENDING {}", data);
            s.send(data.to_le_bytes())?;
        }

        if nums[1] == 2 {
            let transfer_coin = TransferCoins {
                time: datetime.format("%d/%m/%Y %T").to_string(),
                amount: 11.00,
                account_id: 1234,
            };
            let serialized = serde_json::to_string(&transfer_coin).unwrap();
            println!("MESSAGE >> {:?}", serialized);
            // call DB
            //s.send(serialized.as_bytes())?;

            let data = count.load(Ordering::Relaxed) as u64;
            println!("PUBLISHER: SENDING {}", data);
            s.send(data.to_le_bytes())?;
        }

        if nums[1] == 3 {
            let burn_coin = BurnCoins {
                time: datetime.format("%d/%m/%Y %T").to_string(),
                amount: 11.00,
                account_id: 1234,
            };

            let serialized = serde_json::to_string(&burn_coin).unwrap();
            println!("MESSAGE >> {:?}", serialized);
            // call DB
            //s.send(serialized.as_bytes())?;

            let data = count.load(Ordering::Relaxed) as u64;
            println!("PUBLISHER: SENDING {}", data);
            s.send(data.to_le_bytes())?;
        }
    }
}

/// Run the subscriber portion of the program.
fn subscriber(url: &str) -> Result<(), nng::Error> {
    let s = Socket::new(Protocol::Sub0)?;
    s.dial(url)?;

    println!("SUBSCRIBER: SUBSCRIBING TO ALL TOPICS");
    let all_topics = vec![];
    s.set_opt::<Subscribe>(all_topics)?;

    loop {
        // Sleep for a little bit before sending the next message.
        thread::sleep(Duration::from_secs(10));

        let connect_psdb = connect_fetch_all_events::main();
        // returns the id
        println!("PUBLISH INSERT EVENT RESPONSE ::  OK? {:?}", connect_psdb);
        // query database
        let msg = s.recv()?;

        let subs = usize::from_le_bytes(msg[..].try_into().unwrap());
        println!("SUBSCRIBER: THERE ARE {} SUBSCRIBERS", subs);

        // // Stick the Flag Message on the event data
        // msg.push_front(b"Message, ");
        // let subs: Subscribe = serde_json::from_slice<'a, Subscribe>(&msg).unwrap();
        // //let subs = usize::nng_msg_header_len(msg);
        // println!("SUBSCRIBERS EVENT DATA {:?} ", subs);

        //s.send(msg)?;
        //Ok(())
    }
}
