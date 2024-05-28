use core::panic;

use okstd::prelude::*;

#[okstd::log(info)]
#[okstd::crashdump]
#[okstd::main]
async fn main() {
    let a = 0;
    let b = 1;
    let _c = b / a;
    panic!("This is a panic");
}
