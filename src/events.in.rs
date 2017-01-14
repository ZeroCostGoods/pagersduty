use serde::ser::{Serialize, Serializer};
use serde_json::value;

static EVENTS_URL: &'static str = "https://events.pagerduty.com/generic/2010-04-15/create_event.json";

#[derive(Debug, PartialEq)]
pub enum Context {

    /// The link type is used to attach hyperlinks to an incident.
    Link {
        /// The link being attached to the incident.
        href: String,

        /// Plain text that describes the purpose of the link, and can be used as the link's text.
        text: Option<String>,
    },

    /// The image type is used to attach images to an incident. Images must be served via HTTPS.
    Image {
        /// The source of the image being attached to the incident. This image must be served via HTTPS.
        src: String,

        /// Optional link for the image.
        href: Option<String>,

        /// Optional alternative text for the image.
        alt: Option<String>,
    }
}

impl Serialize for Context {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(None)?;

        match *self {
            Context::Link{ ref href, ref text } => {
                serializer.serialize_map_key(&mut state, "type")?;
                serializer.serialize_map_value(&mut state, "link")?;

                serializer.serialize_map_key(&mut state, "href")?;
                serializer.serialize_map_value(&mut state, href)?;

                if let Some(ref text) = *text {
                    serializer.serialize_map_key(&mut state, "text")?;
                    serializer.serialize_map_value(&mut state, text)?;
                }
            },
            Context::Image{ ref src, ref href, ref alt } => {
                serializer.serialize_map_key(&mut state, "type")?;
                serializer.serialize_map_value(&mut state, "image")?;

                serializer.serialize_map_key(&mut state, "src")?;
                serializer.serialize_map_value(&mut state, src)?;

                if let Some(ref href) = *href {
                    serializer.serialize_map_key(&mut state, "href")?;
                    serializer.serialize_map_value(&mut state, href)?;
                }

                if let Some(ref alt) = *alt {
                    serializer.serialize_map_key(&mut state, "alt")?;
                    serializer.serialize_map_value(&mut state, alt)?;
                }
            },
        };

        serializer.serialize_map_end(state)
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
    details: Option<value::Value>,

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

    pub fn incident_key(&mut self, incident_key: String) -> &mut TriggerEvent {
        self.incident_key = Some(incident_key);
        self
    }

    pub fn details(&mut self, details: value::Value) -> &mut TriggerEvent {
        self.details = Some(details);
        self
    }

    pub fn client(&mut self, client: String) -> &mut TriggerEvent {
        self.client = Some(client);
        self
    }

    pub fn client_url(&mut self, client_url: String) -> &mut TriggerEvent {
        self.client_url = Some(client_url);
        self
    }

    pub fn contexts(&mut self, contexts: Vec<Context>) -> &mut TriggerEvent {
        self.contexts = Some(contexts);
        self
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub enum HandleEventType {

    /// Acknowledge events cause the referenced incident to enter the acknowledged state.
    ///
    /// While an incident is acknowledged, it won't generate any additional
    /// notifications, even if it receives new trigger events.
    ///
    /// Your monitoring tools should send PagerDuty an acknowledge event when they know
    /// someone is presently working on the problem.
    Acknowledge,

    /// Resolve events cause the referenced incident to enter the resolved state.
    ///
    /// Once an incident is resolved, it won't generate any additional notifications.
    /// New trigger events with the same incident_key as a resolved incident won't
    /// re-open the incident. Instead, a new incident will be created.
    ///
    /// Your monitoring tools should send PagerDuty a resolve event when the problem
    /// that caused the initial trigger event has been fixed.
    Resolve,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct HandleEvent {

    /// The GUID of one of your "Generic API" services. This is the
    /// "Integration Key" listed on a Generic API's service detail page.
    service_key: String,

    /// The type of event. Can be trigger, acknowledge or resolve.
    event_type: String,

    /// Identifies the incident to acknowledge or resolve.
    incident_key: String,
}

impl HandleEvent {
    pub fn new(service_key: String, event_type: HandleEventType, incident_key: String) -> HandleEvent {
        let event_type = match event_type {
            HandleEventType::Acknowledge => "acknowledge".into(),
            HandleEventType::Resolve => "resolve".into(),
        };

        HandleEvent {
            service_key: service_key,
            event_type: event_type,
            incident_key: incident_key,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;

    #[test]
    fn test_serialization(){
        let event = TriggerEvent::new("Some key".into(), "some description".into());
        let ser = serde_json::to_string_pretty(&event).unwrap();

        print!("{:#?}", ser);

    }
}
