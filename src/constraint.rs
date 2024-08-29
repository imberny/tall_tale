use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::{
    property::{Property, PropertyMap, PropertyName},
    scenario_action::Alias,
    Int, Real,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Constraint {
    Has(PropertyName),
    HasNot(PropertyName),
    Equals(PropertyName, Property),
    IsInRange(PropertyName, Range<Int>),
    IsInRangeFloat(PropertyName, Range<Real>),
}

impl Constraint {
    pub fn is_in_range<N, R>(property_name: N, range: R) -> Self
    where
        N: Into<PropertyName>,
        R: Into<Range<Int>>,
    {
        Self::IsInRange(property_name.into(), range.into())
    }

    pub fn is_in_range_float<N, R>(property_name: N, range: R) -> Self
    where
        N: Into<PropertyName>,
        R: Into<Range<Real>>,
    {
        Self::IsInRangeFloat(property_name.into(), range.into())
    }

    pub fn has<N>(property_name: N) -> Self
    where
        N: Into<PropertyName>,
    {
        Self::Has(property_name.into())
    }

    pub fn has_not<N>(property_name: N) -> Self
    where
        N: Into<PropertyName>,
    {
        Self::HasNot(property_name.into())
    }

    pub fn equals<N, P>(property_name: N, to: P) -> Self
    where
        N: Into<PropertyName>,
        P: Into<Property>,
    {
        Self::Equals(property_name.into(), to.into())
    }

    pub fn is_satisfied_by(&self, properties: &PropertyMap) -> bool {
        match self {
            Constraint::Has(prop_name) => properties.get(prop_name).is_some(),
            Constraint::HasNot(prop_name) => properties.get(prop_name).is_none(),
            Constraint::Equals(prop_name, property) => properties
                .get(prop_name)
                .is_some_and(|ent_prop| property == ent_prop),
            Constraint::IsInRange(prop_name, range) => properties
                .get(prop_name)
                .is_some_and(|prop| prop.is_in_range(range)),
            Constraint::IsInRangeFloat(prop_name, range) => properties
                .get(prop_name)
                .is_some_and(|prop| prop.is_in_range_float(range)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AliasRelation {
    pub me: Alias,
    pub other: Alias,
    pub constraints: Vec<Constraint>,
}

impl AliasRelation {
    pub fn new<A, C>(me: A, other: A, constraints: C) -> Self
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        Self {
            me: me.into(),
            other: other.into(),
            constraints: Vec::from_iter(constraints),
        }
    }

    pub fn is_satisfied_by(&self, properties: &PropertyMap) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(properties))
    }
}
