use okstd::prelude::*;

#[okstd::main]
async fn main() {
    something();
}

#[okstd::log(debug)]
fn something() {
    debug!("Hello, world!");
    println!("Hello, world!");
}
