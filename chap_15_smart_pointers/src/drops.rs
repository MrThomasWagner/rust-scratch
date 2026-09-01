#[derive(Debug, PartialEq)]
pub struct CustomDropper {
    pub value: String,
}

impl Drop for CustomDropper {
    fn drop(&mut self) {
        println!("Dropped off! Value was: {}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_drop() {
        let x = 5;
        {
            let dropper = CustomDropper {
                value: String::from("IDK"),
            };
            assert_eq!(dropper.value, "IDK");
        }
        assert_eq!(x, 5);
    }
}
