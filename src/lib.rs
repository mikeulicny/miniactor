use tokio::sync::mpsc;

/// Actor
pub trait Actor: std::marker::Send {
    /// The type of message that the Actor can accept
    type Msg;
    fn recv(&mut self, msg: Self::Msg);
}

/// Handle holds the lifetime of the Actor.
pub struct Handle<M>(mpsc::UnboundedSender<M>);

impl<M> Handle<M> {
    /// `new` takes an Actor and returns a handle linked to that Actor
    pub fn new<T: Actor + 'static>(actor: T) -> Handle<M>
    where
        <T as Actor>::Msg: Send,
        T: Actor<Msg = M>,
    {
        let (sender, receiver) = mpsc::unbounded_channel::<T::Msg>();
        tokio::spawn(run_actor(receiver, actor));
        Handle(sender)
    }

    /// `send` is used
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
                Message::Test => println!("Recieved message")
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
