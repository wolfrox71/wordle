use colored::*;
use std::io;
use std::collections::HashMap;
use std::fs;
use rand::Rng;
use crossterm_cursor::cursor;

mod letter;
// the game instance
pub struct Game {
    words: Vec<String>,
    pub word: String,
    pub found: bool,
    pub guesses: u8,
    pub max_guesses: u8,
}

impl Game {
    pub fn new(_filename: &str, max_guesses: u8) -> Game{
        let words = fs::read_to_string(_filename).expect(format!("Unable to read file {}", _filename).as_str());
        let _words: Vec<String> = words.lines().map(|s| s.to_string()).collect();
        Game {
            words: _words,
            word: Game::get_word(words.lines().collect()),
            found: false,
            guesses: 0,
            max_guesses,
        }
    }
    fn get_word(words: Vec<&str>) -> String {
        let index: usize = rand::thread_rng().gen_range(0..words.len());
        let current_word = words[index];
        if current_word.chars().count() != 5 {
            panic!("The word must be 5 characters long");
        }
        current_word.to_string()
    }

    fn colour_output(&mut self, guess: String) {
        let mut characters_in_actual: HashMap<char, i8> = HashMap::new();

        // loop through each character in the word to guess
        for character in self.word.chars() {
            // add the character to the hashmap of characters if it is not already there
            let count = characters_in_actual.entry(character).or_insert(0);
            // increment the count of occurences of the character by 1
            *count +=1 ;
        }
        let mut correct_characters: u8 = 0;
        // iterate through each character in the users guess

        let mut chars_to_output = [letter::Letter::empty(), letter::Letter::empty(), letter::Letter::empty(), letter::Letter::empty(), letter::Letter::empty()];
        /*
        0; not in word
        1; in correct place
        2; not in correct place
        4; placeholder colour to designate empty space, panics when printing
        */

        // iterate through the letters first for the green letters
        /*
            This is so that is you guessed a word like tests with a repeat letter and
            the actual word had a t in pos 3 (4th letter)
            it would show up as an orange 1st then a green, as it couldnt look ahead to 
            see that the letter later in the word was in the correct place (so would be coloured green or not)
        */
        for iterator in 0..guess.len() {
            let to_guess_character: char = self.word.chars().nth(iterator).unwrap();
            let guess_character: char = guess.chars().nth(iterator).unwrap();

            // if the guesses character == the character of the word to guess
            if guess_character == to_guess_character {
                // add the letter to its index position in the array of characters to output
                chars_to_output[iterator] = letter::Letter{ character: guess_character, colour: 1,};
                // decrement the hashmap of occurances of that letter remaining
                *characters_in_actual.entry(guess_character).or_insert(1) -=1;
                // increment the ammount of correct characters by 1
                correct_characters+=1;
                continue;
            }
        }
        // then itterate throught the letters again for the red and grey letters
        for iterator in 0..guess.len() {
            let guess_character: char = guess.chars().nth(iterator).unwrap();
            // if the guesses character is in the word and remaining but not in the right place
            if characters_in_actual.contains_key(&guess_character) && characters_in_actual[&guess_character] > 0 && chars_to_output[iterator].colour == 4{
                chars_to_output[iterator] = letter::Letter{ character: guess_character, colour: 2};
                // decrement the hashmaps of ocurances of that letter remaining
                *characters_in_actual.entry(guess_character).or_insert(1) -=1;
                continue;
            }
            // if the letter is not red or green
            if chars_to_output[iterator].colour == 4 {
                // it must be grey
                chars_to_output[iterator] = letter::Letter{ character: guess_character, colour: 0};
            }
        }

        for letter in chars_to_output {
            // this should only happen if the character is not overriden from the char array
            if letter.colour > 2 {
                // through an error because it is not garanteed to work
                panic!("Letter {} is given an invalid colour of {}", letter.character, letter.colour);
            }
            // print each letter in the correct colour
            print!("{}", match letter.colour {
                // if the letter is in the correct place
                1 => letter.character.to_string().green(),
                // if the letter is not in the correct place but is in the word
                2 => letter.character.to_string().red(),
                // if the letter is not in the word
                _ => letter.character.to_string().normal(),
            });
            
        }
        // if all the letters were correct
        if correct_characters == guess.len() as u8 {
            // set found to true to stop the guessing
            self.found = true;
        }
        // output a new line so that the next guess is in the correct place
        println!("");
    } 

    fn guess(&mut self) -> String {
        // specifies if the guessed word can be any 5 letter word or has to be a specificly in the word list
        let must_be_in_wordlist: bool = true;
        // increment the number of guesses the user has done
        self.guesses+=1;
        let mut guess;
        loop {
            guess = String::new();
            // get the users input and put it in guess
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            //remove the \n from the guess
            guess.pop();

            // if the entered string is not the correct number of characters
            if guess.chars().count() != self.word.chars().count() {
                // tell the user the error
                println!("Entered guess must be {} characters long", self.word.chars().count());
                // restart the loop
                continue;
            }
            if !self.words.contains(&guess) && must_be_in_wordlist{
                println!("Must enter a word from the word list");
                continue;
            }
            // if there are no issues
            // exit the loop
            break;
        }
        return guess;
    }
    pub fn round (&mut self) {
        let mut cursor = cursor();
            // let the user guess
        let guess_string = self.guess();
        cursor.move_up(1);
        self.colour_output(guess_string);
    }

    pub fn output_score(&mut self) {
        // if the user found the correct word with guesses remaining
        if self.found {
            // output the users score
            println!("{}/{}",self.guesses.to_string().green(), self.max_guesses);
        }
        // if the user didnt get it in time
        else {
            // output that they didnt
            println!("{}/{}", "n".red(), self.max_guesses);
            println!("{}", self.word);
        }
    }
    pub fn get_username() -> String {
        // get the users input and put it in guess
        let mut username = "".to_string();
        println!("Enter your username:");
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        Game::strip_newline(username.as_str()).to_string()
    }
    fn strip_newline(input: &str) -> &str {
        input
            .strip_suffix("\r\n")
            .or(input.strip_suffix("\n"))
            .unwrap_or(input)
    }
}