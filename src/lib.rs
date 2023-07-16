use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Beat<Attribute> {
    pub name: String,
    constraints: Vec<Constraint<Attribute>>,
}

impl<Attribute: Hash + Eq> Beat<Attribute> {
    pub fn is_satisfied(&self, world: &TaleWorld<Attribute>) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied(world))
    }
}

pub struct Tale<K, Attribute> {
    beats: HashMap<K, Beat<Attribute>>,
}

impl<K: Hash + Eq, Attribute: Hash + Eq> Tale<K, Attribute> {
    pub fn new() -> Self {
        Self {
            beats: HashMap::default(),
        }
    }

    pub fn with(mut self, key: K, constraints: Vec<Constraint<Attribute>>, name: String) -> Self {
        self.beats.insert(key, Beat { name, constraints });
        self
    }

    pub fn what_next(&self, world: &TaleWorld<Attribute>) -> Option<&Beat<Attribute>> {
        self.beats.values().find(|beat| beat.is_satisfied(world))
    }
}

impl<K: Hash + Eq, Attribute: Hash + Eq> Default for Tale<K, Attribute> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Character<Attribute> {
    attributes: HashSet<Attribute>,
}

impl<Attribute: Hash + Eq> Character<Attribute> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, new_trait: Attribute) -> Self {
        self.attributes.insert(new_trait);
        self
    }
}

impl<Attribute: Hash + Eq> Default for Character<Attribute> {
    fn default() -> Self {
        Self {
            attributes: HashSet::default(),
        }
    }
}

pub struct TaleWorld<Attribute> {
    characters: Vec<Character<Attribute>>,
}

impl<Attribute: Hash + Eq> TaleWorld<Attribute> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, character: Character<Attribute>) -> Self {
        self.characters.push(character);
        self
    }
}

impl<Attribute: Hash + Eq> Default for TaleWorld<Attribute> {
    fn default() -> Self {
        Self {
            characters: Vec::default(),
        }
    }
}

pub enum Rule {
    Has,
    HasNot,
}

pub struct Constraint<Attribute> {
    rule: Rule,
    attribute: Attribute,
}

impl<Attribute: Hash + Eq> Constraint<Attribute> {
    pub fn new(rule: Rule, attribute: Attribute) -> Self {
        Self { rule, attribute }
    }

    pub fn is_satisfied(&self, world: &TaleWorld<Attribute>) -> bool {
        let has_attribute = world
            .characters
            .iter()
            .any(|character| character.attributes.contains(&self.attribute));

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
