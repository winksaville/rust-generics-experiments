
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
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    use std::fmt;

    impl<T> fmt::Display for Point<T> where T: fmt::Display, {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{{ x={}, y={} }}", self.x, self.y)
        }
    }

}
