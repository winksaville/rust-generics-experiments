mod generics;

use crate::generics::experiments as ge;

fn main() {
    let number_list = vec![32, 50, 25, 100, 65];

    let result = ge::largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q']; 

    let result = ge::largest(&char_list);
    println!("The largest char is {}", result);

    let pnt_int = ge::Point { x: 5, y: 10 };
    println!("pnt_int is x={} y={}", pnt_int.x, pnt_int.y);

}
