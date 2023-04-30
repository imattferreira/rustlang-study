fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, _, _) = tup;
    let second = tup.1;

    println!("The value of first element is: {x}");
    println!("The value of second element is: {second}");

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];

    println!("Current month is: {first}");
}
