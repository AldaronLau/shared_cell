use std::{future, pin::Pin};

use async_main::{async_main, LocalSpawner};
use shared_cell::{SharedCell, TaskGroup};

#[async_main]
async fn main(_: LocalSpawner) {
    let mut data = [1, 2, 3, 4];
    let mut task_group = TaskGroup::<'_, _>::new(&mut data);

    async fn five(mut data: Pin<&mut SharedCell<'_, [u32; 4]>>) {
        data.as_mut().with(|data| data[0] = 5);
    }

    async fn six(mut data: Pin<&mut SharedCell<'_, [u32; 4]>>) {
        future::ready(()).await;

        data.as_mut().with(|data| data[1] = 6);
    }

    shared_cell::spawn!(task_group, five());
    shared_cell::spawn!(task_group, six());

    while !task_group.is_empty() {
        task_group.advance().await;
    }

    // Release borrow on `data`
    drop(task_group);

    println!("{data:?}");

    assert_eq!([5, 6, 3, 4], data);
}
