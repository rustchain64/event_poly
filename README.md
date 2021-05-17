## An efficent way to handle pubsub requests

## First start the docer-compose up

## start publisher
cargo run publisher inproc://nng/event

## start subscriber
cargo run subscriber inproc://nng/event