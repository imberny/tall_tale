use crate::property::{Property, PropertyMap, PropertyName};

// #[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub type EntityId = usize;

// impl From<usize> for EntityId {
//     fn from(value: usize) -> Self {
//         Self(value)
//     }
// }

pub struct Entity {
    id: EntityId,
    pub properties: PropertyMap,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self {
            id,
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
        self.id
    }

    pub fn get<P>(&self, property: P) -> Option<&Property>
    where
        P: Into<PropertyName>,
    {
        self.properties.get(&property.into())
    }
}
