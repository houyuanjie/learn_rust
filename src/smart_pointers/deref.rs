use std::ops::Deref;

struct Ref<T> {
    value: T,
}

impl<T> Ref<T> {
    fn new(x: T) -> Ref<T> {
        Ref { value: x }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let x = Ref::new(5);

        assert_eq!(x.value, 5);
    }

    #[test]
    fn test_deref() {
        let x = Ref::new(5);

        // type of x is Ref<i32>
        // type of x.deref() is &i32
        // typeof *(x.deref()) is i32
        // *x is a syntax sugar for *(x.deref()) when x has Deref impl
        // type of *x is i32
        assert_eq!(*x, 5);
    }
}
