#![allow(dead_code)]
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs::File;

mod jt_data;
mod jt_data_type;
mod jt_decode;
mod jt_model;
mod jt_reader;
mod mesh;

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
    simple_logger::init()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { input } => {
            let file = File::open(&input)?;

            let jt_model = JtModel::new(file)?;

            let header = &jt_model.header;
            log::info!("JT File Header:");
            log::info!("  Version: {}", header.version.trim());
            log::info!("  Major Version: {}", header.major_version);
            log::info!("  Byte Order: {:?}", header.byte_order);
            log::info!("  Empty Field: {}", header.reserved_field);
            log::info!("  LSG Segment ID: {}", header.lsg_segment_id);
            log::info!("  TOC Offset: {}", header.toc_offset);

            log::info!("\nTable of Contents ({} entries):", jt_model.toc.len());
            for (i, entry) in jt_model.toc.iter().take(10).enumerate() {
                log::info!(
                    "[{}] ID: {}, Offset: {}, Length: {}, Attrs: 0b{:b}",
                    i,
                    entry.segment_id,
                    entry.offset,
                    entry.length,
                    entry.attributes
                );
            }
        }
        Commands::Convert { input, output } => {
            let file = File::open(&input)?;

            let mut jt_model = JtModel::new(file)?;

            if jt_model.header.major_version != 9 {
                log::warn!(
                    "Unsupported major version: {}",
                    jt_model.header.major_version
                );
                log::warn!("Only major version 9 is supported.");
            } else {
                log::info!("Reading JT file...");
                let _meshes = jt_model.extract_meshes()?;
            }
        }
    }

    Ok(())
}
