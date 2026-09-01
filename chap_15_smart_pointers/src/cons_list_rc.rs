use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::List::*;
    use std::rc::Rc;

    #[test]
    fn cons_list() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("Count of RC intially: {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("Count of RC after B created: {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("Count of RC after C created: {}", Rc::strong_count(&a));
            assert_ne!(b, c);
        }

        println!("Count of RC after C out of scope: {}", Rc::strong_count(&a));
        drop(b);
        println!("Count of RC after B dropped explicitly: {}", Rc::strong_count(&a));
    }
}
