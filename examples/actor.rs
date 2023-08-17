use std::pin::{pin, Pin};

use async_main::{async_main, LocalSpawner, Spawn};
use futures::future;
use shared_cell::{SharedCell, TaskGroup};
use whisk::Channel;

enum Command {
    Increment(u32, Channel<u32>),
    Double(Channel<u32>),
}

struct Context {
    counter: u32,
}

struct Actor(Channel<Option<Command>>);

impl Actor {
    /// Create a new `Actor`
    fn new(spawner: &LocalSpawner) -> Self {
        let channel = Channel::new();

        spawner.spawn_local({
            let channel = channel.clone();

            async move { Self::worker(channel).await }
        });

        Self(channel)
    }

    pub async fn increment(&self, x: u32) -> u32 {
        let oneshot = Channel::new();

        self.0
            .send(Some(Command::Increment(x, oneshot.clone())))
            .await;

        oneshot.await
    }

    pub async fn double(&self) -> u32 {
        let oneshot = Channel::new();

        self.0.send(Some(Command::Double(oneshot.clone()))).await;

        oneshot.await
    }

    pub async fn shutdown(&self) {
        self.0.send(None).await;
    }

    async fn next(
        tasks: &mut TaskGroup<'_, Context>,
        channel: &mut Channel<Option<Command>>,
    ) -> Option<Command> {
        loop {
            let mut advance = pin!(tasks.advance());

            if let future::Either::Left((command, _)) =
                future::select(&mut *channel, &mut advance).await
            {
                break command;
            }
        }
    }

    /// Worker thread for this actor
    async fn worker(mut channel: Channel<Option<Command>>) {
        let mut context = Context { counter: 0 };
        let mut tasks = TaskGroup::new(&mut context);

        while let Some(command) = Self::next(&mut tasks, &mut channel).await {
            use Command::*;

            match command {
                Increment(x, oneshot) => {
                    shared_cell::spawn!(tasks, increment(x, oneshot))
                }
                Double(oneshot) => {
                    shared_cell::spawn!(tasks, double(oneshot))
                }
            }
        }

        while !tasks.is_empty() {
            tasks.advance().await;
        }

        println!("Worker task is going down!");
    }
}

async fn increment(
    mut cx: Pin<&mut SharedCell<'_, Context>>,
    x: u32,
    oneshot: Channel<u32>,
) {
    let counter = cx.as_mut().with(|cx| {
        cx.counter += x;
        cx.counter
    });

    oneshot.send(counter).await;
}

async fn double(
    mut cx: Pin<&mut SharedCell<'_, Context>>,
    oneshot: Channel<u32>,
) {
    let counter = cx.as_mut().with(|cx| {
        cx.counter *= 2;
        cx.counter
    });

    oneshot.send(counter).await;
}

#[async_main]
async fn main(spawner: LocalSpawner) {
    let actor = Actor::new(&spawner);
    let mut list = Vec::new();

    list.push(actor.increment(1).await);
    list.push(actor.double().await);
    list.push(actor.increment(2).await);
    list.push(actor.double().await);
    list.push(actor.increment(5).await);
    list.push(actor.double().await);

    assert_eq!(list, [1, 2, 4, 8, 13, 26]);

    actor.shutdown().await;

    println!("Sent shutdown event");
}
