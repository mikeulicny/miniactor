# Rust Actors

A very small actor library for Rust

## Requirements

- tokio

## Example

```rust
// Define our message
pub enum Message {
    Hello,
    SecretMsg(&'static str),
}

pub struct MyActor;

// Implement Actor trait
impl Actor for MyActor {
    type Msg = Message;

    // Just print the message for this example
    fn recv(&mut self, msg: Self::Msg) {
        match msg {
            Message::Hello => println!("Hello World from Actor!"),
            Message::SecretMsg(s) => println!("Secret: {}", s),
        }
    }
}

#[tokio::main]
async fn main() {
    // Create a handle
    let h1 = Handle::new(MyActor);

    // Send messages to the actor
    h1.send(Message::Hello);
    h1.send(Message::SecretMsg("foo"));
}
```
