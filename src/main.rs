use std::process::exit;
use std::io::{stdin, Read};

fn main() {
    loop {
        let mut c = [0];

        while let Ok(_) = stdin().read(&mut c) {;}
    }

    exit(1);
}
