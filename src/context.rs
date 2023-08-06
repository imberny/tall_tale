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
    entities: HashMap<EntityId, Entity>, // characters, items, locations ... matched against alias_constraints
    relations: RelationMap,
    properties: PropertyMap, // miscellanious world variables, matched agains world_constraints
    exclude: HashSet<StoryId>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.insert(entity.id(), entity);
        self
    }

    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        self.entities
            .extend(entities.into_iter().map(|entity| (entity.id(), entity)));
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

    pub(crate) fn is_included(&self, story_id: &StoryId) -> bool {
        !self.exclude.contains(story_id)
    }

    pub(crate) fn entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub(crate) fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub(crate) fn world_property(&self, property_name: &str) -> Option<&Property> {
        self.properties.get(property_name)
    }

    pub(crate) fn properties(&self) -> &PropertyMap {
        &self.properties
    }

    pub(crate) fn relations(&self) -> &RelationMap {
        &self.relations
    }
}
