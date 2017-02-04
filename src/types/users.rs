use types::contact_methods::ContactMethods;
use types::notification_rules::NotificationRules;
use types::teams::Teams;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum User {
    #[serde(rename="user_reference")]
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

    #[serde(rename="user")]
    User {
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


pub type Users = Vec<User>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

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
                User::Reference {
                    id: "PRMXSSO".into(),
                    summary: "Benedict Cumberbatch".into(),
                    self_: "https://api.pagerduty.com/users/PRMXSSO".into(),
                    html_url: Some(
                        "https://webdemo.pagerduty.com/users/PRMXSSO".into()
                    ),
                },
                User::User {
                    id: "P5T36BU".into(),
                    summary: "abhijit@pagerduty.com".into(),
                    self_: "https://api.pagerduty.com/users/P5T36BU".into(),
                    html_url: Some(
                        "https://webdemo.pagerduty.com/users/P5T36BU".into()
                    ),
                    avatar_url: "https://secure.gravatar.com/avatar/267299c8432bf9ab044472009c89a674.png?d=mm&r=PG".into(),
                    color: "olivedrab".into(),
                    contact_methods: vec![
                        ContactMethod::EmailReference {
                            id: "P6YMJEE".into(),
                            summary: "Default".into(),
                            self_: "https://api.pagerduty.com/users/P5T36BU/contact_methods/P6YMJEE".into(),
                            html_url: None,
                        },
                    ],
                    description: None,
                    email: "abhijit@pagerduty.com".into(),
                    invitation_sent: false,
                    job_title: None,
                    name: "abhijit@pagerduty.com".into(),
                    notification_rules: vec![
                        NotificationRule::Reference {
                            id: "PM41X3T".into(),
                            summary: "0 minutes: channel P6YMJEE".into(),
                            self_: "https://api.pagerduty.com/users/P5T36BU/notification_rules/PM41X3T".into(),
                            html_url: None,
                        },
                        NotificationRule::Reference {
                            id: "PNTRY7M".into(),
                            summary: "0 minutes: channel P6YMJEE".into(),
                            self_: "https://api.pagerduty.com/users/P5T36BU/notification_rules/PNTRY7M".into(),
                            html_url: None,
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

