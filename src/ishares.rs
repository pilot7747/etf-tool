use color_eyre::Result;
use crate::etf::ETF;
use crate::utils;
use reqwest::blocking::Client;
use serde_json::Value;

const ISSUER: &str = "iShares";

fn fetch_ishares_product_urls() -> Result<Vec<(String, String)>> {
    let url = "https://www.ishares.com/us/product-screener/product-screener-v3.1.jsn?dcrPath=/templatedata/config/product-screener-v3/data/en/us-ishares/ishares-product-screener-backend-config&siteEntryPassthrough=true";
    
    println!("Fetching product URLs from iShares API...");
    
    let client = Client::new();
    let response = client.get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()?;
    
    if !response.status().is_success() {
        return Err(color_eyre::eyre::eyre!("Failed to fetch product URLs: HTTP status {}", response.status()));
    }
    
    let json: Value = response.json()?;
    let mut product_urls = Vec::new();
    
    // Extract products from the JSON
    if let Some(products) = json.get("products").and_then(|p| p.as_array()) {
        for product in products {
            if let (Some(isin), Some(product_url)) = (
                product.get("isin").and_then(|i| i.as_str()),
                product.get("productPageUrl").and_then(|u| u.as_str())
            ) {
                product_urls.push((isin.to_string(), product_url.to_string()));
            }
        }
    }
    
    println!("Found {} product URLs in the API response", product_urls.len());
    Ok(product_urls)
}

pub fn get_ishares_etfs() -> Result<Vec<ETF>> {
    let file_path = "data/iShares-UnitedKingdom.xls";

    if !std::path::Path::new(file_path).exists() {
        panic!("File does not exist at: {}", file_path);
    }

    let raw_data = utils::read_xlsx(file_path)?;
    
    // Find the header row that contains "Fund Name"
    let header_row_index = raw_data.iter()
        .position(|row| row.get(1).map_or(false, |cell| cell == "Fund Name"))
        .unwrap_or(0);

    // Fetch product URLs from the API
    let product_urls = fetch_ishares_product_urls()?;
    let mut url_map: std::collections::HashMap<String, String> = product_urls.into_iter().collect();

    // Process data starting from the row after headers
    let mut etfs: Vec<ETF> = raw_data.iter()
        .skip(header_row_index + 2) // Skip both header rows
        .filter_map(|row| {
            // Skip empty rows or rows that don't look like ETF data
            if row.len() < 4 || row[1].is_empty() || row[3].is_empty() || 
               row[1].contains("TER / OCF") || row[1].contains("AUM") || 
               row[1].contains("As Of") {
                return None;
            }

            // Parse TER (column 26)
            let ter = row.get(26)
                .and_then(|s| s.trim().replace('%', "").replace(',', "").parse().ok())
                .unwrap_or(0.0);

            // Parse AUM (column 27)
            let aum = row.get(27)
                .map(|s| s.to_string())
                .unwrap_or_default();

            // Get distribution type from the data
            let distribution_policy = if row[1].contains("(Dist)") {
                "Distributing".to_string()
            } else if row[1].contains("(Acc)") {
                "Accumulating".to_string()
            } else {
                "Unknown".to_string()
            };

            let isin = row[3].clone();
            let product_url = url_map.remove(&isin);

            Some(ETF {
                name: row[1].clone(), // Fund Name
                isin, // ISIN
                asset_class: row[2].clone(), // Fund type
                ter,
                currency: row[4].clone(), // Share Class Currency
                aum,
                currency_exposure: row[5].clone(), // Share Class
                distribution_policy,
                launch_date: String::new(), // TODO: Find the correct column
                performance_1y: None, // TODO: Find the correct column for 1Y performance
                performance_ytd: None, // TODO: Find the correct column for YTD performance
                holdings: Vec::new(), // Initialize with empty holdings
                issuer: ISSUER.to_string(),
                product_url,
            })
        })
        .collect();

    // Print statistics about URL matching
    let matched_count = etfs.iter().filter(|etf| etf.product_url.is_some()).count();
    println!("Matched product URLs for {}/{} iShares ETFs", matched_count, etfs.len());

    Ok(etfs)
} 