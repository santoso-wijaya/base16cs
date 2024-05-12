use anyhow::Result;

/// A trait for an object that can serialize and deserialize to and from a utf-8 string.
pub trait Serializable {
    fn serialize(&self) -> Result<String>;
}

#[cfg(feature = "yaml")]
pub mod yaml;
