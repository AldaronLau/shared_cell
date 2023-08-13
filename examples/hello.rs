use shared_cell::shared_cell;

struct Context {
    stuff: u32,
}

fn main() {
    let context = Context { stuff: 42 };

    shared_cell!(context);
    context.with(|cx| {
        println!("Before: {}", cx.stuff);
        cx.stuff += 1;
        println!("After: {}", cx.stuff);

        // Will not compile
        /* context.with(|cx| {
            cx.stuff += 1;
        }); */
    });
}
