use std::time::Duration;
use std::collections::HashMap;
use std::thread;

fn main() {
    generate_workout(5, 7);
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cacher() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
