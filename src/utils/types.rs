use serde::{Deserialize, Serialize};
use std::path::PathBuf;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource_id: String,
    pub allowed_actions: Vec<String>,
}
