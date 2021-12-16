extern crate user_input;

use user_input::input::UserInput;


fn main() {
    let input_1: i32 = UserInput::new("Enter 2: ".to_owned(), None).parse().unwrap();
    assert_eq!(input_1, 2);

    let input_2 = UserInput::new("Enter \"hello\": ".to_owned(), None).to_string();
    assert_eq!(input_2, "hello".to_owned());

    let input3: Vec<i32> = UserInput::new("Enter a numbers 4 5 6: ".to_owned(), None).to_vec().unwrap();
    assert_eq!(input3, vec![4, 5, 6]);
}