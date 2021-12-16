extern crate user_input_with_autocomplete;


use user_input_with_autocomplete::input::UserInput;


fn suggestion(input: String) -> String {
    let main_word = String::from("hello world!");
    let last_word = input.split_whitespace().last().unwrap();
    if main_word.starts_with(last_word) {
        main_word.replace(last_word, "")
    } else {
        String::from("")
    }
}

fn main() {
    

    let input2 = UserInput::new("Enter any word: ".to_owned(), Some(suggestion)).to_string();

    println!("{}", input2);

}