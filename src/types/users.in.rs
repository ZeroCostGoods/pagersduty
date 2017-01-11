use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::reference::Reference;
use super::contact_methods::ContactMethods;
use super::notification_rules::NotificationRules;
use super::teams::Teams;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserUnion {
    // All Reference's
    id: String,
    summary: String,
    #[serde(rename="type")]
    type_: String,
    #[serde(rename="self")]
    self_: String,
    html_url: Option<String>,

    // All Concrete type fields
    avatar_url: Option<String>,
    color: Option<String>,
    contact_methods: Option<ContactMethods>,
    description: Option<String>,
    email: Option<String>,
    invitation_sent: Option<bool>,
    job_title: Option<String>,
    name: Option<String>,
    notification_rules: Option<NotificationRules>,
    role: Option<String>,
    teams: Option<Teams>,
    time_zone: Option<String>,
}


#[derive(Debug, PartialEq)]
pub enum User {
    UserReference {
        reference: Reference,
    },

    User {
        reference: Reference,

        /// The URL of the user's avatar.
        avatar_url: String,

        /// The schedule color.
        color: String,

        /// The list of contact methods for the user.
        contact_methods: ContactMethods,

        /// The user's bio
        description: Option<String>,

        /// The user's email address.
        email: String,

        /// If true, the user has an outstanding invitation.
        invitation_sent: bool,

        /// The user's title.
        job_title: Option<String>,

        /// The name of the user.
        name: String,

        /// The list of notification rules for the user.
        notification_rules: NotificationRules,

        /// The user role. Account must have the `read_only_users`
        /// ability to set a user as a `read_only_user`.
        ///
        /// Expected values include:
        /// `admin`, `limited_user`, `owner`, `read_only_user`, and `user`
        role: String,

        /// The list of teams to which the user belongs. Account must
        /// have the `teams` ability to set this.
        teams: Teams,

        /// The preferred time zone name. If null, the account's time
        /// zone will be used.
        time_zone: String,

    },
}


impl Serialize for User {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            User::UserReference{
                ref reference
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;
            },
            User::User{
                ref reference,
                ref avatar_url, ref color,  ref contact_methods, ref description,
                ref email, ref invitation_sent, ref job_title, ref name,
                ref notification_rules, ref role, ref teams, ref time_zone,
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;

                serializer.serialize_map_key(&mut state, "avatar_url")?;
                serializer.serialize_map_value(&mut state, avatar_url)?;

                serializer.serialize_map_key(&mut state, "color")?;
                serializer.serialize_map_value(&mut state, color)?;

                serializer.serialize_map_key(&mut state, "contact_methods")?;
                serializer.serialize_map_value(&mut state, contact_methods)?;

                serializer.serialize_map_key(&mut state, "description")?;
                serializer.serialize_map_value(&mut state, description)?;

                serializer.serialize_map_key(&mut state, "email")?;
                serializer.serialize_map_value(&mut state, email)?;

                serializer.serialize_map_key(&mut state, "invitation_sent")?;
                serializer.serialize_map_value(&mut state, invitation_sent)?;

                serializer.serialize_map_key(&mut state, "job_title")?;
                serializer.serialize_map_value(&mut state, job_title)?;

                serializer.serialize_map_key(&mut state, "name")?;
                serializer.serialize_map_value(&mut state, name)?;

                serializer.serialize_map_key(&mut state, "notification_rules")?;
                serializer.serialize_map_value(&mut state, notification_rules)?;

                serializer.serialize_map_key(&mut state, "role")?;
                serializer.serialize_map_value(&mut state, role)?;

                serializer.serialize_map_key(&mut state, "teams")?;
                serializer.serialize_map_value(&mut state, teams)?;

                serializer.serialize_map_key(&mut state, "time_zone")?;
                serializer.serialize_map_value(&mut state, time_zone)?;
            },
        }

        serializer.serialize_map_end(state)
    }
}

