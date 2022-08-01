// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    match maybe_number {
        Some(x) => println!("printing: {}", x),
        None => println!("Non a number")
    }
}

fn main() {
    print_number(Option::Some(13));
    print_number(Option::None);
   
   
    let mut numbers: [Option<u16>; 5] = [Some(0); 5];
    for iter in 0..5 {
        let number_to_add: Option<u16> = {
            Some(((iter * 1235) + 2) / (4 * 16))
        };

        numbers[iter as usize] = number_to_add;
        /* let print = match numbers[iter] {
            Some(x) => x,
            None => panic!("not a number")
        };
        println!("{}", print); */
        // numbers[iter as usize] = iter as u16;
    }
}
