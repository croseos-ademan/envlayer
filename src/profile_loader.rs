use std::collections::HashMap;
use crate::error::EnvLayerError;
use crate::profile::Profile;

/// Loads profiles from structured data such as a parsed TOML/JSON map.
/// Each key is a profile name; value is a map of env var key-value pairs.
pub struct ProfileLoader;

impl ProfileLoader {
    /// Load profiles from a nested map (profile_name -> key -> value).
    pub fn from_map(
        data: HashMap<String, HashMap<String, String>>,
    ) -> Result<Vec<Profile>, EnvLayerError> {
        let mut profiles = Vec::new();
        for (name, vars) in data {
            Profile::validate_name(&name)?;
            let mut profile = Profile::new(&name);
            for (k, v) in vars {
                profile.set(k, v);
            }
            profiles.push(profile);
        }
        Ok(profiles)
    }

    /// Load a single profile from a flat key=value map.
    pub fn from_flat(
        name: impl Into<String>,
        vars: HashMap<String, String>,
    ) -> Result<Profile, EnvLayerError> {
        let name = name.into();
        Profile::validate_name(&name)?;
        let mut profile = Profile::new(&name);
        for (k, v) in vars {
            profile.set(k, v);
        }
        Ok(profile)
    }

    /// Load profiles from a vec of (name, key, value) triples.
    pub fn from_triples(
        triples: Vec<(&str, &str, &str)>,
    ) -> Result<Vec<Profile>, EnvLayerError> {
        let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
        for (profile, key, value) in triples {
            map.entry(profile.to_string())
                .or_default()
                .insert(key.to_string(), value.to_string());
        }
        Self::from_map(map)
    }
}
