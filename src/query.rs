use std::collections::{hash_map::Entry, HashMap};

use crate::{
    entity::{Entity, EntityId},
    property::{Property, PropertyMap, PropertyName},
};

// key is a pair of ids, value is property from POV of 1st entity
// TODO: improve api with builder methods
pub type RelationMap = HashMap<(EntityId, EntityId), PropertyMap>;

// TODO: improve api with builder methods
#[derive(Default)]
pub struct Query {
    pub entities: Vec<Entity>, // characters, items, locations ... matched against alias_constraints
    pub entity_relations: RelationMap,
    pub world_state: PropertyMap, // miscellanious world variables, matched agains world_constraints
                                  //* discard: Vec<StoryBeat>, // TODO: some kind of identifier? uuid?
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn with_entities<E>(mut self, entities: E) -> Self
    where
        E: IntoIterator<Item = Entity>,
    {
        self.entities.extend(entities);
        self
    }

    pub fn with_relation<N, P>(
        mut self,
        me: EntityId,
        other: EntityId,
        property_name: N,
        property: P,
    ) -> Self
    where
        N: Into<PropertyName>,
        P: Into<Property>,
    {
        self.entity_relations
            .entry((me, other))
            .or_default()
            .insert(property_name.into(), property.into());

        self
    }
}
