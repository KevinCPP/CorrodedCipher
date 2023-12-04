use std::io::{self, Write};

const SPECIAL_CHARS: [char; 3] = ['\n', '\t', '\r'];

pub fn read_line() -> Option<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input).ok()?;

    Some(input)
}

pub fn remove_special_chars(input_str: Option<String>) -> Option<String> {
    let mut raw_str = input_str?;
    for ch in SPECIAL_CHARS {
        raw_str.remove_matches(ch);
    }
    Some(raw_str)
}

pub fn remove_whitespace(input_str: Option<String>) -> Option<String> {
    Some(input_str?.chars().filter(|&c| !c.is_whitespace()).collect())
}

pub fn to_i64(input_str: Option<String>) -> Option<i64> {
    let raw_str = remove_special_chars(input_str)?;
    let raw_str = remove_whitespace(Some(raw_str))?;
    raw_str.parse().ok()
}

pub fn to_i64_vec(input_str: Option<String>) -> Option<Vec<i64>> {
    // remove special characters from input string and unwrap result
    let raw_str = remove_special_chars(input_str)?;

    // tokenize the string by spaces, "10 20 30" -> ["10", "20", "30"]
    let arr = raw_str.split(' '); 
    
    // initialize vector to store final result
    let mut i64vec: Vec<i64> = Vec::new();

    // iterate through each token, and convert it to an i64 to be added to return vec.
    for token in arr {
        let token_value = match to_i64(Some(token.to_string())) {
            Some(value) => value,
            None => continue
        };
        i64vec.push(token_value);
    }
   
    Some(i64vec)
}

pub fn prompt(prompt_str: Option<String>, input_notifier: Option<String>) -> Option<String> {
    // unwrap these values or use some defaults
    let prompt_str = prompt_str.unwrap_or(String::from("Enter input:"));
    let input_notifier = input_notifier.unwrap_or(String::from("> "));
    
    // print the prompt
    println!("{}", prompt_str);
    // print the prefix to the user input line
    print!("{}", input_notifier); 
    // clear the buffer (prevents the input_notifier from being included in input?)
    // if this fails for whatever reason, simply return None
    io::stdout().flush().ok()?;
    
    read_line() 
}


#[cfg(test)]
mod tests {
    // import all the functions from the module above this one
    use super::*;
    
    #[test]
    fn remove_special_chars_test() {
        let a = String::from("hello\tworld");
        let b = String::from("\r\tt\the\n\r\r qu\r\tick\n b\t\t\rrown\n fox\n ju\t\t\rmps\n ov\r\t\n\ner\n the\n lazy\n dog\n");
        assert_eq!(remove_special_chars(Some(a)), Some(String::from("helloworld")));
        assert_eq!(remove_special_chars(Some(b)), Some(String::from("the quick brown fox jumps over the lazy dog")));
        assert_eq!(remove_special_chars(None), None);
    }
    
    #[test]
    fn to_i64_test() {
        let a = String::from("1");
        let b = String::from("\n\r\t5    ");
        let c = String::from("asdf");
        assert_eq!(to_i64(Some(a)), Some(1 as i64));
        assert_eq!(to_i64(Some(b)), Some(5 as i64));
        assert_eq!(to_i64(Some(c)), None); 
    }

    #[test]
    fn to_i64_vec_test() {
        let a = String::from("1 2 3 4 5");
        let b = String::from("10\n 20\t 30\r  ");
        let c = String::from("a, b, c, d");
        assert_eq!(to_i64_vec(Some(a)), Some(vec![1, 2, 3, 4, 5]));
        assert_eq!(to_i64_vec(Some(b)), Some(vec![10, 20, 30]));
        assert_eq!(to_i64_vec(Some(c)), Some(Vec::new()));
    }
}
