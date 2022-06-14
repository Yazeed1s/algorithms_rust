#[allow(dead_code, unused)]
fn char_to_morse(&letter: &char) -> String {
    match letter {
        'a' => ".-".to_owned(),
        'b' => "-...".to_owned(),
        'c' => "-.-.".to_owned(),
        'd' => "-..".to_owned(),
        'e' => ".".to_owned(),
        'f' => "..-.".to_owned(),
        'g' => "--.".to_owned(),
        'h' => "....".to_owned(),
        'i' => "..".to_owned(),
        'j' => ".---".to_owned(),
        'k' => "-.-".to_owned(),
        'l' => ".-..".to_owned(),
        'm' => "--".to_owned(),
        'n' => "-.".to_owned(),
        'o' => "---".to_owned(),
        'p' => ".--.".to_owned(),
        'q' => "--.-".to_owned(),
        'r' => ".-.".to_owned(),
        's' => "...".to_owned(),
        't' => "-".to_owned(),
        'u' => "..-".to_owned(),
        'v' => "...-".to_owned(),
        'w' => ".--".to_owned(),
        'x' => "-..-".to_owned(),
        'y' => "-.--".to_owned(),
        'z' => "--..".to_owned(),
        '1' => ".----".to_owned(),
        '2' => "..---".to_owned(),
        '3' => "...--".to_owned(),
        '4' => "....-".to_owned(),
        '5' => ".....".to_owned(),
        '6' => "-....".to_owned(),
        '7' => "--...".to_owned(),
        '8' => "---..".to_owned(),
        '9' => "----.".to_owned(),
        '0' => "-----".to_owned(),
        _ => panic!("Found invalid character {}", letter),
    }
}

#[allow(dead_code, unused)]
fn morse_to_char(_strn: &str) -> String {
    if _strn == ".-" {
        "a".to_owned()
    } else if _strn == "-..." {
        "b".to_owned()
    } else if _strn == "-.-." {
        "c".to_owned()
    } else if _strn == "-.." {
        "d".to_owned()
    } else if _strn == "." {
        "e".to_owned()
    } else if _strn == "..-." {
        "f".to_owned()
    } else if _strn == "--." {
        "g".to_owned()
    } else if _strn == "...." {
        "h".to_owned()
    } else if _strn == ".." {
        "i".to_owned()
    } else if _strn == ".---" {
        "j".to_owned()
    } else if _strn == "-.-" {
        "k".to_owned()
    } else if _strn == ".-.." {
        "l".to_owned()
    } else if _strn == "--" {
        "m".to_owned()
    } else if _strn == "-." {
        "n".to_owned()
    } else if _strn == "---" {
        "o".to_owned()
    } else if _strn == ".--." {
        "p".to_owned()
    } else if _strn == "--.-" {
        "q".to_owned()
    } else if _strn == ".-." {
        "r".to_owned()
    } else if _strn == "..." {
        "s".to_owned()
    } else if _strn == "-" {
        "t".to_owned()
    } else if _strn == "..-" {
        "u".to_owned()
    } else if _strn == "...-" {
        "v".to_owned()
    } else if _strn == ".--" {
        "w".to_owned()
    } else if _strn == "-..-" {
        "x".to_owned()
    } else if _strn == "-.--" {
        "y".to_owned()
    } else if _strn == "--.." {
        "z".to_owned()
    } else if _strn == ".----" {
        "1".to_owned()
    } else if _strn == "..---" {
        "2".to_owned()
    } else if _strn == "...--" {
        "3".to_owned()
    } else if _strn == "....-" {
        "4".to_owned()
    } else if _strn == "....." {
        "5".to_owned()
    } else if _strn == "-...." {
        "6".to_owned()
    } else if _strn == "--..." {
        "7".to_owned()
    } else if _strn == "---.." {
        "8".to_owned()
    } else if _strn == "----." {
        "9".to_owned()
    } else if _strn == "-----" {
        "0".to_owned()
    } else {
        panic!("Found invalid Morse code: {}", _strn)
    }
}

#[allow(dead_code, unused)]
fn encrypt(text: &str) -> String {
    return text
        .chars()
        .into_iter()
        .map(|c| char_to_morse(&c) + " ")
        .collect::<String>();
}

#[allow(dead_code, unused)]
fn decrypt(text: &str) -> String {
    let mut decrypted_text: String = String::new();
    let vec: core::str::SplitWhitespace = text.split_whitespace();
    for c in vec {
        decrypted_text += &morse_to_char(c);
    }
    decrypted_text
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let text: &str = "hello";
        let en: String = encrypt(text);
        assert_eq!(en, ".... . .-.. .-.. --- ");
        assert_eq!(decrypt(&en), text);
    }

}