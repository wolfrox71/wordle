// this is for reading from the scores file

#[derive(Clone)]
pub struct Player {
    pub username: String,
    total_score: u16,
    avg_score: f32,
    times_played: u16,
}


impl Player {
    pub fn new(user: &str, score:u16) -> Player {
        Player{
            username: String::from(user),
            total_score: score,
            avg_score: score as f32,
            times_played: 1,
        }
    }
    pub fn add_score(&mut self, score: u16) {
        // increment the number of times played by 1 as a new score has been added
        self.times_played += 1;
        // add this turns score to the total score
        self.total_score += score;
        // calculate the avg score of the player by deviding the total score by the times played
        self.avg_score = self.total_score as f32 /self.times_played as f32;
    }
    // for outputing the score to the screen, if values are needed properly they can be read
    pub fn to_string(&self) -> String {
        // format the username, total score, avg score, and turns played into one string
        let mut line = String::from(" ");
        for x in 0..self.username.len() {
            line+="=";
        }
        format!(
        "
{4}
|{0}|
{4}
    Total Score: {1}
    Avg Score: {2}
    Turns Played: {3}",
         self.username, self.total_score,self.avg_score,self.times_played, line)
    }
}
