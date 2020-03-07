
pub fn get_weather(country:String) -> Result<String, reqwest::Error>  {
    let url = format!(
        "http://api.weatherstack.com/current?access_key={apiKey}&query={countryName}",
        apiKey="5b4511e0bc65a0fdf3c4349d57a1ad2e",
        countryName=country,
    );
    let body = reqwest::blocking::get(url.as_str())?
        .text()?;

    println!("body = {:?}", body);
    Result::Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let weather = get_weather("sweden".to_owned());
    }
}