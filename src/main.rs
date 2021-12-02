use advent_of_code::get_input_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_data = get_input_data(1).await;
    println!("{:?}", input_data);
    Ok(())
}