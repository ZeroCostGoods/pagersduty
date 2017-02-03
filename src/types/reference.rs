//! Many of the types you receive back from the PagerDuty API contain references
//! to other types. Most types, whether a reference or not, contain the same fields
//! consisently. Rather than duplicate those fields on each type we compose the
//! `Reference` type in other types. This type is not something you'd normally
//! access directly, but through another type.
//!
//! You can read more about references here:
//! https://v2.developer.pagerduty.com/v2/docs/references


use serde::ser::SerializeMap;


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
    pub fn serialize_key_vals<S>(&self, mut state: &mut S) -> Result<(), S::Error>
        where S: SerializeMap
    {
        state.serialize_key("id")?;
        state.serialize_value(&self.id)?;

        state.serialize_key("summary")?;
        state.serialize_value(&self.summary)?;

        state.serialize_key("type")?;
        state.serialize_value(&self.type_)?;

        state.serialize_key("self")?;
        state.serialize_value(&self.self_)?;

        state.serialize_key("html_url")?;
        state.serialize_value(&self.html_url)?;

        Ok(())
    }
}

