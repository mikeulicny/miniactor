#![warn(missing_docs)]

//! Simple low level actor library for creating and implementing actors.
//! Requires the tokio::async runtime
//!
//! [`Actor Model`]: https://grokipedia.com/page/Actor_model
//!
//! Example
//! ```rust
//! // Define our message
//! pub enum Message {
//!     Hello,
//!     SecretMsg(&'static str),
//! }
//!
//! pub struct MyActor;
//!
//! // Implement Actor trait
//! impl Actor for MyActor {
//!     type Msg = Message;
//!
//!     // Just print the message for this example
//!     fn recv(&mut self, msg: Self::Msg) {
//!         match msg {
//!             Message::Hello => println!("Hello World from Actor!"),
//!             Message::SecretMsg(s) => println!("Secret: {}", s),
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a handle
//!     let h1 = Handle::new(MyActor);
//!
//!     // Send messages to the actor
//!     h1.send(Message::Hello);
//!     h1.send(Message::SecretMsg("foo"));
//! }
//! ```

use tokio::sync::mpsc;

/// Actor trait implements the message type and receiver function
pub trait Actor: Send {
    /// The user defined type of message that the Actor can accept
    type Msg;

    /// recv is called on the [`Actor`] every time a message is received.
    fn recv(&mut self, msg: Self::Msg);
}

/// Handle provides an interface for sending messages to the [`Actor`].
/// The [`Handle`] can be cloned and passed around.
/// The handle holds the lifetime of the [`Actor`] and when the _last_ handle is dropped the Actor will stop.
pub struct Handle<M>(mpsc::UnboundedSender<M>);

impl<M> Handle<M> {
    /// Generates an [`Actor`] and returns a [`Handle`] for that [`Actor`].
    pub fn new<T: Actor + 'static>(actor: T) -> Handle<M>
    where
        <T as Actor>::Msg: Send,
        T: Actor<Msg = M>,
    {
        let (sender, receiver) = mpsc::unbounded_channel::<T::Msg>();
        tokio::spawn(run_actor(receiver, actor));
        Handle(sender)
    }

    /// Send a message to the [`Actor`].
    pub fn send(&self, msg: M) {
        let _ = self.0.send(msg);
    }
}

impl<M> Clone for Handle<M> {
    fn clone(&self) -> Self {
        Handle(self.0.clone())
    }
}

async fn run_actor<T: Actor>(mut receiver: mpsc::UnboundedReceiver<T::Msg>, mut actor: T) {
    while let Some(msg) = receiver.recv().await {
        actor.recv(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub enum Message {
        Test,
    }

    pub struct TestActor;

    impl Actor for TestActor {
        type Msg = Message;
        fn recv(&mut self, msg: Self::Msg) {
            match msg {
                Message::Test => println!("Recieved message"),
            }
        }
    }
    #[tokio::test]
    async fn test_clone() {
        let h1 = Handle::new(TestActor);
        let h2 = h1.clone();
        h1.send(Message::Test);
        h2.send(Message::Test);
    }
}
