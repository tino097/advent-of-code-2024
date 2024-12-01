use std::error::Error;
use std::fs;
use std::path::Path;
use dotenv::dotenv;

pub fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
    // First check if we have cached the input
    let filename = format!("inputs/day{:02}.txt", day);
    if Path::new(&filename).exists() {
        return Ok(fs::read_to_string(filename)?);
    }

    // If not, fetch it from the website
    dotenv().ok();
    let session = std::env::var("AOC_SESSION")?;
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://adventofcode.com/2024/day/{}/input", day))
        .header("Cookie", format!("session={}", session))
        .send()?
        .text()?;

    // Cache the input for future runs
    fs::create_dir_all("inputs")?;
    fs::write(&filename, &response)?;

    Ok(response)
}