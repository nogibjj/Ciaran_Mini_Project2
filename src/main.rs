//create a function that determines if a word input by the user has palindromic properties
use std::io;
fn main() {
    //create a variable that will hold the user input
    let mut word = String::new();
    //prompt the user to enter a word
    println!("Enter a word to determine if it is a palindrome");
    //read the user input and store it in the word variable
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    //remove the newline character from the word
    word = word.trim().to_string();
    //create a variable that will hold the reversed word
    let mut reversed_word = String::new();
    //loop through the word backwards and add each character to the reversed_word variable
    for c in word.chars().rev() {
        reversed_word.push(c);
    }
    //compare the word and reversed_word variables to determine if they are equal
    if word == reversed_word {
        println!("{} is a palindrome", word);
    } else {
        println!("{} is not a palindrome", word);
    }
}
