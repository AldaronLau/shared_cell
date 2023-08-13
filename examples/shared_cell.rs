use shared_cell::SharedCell;

struct Context {
    stuff: u32,
}

fn main() {
    let mut context = Context { stuff: 42 };
    let mut context = SharedCell::new(&mut context);

    context.with(|cx| {
        println!("Before: {}", cx.stuff);
        cx.stuff += 1;
        println!("After: {}", cx.stuff);

        // Doesn't compile
        /* context.with(|cx| {
            println!("Before: {}", cx.stuff);
            cx.stuff += 1;
            println!("After: {}", cx.stuff);
        }); */
    });

    context.with(|cx| {
        println!("Before: {}", cx.stuff);
        cx.stuff += 1;
        println!("After: {}", cx.stuff);
    });
}
