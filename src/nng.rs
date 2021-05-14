use nng::*;

const ADDRESS: &'static str = "inproc://nng/example";

fn request() -> Result<()> {
    // Set up the client and connect to the specified address
    let client = Socket::new(Protocol::Req0)?;
    client.dial(ADDRESS)?;

    // Send the request from the client to the server. In general, it will be
    // better to directly use a `Message` to enable zero-copy, but that doesn't
    // matter here.
    client.send("Ferris".as_bytes())?;

    // Wait for the response from the server.
    let msg = client.recv()?;
    let reply = String::from_utf8_lossy(&msg);
    assert_eq!(reply, "Hello, Ferris!");
    Ok(())
}

fn reply() -> Result<()> {
    // Set up the server and listen for connections on the specified address.
    let server = Socket::new(Protocol::Rep0)?;
    server.listen(ADDRESS)?;

    // Receive the message from the client.
    let mut msg = server.recv()?;
    let name = String::from_utf8_lossy(&msg).into_owned();
    assert_eq!(name, "Ferris");

    // Reuse the message to be more efficient.
    msg.clear();
    write!(msg, "Hello, {}!", name).unwrap();

    server.send(msg)?;
    Ok(())
}
