use std::collections::HashMap;
use crate::error::EnvLayerError;
use crate::profile::Profile;

/// Stores and manages named profiles, allowing lookup and activation.
#[derive(Debug, Default)]
pub struct ProfileRegistry {
    profiles: HashMap<String, Profile>,
    active: Option<String>,
}

impl ProfileRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, profile: Profile) -> Result<(), EnvLayerError> {
        Profile::validate_name(&profile.name)?;
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    pub fn activate(&mut self, name: &str) -> Result<(), EnvLayerError> {
        if self.profiles.contains_key(name) {
            self.active = Some(name.to_string());
            Ok(())
        } else {
            Err(EnvLayerError::NotFound(format!(
                "Profile '{}' not found",
                name
            )))
        }
    }

    pub fn active_profile(&self) -> Option<&Profile> {
        self.active.as_deref().and_then(|n| self.profiles.get(n))
    }

    pub fn get(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn list_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.profiles.keys().map(String::as_str).collect();
        names.sort();
        names
    }

    pub fn remove(&mut self, name: &str) -> Option<Profile> {
        if self.active.as_deref() == Some(name) {
            self.active = None;
        }
        self.profiles.remove(name)
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}
