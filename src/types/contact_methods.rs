use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use serde::de::Error;
use serde::ser::SerializeMap;

use types::reference::Reference;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ContactMethodUnion {
    // All Reference's
    id: String,
    summary: String,
    #[serde(rename="type")]
    type_: String,
    #[serde(rename="self")]
    self_: String,
    html_url: Option<String>,

    // All Concrete type fields
    address: Option<String>,
    label: Option<String>,
    send_short_email: Option<bool>,
    send_html_email: Option<bool>,
    blacklisted: Option<bool>,
    country_code: Option<u32>,
    enabled: Option<bool>,
    created_at: Option<String>,
    device_type: Option<String>,
    sounds: Option<Vec<PushContactMethodSound>>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PushContactMethodSound {
    /// The sound file name.
    pub file: String,

    /// The type of sound. Expected values include:
    /// `alert_high_urgency` and `alert_high_urgency`.
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, PartialEq)]
pub enum ContactMethod {
    ContactMethodReference {
        reference: Reference,
    },

    EmailContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// Send an abbreviated email message instead of the standard email
        /// output. Useful for email-to-SMS gateways and email based pagers.
        send_short_email: bool,

        /// Send HTML e-mails.
        send_html_email: bool,
    },

    PhoneContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// If true, this phone has been blacklisted by
        /// PagerDuty and no messages will be sent to it.
        blacklisted: bool,

        /// The 1-to-3 digit country calling code.
        country_code: u32,
    },

    SmsContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// If true, this phone has been blacklisted by
        /// PagerDuty and no messages will be sent to it.
        blacklisted: bool,

        /// The 1-to-3 digit country calling code.
        country_code: u32,

        /// If true, this phone is capable of receiving SMS messages.
        enabled: bool,
    },

    PushNotificationContactMethod{
        reference: Reference,

        /// The `address` to deliver to: email, phone number, etc.,
        ///  depending on the type.
        address: String,

        /// The label (e.g., "Work", "Mobile", etc.).
        label: String,

        /// If true, this phone has been blacklisted by PagerDuty and no messages will be sent to it.",
        blacklisted: bool,

        // TODO(gary): Use date-time field?
        /// Time at which the contact method was created.
        created_at: String,

        /// The type of device. Expected values include:
        /// `ios` and `android`.
        device_type: String,

        sounds: Vec<PushContactMethodSound>,
    },

}


impl Serialize for ContactMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            ContactMethod::ContactMethodReference{
                ref reference
            } => {
                reference.serialize_key_vals(&mut state)?;
            },
            ContactMethod::EmailContactMethod{
                ref reference, ref address, ref label,
                ref send_short_email, ref send_html_email,
            } => {
                reference.serialize_key_vals(&mut state)?;

                state.serialize_key("address")?;
                state.serialize_value(address)?;

                state.serialize_key("label")?;
                state.serialize_value(label)?;

                state.serialize_key("send_short_email")?;
                state.serialize_value(send_short_email)?;

                state.serialize_key("send_html_email")?;
                state.serialize_value(send_html_email)?;
            },
            ContactMethod::PhoneContactMethod{
                ref reference, ref address, ref label,
                ref blacklisted, ref country_code,
            } => {
                reference.serialize_key_vals(&mut state)?;

                state.serialize_key("address")?;
                state.serialize_value(address)?;

                state.serialize_key("label")?;
                state.serialize_value(label)?;

                state.serialize_key("country_code")?;
                state.serialize_value(country_code)?;

                state.serialize_key("blacklisted")?;
                state.serialize_value(blacklisted)?;

            },
            ContactMethod::SmsContactMethod{
                ref reference, ref address, ref label,
                ref blacklisted, ref country_code, ref enabled,
            } => {
                reference.serialize_key_vals(&mut state)?;

                state.serialize_key("address")?;
                state.serialize_value(address)?;

                state.serialize_key("label")?;
                state.serialize_value(label)?;

                state.serialize_key("country_code")?;
                state.serialize_value(country_code)?;

                state.serialize_key("blacklisted")?;
                state.serialize_value(blacklisted)?;

                state.serialize_key("enabled")?;
                state.serialize_value(enabled)?;
            },
            ContactMethod::PushNotificationContactMethod{
                ref reference, ref address, ref label,
                ref blacklisted, ref created_at, ref device_type,
                ref sounds,
            } => {
                reference.serialize_key_vals(&mut state)?;

                state.serialize_key("address")?;
                state.serialize_value(address)?;

                state.serialize_key("label")?;
                state.serialize_value(label)?;

                state.serialize_key("device_type")?;
                state.serialize_value(device_type)?;

                state.serialize_key("sounds")?;
                state.serialize_value(sounds)?;

                state.serialize_key("blacklisted")?;
                state.serialize_value(blacklisted)?;

                state.serialize_key("created_at")?;
                state.serialize_value(created_at)?;
            },
        }

        state.end()
    }
}

