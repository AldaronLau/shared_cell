use shared_cell::shared_cell;
use tokio::join;

struct Context {
    stuff: u32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let context = Context { stuff: 12 };

    shared_cell!(context);

    let task_a = async { context.with(|context| context.stuff += 2) };
    let task_b = async { context.with(|context| context.stuff -= 1) };

    join!(task_a, task_b);
    assert_eq!(13, context.with(|context| context.stuff));
}
