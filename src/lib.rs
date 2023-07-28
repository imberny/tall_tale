mod story_beat;

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

mod prelude {}

// TODO: build a proof of concept with simple dictionaries of strings

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn single_beat() {
        #[derive(Hash, PartialEq, Eq)]
        pub enum Attribute {
            Name(String),
            // Age(usize),
        }

        // let tale = Tale::new().with(
        //     "MeetingJoe",
        //     vec![Constraint::new(
        //         Rule::Has,
        //         Attribute::Name("Joe".to_string()),
        //     )],
        //     "MeetingJoe".to_string(),
        // );

        // let world =
        //     TaleWorld::new().with(Character::new().with(Attribute::Name("Joe".to_string())));

        // let beat = tale.what_next(&world);

        // assert_eq!(beat.unwrap().name, "MeetingJoe".to_string())
    }

    // can I have a character struct that holds a set of user defined attributes, each attribute being a struct deriving CharacterAttribute?
    // with macros, maybe

    // trait CharacterAttribute {}

    // #[derive(CharacterAttribute)]
    // struct Age(usize);
}
