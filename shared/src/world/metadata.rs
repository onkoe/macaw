//! # Metadata
//!
//! A module containing representations of the various kinds of `MacawWorld`
//! metadata!

use std::fmt::{Display, Pointer};

/// Metadata important to maintain a world's consistency.
/// You should keep these in an `std::sync::Arc` for the most part.
pub struct WorldMetadata {
    name: String,
    seed: u64,
    generator: GeneratorId,
}

/// The Java-style qualifier to uniquely identify a world's `Generator`.
///
/// Looks like: `tld.organization.name.subname`, where the `subname` element
/// is optional.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct GeneratorId {
    tld: String,
    organization: String,
    name: String,
    subname: Option<String>,
}

impl Into<String> for GeneratorId {
    fn into(self) -> String {
        format!("{}", self)
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
            tld: "com".into(),
            organization: "youtube".into(),
            name: "www".into(),
            subname: Some("dQw4w9WgXcQ".into()),
        };

        assert_eq!("com.youtube.www.dQw4w9WgXcQ", my_generator_id.to_string());
    }
}
