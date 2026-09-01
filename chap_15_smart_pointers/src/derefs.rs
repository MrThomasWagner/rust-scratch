use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hello_deref(s: &str) -> String {
    format!("Hello {s}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let x = 5;
        let y = &x;

        assert_eq!(x, *y);
    }

    #[test]
    fn testing_2() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(x, *y);
    }

    #[test]
    fn testing_3() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }

    #[test]
    fn deref_coercion() {
        let name = "Bobby";
        assert_eq!("Hello Bobby", hello_deref(name));

        let boxed_name = MyBox::new("Billy");
        assert_eq!("Hello Billy", hello_deref(&boxed_name));
    }
}
