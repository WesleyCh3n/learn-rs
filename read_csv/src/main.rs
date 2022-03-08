use csv::{Reader, ReaderBuilder, Writer};
use std::io;

fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn read_without_header() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        println!("{:?}", record);
        if i == 1 {
            break;
        };
    }
    Ok(())
}

fn write_csv() -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_writer(io::stdout());

    // When writing records without Serde, the header record is written just
    // like any other record.
    wtr.write_record(&["city", "region", "country", "population"])?;
    wtr.write_record(&["Southborough", "MA", "United States", "9686"])?;
    wtr.write_record(&["Northbridge", "MA", "United States", "14061"])?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    // if let Err(err) = read_without_header() {
        // println!("error running example: {}", err);
        // std::process::exit(1);
    // }
    if let Err(err) = write_without_header() {
        println!("error running example: {}", err);
        std::process::exit(1);
    }
}
