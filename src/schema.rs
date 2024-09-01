use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    entity::{EntityAlias, EntityDefMap, EntityDefName},
    instruction::InstructionDefMap,
    property::PropertyDefMap,
    relationship::RelationshipDefMap,
};

type EnumName = String;

type EnumDef = Vec<String>;

type EnumDefMap = HashMap<EnumName, EnumDef>;

type FlagSet = HashSet<String>;

#[derive(Default, Serialize, Deserialize)]
struct Schema {
    enums: EnumDefMap,
    properties: PropertyDefMap,
    entity_types: EntityDefMap,
    flags: FlagSet,
    relationships: RelationshipDefMap,
    instructions: InstructionDefMap,
    global_entities: HashMap<EntityAlias, EntityDefName>,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Schema;

    #[test]
    fn validate_schema() {
        let schema_content =
            fs::read_to_string("tests/resources/schema_wishful_thinking.ron").expect("");
        let _: Schema = ron::from_str(&schema_content).expect("");
    }
}
