use std::io;
use std::io::prelude::*;
use std::time::{Instant};
use rand::Rng;


fn compare_min(a: i32, b: i32) -> i32{
    return if a <= b { a } else { b }
}

fn compare_max(a: i32, b: i32) -> i32{
    return if a >= b { a } else { b }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}


fn main()
{
    let mut rng = rand::thread_rng();
    let mut array_int = Vec::new();

    let now = Instant::now();
    for _i in 0..18000000 {
        array_int.push(1);
    }
    println!("length : {}", array_int.len());
    println!("{}", now.elapsed().as_millis());
    pause();
}
