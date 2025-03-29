use serde::{Deserialize, Serialize};
use color_eyre::Result;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ETF {
    pub name: String,
    pub isin: String,
    pub asset_class: String,
    pub ter: f64,
    pub currency: String,
    pub aum: String,
    pub currency_exposure: String,
    pub distribution_policy: String,
    pub launch_date: String,
    pub performance_1y: Option<f64>,
    pub performance_ytd: Option<f64>,
    pub holdings: Vec<(String, f64)>, // (ISIN, weight in percent)
    pub issuer: String,
    pub product_url: Option<String>, // URL to the product page
}

impl ETF {
    pub fn from_row(row: &[String], issuer: String) -> Option<Self> {
        // Skip empty rows or rows that don't look like ETF data
        if row.len() < 17 || row[0].is_empty() || row[1].is_empty() || 
           row[0].starts_with("Past performance") || row[0].starts_with("©") {
            return None;
        }

        Some(Self {
            name: row[0].clone(),
            isin: row[1].clone(),
            asset_class: row[2].clone(),
            ter: row[3].trim().replace('%', "").parse().unwrap_or(0.0),
            currency: row[4].clone(),
            aum: row[5].clone(),
            currency_exposure: row[6].clone(),
            distribution_policy: row[7].clone(),
            launch_date: row[8].clone(),
            performance_1y: row[15].trim().replace('%', "").parse().ok(),
            performance_ytd: row[14].trim().replace('%', "").parse().ok(),
            holdings: Vec::new(), // Initialize with empty holdings
            issuer,
            product_url: None, // Initialize with None
        })
    }
    
    // Load holdings information based on the ETF issuer
    pub fn load_holdings(&mut self) -> Result<()> {
        match self.issuer.as_str() {
            "Invesco" => self.load_invesco_holdings()?,
            // Add other issuers here as needed
            _ => {} // Do nothing for unsupported issuers
        }
        
        Ok(())
    }
    
    // Load holdings for Invesco ETFs
    fn load_invesco_holdings(&mut self) -> Result<()> {
        // For Invesco ETFs, we don't need to add the 'I' prefix
        let url = format!(
            "https://dng-api.invesco.com/cache/v1/accounts/en_GB/shareclasses/{}/holdings/index?idType=isin",
            self.isin
        );
        
        println!("Fetching holdings from URL: {}", url);
        
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()?;
        
        if !response.status().is_success() {
            println!("Failed to fetch holdings for {}: HTTP status {}", self.isin, response.status());
            return Ok(());
        }
        
        let json: Value = response.json()?;
        
        // Extract holdings from the JSON
        if let Some(holdings) = json.get("holdings").and_then(|h| h.as_array()) {
            self.holdings.clear(); // Clear existing holdings
            
            for holding in holdings {
                if let (Some(isin), Some(weight)) = (
                    holding.get("isin").and_then(|i| i.as_str()),
                    holding.get("weight").and_then(|w| w.as_f64())
                ) {
                    self.holdings.push((isin.to_string(), weight));
                }
            }
            
            println!("Successfully loaded {} holdings", self.holdings.len());
        } else {
            println!("No holdings found in the response for {}", self.isin);
        }
        
        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    pub name: String,
    pub isin: String,
    pub country: String,
    pub currency: String,
    pub exchange: String,
    pub security_type: String,
    pub rating: String,
    pub primary_listing: Option<String>,
    pub industry_classification: Option<String>,
    pub weight: f64,
}

impl Security {
    pub fn from_row(row: &[String]) -> Option<Self> {
        Some(Self {
            name: row[0].clone(),
            isin: row[1].clone(),
            country: row[2].clone(),
            currency: row[3].clone(),
            exchange: row[4].clone(),
            security_type: row[5].clone(),
            rating: row[6].clone(),
            primary_listing: Some(row[7].clone()),
            industry_classification: Some(row[8].clone()),
            weight: row[9].trim().replace('%', "").parse().unwrap_or(0.0),
        })
    }
}


