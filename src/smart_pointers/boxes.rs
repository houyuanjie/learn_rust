#[derive(Debug, PartialEq, Eq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_stores_value_on_heap() {
        let b = Box::new(5);
        println!("b = {}", b);

        assert_eq!(*b, 5);
    }

    #[test]
    fn test_recursive_list_with_box_compiles() {
        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);

        assert_eq!(format!("{:?}", list), "Cons(1, Cons(2, Cons(3, Nil)))");
    }
}
