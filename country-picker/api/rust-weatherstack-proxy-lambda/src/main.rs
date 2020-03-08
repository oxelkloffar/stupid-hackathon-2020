#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate simple_logger;

use std::error::Error;

use lambda::error::HandlerError;

mod weather_client;

// Struct that will hold information of the request.
// When we use an API Gateway as a proxy, which is the default
// behaviour when we create it from the Lambda website, the request
// will have a specific format with many different parameters.
// We're only going to use `queryStringParameters` to check the
// query string parameters (normally for GET requests) and `body`
// to check for messages usually coming from POST requests.
#[derive(Deserialize, Clone, Debug)]
struct CustomEvent {
    // note that we're using serde to help us to change
    // the names of parameters accordingly to conventions.
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<QueryString>,
    body: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct QueryString {
    #[serde(rename = "country")]
    country: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct Body {
    #[serde(rename = "country")]
    country: Option<String>,
}

// Struct used for our function's response.
// Note again that we're using `serde`.
// It's also important to notice that you will need to explicitely
// inform these properties for our API Gateway to work.
// If you miss some of these properties you will likely get
// a 502 error.
#[derive(Serialize, Clone, Debug)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    headers: Header,
    body: String,
}

#[derive(Serialize, Clone, Debug)]
struct Header {
    #[serde(rename = "Access-Control-Allow-Origin")]
    cors: String,
}

// Just a static method to help us build the `CustomOutput`.
impl CustomOutput {
    fn new(body: String) -> Self {
        let cors = "*";
        CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body,
            headers: Header {cors: cors.to_owned()},
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    info!("Received event: {:?}", e);

    let country = e.query_string_parameters
        .map(|q| q.country)
        .flatten();
    let country = country.or(e.body);
    let country = country.as_deref();

    match country {
        Some("") => {
            error!("Empty country in request {}", c.aws_request_id);
            Err(c.new_error("Empty country"))
        }
        Some(country) => {
            let weather = weather_client::get_weather(country)
                .map_err(|err| c.new_error(err.to_string().as_str()))?;
            Ok(CustomOutput::new(weather))
        }
        None => Err(c.new_error("No country"))
    }
}
