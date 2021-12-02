use reqwest;

pub async fn get_input_data(day: i32) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    Ok(reqwest::get(url).await?.text().await?)
}