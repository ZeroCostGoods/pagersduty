//! Many of the types you receive back from the PagerDuty API contain references
//! to other types. Most types, whether a reference or not, contain the same fields
//! consisently. Rather than duplicate those fields on each type we compose the
//! `Reference` type in other types. This type is not something you'd normally
//! access directly, but through another type.
//!
//! You can read more about references here:
//! https://v2.developer.pagerduty.com/v2/docs/references


use serde::ser::Serializer;


#[derive(Debug, PartialEq)]
pub struct Reference {
    pub id: String,

    /// A short-form, server-generated string that provides succinct,
    /// important information about an object suitable for primary
    /// labeling of an entity in a client. In many cases, this will be
    /// identical to `name`, though it is not intended to be an identifier.
    pub summary: String,

    /// The type of contact method.
    pub type_: String,

    /// The API show URL at which the object is accessible.
    pub self_: String,

    /// A URL at which the entity is uniquely displayed in the Web app.
    pub html_url: Option<String>,
}


impl Reference {
    pub fn serialize_key_vals<S>(&self, serializer: &mut S, mut state: &mut S::MapState) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_map_key(&mut state, "id")?;
        serializer.serialize_map_value(&mut state, &self.id)?;

        serializer.serialize_map_key(&mut state, "summary")?;
        serializer.serialize_map_value(&mut state, &self.summary)?;

        serializer.serialize_map_key(&mut state, "type")?;
        serializer.serialize_map_value(&mut state, &self.type_)?;

        serializer.serialize_map_key(&mut state, "self")?;
        serializer.serialize_map_value(&mut state, &self.self_)?;

        serializer.serialize_map_key(&mut state, "html_url")?;
        serializer.serialize_map_value(&mut state, &self.html_url)?;

        Ok(())
    }
}

