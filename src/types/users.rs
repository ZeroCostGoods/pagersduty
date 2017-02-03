use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use serde::de::Error;
use serde::ser::SerializeMap;

use types::reference::Reference;
use types::contact_methods::ContactMethods;
use types::notification_rules::NotificationRules;
use types::teams::Teams;


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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            User::UserReference{
                ref reference
            } => {
                reference.serialize_key_vals(&mut state)?;
            },
            User::User{
                ref reference,
                ref avatar_url, ref color,  ref contact_methods, ref description,
                ref email, ref invitation_sent, ref job_title, ref name,
                ref notification_rules, ref role, ref teams, ref time_zone,
            } => {
                reference.serialize_key_vals(&mut state)?;

                state.serialize_key("avatar_url")?;
                state.serialize_value(avatar_url)?;

                state.serialize_key("color")?;
                state.serialize_value(color)?;

                state.serialize_key("contact_methods")?;
                state.serialize_value(contact_methods)?;

                state.serialize_key("description")?;
                state.serialize_value(description)?;

                state.serialize_key("email")?;
                state.serialize_value(email)?;

                state.serialize_key("invitation_sent")?;
                state.serialize_value(invitation_sent)?;

                state.serialize_key("job_title")?;
                state.serialize_value(job_title)?;

                state.serialize_key("name")?;
                state.serialize_value(name)?;

                state.serialize_key("notification_rules")?;
                state.serialize_value(notification_rules)?;

                state.serialize_key("role")?;
                state.serialize_value(role)?;

                state.serialize_key("teams")?;
                state.serialize_value(teams)?;

                state.serialize_key("time_zone")?;
                state.serialize_value(time_zone)?;
            },
        }

        state.end()
    }
}

impl Deserialize for User {
    fn deserialize<D>(deserializer: D) -> Result<User, D::Error>
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
                let avatar_url = union.avatar_url.ok_or(
                    D::Error::missing_field("avatar_url"))?;
                let color = union.color.ok_or(
                    D::Error::missing_field("color"))?;
                let contact_methods = union.contact_methods.ok_or(
                    D::Error::missing_field("contact_methods"))?;
                let email = union.email.ok_or(
                    D::Error::missing_field("email"))?;
                let invitation_sent = union.invitation_sent.ok_or(
                    D::Error::missing_field("invitation_sent"))?;
                let name = union.name.ok_or(
                    D::Error::missing_field("name"))?;
                let notification_rules = union.notification_rules.ok_or(
                    D::Error::missing_field("notification_rules"))?;
                let role = union.role.ok_or(
                    D::Error::missing_field("role"))?;
                let teams = union.teams.ok_or(
                    D::Error::missing_field("teams"))?;
                let time_zone = union.time_zone.ok_or(
                    D::Error::missing_field("time_zone"))?;

                Ok(User::User {
                    reference: reference,
                    avatar_url: avatar_url,
                    color: color,
                    contact_methods: contact_methods,
                    description: union.description,
                    email: email,
                    invitation_sent: invitation_sent,
                    job_title: union.job_title,
                    name: name,
                    notification_rules: notification_rules,
                    role: role,
                    teams: teams,
                    time_zone: time_zone,
                })
            },
            _ => Err(D::Error::custom("type received was unexpected.")),
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

    use ::types::reference::Reference;
    use ::types::contact_methods::ContactMethod;
    use ::types::notification_rules::NotificationRule;

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

