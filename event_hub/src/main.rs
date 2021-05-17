//! Pub/Sub (topics & broadcast) example.
//!
//! This pattern is used to allow a single broadcaster to publish messages to many subscribers,
//! which may choose to limit which messages they receive.
//! cargo run publisher inproc://nng/event
//! cargo run subscriber inproc://nng/event
//!
//! cargo run publisher ipc://nng/event
//! cargo run subscriber ipc://nng/event
use db_connect;
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
};

/// Entry point of the application.
fn main() -> Result<(), nng::Error> {
    // Begin by parsing the arguments to determine whether this is the
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
// resource https://nng.nanomsg.org/man/v1.2.2/nng_inproc.7
//inproc://, followed by an arbitrary string of text, terminated by a NUL byte
//cargo run --example pubsub publisher inproc://nng/event
//cargo run --example pubsub subscriber inproc://nng/listen
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

        // Load the number of subscribers and send the value across
        let data = count.load(Ordering::Relaxed) as u64;
        println!("PUBLISHER: SENDING {}", data);
        s.send(data.to_le_bytes())?;
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
        let msg = s.recv()?;
        let subs = usize::from_le_bytes(msg[..].try_into().unwrap());
        println!("SUBSCRIBER: THERE ARE {} SUBSCRIBERS", subs);
    }
}
