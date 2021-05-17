## An efficent Event Hub to handle pubsub requests

## Start POSTGRES DB
docker-compose up

## Start publisher
cargo run publisher inproc://nng/event

## Start subscriber or subscribers, you can add as many as you like
## Three is a good start, repeate the command in three different bash terminals
cargo run subscriber inproc://nng/event