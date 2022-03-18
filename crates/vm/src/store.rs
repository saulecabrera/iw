use std::collections::HashMap;

use crate::instance::{Index as InstanceIndex, Instance};

pub struct Store {
    instances: Vec<Instance>,
    instances_env: HashMap<String, InstanceIndex>,
}
