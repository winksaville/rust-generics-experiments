mod generics_experiments {

    pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        return largest;
    }

}

use crate::generics_experiments as ge;

fn main() {
    let number_list = vec![32, 50, 25, 100, 65];

    let result = ge::largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q']; 

    let result = ge::largest(&char_list);
    println!("The largest char is {}", result);
}
