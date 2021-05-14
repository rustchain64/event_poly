use event_listener::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::usize;

fn new() {
    let flag = Arc::new(AtomicBool::new(false));
    let event = Arc::new(Event::new());

    // Spawn a thread that will set the flag after 1 second.
    thread::spawn({
        let flag = flag.clone();
        let event = event.clone();
        move || {
            // Wait for a second.
            thread::sleep(Duration::from_secs(1));

            // Set the flag.
            flag.store(true, Ordering::SeqCst);

            // Notify all listeners that the flag has been set.
            event.notify(usize::MAX);
        }
    });

    // Wait until the flag is set.
    loop {
        // Check the flag.
        if flag.load(Ordering::SeqCst) {
            println!("Flag Loads");
            break;
        }

        // Start listening for events.
        let listener = event.listen();

        // Check the flag again after creating the listener.
        if flag.load(Ordering::SeqCst) {
            break;
        }

        // Wait for a notification and continue the loop.
        listener.wait();
    }
}