impl Deserialize for User {
    fn deserialize<D>(deserializer: &mut D) -> Result<User, D::Error>
        where D: Deserializer
    {
        let union = UserUnion::deserialize(deserializer)?;

        let reference = Reference {
            id: union.id,
            summary: union.summary,
            type_: union.type_,
            self_: union.self_,
            html_url: union.html_url,
        };

        match reference.type_.as_ref() {
            "user_reference"  => {
                Ok(User::UserReference {
                    reference: reference,
                })
            },
            "user" => {
                Ok(User::User {
                    reference: reference,
                    avatar_url: union.avatar_url.expect("avatar_url"),
                    color: union.color.expect("color"),
                    contact_methods: union.contact_methods.expect("contact_methods"),
                    description: union.description,
                    email: union.email.expect("email"),
                    invitation_sent: union.invitation_sent.expect("invitation_sent"),
                    job_title: union.job_title,
                    name: union.name.expect("name"),
                    notification_rules: union.notification_rules.expect("notification_rules"),
                    role: union.role.expect("role"),
                    teams: union.teams.expect("teams"),
                    time_zone: union.time_zone.expect("time_zone"),
                })
            },
            _ => panic!("fuuuuuuu"),
        }
    }
}


pub type Users = Vec<User>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use super::super::reference::Reference;
    use super::super::contact_methods::ContactMethod;
    use super::super::notification_rules::NotificationRule;

    #[test]
    fn test_serde() {
        let mut file = File::open("testdata/types/users.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let users: Users = serde_json::from_str(&data).unwrap();

        // Verify deserialization.
        assert_eq!(
            users,
            vec![
                User::UserReference {
                    reference: Reference {
                        id: "PRMXSSO".into(),
                        summary: "Benedict Cumberbatch".into(),
                        type_: "user_reference".into(),
                        self_: "https://api.pagerduty.com/users/PRMXSSO".into(),
                        html_url: Some(
                            "https://webdemo.pagerduty.com/users/PRMXSSO".into()
                        ),
                    },
                },
                User::User {
                    reference: Reference {
                        id: "P5T36BU".into(),
                        summary: "abhijit@pagerduty.com".into(),
                        type_: "user".into(),
                        self_: "https://api.pagerduty.com/users/P5T36BU".into(),
                        html_url: Some(
                            "https://webdemo.pagerduty.com/users/P5T36BU".into()
                        ),
                    },
                    avatar_url: "https://secure.gravatar.com/avatar/267299c8432bf9ab044472009c89a674.png?d=mm&r=PG".into(),
                    color: "olivedrab".into(),
                    contact_methods: vec![
                        ContactMethod::ContactMethodReference {
                            reference: Reference {
                                id: "P6YMJEE".into(),
                                summary: "Default".into(),
                                type_: "email_contact_method_reference".into(),
                                self_: "https://api.pagerduty.com/users/P5T36BU/contact_methods/P6YMJEE".into(),
                                html_url: None,
                            },
                        },
                    ],
                    description: None,
                    email: "abhijit@pagerduty.com".into(),
                    invitation_sent: false,
                    job_title: None,
                    name: "abhijit@pagerduty.com".into(),
                    notification_rules: vec![
                        NotificationRule::NotificationRuleReference {
                            reference: Reference {
                                id: "PM41X3T".into(),
                                summary: "0 minutes: channel P6YMJEE".into(),
                                type_: "assignment_notification_rule_reference".into(),
                                self_: "https://api.pagerduty.com/users/P5T36BU/notification_rules/PM41X3T".into(),
                                html_url: None,
                            },
                        },
                        NotificationRule::NotificationRuleReference {
                            reference: Reference {
                                id: "PNTRY7M".into(),
                                summary: "0 minutes: channel P6YMJEE".into(),
                                type_: "assignment_notification_rule_reference".into(),
                                self_: "https://api.pagerduty.com/users/P5T36BU/notification_rules/PNTRY7M".into(),
                                html_url: None,
                            },
                        },
                    ],
                    role: "user".into(),
                    teams: vec![],
                    time_zone: "America/Los_Angeles".into(),
                },
            ]
        );

        // Verify that serialization round-trips.
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&users).unwrap().as_ref()
        ).unwrap();
        assert_eq!(serialized, expected)
    }
}

