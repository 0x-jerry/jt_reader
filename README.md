# JT Reader & Converter

A Rust tool to read Siemens JT files (`.jt`) and convert them to GLB format (`.glb`) using the `mesh_tools` crate.

## Usage

### Build
```bash
cargo build --release
```

### Inspect JT File
View the header and table of contents of a JT file to understand its structure.
```bash
cargo run -- inspect input.jt
```

### Convert to GLB
Convert the JT file to a GLB model.
```bash
cargo run -- convert input.jt output.glb
```

## Status
- **Parsing**: Reads File Header and Table of Contents.
- **Mesh Extraction**: Currently implements a skeleton for mesh extraction. 
  - *Note*: Actual geometry decoding (Tri-Strip Set, Quantization, Zlib) is complex and currently returns a placeholder mesh (a box) or empty mesh to demonstrate the pipeline.
  - To enable full extraction, the `extract_meshes` function in `src/jt_reader.rs` needs to be populated with logic to parse specific Shape LOD segments based on the JT Specification.
- **Conversion**: Uses `mesh_tools` to export the data to GLB.

## Dependencies
- `mesh-tools`: For GLB export.
- `byteorder`: For binary parsing.
- `uuid`: For Segment IDs.
- `flate2`: For Zlib decompression (ready for use).
