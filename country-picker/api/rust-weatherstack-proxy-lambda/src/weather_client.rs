
pub fn get_weather(country:&str) -> Result<String, reqwest::Error>  {
    let api_key: &'static str = env!("API_KEY","You forgot to export API_KEY path");
    let url = format!(
        "http://api.weatherstack.com/current?access_key={apiKey}&query={countryName}",
        apiKey=api_key,
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

    //#[test]
    fn test_add() {
        let weather = get_weather("sweden");
    }
}