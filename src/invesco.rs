use color_eyre::Result;
use crate::etf::ETF;
use crate::utils;

const ISSUER: &str = "Invesco";

pub fn get_invesco_etfs() -> Result<Vec<ETF>> {
    let file_path = "data/invesco_etfs.xlsx";

    if !std::path::Path::new(file_path).exists() {
        panic!("File does not exist at: {}", file_path);
    }

    let raw_data = utils::read_xlsx(file_path)?;
    
    // Find the header row that contains "Bloomberg"
    let header_row_index = raw_data.iter()
        .position(|row| row.iter().any(|cell| cell == "Bloomberg"))
        .unwrap_or(0);

    // Process data starting from the row after headers
    let etfs: Vec<ETF> = raw_data.iter()
        .skip(header_row_index + 1) // Skip header row
        .filter_map(|row| {
            // Skip empty rows or rows that don't look like ETF data
            if row.len() < 8 || row[2].is_empty() || row[7].is_empty() {
                return None;
            }

            // Parse TER/OCF (column 12)
            let ter = row.get(12)
                .and_then(|s| s.trim().replace('%', "").replace(',', "").parse().ok())
                .unwrap_or(0.0);

            // Parse AUM (column 23)
            let aum = row.get(23)
                .map(|s| s.to_string())
                .unwrap_or_default();

            // Get distribution type from the column 28
            let distribution_policy = match row.get(28).map(|s| s.as_str()) {
                Some("Reinvested") => "Accumulating".to_string(),
                Some("Distributed") => "Distributing".to_string(),
                _ => "Unknown".to_string(),
            };

            // let row_0 = row[0].clone();
            // let row_1 = row[1].clone();
            // let row_2 = row[2].clone();
            // let row_3 = row[3].clone();
            // let row_4 = row[4].clone();
            // let row_5 = row[5].clone();
            // let row_6 = row[6].clone();
            // let row_7 = row[7].clone();
            // let row_8 = row[8].clone();
            // let row_9 = row[9].clone();
            // let row_10 = row[10].clone();
            // let row_11 = row[11].clone();
            // let row_12 = row[12].clone();
            // let row_13 = row[13].clone();
            // let row_14 = row[14].clone();
            // let row_15 = row[15].clone();
            // let row_16 = row[16].clone();
            // let row_17 = row[17].clone();
            
            

            Some(ETF {
                name: row[1].clone(), // Fund Name (Equity ETFs column)
                isin: row[6].clone(), // ISIN
                asset_class: "Equity".to_string(), // All ETFs in the file are equity ETFs
                ter,
                currency: row[12].clone(), // Base currency
                aum,
                currency_exposure: row[15].clone(), // Index currency
                distribution_policy,
                launch_date: row[16].clone(), // Date of issue
                performance_1y: None, // Not available in the file
                performance_ytd: None, // Not available in the file
                holdings: Vec::new(), // Initialize with empty holdings
                issuer: ISSUER.to_string(),
                product_url: None, // Initialize with None
            })
        })
        .collect();

    Ok(etfs)
} 