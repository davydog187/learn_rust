use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(5, 7);
}

struct Cacher<I, O, F>
where
    F: Fn(I) -> O,
{
    values: HashMap<I, O>,
    calculation: F,
}

impl<I, O, F> Cacher<I, O, F>
where
    I: Eq + Hash + Clone,
    O: Clone,
    F: Fn(I) -> O,
{
    fn new(calculation: F) -> Cacher<I, O, F> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: I) -> O {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.values.insert(arg, v.clone());
                v
            }
        }
    }

    fn size(&self) -> usize {
        self.values.len()
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
        assert_eq!(c.size(), 0);

        let v1 = c.value(1);

        assert_eq!(v1, 1);
        assert_eq!(c.size(), 1);

        let v2 = c.value(2);

        assert_eq!(v2, 2);
        assert_eq!(c.size(), 2);

        let mut boolean_cacher = Cacher::new(|a| a);

        assert_eq!(boolean_cacher.value(true), true);
        assert_eq!(boolean_cacher.size(), 1);
        assert_eq!(boolean_cacher.value(false), false);
        assert_eq!(boolean_cacher.size(), 2);
    }
}
