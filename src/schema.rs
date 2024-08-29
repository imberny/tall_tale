use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entity::EntityType,
    property::{PropertyName, PropertyType},
};

type EnumName = String;

#[derive(Default, Serialize, Deserialize)]
struct EnumDef(Vec<String>);

type EnumDefMap = HashMap<EnumName, EnumDef>;

type PropertyDefMap = HashMap<PropertyName, PropertyType>;

type EntityDefName = String;
type EntityDefMap = HashMap<EntityDefName, Vec<PropertyName>>;

type RelationshipDefName = String;
type RelationshipDef = (EntityDefName, EntityDefName, PropertyType);
type RelationshipDefMap = HashMap<RelationshipDefName, RelationshipDef>;

type EventDefName = String;
type EventDef = PropertyType;
type EventDefMap = HashMap<EventDefName, EventDef>;

#[derive(Default, Serialize, Deserialize)]
struct Schema {
    enums: EnumDefMap,
    properties: PropertyDefMap,
    entities: EntityDefMap,
    relationships: RelationshipDefMap,
    events: EventDefMap,
}

#[cfg(test)]
mod tests {
    use super::Schema;

    #[test]
    fn validate_schema() {
        let mut schema = Schema::default();
        assert!(false, "hello");
    }
}
