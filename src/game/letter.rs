// this is for reading the users guess
pub struct Letter {
    pub character: char,
    pub colour: u8,
}
impl Letter {
    // this is like new but with no values to pass in so can be checked against
    pub fn empty() -> Letter {
        Letter {
            character: 'a',
            colour: 4,
        }
    }
}
