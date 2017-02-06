use std::io::Read;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde::ser::Serialize;

use serde_json;

static EVENTS_URL: &'static str = "https://events.pagerduty.com/generic/2010-04-15/create_event.json";

fn get_https_client() -> Client {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    Client::with_connector(connector)
}


pub fn send<T: Serialize>(event: T) {
    let client = get_https_client();
    let body = serde_json::to_string(&event).unwrap();
    let mut output = String::new();

    let mut response = client
        .post(EVENTS_URL)
        .body(&body)
        .send()
        .unwrap();

    response.read_to_string(&mut output).unwrap();

    println!("{:#?}", response);
    println!("{:#?}", output);
}


#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Context {

    /// The link type is used to attach hyperlinks to an incident.
    #[serde(rename="link")]
    Link {
        /// The link being attached to the incident.
        href: String,

        /// Plain text that describes the purpose of the link, and can be used as the link's text.
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },

    /// The image type is used to attach images to an incident. Images must be served via HTTPS.
    #[serde(rename="image")]
    Image {
        /// The source of the image being attached to the incident. This image must be served via HTTPS.
        src: String,

        /// Optional link for the image.
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,

        /// Optional alternative text for the image.
        #[serde(skip_serializing_if = "Option::is_none")]
        alt: Option<String>,
    }
}


#[derive(Serialize, Debug, PartialEq)]
pub struct TriggerEvent {

    /// The GUID of one of your "Generic API" services. This is the
    /// "Integration Key" listed on a Generic API's service detail page.
    service_key: String,

    /// The type of event. Can be trigger, acknowledge or resolve.
    event_type: String,

    /// Text that will appear in the incident's log associated with this event. Required for trigger events.
    description: String,

    /// Identifies the incident to trigger, acknowledge, or resolve. Required unless the event_type is trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_key: Option<String>,

    /// An arbitrary JSON object containing any data you'd like included in the incident log.
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::value::Value>,

    /// The name of the monitoring client that is triggering this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<String>,

    /// The URL of the monitoring client that is triggering this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    client_url: Option<String>,

    /// Contexts to be included with the incident trigger such as links to graphs or images.
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<Vec<Context>>,
}

impl TriggerEvent {
    pub fn new(service_key: String, description: String) -> TriggerEvent {
        TriggerEvent {
            service_key: service_key,
            event_type: "trigger".into(),
            description: description,
            incident_key: None,
            details: None,
            client: None,
            client_url: None,
            contexts: None,
        }
    }

    pub fn incident_key(mut self, incident_key: String) -> TriggerEvent {
        self.incident_key = Some(incident_key);
        self
    }

    pub fn details(mut self, details: serde_json::value::Value) -> TriggerEvent {
        self.details = Some(details);
        self
    }

    pub fn client(mut self, client: String) -> TriggerEvent {
        self.client = Some(client);
        self
    }

    pub fn client_url(mut self, client_url: String) -> TriggerEvent {
        self.client_url = Some(client_url);
        self
    }

    pub fn contexts(mut self, contexts: Vec<Context>) -> TriggerEvent {
        self.contexts = Some(contexts);
        self
    }

    pub fn send(self) {
        send(self)
    }
}

/// Acknowledge events cause the referenced incident to enter the acknowledged state.
///
/// While an incident is acknowledged, it won't generate any additional
/// notifications, even if it receives new trigger events.
///
/// Your monitoring tools should send PagerDuty an acknowledge event when they know
/// someone is presently working on the problem.
#[derive(Serialize, Debug, PartialEq)]
pub struct AcknowledgeEvent {

    /// The GUID of one of your "Generic API" services. This is the
    /// "Integration Key" listed on a Generic API's service detail page.
    service_key: String,

    /// The type of event. Can be trigger, acknowledge or resolve.
    event_type: String,

    /// Identifies the incident to acknowledge or resolve.
    incident_key: String,
}

impl AcknowledgeEvent {
    pub fn new(service_key: String, incident_key: String) -> AcknowledgeEvent {
        AcknowledgeEvent {
            service_key: service_key,
            event_type: "acknowledge".into(),
            incident_key: incident_key,
        }
    }

    pub fn send(self) {
        send(self)
    }
}

/// Resolve events cause the referenced incident to enter the resolved state.
///
/// Once an incident is resolved, it won't generate any additional notifications.
/// New trigger events with the same incident_key as a resolved incident won't
/// re-open the incident. Instead, a new incident will be created.
///
/// Your monitoring tools should send PagerDuty a resolve event when the problem
/// that caused the initial trigger event has been fixed.
#[derive(Serialize, Debug, PartialEq)]
pub struct ResolveEvent {

    /// The GUID of one of your "Generic API" services. This is the
    /// "Integration Key" listed on a Generic API's service detail page.
    service_key: String,

    /// The type of event. Can be trigger, acknowledge or resolve.
    event_type: String,

    /// Identifies the incident to acknowledge or resolve.
    incident_key: String,
}

impl ResolveEvent {
    pub fn new(service_key: String, incident_key: String) -> ResolveEvent {
        ResolveEvent {
            service_key: service_key,
            event_type: "resolve".into(),
            incident_key: incident_key,
        }
    }

    pub fn send(self) {
        send(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;


    #[test]
    fn test_trigger_event_serialization_1(){
        let event = TriggerEvent::new("Some key".into(), "some description".into());
        let json: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&event).unwrap().as_ref()
        ).unwrap();

        let mut file = File::open("testdata/events/trigger_event_serialization_1.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();

        assert_eq!(json, expected);
    }

    #[test]
    fn test_trigger_event_serialization_2(){

        let details = json!({
            "key1": "value1",
            "key2": 3.14,
        });

        let contexts = vec![
            Context::Link {
                href: "http://zombo.com/".into(),
                text: Some("You can do anything at Zombo.com".into()),
            },
            Context::Image {
                src: "http://localhost/nope.gif".into(),
                href: None,
                alt: None,
            },
        ];



        let event = TriggerEvent::new("Some key".into(), "some description".into())
            .details(details)
            .incident_key("AG23DR1".into())
            .client("pagersduty-test".into())
            .client_url("http://localhost".into())
            .contexts(contexts);


        let json: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&event).unwrap().as_ref()
        ).unwrap();


        let mut file = File::open("testdata/events/trigger_event_serialization_2.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();

        assert_eq!(json, expected);
    }

    #[test]
    fn test_acknowledge_event_serialization(){
        let event = AcknowledgeEvent::new(
            "Some key".into(), "ASF123S".into()
        );
        let json: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&event).unwrap().as_ref()
        ).unwrap();

        let mut file = File::open("testdata/events/handle_event_serialization_1.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();

        assert_eq!(json, expected);
    }

    #[test]
    fn test_resolve_event_serialization(){
        let event = ResolveEvent::new(
            "Some key".into(), "ASF123S".into()
        );
        let json: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&event).unwrap().as_ref()
        ).unwrap();

        let mut file = File::open("testdata/events/handle_event_serialization_2.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();

        assert_eq!(json, expected);
    }
}
