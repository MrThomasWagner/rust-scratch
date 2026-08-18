fn add_two(a: u32) -> u32 {
    a + 2
}

#[cfg(test)]
pub mod tests { 
    use super::*;

    #[test]
    fn adding_two() {
        let a: u32 = 10;
        assert_eq!(add_two(a), 12);
    }
}
