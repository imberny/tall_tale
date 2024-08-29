use crate::property::{Property, PropertyMap, PropertyName};

// #[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub type EntityId = usize;

// impl From<usize> for EntityId {
//     fn from(value: usize) -> Self {
//         Self(value)
//     }
// }
pub type EntityType = String; // A label to distinguish types

pub struct Entity {
    id: EntityId, // user provided id to let them map story entities to game objects
    pub properties: PropertyMap,
    pub exclusory_properties: PropertyMap,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            properties: PropertyMap::default(),
            exclusory_properties: PropertyMap::default(),
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

    pub fn with_exclusory(
        mut self,
        property_name: impl Into<PropertyName>,
        property: impl Into<Property>,
    ) -> Self {
        self.exclusory_properties
            .insert(property_name.into(), property.into());
        self
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn get<P>(&self, property: P) -> Option<&Property>
    where
        P: Into<PropertyName>,
    {
        self.properties.get(&property.into())
    }

    pub fn get_exclusory<P>(&self, property: P) -> Option<&Property>
    where
        P: Into<PropertyName>,
    {
        self.exclusory_properties.get(&property.into())
    }
}
