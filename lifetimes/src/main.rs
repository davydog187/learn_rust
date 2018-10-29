fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("thing");
    let result;

    {
        let b = String::from("weird thing");

        result = longest(a.as_str(), b.as_str());

    }
    println!("Longest is '{}'", result);
}
