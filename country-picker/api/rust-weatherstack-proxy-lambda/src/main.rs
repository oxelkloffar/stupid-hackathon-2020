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

#[derive(Deserialize, Clone)]
struct CustomEvent {
    country: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.country == "" {
        error!("Empty country in request {}", c.aws_request_id);
        return Err(c.new_error("Empty country"));
    }
    let weather = weather_client::get_weather(e.country)
        .map_err(|err|c.new_error(err.to_string().as_str()))?;
    Ok(CustomOutput { message: weather })
}
