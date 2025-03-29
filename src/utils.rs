use std::fs::File;
use std::io::{copy, BufReader};
use std::path::Path;
use reqwest::blocking::Client;
use calamine::{open_workbook, Reader, Xlsx};
use color_eyre::{Result, eyre::eyre, eyre::WrapErr};
use quick_xml::events::Event;

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
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| eyre!("Invalid file extension"))?;

    let mut data = Vec::new();
    
    match extension.to_lowercase().as_str() {
        "xlsx" => {
            let mut workbook: Xlsx<_> = open_workbook(path).wrap_err("Failed to open XLSX workbook")?;
            let sheet_name = workbook.sheet_names().get(0).cloned()
                .ok_or_else(|| eyre!("No sheets found in workbook"))?;

            let sheet = workbook.worksheet_range(&sheet_name)
                .wrap_err("Failed to get worksheet")?;

            for row in sheet.rows() {
                let row_data: Vec<String> = row.iter()
                    .map(|cell| cell.to_string())
                    .collect();
                data.push(row_data);
            }
        }
        "xls" => {
            // For XML-based Excel files
            let file = File::open(path).wrap_err("Failed to open XML file")?;
            let reader = BufReader::new(file);
            let mut xml_reader = quick_xml::Reader::from_reader(reader);
            xml_reader.trim_text(true);

            let mut buf = Vec::new();
            let mut current_row: Vec<String> = Vec::new();
            let mut in_cell = false;
            let mut in_data = false;
            let mut cell_content = String::new();
            let mut found_header = false;
            let mut header_row_count = 0;

            loop {
                match xml_reader.read_event_into(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        match e.name().as_ref() {
                            b"Row" => {
                                current_row = Vec::new();
                            }
                            b"Cell" => {
                                in_cell = true;
                                cell_content = String::new();
                            }
                            b"Data" => {
                                in_data = true;
                            }
                            _ => {}
                        }
                    }
                    Ok(Event::Text(e)) => {
                        if in_data {
                            cell_content = e.unescape()?.to_string();
                        }
                    }
                    Ok(Event::End(ref e)) => {
                        match e.name().as_ref() {
                            b"Row" => {
                                if !current_row.is_empty() {
                                    // Skip the header rows
                                    if !found_header {
                                        if current_row[1] == "Fund Name" {
                                            found_header = true;
                                        }
                                        header_row_count += 1;
                                    } else {
                                        // Skip rows that don't look like ETF data
                                        if current_row.len() >= 4 && 
                                           !current_row[1].is_empty() && // Fund Name
                                           !current_row[3].is_empty() && // Fund type
                                           !current_row[1].contains("TER / OCF") &&
                                           !current_row[1].contains("AUM") &&
                                           !current_row[1].contains("As Of") {
                                            data.push(current_row.clone());
                                        }
                                    }
                                }
                            }
                            b"Cell" => {
                                in_cell = false;
                                current_row.push(cell_content.clone());
                            }
                            b"Data" => {
                                in_data = false;
                            }
                            _ => {}
                        }
                    }
                    Ok(Event::Eof) => break,
                    Err(e) => return Err(eyre!("Error parsing XML: {}", e)),
                    _ => {}
                }
                buf.clear();
            }
        }
        _ => return Err(eyre!("Unsupported file format: {}", extension)),
    }

    Ok(data)
} 