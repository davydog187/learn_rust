extern crate rand;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    const INPUT: i32 = 10;

    println!("twice {} is {}", INPUT, times_two(INPUT));

    let mut i = 3;

    while i >= 0 {
        println!("fibonacci {} is {}", i, fibonacci(i as u32));
        i -= 1
    }

    for number in 0..13 {
        println!("for fibonacci {} is {}", number, fibonacci(number as u32));
    }

    if random_stuff() < 50 {
        println!("Less than 50!");
    } else {
        println!("Greater or equal to 50");
    }
}

fn times_two(num: i32) -> i32 {
    num * 2
}

fn random_stuff() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

fn fibonacci(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        other => fibonacci(other - 1) + fibonacci(other - 2)
    }
}
