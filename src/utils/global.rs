use csv;
use std::fs::File;
use std::error::Error;

#[derive(Debug)]
pub struct CsvRow {
    pub name: String,
    pub age: u32,
}

pub fn get_csv(loc: &str) -> Result<Vec<CsvRow>, Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(loc)?;

    // Create a CSV reader
    let mut rdr = csv::Reader::from_reader(file);

    // Get the headers from the CSV file
    let headers = rdr.headers()?.clone();

    // Find the indices of the columns by name
    let name_idx = headers.iter().position(|h| h == "name").ok_or("Name column not found")?;
    let age_idx = headers.iter().position(|h| h == "age").ok_or("Age column not found")?;

    // Initialize a vector to store the CsvRow structs
    let mut data = Vec::new();

    // Iterate over the records and convert each to a CsvRow
    for result in rdr.records() {
        let record = result?;
        let csv_row = CsvRow {
            // Map CSV columns to struct fields using column names
            name: record.get(name_idx).unwrap_or("").to_string(),
            age: record.get(age_idx).unwrap_or("0").parse().unwrap_or(0),
        };
        data.push(csv_row);
    }

    Ok(data)
}