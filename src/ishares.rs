use color_eyre::Result;
use crate::etf::ETF;
use crate::utils;

const ISSUER: &str = "iShares";

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

    // Process data starting from the row after headers
    let etfs: Vec<ETF> = raw_data.iter()
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

            Some(ETF {
                name: row[1].clone(), // Fund Name
                isin: row[3].clone(), // ISIN
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
            })
        })
        .collect();

    Ok(etfs)
} 