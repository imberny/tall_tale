use std::collections::HashMap;

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
    pub world_properties: PropertyMap, // miscellanious world variables, matched agains world_constraints
                                       //* discard: Vec<StoryBeat>, // TODO: filter out already used stories... some kind of identifier? uuid?
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

    pub fn with_relation<ID, N, P>(
        mut self,
        me: ID,
        other: ID,
        property_name: N,
        property: P,
    ) -> Self
    where
        ID: Into<EntityId>,
        N: Into<PropertyName>,
        P: Into<Property>,
    {
        self.entity_relations
            .entry((me.into(), other.into()))
            .or_default()
            .insert(property_name.into(), property.into());

        self
    }

    pub fn with_world_property<N, P>(mut self, property_name: N, property: P) -> Self
    where
        N: Into<PropertyName>,
        P: Into<Property>,
    {
        self.world_properties
            .insert(property_name.into(), property.into());
        self
    }
}
