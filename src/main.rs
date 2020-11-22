use std::collections::HashMap;
use std::io;

fn main() {
    //build lookup table for morse code to text
    let mut translations = HashMap::new();
    build_translations(&mut translations);

    //store user input in inp
    let mut inp = String::new();
    println!("Please enter morse code using \".\" & \"-\" or type exit to exit: ");

    io::stdin()
        .read_line(&mut inp)
        .expect("Error reading user input.");
    inp = inp.trim().to_string();

    println!();

    //main logic to chech user input and translate it
    if inp == "exit" || inp == "quit" {
        println!("Exiting");
    } else {

        let mut output_string = String::with_capacity(((inp.len() as f32)/3.2) as usize);

        // println!("Estimated String Size: {}", output_string.capacity());
        for split_str in inp.split_whitespace() {
            // println!("{}", split_str);
            output_string.push_str(translations.get(split_str).unwrap_or(&split_str));
        }
        // println!("String Size: {}", output_string.len());
        println!("{}", output_string);
    }
}

fn build_translations(translations: &mut HashMap<&str, &str>) {
    translations.insert(".-", "a");
    translations.insert("-...", "b");
    translations.insert("-.-.", "c");
    translations.insert("-..", "d");
    translations.insert(".", "e");
    translations.insert("..-.", "f");
    translations.insert("--.", "g");
    translations.insert("....", "h");
    translations.insert("..", "i");
    translations.insert(".---", "j");
    translations.insert("-.-", "k");
    translations.insert(".-..", "l");
    translations.insert("--", "m");
    translations.insert("-.", "n");
    translations.insert("---", "o");
    translations.insert(".--.", "p");
    translations.insert("--.-", "q");
    translations.insert(".-.", "r");
    translations.insert("...", "s");
    translations.insert("-", "t");
    translations.insert("..-", "u");
    translations.insert("...-", "v");
    translations.insert(".--", "w");
    translations.insert("-..-", "x");
    translations.insert("-.--", "y");
    translations.insert("--..", "z");
    translations.insert(".----", "1");
    translations.insert("..---", "2");
    translations.insert("...--", "3");
    translations.insert("....-", "4");
    translations.insert(".....", "5");
    translations.insert("-....", "6");
    translations.insert("--...", "7");
    translations.insert("---..", "8");
    translations.insert("----.", "9");
    translations.insert("-----", "0");
    translations.insert("/", " ");
    translations.insert(" ", " ");
    translations.insert("-.-.--", "!");
    translations.insert("..--..", "?");
    translations.insert("--..--", ",");
    translations.insert(".----.", "'");
    translations.insert("-....-", "-");
    translations.insert("-.--.", "(");
    translations.insert("-.--.-", ")");
    translations.insert(".-.-.-", ".");
}
