mod eq;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u32) -> u32 {
    internal_adder(a, 2)
}

fn internal_adder(a: u32, b: u32) -> u32 {
    a + b
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn larger_holds_smaller() {
        let bigger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(bigger.can_hold(&smaller));
    }

    #[test]
    fn smaller_does_not_hold_bigger() {
        let bigger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(!smaller.can_hold(&bigger));
    }

    #[test]
    fn adding_two() {
        let a: u32 = 10;
        assert_eq!(add_two(a), 12);
    }

     #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
