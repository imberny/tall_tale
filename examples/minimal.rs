use std::{collections::HashSet, hash::Hash};

use tall_tale::{Character, Constraint, Rule, Tale, TaleWorld};

fn main() {
    let tale = Tale::new().with(
        "MeetingJoe",
        vec![Constraint::new(
            Rule::Has,
            Attribute::Name("Joe".to_string()),
        )],
        "MeetingJoe".to_string(),
    );

    let world = TaleWorld::new().with(Character::new().with(Attribute::Name("Joe".to_string())));

    let beat = tale.what_next(&world);

    assert_eq!(beat.unwrap().name, "MeetingJoe".to_string())
}

// user defined
#[derive(Hash, PartialEq, Eq)]
pub enum Attribute {
    Name(String),
    Age(usize),
}
