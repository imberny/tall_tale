use std::collections::HashMap;

use crate::{entity::EntityDefName, property::PropertyName};

pub(crate) type RelationshipDefName = String;
pub(crate) type RelationshipDef = (EntityDefName, EntityDefName, Option<PropertyName>);
pub(crate) type RelationshipDefMap = HashMap<RelationshipDefName, Vec<RelationshipDef>>;
