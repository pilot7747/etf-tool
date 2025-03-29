use etf_tool::etf::ETF;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    // Create a sample Invesco ETF
    let mut etf = ETF {
        name: "Invesco S&P 500 UCITS ETF".to_string(),
        isin: "IE00B23D9570".to_string(),
        asset_class: "Equity".to_string(),
        ter: 0.05,
        currency: "USD".to_string(),
        aum: "1000000000".to_string(),
        currency_exposure: "USD".to_string(),
        distribution_policy: "Accumulating".to_string(),
        launch_date: "2010-05-20".to_string(),
        performance_1y: Some(15.0),
        performance_ytd: Some(8.5),
        holdings: Vec::new(),
        issuer: "Invesco".to_string(),
    };
    
    println!("Loading holdings for {} ({})", etf.name, etf.isin);
    
    // Load holdings
    etf.load_holdings()?;
    
    // Print first 10 holdings
    println!("First 10 holdings:");
    for (i, (isin, weight)) in etf.holdings.iter().take(10).enumerate() {
        println!("{}. ISIN: {}, Weight: {:.2}%", i + 1, isin, weight);
    }
    
    println!("Total holdings: {}", etf.holdings.len());
    
    Ok(())
} 