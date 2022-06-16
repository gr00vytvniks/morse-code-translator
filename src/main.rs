use std::{
    collections::HashMap,
    io::{self, BufRead},
    process,
};

fn main() -> Result<(), io::Error> {
    println!("Starting...\n- Type in text to turn into Morse Code\n");

    let morse_code_alphabet: HashMap<char, &'static str> = [
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----"),
        ('.', ".-.-.-"),
        (',', "--..--"),
        ('?', "..--.."),
        ('\'', ".----."),
        ('!', "-.-.--"),
        ('/', "-..-."),
        ('(', "-.--."),
        (')', "-.--.-"),
        ('&', ".-..."),
        (':', "---..."),
        (';', "-.-.-."),
        ('=', "-...-"),
        ('+', ".-.-."),
        ('-', "-....-"),
        ('_', "..--.-"),
        ('"', ".-..-."),
        ('$', "...-..-"),
        ('@', ".--.-."),
        (' ', " "),
    ]
    .iter()
    .cloned()
    .collect();

    // Read standard input per line
    for line_res in io::stdin().lock().lines() {
        match line_res {
            Ok(line) => {
                let mut morse = String::new();
                to_morse_code(line, &morse_code_alphabet, &mut morse)?;

                println!("Translation: {}\nExiting...", &morse);
                process::exit(0);
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

/// Turn line into morse code
pub fn to_morse_code(
    line: String,
    morse_code_alphabet: &HashMap<char, &'static str>,
    morse: &mut String,
) -> Result<(), io::Error> {
    for c in line.to_lowercase().chars() {
        if let Some(morse_code_char) = morse_code_alphabet.get(&c) {
            morse.push_str(morse_code_char);
            // Separate characters with a comma
            morse.push(',');
        } else {
            let error_msg = io::Error::new(io::ErrorKind::Other, format!("Invalid Character: {c}"));
            return Err(error_msg);
        }
    }
    Ok(())
}
