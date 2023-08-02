use aoc::day1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (largest, top3) = day1::solve();
    println!("{}", largest);
    println!("{}", top3);
    Ok(())
}
