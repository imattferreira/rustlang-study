fn main() {
    let year = 2023;
    let century = if year > 2000 { 21 } else { 20 };

    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");

        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1
    }

    // while century == 21 {

    // }

    let collection = [10, 20, 30, 40, 50];

    for element in collection {
        println!("the value is: {element}");
    }

    // countdown
    for number in (1..4).rev() {
        println!("{number}")
    }
}
