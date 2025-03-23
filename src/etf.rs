use serde::{Deserialize, Serialize};

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
}

impl ETF {
    pub fn from_row(row: &[String]) -> Option<Self> {
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
        })
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


