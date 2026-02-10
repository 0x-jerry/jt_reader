#![allow(dead_code)]
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs::File;

mod converter;
mod jt_data;
mod jt_data_type;
mod jt_model;
mod jt_parsing;
mod mesh;
mod jt_reader;

use converter::convert_to_glb;

use crate::jt_model::JtModel;

#[derive(Parser)]
#[command(name = "jt-reader")]
#[command(about = "Reads Siemens JT files and converts to GLB", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspects the JT file structure (Header and TOC)
    Inspect {
        /// Path to the input .jt file
        input: String,
    },
    /// Converts the JT file to GLB
    Convert {
        /// Path to the input .jt file
        input: String,
        /// Path to the output .glb file
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { input } => {
            let file = File::open(&input)?;

            let jt_model = JtModel::new(file)?;

            let header = &jt_model.header;
            println!("JT File Header:");
            println!("  Version: {}", header.version);
            println!("  Byte Order: {:?}", header.byte_order);
            println!("  Empty Field: {}", header.reserved_field);
            println!("  LSG Segment ID: {}", header.lsg_segment_id);
            println!("  TOC Offset: {}", header.toc_offset);

            println!("\nTable of Contents ({} entries):", jt_model.toc.len());
            for (i, entry) in jt_model.toc.iter().take(10).enumerate() {
                println!(
                    "[{}] ID: {}, Offset: {}, Length: {}, Attrs: 0b{:b}",
                    i, entry.segment_id, entry.offset, entry.length, entry.attributes
                );
            }
        }
        Commands::Convert { input, output } => {
            let file = File::open(&input)?;

            let mut jt_model = JtModel::new(file)?;

            println!("Reading JT file...");
            let meshes = jt_model.extract_meshes()?;

            println!("Converting to GLB...");
            convert_to_glb(&meshes, &output)?;
            println!("Successfully wrote to {}", output);
        }
    }

    Ok(())
}
