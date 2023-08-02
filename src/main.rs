#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    //Any usage of Box::pin does crash on nightly newer then 2023-05-25 if -Cprefer-dynamic is set
    let exec = Box::pin(move || {
        println!("hello world");
    });
    exec();
    println!("exit");
}
