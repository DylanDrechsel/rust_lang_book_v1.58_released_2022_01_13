// Backtrace will liust out all the functions call leading up to the erroring code
// $env:RUST_BACKTRACE=1 --> PowerShell
// RUST_BACKTRACE=1 cargo run

// Scope 3+
use std::fs::{ self, File };
use std::io::ErrorKind;

use std::io;
use std::io::Read;

use std::net::IpAddr;

fn main() {
    // Error Handling --> RUST_BACKTRACE
    {
        a();

        fn a() {
            b();
        }
        
        fn b() {
            c(21);
        }
        
        fn c(num: i32) {
            if num == 22 {
                panic!("Don't pass in 22!")
            }
        }
    }

    // Handling Errors Gracefully
    {
        // The “Option” enum represents some value || no value
        // While the “Result” enum represents success || failure
        // Just like the "Option" enum "Result" brought in to scope by defult
        enum Result<T, E> {
            Ok(T),
            Err(E)
        }
    }

    {
        // Type = Result --> f: Result<File, Error>
        let f = File::open("hello.txt");

        let f = match f {
            Ok(File) => File,
            // If "File" DOES NOT exist --> Create the file
            Err(error) => match error.kind() {
                // If File Creation FAILS --> Handle Error
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e)
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            }
        };
    }

    {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");

        // let f = match f {
        //     Ok(file) => file,
        //     Err(error) => panic!("Problem opening the file: {:?}", error)
        // };
    }

    {
        let home: IpAddr = "127.0.0.1".parse().unwrap();
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string("hello.txt")
}


