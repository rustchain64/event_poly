## An efficent Event Hub to handle pubsub requests

## Start POSTGRES DB
docker-compose up

## Start publisher random event generator
## Events are generated from 1 to 3 seconds
cargo run publisher ipc://nng/event

## Start subscriber or subscribers, 
## Data is collected once every ten seconds
## Create as many "Subscribers" as you want
cargo run subscriber ipc://nng/event
cargo run subscriber ipc://nng/event
cargo run subscriber ipc://nng/event
cargo run subscriber ipc://nng/event
