use std::fs::File;
use std::io::BufReader;

use mapping_quality::MappingQuality;

mod reader;
mod record;
mod alignment;
mod sam_bam;
mod mapping_quality;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("samfile.txt")?; //automatically return error if function call fails

    let reader = BufReader::new(file);

    let records = reader::parse_records(reader)?;

    let mut mq = MappingQuality::new();

    for record in records {
        println!("{:?}", record); //debug formatting
        mq.update(record.mapq);
    }

    println!("Average mapping quality: {:.2}", mq.get_average());


    Ok(())
}
