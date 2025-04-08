use std::fs::File;
use std::io::BufReader;

mod reader;
mod record;

fn main() -> std::io::Result<()> {
    let file = File::open("samfile.txt")?;
    let reader = BufReader::new(file);

    let records = reader::parse_records(reader)?;

    for record in records {
        println!("{:?}", record);
    }

    Ok(())
}
