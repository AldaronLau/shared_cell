use async_main::{async_main, LocalSpawner};
use shared_cell::TaskGroup;

#[async_main]
async fn main(_: LocalSpawner) {
    let mut data = [1, 2, 3, 4];
    let mut task_group = TaskGroup::<'_, _>::new(&mut data);

    task_group.spawn(|mut data| async move {
        data.with(|data| data[0] = 5);
    });
    task_group.spawn(|mut data| async move {
        data.with(|data| data[1] = 6);
    });

    while !task_group.is_empty() {
        task_group.advance().await;
    }

    let data = task_group
        .into_shared_cell()
        .unwrap()
        .with(|data| data.clone());

    println!("{data:?}");

    assert_eq!([5, 6, 3, 4], data);
}
