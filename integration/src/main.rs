use okstd::prelude::*;

#[okstd::main]
async fn main() {
    something();
}

#[okstd::log(info)]
fn something() {
    info!("Hello, world!");
    println!("Hello, world!");
}