impl Deserialize for ContactMethod {
    fn deserialize<D>(deserializer: D) -> Result<ContactMethod, D::Error>
        where D: Deserializer
    {
        let union = ContactMethodUnion::deserialize(deserializer)?;

        let reference = Reference {
            id: union.id,
            summary: union.summary,
            type_: union.type_,
            self_: union.self_,
            html_url: union.html_url,
        };

        match reference.type_.as_ref() {
            "contact_method_reference" |
            "email_contact_method_reference" |
            "phone_contact_method_reference" |
            "sms_contact_method_reference" |
            "push_notification_contact_method_reference"
            => {
                Ok(ContactMethod::ContactMethodReference {
                    reference: reference,
                })
            },
            "email_contact_method" => {
                let address = union.address.ok_or(
                    D::Error::missing_field("address"))?;
                let label = union.label.ok_or(
                    D::Error::missing_field("label"))?;
                let send_short_email = union.send_short_email.ok_or(
                    D::Error::missing_field("send_short_email"))?;
                let send_html_email = union.send_html_email.ok_or(
                    D::Error::missing_field("send_html_email"))?;

                Ok(ContactMethod::EmailContactMethod {
                    reference: reference,
                    address: address,
                    label: label,
                    send_short_email: send_short_email,
                    send_html_email: send_html_email,
                })
            },
            "phone_contact_method" => {
                let address = union.address.ok_or(
                    D::Error::missing_field("address"))?;
                let label = union.label.ok_or(
                    D::Error::missing_field("label"))?;
                let blacklisted = union.blacklisted.ok_or(
                    D::Error::missing_field("blacklisted"))?;
                let country_code = union.country_code.ok_or(
                    D::Error::missing_field("country_code"))?;

                Ok(ContactMethod::PhoneContactMethod {
                    reference: reference,
                    address: address,
                    label: label,
                    blacklisted: blacklisted,
                    country_code: country_code,
                })
            },
            "sms_contact_method" => {
                let address = union.address.ok_or(
                    D::Error::missing_field("address"))?;
                let label = union.label.ok_or(
                    D::Error::missing_field("label"))?;
                let blacklisted = union.blacklisted.ok_or(
                    D::Error::missing_field("blacklisted"))?;
                let country_code = union.country_code.ok_or(
                    D::Error::missing_field("country_code"))?;
                let enabled = union.enabled.ok_or(
                    D::Error::missing_field("enabled"))?;

                Ok(ContactMethod::SmsContactMethod {
                    reference: reference,
                    address: address,
                    label: label,
                    blacklisted: blacklisted,
                    country_code: country_code,
                    enabled: enabled,
                })
            },
            "push_notification_contact_method" => {
                let address = union.address.ok_or(
                    D::Error::missing_field("address"))?;
                let label = union.label.ok_or(
                    D::Error::missing_field("label"))?;
                let blacklisted = union.blacklisted.ok_or(
                    D::Error::missing_field("blacklisted"))?;
                let created_at = union.created_at.ok_or(
                    D::Error::missing_field("created_at"))?;
                let device_type = union.device_type.ok_or(
                    D::Error::missing_field("device_type"))?;
                let sounds = union.sounds.ok_or(
                    D::Error::missing_field("sounds"))?;

                Ok(ContactMethod::PushNotificationContactMethod {
                    reference: reference,
                    address: address,
                    label: label,
                    blacklisted: blacklisted,
                    created_at: created_at,
                    device_type: device_type,
                    sounds: sounds,
                })
            },
            _ => Err(D::Error::custom("type received was unexpected.")),
        }
    }
}


pub type ContactMethods = Vec<ContactMethod>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use types::reference::Reference;

    #[test]
    fn test_serde() {
        let mut file = File::open("testdata/types/contact_methods.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let contact_methods: ContactMethods = serde_json::from_str(&data).unwrap();

        // Verify deserialization.
        assert_eq!(
            contact_methods,
            vec![

                ContactMethod::ContactMethodReference {
                    reference: Reference {
                        id: "PPPIOPG".into(),
                        summary: "Default".into(),
                        type_: "email_contact_method_reference".into(),
                        self_: "https://api.pagerduty.com/users/PZ7JFQ7/contact_methods/PPPIOPG".into(),
                        html_url: None,
                    },
                },
                ContactMethod::EmailContactMethod {
                    reference: Reference {
                        id: "P33R0ZA".into(),
                        summary: "Work".into(),
                        type_: "email_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/PZ7JFQ7/contact_methods/P33R0ZA".into(),
                        html_url: None,
                    },
                    address: "alejandro@example.com".into(),
                    label: "Work".into(),
                    send_short_email: false,
                    send_html_email: false,
                },
                ContactMethod::SmsContactMethod {
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
                ContactMethod::PhoneContactMethod {
                    reference: Reference {
                        id: "PBUSVMD".into(),
                        summary: "Mobile".into(),
                        type_: "phone_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/P1RQ0Z6/contact_methods/PBUSVMD".into(),
                        html_url: None,
                    },
                    address: "7076949626".into(),
                    label: "Mobile".into(),
                    blacklisted: false,
                    country_code: 1,
                },
                ContactMethod::PushNotificationContactMethod {
                    reference: Reference {
                        id: "P4G3JKD".into(),
                        summary: "Alex\'s iPhone".into(),
                        type_: "push_notification_contact_method".into(),
                        self_: "https://api.pagerduty.com/users/P1RQ0Z6/contact_methods/P4G3JKD".into(),
                        html_url: None,
                    },
                    address: "fcbaba06abe7533794b0dd7c3f4427b574772c01445e06bb5a006c33f14d95d0".into(),
                    label: "Alex\'s iPhone".into(),
                    blacklisted: false,
                    created_at: "2016-07-11T11:36:41-07:00".into(),
                    device_type: "ios".into(),
                    sounds: vec![
                        PushContactMethodSound {
                            file: "default".into(),
                            type_: "alert_high_urgency".into(),
                        }
                    ],
                }
            ]
        );

        // Verify that serialization round-trips.
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&contact_methods).unwrap().as_ref()
        ).unwrap();
        assert_eq!(serialized, expected)
    }
}

