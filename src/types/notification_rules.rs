use types::contact_methods::ContactMethod;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum NotificationRule {
    #[serde(rename="assignment_notification_rule_reference")]
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

    #[serde(rename="assignment_notification_rule")]
    NotificationRule {
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


        /// The delay before firing the rule, in minutes.
        start_delay_in_minutes: u32,

        /// The contact method invoked by the rule.
        contact_method: ContactMethod,

        /// Which incident urgency this rule is used for. Account must have the
        ///  `urgencies` ability to have a low urgency notification rule.
        /// Expected values include: `high` and `low`.
        urgency: String,
    },
}


pub type NotificationRules = Vec<NotificationRule>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

    use types::contact_methods::ContactMethod;

    #[test]
    fn test_serde() {
        let mut file = File::open("testdata/types/notification_rules.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let notification_rules: NotificationRules = serde_json::from_str(&data).unwrap();

        // Verify deserialization.
        assert_eq!(
            notification_rules,
            vec![
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
                NotificationRule::NotificationRule {
                    id: "PW0WSB8".into(),
                    summary: "0 minutes: channel PX11CYC".into(),
                    self_: "https://api.pagerduty.com/users/PGJ36Z3/notification_rules/PW0WSB8".into(),
                    html_url: None,
                    start_delay_in_minutes: 0,
                    contact_method: ContactMethod::Email {
                        id: "PX11CYC".into(),
                        summary: "Default".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PX11CYC".into(),
                        html_url: None,
                        address: "acunningham@pagerduty.com".into(),
                        label: "Default".into(),
                        send_short_email: false,
                        send_html_email: false,
                    },
                    urgency: "high".into(),
                },
                NotificationRule::NotificationRule {
                    id: "PGITUFO".into(),
                    summary: "0 minutes: channel PX11CYC".into(),
                    self_: "https://api.pagerduty.com/users/PGJ36Z3/notification_rules/PGITUFO".into(),
                    html_url: None,
                    start_delay_in_minutes: 0,
                    contact_method: ContactMethod::Email {
                        id: "PX11CYC".into(),
                        summary: "Default".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PX11CYC".into(),
                        html_url: None,
                        address: "acunningham@pagerduty.com".into(),
                        label: "Default".into(),
                        send_short_email: false,
                        send_html_email: false,
                    },
                    urgency: "low".into(),
                },
                NotificationRule::NotificationRule {
                    id: "PEY06R9".into(),
                    summary: "0 minutes: channel PEC83HY".into(),
                    self_: "https://api.pagerduty.com/users/PGJ36Z3/notification_rules/PEY06R9".into(),
                    html_url: None,
                    start_delay_in_minutes: 0,
                    contact_method: ContactMethod::Sms {
                        id: "PEC83HY".into(),
                        summary: "Mobile".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PEC83HY".into(),
                        html_url: None,
                        address: "4155809923".into(),
                        label: "Mobile".into(),
                        blacklisted: false,
                        country_code: 1,
                        enabled: true,
                    },
                    urgency: "high".into(),
                },
            ]
        );

        // Verify that serialization round-trips.
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&notification_rules).unwrap().as_ref()
        ).unwrap();
        assert_eq!(serialized, expected)
    }
}

