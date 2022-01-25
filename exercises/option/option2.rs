// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if optional_word.is_some(){
        println!("The word is: {}", optional_word.unwrap());
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while optional_integers_vec.len() > 0 {
        let integer = optional_integers_vec.pop().unwrap();
        println!("current value: {}", integer.unwrap());

    }
}
