use std::collections::HashMap;

use crate::property::PropertyName;

pub type InstructionDefName = String;
pub type InstructionDefMap = HashMap<InstructionDefName, Vec<PropertyName>>;
