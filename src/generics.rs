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

}
