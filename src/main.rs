use std::fs::File;
use std::io::BufReader;

mod reader;
mod record;
mod alignment;
mod sam_bam;

fn main() -> std::io::Result<(), Box<dyn std::error::Error>> {
    let file = File::open("samfile.txt")?; //automatically return error if function call fails

    let reader = BufReader::new(file);

    let records = reader::parse_records(reader)?;

    for record in records {
        println!("{:?}", record); //debug formatting
    }

    let input = "samfile.txt";
    let output = "bamfile.txt";
    sam_bam::sam_to_bam(input, output)?;

    Ok(())
}
