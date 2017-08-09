#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PushContactMethodSound {
    /// The sound file name.
    pub file: String,

    /// The type of sound. Expected values include:
    /// `alert_high_urgency` and `alert_high_urgency`.
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ContactMethod {
    #[serde(rename="contact_method_reference")]
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

    #[serde(rename="email_contact_method_reference")]
    EmailReference {
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

    #[serde(rename="email_contact_method")]
    Email{
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

    #[serde(rename="phone_contact_method_reference")]
    PhoneReference {
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

    #[serde(rename="phone_contact_method")]
    Phone{
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

    #[serde(rename="sms_contact_method_reference")]
    SmsReference {
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

    #[serde(rename="sms_contact_method")]
    Sms{
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

    #[serde(rename="push_notification_contact_method_reference")]
    PushNotificationReference {
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

    #[serde(rename="push_notification_contact_method")]
    PushNotification{
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


pub type ContactMethods = Vec<ContactMethod>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

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

                ContactMethod::EmailReference {
                    id: "PPPIOPG".into(),
                    summary: "Default".into(),
                    self_: "https://api.pagerduty.com/users/PZ7JFQ7/contact_methods/PPPIOPG".into(),
                    html_url: None,
                },
                ContactMethod::Email {
                    id: "P33R0ZA".into(),
                    summary: "Work".into(),
                    self_: "https://api.pagerduty.com/users/PZ7JFQ7/contact_methods/P33R0ZA".into(),
                    html_url: None,
                    address: "alejandro@example.com".into(),
                    label: "Work".into(),
                    send_short_email: false,
                    send_html_email: false,
                },
                ContactMethod::Sms {
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
                ContactMethod::Phone {
                    id: "PBUSVMD".into(),
                    summary: "Mobile".into(),
                    self_: "https://api.pagerduty.com/users/P1RQ0Z6/contact_methods/PBUSVMD".into(),
                    html_url: None,
                    address: "7076949626".into(),
                    label: "Mobile".into(),
                    blacklisted: false,
                    country_code: 1,
                },
                ContactMethod::PushNotification {
                    id: "P4G3JKD".into(),
                    summary: "Alex\'s iPhone".into(),
                    self_: "https://api.pagerduty.com/users/P1RQ0Z6/contact_methods/P4G3JKD".into(),
                    html_url: None,
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

