use serde::{Deserialize, Serialize};

use super::{EnvironmentConfiguration, ResourceConfiguration};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Container {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvironmentConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceConfiguration>,
}
