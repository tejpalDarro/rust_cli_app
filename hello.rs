fn main() {
    let a = "Saturday";
    println!("{}", a);
    match a {
        "Sunday" => print!("0"),
        "Monday" => print!("1"),
        "Tuesday" => print!("2"),
        "Wednesday" => print!("3"),
        "Thursday" => print!("4"),
        "Friday" => print!("5"),
        "Saturday" => print!("6"),
        _=> print!("default"),
    }
}
