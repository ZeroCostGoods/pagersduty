#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Team {
    #[serde(rename="team_reference")]
    Reference {
        id: String,

        /// A short-form, server-generated string that provides succinct,
        /// important information about an object suitable for primary
        /// labeling of an entity in a client. In many cases, this will be
        /// identical to `name`, though it is not intended to be an identifier.
        summary: String,

        /// The API show URL at which the object is accessible.
        #[serde(rename="self")]
        self_: String,

        /// A URL at which the entity is uniquely displayed in the Web app.
        html_url: Option<String>,
    },

    #[serde(rename="team")]
    Team {
        id: String,

        /// A short-form, server-generated string that provides succinct,
        /// important information about an object suitable for primary
        /// labeling of an entity in a client. In many cases, this will be
        /// identical to `name`, though it is not intended to be an identifier.
        summary: String,

        /// The API show URL at which the object is accessible.
        #[serde(rename="self")]
        self_: String,

        /// A URL at which the entity is uniquely displayed in the Web app.
        html_url: Option<String>,

        /// The name of the team.
        name: String,

        /// The description of the team.
        description: Option<String>,
    },
}

pub type Teams = Vec<Team>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

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
                Team::Reference {
                    id: "PRJ4D5C".into(),
                    summary: "ops".into(),
                    self_: "https://api.pagerduty.com/teams/PRJ4D5C".into(),
                    html_url: Some(
                        "https://webdemo.pagerduty.com/teams/PRJ4D5C".into()
                    ),
                },
                Team::Team {
                    id: "P7W0ZIU".into(),
                    summary: "Monitoring Tools Team".into(),
                    self_: "https://api.pagerduty.com/teams/P7W0ZIU".into(),
                    html_url: Some(
                        "https://webdemo.pagerduty.com/teams/P7W0ZIU".into()
                    ),
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

