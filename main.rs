fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();

    // Assertions for the test cases
    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);
    assert_eq!(q1_parser("AB".to_string()), true);
    assert_eq!(q1_parser("Ab123".to_string()), false);

    rust_examples();
}

// Utility method that takes a character and returns true if digit
// or upper case letter.
fn is_uppercase_or_digit(c: char) -> bool {
    is_uppercase_letter(c) || (c >= '0' && c <= '9')
}

// Utility method that takes a character and returns true if it is
// an upper case letter.
fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn q1_parser(text: String) -> bool {
    // make String into char vector
    let characters_array: Vec<char> = text.chars().collect();

    // for each character in the character vector do...
    for (i, character) in characters_array.iter().enumerate() {
        if i < 2 && !is_uppercase_letter(*character) {
            return false;
        } else if i >= 2 && !is_uppercase_or_digit(*character) {
            return false;
        }
    }

    true
}

// Task 7 - Rust Essential Training examples
fn rust_examples() {
    // Variables and data types
    let name = "SHAHRAM";
    let age: i32 = 22;
    let score: f64 = 95.5;
    let is_student: bool = true;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Score: {}", score);
    println!("Student: {}", is_student);
}