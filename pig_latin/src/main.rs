/*
 * @file main.rs
 * @brief This programs converts strings to pig-latin.
 * @author Ruturaj A. Nanoti
 * @version 0.1.0
 * @date 2022-06-29
 */

use std::io::{self, Write};

fn main() {
    print!("Enter the word to be converted to pig-latin : ");
    let _ = io::stdout().flush();
    let mut string_input = String::new();

    // Take input from the user.
    io::stdin()
        .read_line(&mut string_input)
        .expect("Failed to read Input!");
    
    // Print out user input.
    print!("You chose : {}", string_input);
    
    // Get the first letter.
    let first_letter = string_input.chars().next();
    
    // Remove '\r' and '\n', which get generated while taking user input.
    string_input.pop();
    string_input.pop();

    // Look at the individial characters.
    // let characts = string_input.chars();
    // println!("{:?}", characts);
    
    if let Some(letter) = first_letter {
        // println!("The first letter is : {}", letter);
        
        // Define a vector of vowels.
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut vowel_flag = false;

        // Iterate through vowels to check if the first letter of the
        // word is a vowel.
        for v in &vowels {
            if letter == *v {
                let suffix = "hay".to_string();
                string_input = format!("{}-{}", string_input, suffix);
                vowel_flag = true;
                break;
            }
        }
        if vowel_flag == false {
            // Reverse the string
            string_input = string_input.chars().rev().collect();
            // Pop the first letter
            string_input.pop();
            // Restore the original string
            string_input = string_input.chars().rev().collect();

            string_input = format!("{}-{}{}", string_input, letter, "ay".to_string());
        }
        print!("The pig-latin version is : {}", string_input);
    }
}
