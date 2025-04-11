// Umwandlung von sam zu bam und visa verca
/*use std::path::Path;
use rust_htslib::bam::{self, Format, Header, Reader, Writer};

pub fn sam_to_bam(input_sam: &str, output_bam: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the SAM file
    let mut reader = Reader::from_path(input_sam)?;

    // Get the SAM header
    let header = Header::from_template(reader.header());

    // Create a BAM writer with compressed output
    let mut writer = Writer::from_path(output_bam, &header, Format::Bam)?;

    // Write each record from SAM to BAM
    for result in reader.records() {
        let record = result?;
        writer.write(&record)?;
    }

    println!("Successfully converted {} to {}", input_sam, output_bam);

    Ok(())
}*/
