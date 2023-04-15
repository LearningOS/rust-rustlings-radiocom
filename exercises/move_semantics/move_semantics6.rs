// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&mut data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase( data: &mut String) {
    // take owner ship
    *data = data.to_string().to_uppercase();

    println!("{}", data);
}
