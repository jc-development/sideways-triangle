use std::io;

fn main() {
    println!("Please enter the start number:");

    let mut start_str = String::new();
    capture_val(&mut start_str, "start");

    println!("Please enter the end number:");

    let mut end_str = String::new();
    capture_val(&mut end_str, "end");

    let start: i32 = convert_to_num(start_str);
    let end: i32 = convert_to_num(end_str);

    for i in (start..=end).chain((start..end).rev()) {
        println!();
        for _ in 0..i {
            print!("#");
        }
    }
}

fn capture_val(val: &mut String, pos: &str) {
    io::stdin()
        .read_line(val)
        .expect(&format!("Failed to read {pos} number"));
}

fn convert_to_num(val: String) -> i32 {
    val.trim().parse().expect("Invalid input")
}
