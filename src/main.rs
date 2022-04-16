extern crate colored;
use colored::*;

/*
use std::io;
use std::collections::HashMap;
use std::fs;
use rand::Rng;
use crossterm_cursor::cursor;
*/

pub mod game;
pub mod scores;

fn main() {
    println!("Run");
    let max_guesses: u8 = 8;
    let mut game = game::Game::new("words.txt", max_guesses);
    // if the user wants to play
    let want_to_play: bool = true;
    //add some code so read line and get's the users responce to wanting to play

    // if the user wants to play
    if want_to_play {
        // get the users username
        let username = game::Game::get_username();
        // output the word for debugging perposes
        //println!("Word: {}", game.word.red());
        // while the user can still guess -> guesses left and word not found
        while game.guesses < game.max_guesses && !game.found {
            // let the user guess and colour it
            game.round();
        }
        // output the score to the user
        game.output_score();
        let user_to_add: scores::User = scores::User::new(username, game.guesses.into());
        scores::Scores::add_score("scores.txt".to_string(), user_to_add);
    }
    // read the scores from the file
    scores::Scores::read("scores.txt".to_string());
}