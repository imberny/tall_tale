use crate::property::{Property, PropertyMap, PropertyName};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct EntityId(pub usize);

impl From<usize> for EntityId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

pub struct Entity {
    id_: EntityId,
    pub properties: PropertyMap,
}

impl Entity {
    pub fn new<I>(id: I) -> Self
    where
        I: Into<usize>,
    {
        Self {
            id_: EntityId(id.into()),
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
