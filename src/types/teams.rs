use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use serde::de::Error;
use serde::ser::SerializeMap;

use types::reference::Reference;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TeamUnion {
    // All Reference's
    id: String,
    summary: String,
    #[serde(rename="type")]
    type_: String,
    #[serde(rename="self")]
    self_: String,
    html_url: Option<String>,

    // All Concrete type fields
    name: Option<String>,
    description: Option<String>,
}


#[derive(Debug, PartialEq)]
pub enum Team {
    TeamReference {
        reference: Reference,
    },

    Team {
        reference: Reference,

        /// The name of the team.
        name: String,

        /// The description of the team.
        description: Option<String>,
    },
}


impl Serialize for Team {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            Team::TeamReference{
                ref reference
            } => {
                reference.serialize_key_vals(&mut state)?;
            },
            Team::Team{
                ref reference, ref name, ref description
            } => {
                reference.serialize_key_vals(&mut state)?;

                state.serialize_key("name")?;
                state.serialize_value(name)?;

                state.serialize_key("description")?;
                state.serialize_value(description)?;
            },
        }

        state.end()
    }
}

impl Deserialize for Team {
    fn deserialize<D>(deserializer: D) -> Result<Team, D::Error>
        where D: Deserializer
    {
        let union = TeamUnion::deserialize(deserializer)?;

        let reference = Reference {
            id: union.id,
            summary: union.summary,
            type_: union.type_,
            self_: union.self_,
            html_url: union.html_url,
        };

        match reference.type_.as_ref() {
            "team_reference"  => {
                Ok(Team::TeamReference {
                    reference: reference,
                })
            },
            "team" => {
                let name = union.name.ok_or(D::Error::missing_field("name"))?;

                Ok(Team::Team {
                    reference: reference,
                    name: name,
                    description: union.description,
                })
            },
            _ => Err(D::Error::custom("type received was unexpected.")),
        }
    }
}


pub type Teams = Vec<Team>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

    use types::reference::Reference;

    #[test]
    fn test_serde() {
        let mut file = File::open("testdata/types/teams.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let teams: Teams = serde_json::from_str(&data).unwrap();

        // Verify deserialization.
        assert_eq!(
            teams,
            vec![
                Team::TeamReference {
                    reference: Reference {
                        id: "PRJ4D5C".into(),
                        summary: "ops".into(),
                        type_: "team_reference".into(),
                        self_: "https://api.pagerduty.com/teams/PRJ4D5C".into(),
                        html_url: Some(
                            "https://webdemo.pagerduty.com/teams/PRJ4D5C".into()
                        ),
                    },
                },
                Team::Team {
                    reference: Reference {
                        id: "P7W0ZIU".into(),
                        summary: "Monitoring Tools Team".into(),
                        type_: "team".into(),
                        self_: "https://api.pagerduty.com/teams/P7W0ZIU".into(),
                        html_url: Some(
                            "https://webdemo.pagerduty.com/teams/P7W0ZIU".into()
                        ),
                    },
                    name: "Monitoring Tools Team".into(),
                    description: None,
                },
            ]
        );

        // Verify that serialization round-trips.
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&teams).unwrap().as_ref()
        ).unwrap();
        assert_eq!(serialized, expected)
    }
}

