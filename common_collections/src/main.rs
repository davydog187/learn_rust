extern crate common_collections;

use common_collections::pig_latin;
use common_collections::stats;

fn main() {
    let one = vec![1, 2, 3, 4];
    let two = Vec::new();
    let three = vec![4, 3, 2, 1];
    let four = vec![1, 4, 2, 2, 2, 3, 1];

    for list in vec![&one, &two, &three, &four] {
        println!("for {:?}", list);
        println!("\tmean: {}", stats::mean(&list));
        println!("\tmedian: {}", stats::median(&list));
        println!("\tmode: {}", stats::mode(&list));
    }

    let words = [
        String::from("buttz"),
        String::from("apple"),
        String::from("fooseball"),
    ];

    for word in words.iter() {
        println!("pig latin for {} is {}", word, pig_latin::translate(&word));
    }
}
