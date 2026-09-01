pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::List::*;

    #[test]
    fn cons_list() {
        let list = Cons(
            1,
            Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
        );

        match &list {
            Cons(v, boxed) => {
                assert_eq!(*v, 1);
                match boxed.as_ref() {
                    Cons(v2, _) => assert_eq!(*v2, 2),
                    Nil => panic!("uh oh!"),
                }
            }
            Nil => panic!("uh oh!"),
        };
    }
}
