#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    #[should_panic(expected = "Guess must be between 1 and 100")]
    fn guess_cant_be_less_than_1() {
        Guess::new(0);
    }

    #[test]
    fn returns_a_guess() {
        assert_eq!(Guess::new(1).value, 1)
    }
}

pub struct Guess {
    value: i32,
}

pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100")
        }

        Guess { value }
    }
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
