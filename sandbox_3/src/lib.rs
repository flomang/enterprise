#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2 
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}

pub fn add_two_with_private(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explore() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rect_can_hold_smaller() {
        let rect1 = Rectangle{
            width: 8,
            height: 7,
        };
        let rect2 = Rectangle{
            width: 5, 
            height: 1,
        };

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn rect_cannot_hold_larger() {
        let rect1 = Rectangle{
            width: 8,
            height: 7,
        };
        let rect2 = Rectangle{
            width: 5, 
            height: 1,
        };

        assert!(!rect2.can_hold(&rect1));
    }

    #[test]
    fn add_two_test() {
        let x = add_two(2);
        assert_eq!(4, x);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}