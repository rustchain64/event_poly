
# An efficent way to handle pubsub requests\

# start publisher
cargo run publisher inproc://nng/event

# start subscriber
//! cargo run subscriber inproc://nng/event