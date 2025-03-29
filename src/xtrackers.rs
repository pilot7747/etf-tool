use std::path::Path;
use color_eyre::Result;
use crate::etf::ETF;
use crate::utils;

const ISSUER: &str = "Xtrackers";

pub fn get_xtrackers_etfs() -> Result<Vec<ETF>> {
    let file_path = "data/xtrackers_etfs.xlsx";

    if !Path::new(file_path).exists() {
        panic!("File does not exist at: {}", file_path);
    }

    let raw_data = utils::read_xlsx(file_path)?;
    
    let header_row_index = raw_data.iter()
        .position(|row| row.get(0).map_or(false, |cell| cell == "Name"))
        .unwrap_or(6);

    // Process data starting from the row after headers
    let etfs: Vec<ETF> = raw_data.iter()
        .skip(header_row_index + 1)
        .filter_map(|row| ETF::from_row(row, ISSUER.to_string()))
        .collect();

    Ok(etfs)
} 