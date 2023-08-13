use core::pin::pin;

use pasts::{notify, prelude::*, Executor};
use shared_cell::shared_cell;

struct Context {
    stuff: u32,
}

fn main() {
    Executor::default().block_on(async move {
        let context = Context { stuff: 12 };

        shared_cell!(context);

        let task_a = pin!(async { context.with(|context| context.stuff += 2) });
        let task_b = pin!(async { context.with(|context| context.stuff -= 1) });
        let (task_a, task_b) = (&mut task_a.fuse(), &mut task_b.fuse());
        let mut select = notify::select([task_a, task_b]);

        select.next().await;
        context.with(|context| assert_eq!(14, context.stuff));
        select.next().await;
        context.with(|context| assert_eq!(13, context.stuff));
    });
}
