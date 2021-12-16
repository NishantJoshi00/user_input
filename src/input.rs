use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{execute};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use colored::Colorize;





fn get_input(prompt: String) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
// Argument is a function that takes a string and returns a bool
fn with_autocomplete(prompt: String, suggestion: fn(String) -> String) -> String {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let mut input_string = String::new();
    let mut gen_suggestion: String;
    let mut show_suggestion: bool;
    let mut end_loop = false;
    execute!(stdout, Clear(ClearType::CurrentLine), Print(format!("{}", prompt.clone()))).unwrap();
    // make cursor to line
    execute!(stdout, cursor::SetCursorShape(cursor::CursorShape::Line)).unwrap();

    loop {
        show_suggestion = true;
        
        match read().unwrap() {
            // Read key pressed if not enter key, add it to input_string
            Event::Key(KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE }) => {
                input_string.push(c);
            }
            // if key pressed with shift key, capatalize it and add it to input_string
            Event::Key(KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::SHIFT }) => {
                input_string.push(c.to_ascii_uppercase());
            }
            // If enter key, break the loop
            Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
                end_loop = true;
            }
            // If backspace key, remove last character from input_string
            Event::Key(KeyEvent { code: KeyCode::Backspace, modifiers: KeyModifiers::NONE }) => {
                input_string.pop();
                execute!(stdout, cursor::MoveLeft(1)).unwrap();
            }
            // If escape key, break the loop
            Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE }) => {
                show_suggestion = false;
            }
            // if ctrl+c or ctrl+d, break the loop
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => {
                end_loop = true;
            }
            Event::Key(KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL }) => {
                end_loop = true;
            }
            // if tab is pressed
            Event::Key(KeyEvent { code: KeyCode::Tab, modifiers: KeyModifiers::NONE }) => {
                let gen_suggestion = suggestion(input_string.clone());
                input_string.extend(gen_suggestion.chars());
                execute!(stdout, cursor::MoveToColumn(input_string.len() as u16 + 1)).unwrap();
                show_suggestion = false;
            }


            _ => {}

        };
        execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveToColumn(0), Print(format!("{}{}", prompt, input_string)), cursor::MoveToColumn((input_string.len() + prompt.len()) as u16 + 1)).unwrap();
        if show_suggestion {
            gen_suggestion = suggestion(input_string.clone());
            execute!(stdout, Print(format!("{}", gen_suggestion).truecolor(100,100,100)), cursor::MoveLeft(gen_suggestion.len() as u16)).unwrap();
        }
        if end_loop {
            execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveToColumn(0)).unwrap();
            execute!(stdout, Print(format!("{}{}", prompt, input_string))).unwrap();
            break;
        }
    }

    // disable raw mode
    execute!(stdout, cursor::SetCursorShape(cursor::CursorShape::Block)).unwrap();
    disable_raw_mode().unwrap();
    print!("\n");
    input_string
}








#[derive(Clone, Debug)]
pub struct UserInput {
    /// This struct is used to get user input from terminal
    /// cross-platform, and with autocomplete
    /// To use auto complete you need to implement a function that takes a string and returns a string
    /// This function is passed as an argument to the constructor of this struct, None if you don't want to use auto complete
    content: String
}

impl UserInput {
    /// Constructor of UserInput
    /// prompt is the string that will be displayed before the user input
    /// suggestion is a function that takes a string and returns a string
    /// if you don't want to use auto complete, pass None
    /// 
    /// When using auto-complete until the user press enter, the terminal will be in raw mode
    /// and the cursor will be in line mode
    /// Press `ctrl+c` to exit the raw mode and the cursor will be in block mode
    /// Press `ctrl+d` to exit the raw mode and the cursor will be in block mode
    /// Press `escape` cancel auto-suggestion for the time being
    pub fn new(prompt: String, suggestion: Option<fn(String) -> String>) -> UserInput {

        
        match suggestion {
            Some(s) => UserInput {
                content: with_autocomplete(prompt, s)
            },
            None => UserInput {
                content: get_input(prompt)
            }
        }    
    }

    /// Returns the content of the user input as `String`
    pub fn to_string(&self) -> String {
        self.content.clone()
    }

    /// Parses the content of the user input as T: FromStr and returns it
    /// If the parsing fails, it returns T::Err
    pub fn parse<T>(&self) -> Result<T, T::Err>
    where T: FromStr
    {
        self.content.clone().parse()
    }

    /// Similar to parse, but parses input seperated with whitespaces and return a Vec<T> where T: FromStr
    pub fn to_vec<T>(&self) -> Result<Vec<T>, T::Err>
    where T: FromStr
    {
        self.content.clone().split_whitespace().map(|s| s.parse()).collect()
    }
}
impl Display for UserInput {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.content.clone())
    }
}