
pub mod experiments {

    pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        return largest;
    }

    #[derive(Debug)]
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }

    use std::fmt;

    impl<T, U> fmt::Display for Point<T, U>
    where
        T: fmt::Display,
        U: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{{ x={}, y={} }}", self.x, self.y)
        }
    }

}
