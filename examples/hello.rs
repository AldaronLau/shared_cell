use core::cell::Cell;

use shared_cell::CellExt;

struct Context {
    stuff: u32,
}

fn main() {
    let cell = Cell::new(Context { stuff: 42 });

    cell.with(|context| {
        println!("Before: {}", context.stuff);
        context.stuff += 1;
        println!("After: {}", context.stuff);

        // Will not compile
        /* cell.with(|context| {
            context.stuff += 1;
        }); */
    });
}
