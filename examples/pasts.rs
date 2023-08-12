use core::{cell::Cell, pin::pin};

use pasts::{notify, prelude::*, Executor};
use shared_cell::CellExt;

struct Context {
    stuff: u32,
}

fn main() {
    // A `Cell` is enough to share data between async tasks on the same thread
    let context = Cell::new(Context { stuff: 12 });

    Executor::default().block_on(async move {
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
