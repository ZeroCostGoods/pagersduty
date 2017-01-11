use hyper;
use hyper::client::response::{Response};
use hyper::header::{Headers, Accept, Authorization, qitem};
use hyper::mime::{Mime};

static PD_API_URL: &'static str = "https://api.pagerduty.com";

static PD_API_MIME_TYPE: &'static str = "application/vnd.pagerduty+json;version=2";


#[derive(Debug, PartialEq)]
pub struct Client {
    auth_token: String
}


impl Client {
    pub fn new<T: Into<String>>(auth_token: T) -> Client {
        Client {
            auth_token: auth_token.into(),
        }
    }

    // TODO(gary): Take an optional user argument to produce a `From` header.
    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(self.get_accept_header());
        headers.set(Authorization(
            format!("Token token={}", self.auth_token)
        ));

        headers
    }

    fn get_accept_header(&self) -> Accept {
        let mime: Mime = PD_API_MIME_TYPE.parse().unwrap();
        Accept(vec![qitem(mime)])
    }

    pub fn get(&self, path: &str) -> Response {

        let http_client = hyper::Client::new();

        let url = format!("{}/{}", PD_API_URL, path);

        http_client.get(&url)
            .headers(self.get_headers())
            .send()
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client() {
        // Test instantiation with &str
        assert_eq!(
            Client::new("Hello, World!"),
            Client {
                auth_token: "Hello, World!".into(),
            }
        );

        // Test instantiation with String
        assert_eq!(
            Client::new(String::from("Hello, World!")),
            Client {
                auth_token: "Hello, World!".into(),
            }
        );
    }
}
