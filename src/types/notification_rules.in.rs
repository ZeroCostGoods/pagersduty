use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::reference::Reference;
use super::contact_methods::ContactMethod;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NotificationRuleUnion {
    // All Reference's
    id: String,
    summary: String,
    #[serde(rename="type")]
    type_: String,
    #[serde(rename="self")]
    self_: String,
    html_url: Option<String>,

    // All Concrete type fields
    start_delay_in_minutes: Option<u32>,
    contact_method: Option<ContactMethod>,
    urgency: Option<String>,
}


#[derive(Debug, PartialEq)]
pub enum NotificationRule {
    NotificationRuleReference {
        reference: Reference,
    },

    NotificationRule {
        reference: Reference,

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


impl Serialize for NotificationRule {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            NotificationRule::NotificationRuleReference{
                ref reference
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;
            },
            NotificationRule::NotificationRule{
                ref reference, ref start_delay_in_minutes,
                ref contact_method, ref urgency,
            } => {
                reference.serialize_key_vals(serializer, &mut state)?;

                serializer.serialize_map_key(&mut state, "start_delay_in_minutes")?;
                serializer.serialize_map_value(&mut state, start_delay_in_minutes)?;

                serializer.serialize_map_key(&mut state, "contact_method")?;
                serializer.serialize_map_value(&mut state, contact_method)?;

                serializer.serialize_map_key(&mut state, "urgency")?;
                serializer.serialize_map_value(&mut state, urgency)?;
            },
        }

        serializer.serialize_map_end(state)
    }
}

impl Deserialize for NotificationRule {
    fn deserialize<D>(deserializer: &mut D) -> Result<NotificationRule, D::Error>
        where D: Deserializer
    {
        let union = NotificationRuleUnion::deserialize(deserializer)?;

        let reference = Reference {
            id: union.id,
            summary: union.summary,
            type_: union.type_,
            self_: union.self_,
            html_url: union.html_url,
        };

        match reference.type_.as_ref() {
            "assignment_notification_rule_reference"  => {
                Ok(NotificationRule::NotificationRuleReference {
                    reference: reference,
                })
            },
            "assignment_notification_rule" => {
                Ok(NotificationRule::NotificationRule {
                    reference: reference,
                    start_delay_in_minutes: union.start_delay_in_minutes.expect("start_delay_in_minutes"),
                    contact_method: union.contact_method.expect("contact_method"),
                    urgency: union.urgency.expect("urgency"),
                })
            },
            _ => panic!("fuuuuuuu"),
        }
    }
}


pub type NotificationRules = Vec<NotificationRule>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use super::super::reference::Reference;
    use super::super::contact_methods::ContactMethod;

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
                NotificationRule::NotificationRule {
                    reference: Reference {
                        id: "PW0WSB8".into(),
                        summary: "0 minutes: channel PX11CYC".into(),
                        type_: "assignment_notification_rule".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/notification_rules/PW0WSB8".into(),
                        html_url: None,
                    },
                    start_delay_in_minutes: 0,
                    contact_method: ContactMethod::EmailContactMethod {
                        reference: Reference {
                            id: "PX11CYC".into(),
                            summary: "Default".into(),
                            type_: "email_contact_method".into(),
                            self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PX11CYC".into(),
                            html_url: None,
                        },
                        address: "acunningham@pagerduty.com".into(),
                        label: "Default".into(),
                        send_short_email: false,
                        send_html_email: false,
                    },
                    urgency: "high".into(),
                },
                NotificationRule::NotificationRule {
                    reference: Reference {
                        id: "PGITUFO".into(),
                        summary: "0 minutes: channel PX11CYC".into(),
                        type_: "assignment_notification_rule".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/notification_rules/PGITUFO".into(),
                        html_url: None,
                    },
                    start_delay_in_minutes: 0,
                    contact_method: ContactMethod::EmailContactMethod {
                        reference: Reference {
                            id: "PX11CYC".into(),
                            summary: "Default".into(),
                            type_: "email_contact_method".into(),
                            self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PX11CYC".into(),
                            html_url: None,
                        },
                        address: "acunningham@pagerduty.com".into(),
                        label: "Default".into(),
                        send_short_email: false,
                        send_html_email: false,
                    },
                    urgency: "low".into(),
                },
                NotificationRule::NotificationRule {
                    reference: Reference {
                        id: "PEY06R9".into(),
                        summary: "0 minutes: channel PEC83HY".into(),
                        type_: "assignment_notification_rule".into(),
                        self_: "https://api.pagerduty.com/users/PGJ36Z3/notification_rules/PEY06R9".into(),
                        html_url: None,
                    },
                    start_delay_in_minutes: 0,
                    contact_method: ContactMethod::SmsContactMethod {
                        reference: Reference {
                            id: "PEC83HY".into(),
                            summary: "Mobile".into(),
                            type_: "sms_contact_method".into(),
                            self_: "https://api.pagerduty.com/users/PGJ36Z3/contact_methods/PEC83HY".into(),
                            html_url: None,
                        },
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

