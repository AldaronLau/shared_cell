use std::{
    future::Future,
    mem,
    thread::{self, JoinHandle},
};

use async_main::{async_main, LocalSpawner};
use futures::{
    executor,
    future::{self, Either},
};
use shared_cell::{shared_cell, SharedCell, TaskGroup};
use whisk::Channel;

/// Actor using the futures crate
struct FuturesActor<'a, T> {
    task_group: TaskGroup<'a, T>,
}

impl<'a, T> FuturesActor<'a, T> {
    pub fn new(shared_cell: &'a SharedCell<'a, T>) -> Self {
        Self {
            task_group: TaskGroup::new(shared_cell),
        }
    }

    pub fn spawn<F>(&mut self, f: impl FnOnce(&'a SharedCell<T>) -> F)
    where
        F: Future<Output = ()> + 'a,
    {
        self.task_group.spawn(|cell| Box::pin(f(cell)));
    }

    pub fn shared_cell(&self) -> &'a SharedCell<T> {
        self.task_group.shared_cell()
    }

    pub async fn next<R>(
        &mut self,
        mut channel: &mut Channel<Option<R>>,
    ) -> Option<R> {
        loop {
            let mut tasks = Vec::new();

            mem::swap(&mut tasks, self.task_group.tasks());

            if tasks.is_empty() {
                break channel.recv().await;
            } else {
                let mut select_all = future::select_all(tasks);

                match future::select(&mut channel, &mut select_all).await {
                    Either::Left((oneshot, _)) => {
                        let mut tasks = select_all.into_inner();

                        mem::swap(&mut tasks, self.task_group.tasks());

                        break oneshot;
                    }
                    Either::Right((((), _index, mut tasks), _)) => {
                        mem::swap(&mut tasks, self.task_group.tasks());
                    }
                }
            }
        }
    }
}

/// Example actor
struct Actor(Channel<Option<Channel<u32>>>);

impl Actor {
    pub fn new(counter: u32) -> (Self, JoinHandle<u32>) {
        let channel = Channel::new();
        // Start actor loop
        let join_handle = thread::spawn({
            let channel = channel.clone();

            move || executor::block_on(Self::worker(channel, counter))
        });

        (Self(channel), join_handle)
    }

    pub async fn increment(&self) -> u32 {
        let oneshot = Channel::new();

        self.0.send(Some(oneshot.clone())).await;

        oneshot.await
    }

    pub async fn shutdown(&self) {
        self.0.send(None).await;
    }

    /// Worker thread for this actor
    async fn worker(
        mut channel: Channel<Option<Channel<u32>>>,
        counter: u32,
    ) -> u32 {
        let counter = counter;

        shared_cell!(counter);

        let mut actor = FuturesActor::new(counter);

        while let Some(oneshot) = actor.next(&mut channel).await {
            // Spawn a task
            actor.spawn(|counter| async move {
                oneshot
                    .send(counter.with(|counter| {
                        *counter += 1;
                        *counter
                    }))
                    .await;
            })
        }

        actor.shared_cell().with(|counter: &mut u32| *counter)
    }
}

#[async_main]
async fn main(_spawner: LocalSpawner) {
    let (actor, join) = Actor::new(12);

    println!("Incrementing to 13");
    assert_eq!(13, actor.increment().await);
    println!("Incrementing to 14");
    assert_eq!(14, actor.increment().await);
    println!("Shutting Down");
    actor.shutdown().await;
    println!("Joining Task");
    assert_eq!(14, join.join().unwrap());
}
