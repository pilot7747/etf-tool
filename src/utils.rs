use std::fs::File;
use std::io::copy;
use std::path::Path;
use reqwest::blocking::Client;
use calamine::{open_workbook, Reader, Xlsx};
use color_eyre::{Result, eyre::eyre, eyre::WrapErr};

pub fn download_xlsx(url: &str, file_path: &str) -> Result<()> {
    let client = Client::new();
    let mut response = client.get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .wrap_err("Failed to send request")?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(eyre!("Failed to download file: HTTP {}", response.status()));
    }

    // Create a file to write to
    let mut file = File::create(file_path).wrap_err("Failed to create file")?;

    // Copy the response body to the file
    copy(&mut response, &mut file).wrap_err("Failed to write response to file")?;

    Ok(())
}

pub fn read_xlsx(file_path: &str) -> Result<Vec<Vec<String>>> {
    // Open the workbook
    let path = Path::new(file_path);
    let mut workbook: Xlsx<_> = open_workbook(path).wrap_err("Failed to open workbook")?;

    // Get the first worksheet
    let sheet_name = workbook.sheet_names().get(0).cloned()
        .ok_or_else(|| eyre!("No sheets found in workbook"))?;

    let sheet = workbook.worksheet_range(&sheet_name)
        .wrap_err("Failed to get worksheet")?;

    // Convert the data to a more manageable format
    let mut data = Vec::new();
    for row in sheet.rows() {
        let row_data: Vec<String> = row.iter()
            .map(|cell| cell.to_string())
            .collect();
        data.push(row_data);
    }

    Ok(data)
} 