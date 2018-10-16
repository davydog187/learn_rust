#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IPAddr::V4(127, 0, 0, 1);
    let loopback = IPAddr::V6(String::from("::1"));

    println!("My address is {:?}", home);
    println!("Loopback is {:?}", loopback);

    let _some_num = Some(5);
    let _some_string = Some("a string");
    let _none_num: Option<i32> = None;

    let x: Option<i8> = Some(5);
    let y: Option<i8> = Some(8);
}
