use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let response = reqwest::blocking::get(url)?;

    let content = response.text()?;
    println!("{}", content);
    Ok(())
}
