// Backtrace will liust out all the functions call leading up to the erroring code
// $env:RUST_BACKTRACE=1 --> PowerShell
// RUST_BACKTRACE=1 cargo run

fn main() {
    // Error Handling --> RUST_BACKTRACE
    {
        a();
    }

    // Handling Errors Gracefully
    {

    }
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!")
    }
}
