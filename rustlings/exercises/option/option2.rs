// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    if let word = optional_word {
        println!("The word is: {:?}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // You can stack `Option<T>`'s into while let and if let
    if let integer = optional_integers_vec.pop() {
        println!("current value: {:?}", integer);
    }
}
