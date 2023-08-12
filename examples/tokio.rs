extern crate alloc;

use alloc::rc::Rc;
use core::cell::Cell;

use shared_cell::CellExt;
use tokio::{runtime::Builder as RuntimeBuilder, task};

struct Context {
    stuff: u32,
}

fn main() {
    let rt = RuntimeBuilder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let local = task::LocalSet::new();

    local.block_on(&rt, async {
        let context = Context { stuff: 12 };
        // Wrap in `Rc`, to control the lifetime based on the last task running.
        //
        // Could also use thread-local instead of `Rc` when targeting std.
        //
        // Or use `Box::leak()` to get a `'static` reference.
        //
        // Alternatively, `select!()` for non-`'static` futures.
        let context = Rc::new(Cell::new(context));
        let join_a = task::spawn_local({
            let context = context.clone();

            async move {
                context.with(|context| context.stuff += 2);
            }
        });
        let join_b = task::spawn_local({
            let context = context.clone();

            async move {
                context.with(|context| context.stuff -= 1);
            }
        });
        let (result_a, result_b) = tokio::join!(join_a, join_b);

        result_a.unwrap();
        result_b.unwrap();
        assert_eq!(13, context.with(|context| context.stuff));
    })
}
