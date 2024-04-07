//! # Metadata
//!
//! A module containing representations of the various kinds of `MacawWorld`
//! metadata!

use std::fmt::Display;

use bevy::utils::Uuid;
use chrono::DateTime;

use super::generation::generators::blank;

/// Metadata important to maintain a world's consistency.
/// You should keep these in an `std::sync::Arc` for the most part.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct WorldMetadata {
    /// A unique name for a world.
    name: String,
    /// A unique seed used during world generation.
    seed: u64,
    /// A representation of this world's generator.
    generator: GeneratorId,
    /// Date/time when the world was made. (i am god)
    creation_date: DateTime<chrono::Utc>,
}

impl WorldMetadata {
    /// Creates a new `WorldMetadata` object.
    pub fn new(
        name: String,
        seed: u64,
        generator: GeneratorId,
        creation_date: DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            name,
            seed,
            generator,
            creation_date,
        }
    }

    /// Creates a new `WorldMetadata` where the creation date is now!
    pub fn new_now(name: String, seed: u64, generator: GeneratorId) -> Self {
        Self {
            name,
            seed,
            generator,
            creation_date: chrono::Utc::now(),
        }
    }

    /// The name of the world.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Seed used to generate the world.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The `GeneratorId` of the world.
    pub fn generator(&self) -> &GeneratorId {
        &self.generator
    }

    /// Date/time when the user created the save.
    pub fn creation_date(&self) -> &DateTime<chrono::Utc> {
        &self.creation_date
    }
}

impl Default for WorldMetadata {
    /// Provides the default value for `WorldMetadata`.
    ///
    /// In this case, it gets a random seed, random name, a blank generator,
    /// and the current time as the creation date.
    fn default() -> Self {
        Self {
            name: Uuid::new_v4().to_string(),
            seed: rand::random(),
            generator: blank::BLANK_GENERATOR_ID,
            creation_date: chrono::DateTime::default(),
        }
    }
}

/// The Java-style qualifier to uniquely identify a world's `Generator`.
///
/// Looks like: `tld.organization.name.subname`, where the `subname` element
/// is optional.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct GeneratorId {
    tld: &'static str,
    organization: &'static str,
    name: &'static str,
    subname: Option<&'static str>,
}

impl GeneratorId {
    pub const fn new(
        tld: &'static str,
        organization: &'static str,
        name: &'static str,
        subname: Option<&'static str>,
    ) -> Self {
        Self {
            tld,
            organization,
            name,
            subname,
        }
    }
}

impl From<GeneratorId> for String {
    fn from(value: GeneratorId) -> Self {
        format!("{}", &value)
    }
}

impl Display for GeneratorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // get a subname (with prefixed `.`) if there is one
        let subname = if let Some(subname) = &self.subname {
            format!(".{}", subname)
        } else {
            "".into()
        };

        // put them together!
        let result = format!(
            "{}.{}.{}{}",
            self.tld, self.organization, self.name, subname
        );

        f.write_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::GeneratorId;

    #[test]
    fn generator_name_subname() {
        let my_generator_id = GeneratorId {
            tld: "com",
            organization: "youtube",
            name: "www",
            subname: Some("dQw4w9WgXcQ"),
        };

        assert_eq!("com.youtube.www.dQw4w9WgXcQ", my_generator_id.to_string());
    }
}
