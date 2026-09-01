pub fn boxed_up(x: u32) -> Box<u32> {
    Box::new(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boxed() {
        let in_a_box = boxed_up(4);
        println!("Boxed value is: {}", in_a_box);
        assert_eq!(4, *in_a_box);
    }
}
