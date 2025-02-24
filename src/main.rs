mod models;

use anyhow::Result;
use chrono::NaiveDate;


#[tokio::main]
async fn main() -> Result<()> {
    // Starting Jan 1st, adjust year if needed.
    let start_date = NaiveDate::from_ymd(2025, 1, 1);
    let end_date = chrono::Utc::today().naive_utc();
    let symbols = vec!["BTC", "SOL"];
    
    for symbol in symbols {
        let mut all_klines = Vec::new();
        let mut current_date = start_date;
        // Loop through each day.
        while current_date <= end_date {
            println!("Fetching {} klines for {}", symbol, current_date);
            match fetch_klines_for_date(symbol, current_date).await {
                Ok(data) => {
                    println!("Got {} klines for {}", data.len(), current_date);
                    all_klines.extend(data);
                },
                Err(e) => {
                    println!("Error fetching {} on {}: {}", symbol, current_date, e);
                },
            }
            current_date = current_date.succ();
        }
        store_klines(symbol, &all_klines)?;
    }
    Ok(())
}