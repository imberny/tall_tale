use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    marker::PhantomData,
};

pub struct Beat<T> {
    pub name: String,
    constraints: Vec<Constraint<T>>,
}

impl<T: Hash + Eq> Beat<T> {
    pub fn is_satisfied(&self, world: &TaleWorld<T>) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied(world))
    }
}

pub struct Tale<K, T> {
    beats: HashMap<K, Beat<T>>,
}

impl<K: Hash + Eq, T: Hash + Eq> Tale<K, T> {
    pub fn new() -> Self {
        Self {
            beats: HashMap::default(),
        }
    }

    pub fn with(mut self, key: K, constraints: Vec<Constraint<T>>, name: String) -> Self {
        self.beats.insert(key, Beat { name, constraints });
        self
    }

    pub fn what_next(&self, world: &TaleWorld<T>) -> Option<&Beat<T>> {
        self.beats.values().find(|beat| beat.is_satisfied(world))
    }
}

impl<K: Hash + Eq, T: Hash + Eq> Default for Tale<K, T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Character<T> {
    traits: HashSet<T>,
}

impl<T: Hash + Eq> Character<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, new_trait: T) -> Self {
        self.traits.insert(new_trait);
        self
    }
}

impl<T: Hash + Eq> Default for Character<T> {
    fn default() -> Self {
        Self {
            traits: HashSet::default(),
        }
    }
}

pub struct TaleWorld<T> {
    phantom_marker: PhantomData<T>,
    characters: Vec<Character<T>>,
}

impl<T: Hash + Eq> TaleWorld<T> {
    pub fn new() -> Self {
        Self {
            phantom_marker: PhantomData::default(),
            characters: Vec::default(),
        }
    }

    pub fn with(mut self, character: Character<T>) -> Self {
        self.characters.push(character);
        self
    }
}

impl<T: Hash + Eq> Default for TaleWorld<T> {
    fn default() -> Self {
        Self {
            phantom_marker: PhantomData::default(),
            characters: Vec::default(),
        }
    }
}

pub enum Rule {
    Has,
    HasNot,
}

pub struct Constraint<T> {
    rule: Rule,
    attribute: T,
}

impl<T: Hash + Eq> Constraint<T> {
    pub fn new(rule: Rule, attribute: T) -> Self {
        Self { rule, attribute }
    }

    pub fn is_satisfied(&self, world: &TaleWorld<T>) -> bool {
        let has_attribute = world
            .characters
            .iter()
            .any(|character| character.traits.contains(&self.attribute));

        match self.rule {
            Rule::Has => has_attribute,
            Rule::HasNot => !has_attribute,
        }
    }
}

pub mod prelude {
    pub use crate::{Beat, Character, Constraint, Rule, Tale, TaleWorld};
}

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

        let tale = Tale::new().with(
            "MeetingJoe",
            vec![Constraint::new(
                Rule::Has,
                Attribute::Name("Joe".to_string()),
            )],
            "MeetingJoe".to_string(),
        );

        let world =
            TaleWorld::new().with(Character::new().with(Attribute::Name("Joe".to_string())));

        let beat = tale.what_next(&world);

        assert_eq!(beat.unwrap().name, "MeetingJoe".to_string())
    }
}
