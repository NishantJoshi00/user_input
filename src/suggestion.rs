use ureq;
use serde_json::Value;
use serde_json;


/// This function uses the api.datamuse.com to auto-complete the word
pub fn datamuse(input_str: String) -> String {
    // get the last word from input_str
    let last_word = input_str.split_whitespace().last().unwrap();

    let url = format!("https://api.datamuse.com/words?sp={}*&max=1", last_word);
    let res = ureq::get(&url).call().unwrap().into_string().unwrap();

    let json: Value = serde_json::from_str(&res).unwrap();
    let suggestion = json[0]["word"].as_str().unwrap().to_string();
    // remove the prefix from suggestion that is last_word
    let suggestion = suggestion.replace(last_word, "");
    suggestion
    
}