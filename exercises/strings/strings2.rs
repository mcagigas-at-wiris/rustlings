// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }

    //print_type_of("patata"); // Despite is_a_color_word signature expects a &str and &word is String 
    // Rust is able "transform"  &String to &str ?    
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
 
fn patata(st: &str) {
    println!("{st}");
}