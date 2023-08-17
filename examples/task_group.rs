use std::future;

use async_main::{async_main, LocalSpawner};
use shared_cell::{Shared, TaskGroup};

#[async_main]
async fn main(_: LocalSpawner) {
    let mut data = [1, 2, 3, 4];
    let mut task_group = TaskGroup::new(&mut data);

    async fn five(data: &mut Shared<'_, [u32; 4]>) {
        data.with(|data| data[0] = 5);
    }

    async fn six(data: &mut Shared<'_, [u32; 4]>) {
        future::ready(()).await;

        data.with(|data| data[1] = 6);
    }

    shared_cell::spawn!(task_group, five());
    shared_cell::spawn!(task_group, six());

    // Wait for subtasks to complete, and release borrow on `data`
    task_group.finish().await;

    println!("{data:?}");

    assert_eq!([5, 6, 3, 4], data);
}
