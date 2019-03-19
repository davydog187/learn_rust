use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(str: &str) {
    println!("hello {}", str);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer({}) is being dropped", &self.data)
    }
}

fn main() {
    let val = MyBox::new(String::from("dave"));
    let sp1 = CustomSmartPointer {
        data: String::from("cool"),
    };
    let _sp2 = CustomSmartPointer {
        data: String::from("real cool"),
    };

    drop(sp1);


    hello(&val);
}
