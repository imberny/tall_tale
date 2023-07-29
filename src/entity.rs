use crate::property::{Property, PropertyMap, PropertyName};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct EntityId(usize);

impl EntityId {
    pub const fn new(id: usize) -> Self {
        Self(id)
    }
}

pub struct Entity {
    id_: EntityId,
    pub properties: PropertyMap,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id_: id,
            properties: PropertyMap::default(),
        }
    }

    pub fn with(
        mut self,
        property_name: impl Into<PropertyName>,
        property: impl Into<Property>,
    ) -> Self {
        self.properties
            .insert(property_name.into(), property.into());
        self
    }

    pub fn id(&self) -> EntityId {
        self.id_
    }
}
