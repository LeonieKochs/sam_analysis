use std::fs::File;
use std::io::BufReader;

use mapping_quality::MappingQuality;
use q30::Q30;
use statistic::Statistic;

mod reader;
mod record;
mod alignment;
mod sam_bam;
mod mapping_quality;
mod q30;
mod statistic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("samfile2.sam")?; //automatically return error if function call fails

    let reader = BufReader::new(file);

    let records = reader::parse_records(reader)?;

    // Initialize all statistics
    let mut statistics: Vec<Box<dyn Statistic>> = vec![
        Box::new(MappingQuality::new()),
        Box::new(Q30::new()),
    ];

    // Process records through all statistics
    for record in &records {
        for stat in statistics.iter_mut() {
            stat.process(record);
        }
    }

    // Print all final results
    for stat in statistics {
        println!("{}: {}", stat.name(), stat.report());
    }

    Ok(())
}
