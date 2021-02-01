use std::process::exit;
use std::io::{stdin, Read};

fn main() {
    loop {
        let mut c = [0];

        while let Ok(_) = stdin().read(&mut c) {
            if c == ['q' as u8] {
                exit(1);
            }
        }
    }

    exit(0);
}
