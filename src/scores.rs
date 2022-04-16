use std::collections::HashMap;
use std::fs;
use std::io::Write;
pub mod player;

pub struct User {
    username: String,
    score: u16,
}
impl User {
    pub fn new (username: String, score: u16) -> User { User { username, score} }
}

pub struct Scores {
    users: HashMap<String, player::Player>,
}
impl Scores {
    pub fn new() -> Scores {
        Scores {
            users: HashMap::new(),
        }
    }

    pub fn read(filename: String) -> HashMap<String, player::Player> {
        // read the file and panic if an error ocures
        let lines = fs::read_to_string(filename).expect("Unable to read file");
        // split the lines at line break
        let lines = lines.lines();
        // create an empty vector of users
        let mut users: Vec<User> = Vec::new();
        // go through each line in the file
        for user in lines{
            let username;
            let score: u16;
            {
                // read the line and split it into the needed data
                let lines: Vec<&str> = user.split('§').collect();
                // if there is not the correct ammount of data
                if lines.len() != 2 {
                    // output the error and quit
                    panic!("Line does has {}, that is not 2", lines.len());
                }
                // get the username as the first half of the data
                username = lines[0];
                // get the score as u16 as the second half of the data
                score = lines[1].parse::<u16>().unwrap();
            }
            // add a new score for that user and the score they got
            users.push(User::new(String::from(username), score));
        }
        let mut users_map: HashMap<String, player::Player> = HashMap::new();
        let mut username : String;
        // go through each users score in the scores file
        for user in users {
            username = user.username.clone();
            if users_map.contains_key(&username) {
                // get the old score from the hashmap
                let mut old = users_map[&username].clone();
                // add the score to that
                old.add_score(user.score);
                // overwrite the data in the hashmap with the new, updated data
                users_map.insert(username, old);
                continue;
            }
            // insert a new item into the dictinary
            users_map.insert(username, player::Player::new(user.username.as_str(), user.score));
        }
        // go through each user in the dictionary
        for key in users_map.keys() {
            // and output there data
            println!("{}: {}", key, users_map[key].to_string());
        }
        users_map
    }
    pub fn add_score(filename: String, user: User) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        // add the users score to the scores file
        let result = file.write_all(format!("{}§{}\n", user.username, user.score.to_string()).as_bytes());
        match result {
            Err(e) => panic!("Failed to write score\nErr: '{}'", e),
            Ok(_r) => return,
        }
    }
}