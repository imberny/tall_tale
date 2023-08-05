use std::collections::{HashMap, HashSet};

use crate::{
    entity::{Entity, EntityId},
    prelude::StoryId,
    property::{Property, PropertyMap, PropertyName},
};

// key is a pair of ids, value is property from POV of 1st entity
type RelationMap = HashMap<(EntityId, EntityId), PropertyMap>;

#[derive(Default)]
pub struct Context {
    pub(crate) entities: Vec<Entity>, // characters, items, locations ... matched against alias_constraints
    pub(crate) relations: RelationMap,
    pub(crate) properties: PropertyMap, // miscellanious world variables, matched agains world_constraints
    pub(crate) exclude: HashSet<StoryId>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
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
        self.relations
            .entry((me, other))
            .or_default()
            .insert(property_name.into(), property.into());

        self
    }

    pub fn with_world_property<N, P>(mut self, property_name: N, property: P) -> Self
    where
        N: Into<PropertyName>,
        P: Into<Property>,
    {
        self.properties
            .insert(property_name.into(), property.into());
        self
    }

    pub fn exclude(&mut self, story_ids: &[StoryId]) {
        self.exclude.extend(story_ids);
    }

    pub fn is_included(&self, story_id: &StoryId) -> bool {
        !self.exclude.contains(story_id)
    }
}
