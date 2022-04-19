#[derive(PartialEq)]
pub enum Colour {
    None,
    Green,
    Red,
    Grey,
}
// this is for reading the users guess
pub struct Letter {
    pub character: char,
    pub colour: Colour,
}
impl Letter {
    // this is like new but with no values to pass in so can be checked against
    pub fn empty() -> Letter {
        Letter {
            character: '1',
            colour: Colour::None,
        }
    }
}
