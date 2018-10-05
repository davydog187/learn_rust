fn main() {
    let mut str = String::from("buttz");

    str.push_str(" more buzztzzzzzz");

    println!("Hello, world!, {}", str);

    let one = String::from("mmm, borrowing");
    let two = one.clone();

    println!("one {} two {}", one, two);

    println!("from func {}", gives_ownership());

    println!("len of {} is {}", one, calculate_len(&one));

    let mut thing = String::from("not changed");

    change(&mut thing);

    println!("we changed it! {}", thing);

    let mut str = String::from("mutable baby");
    let r1 = &str;
    let r2 = &str;
    //let r3 = &mut str;
}

fn gives_ownership() -> String {
    String::from("owned baby")
}

fn calculate_len(str: &String) -> usize {
    str.len()
}

fn change(str: &mut String) {
    str.push_str(" was CHANGED");
}
