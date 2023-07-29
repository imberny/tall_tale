use std::collections::HashMap;

use crate::{
    entity::{Entity, EntityId},
    property::PropertyMap,
};

// key is a pair of ids, value is property from POV of 1st entity
// TODO: improve api with builder methods
pub type RelationMap = HashMap<(EntityId, EntityId), PropertyMap>;

// TODO: improve api with builder methods
pub struct Query {
    pub entities: Vec<Entity>, // characters, items, locations ... matched against alias_constraints
    pub entity_relations: RelationMap,
    pub world_state: PropertyMap, // miscellanious world variables, matched agains world_constraints
                                  //* discard: Vec<StoryBeat>, // TODO: some kind of identifier? uuid?
}
