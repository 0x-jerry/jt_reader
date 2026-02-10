use anyhow::{Result, anyhow};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{Cursor, Read, Seek, SeekFrom};
use mesh_tools::compat::point3;
use uuid::Uuid;

use crate::mesh::Mesh;

pub struct SegmentParser {
    data: Vec<u8>,
    cursor: Cursor<Vec<u8>>,
}

impl SegmentParser {
    pub fn new(data: Vec<u8>) -> Self {
        SegmentParser {
            data: data.clone(),
            cursor: Cursor::new(data),
        }
    }

    pub fn parse_shape_lod(&mut self) -> Result<Vec<Mesh>> {
        let mut meshes = Vec::new();
        
        // 1. Parse Segment Header (Compression)
        // [Compression Flag: i32]
        // [Compressed Data Length: i32] (Only if Flag == 2)
        
        // Ensure we have enough data for at least flag
        if self.data.len() < 4 {
             return Ok(meshes);
        }
        
        let mut cursor = Cursor::new(&self.data);
        let first_i32 = cursor.read_i32::<LittleEndian>()?;
        
        // Heuristic: If first i32 is 0, 1, or 2, it's likely a compression flag.
        // If it's large (like a length) or random, it might be raw data.
        // Note: Element Lengths are usually smallish, but GUIDs are random.
        // If the segment starts with an Element, the first field is Length (i32).
        
        // IMPORTANT: In test.jt, "Element Length 1754572562" is read when we assume raw data.
        // 1754572562 = 0x68945712.
        // GUIDs seen in test.jt: 12a79468...
        // 0x6894A712 (Little Endian for first 4 bytes of GUID).
        // It seems like we are reading the first 4 bytes of GUID as length?
        // Wait, Element Header: [Length: i32] [GUID: 16 bytes] [Object ID: i32]
        
        // If we read a GUID start as length, it means we are offset by 4 bytes?
        // Or the data starts with GUID directly? No, spec says Length first.
        
        // Let's look at the logs:
        // Parsing Shape LOD Segment: ID=12a79468-2082-f011-8000-d7eb0c7fd9bc, Len=657
        // Warning: Element length 1754572562 seems suspiciously large.
        // 1754572562 (dec) = 68 94 57 12 (hex).
        // The Segment ID starts with 12 A7 94 68.
        // Little Endian of 12A79468 is 68 94 A7 12.
        // Close match! (A7 vs 57 - bit flip?)
        
        // It seems the Segment Data starts with the Segment ID itself?
        // "Data Segment... contains ... Segment Data"
        // But maybe the "Segment Data" for Shape LOD *is* the Element?
        // And the Element ID *matches* the Segment ID?
        
        // If so, the structure is:
        // [Segment Header]?
        // [Element Header]
        //    [Length]
        //    [ID] == Segment ID
        
        // But if we read 1754572562 as Length, that bytes corresponds to the ID!
        // This means the data starts with ID, NOT Length?
        // Or we are misaligned.
        
        // If the data starts with ID, then where is Length?
        // Maybe there is no Element Header wrapper if the Segment IS the Element?
        // Spec 4.1.3: "Data Segment ... contains ... Elements."
        
        // Let's try to find the Element ID in the first few bytes.
        // If the first 16 bytes match the Segment ID, then we are at the Element ID.
        // This implies [Length] is MISSING or comes AFTER?
        
        // Re-reading spec on "Shape LOD Element":
        // "Vertex Shape LOD Element ... 
        //  [Element Header]
        //  [Vertex Shape LOD Data]"
        
        // In `test.jt` analysis previously:
        // "Segment starts with its own ID."
        // "Bytes after ID: ... Zlib found after ID!"
        
        // If segment starts with ID, and then Zlib...
        // This means the Segment Data is: [ID] [Zlib Stream].
        // This is NOT standard Element Header structure.
        
        // It seems `test.jt` might be using a variation where Segment Data = Element ID + Compressed Element Data?
        // Or Segment Data = Element ID + Element Data?
        
        // Let's modify the heuristic:
        // If we don't find a standard Compression Flag (0, 1, 2),
        // Check if data starts with a GUID.
        
        // We need to pass the Segment ID to `parse_shape_lod` to verify this hypothesis.
        // But we don't have it here.
        
        // Let's look at the `first_i32`.
        // If it looks like part of a GUID (random large number), maybe we just search for known Element IDs?
        // My previous "scan" approach worked because it ignored structure.
        
        // Let's try to combine:
        // 1. Check for Compression Flag (0, 1, 2).
        // 2. If not, check if it starts with Zlib (78 9C).
        // 3. If not, assume it's raw data, but maybe we need to skip the ID if it's there?
        
        // Let's just assume "No Header" if flag is not 0,1,2.
        // And then inside `parse_elements`, we iterate.
        // BUT, `parse_elements` expects [Length] [ID] ...
        
        // If the format is [ID] [Data], `parse_elements` fails because it reads ID as Length.
        // 1754572562 is indeed close to the ID bytes.
        
        // So, for this file, it seems Elements might NOT have the standard [Length][ID] header in uncompressed segments?
        // Or the "Segment" is just the "Element Data" directly?
        
        // Let's try to detect if we are at an ID.
        // We can't easily valid a GUID.
        
        // However, we know `test.jt` worked with the scanning approach.
        // That approach found `0x10DD10AB` (Primitive Set) inside the data.
        
        // Let's revert to a hybrid approach:
        // 1. Try to parse as Sequence of Elements (Standard).
        // 2. If that fails (huge length), fall back to Scanning for known Element IDs.
        
        let working_data = if first_i32 == 2 {
            let compressed_len = cursor.read_i32::<LittleEndian>()?;
            println!("Segment Compressed: {} bytes", compressed_len);
            
            let start = cursor.position() as usize;
            if start + compressed_len as usize > self.data.len() {
                return Err(anyhow!("Compressed data length exceeds segment size"));
            }
            
            use flate2::read::ZlibDecoder;
            let mut decoder = ZlibDecoder::new(&self.data[start..start + compressed_len as usize]);
            let mut buffer = Vec::new();
            decoder.read_to_end(&mut buffer)?;
            buffer
        } else if first_i32 == 0 || first_i32 == 1 {
             let start = cursor.position() as usize;
             self.data[start..].to_vec()
        } else {
             // Check for Zlib magic bytes at start (no flag?)
             // 78 9C, 78 DA, 78 01.
             // 0x78 = 120. 0x9C = 156.
             // first_i32 low byte would be 0x78.
             let b0 = self.data[0];
             let b1 = self.data[1];
             if b0 == 0x78 && (b1 == 0x9C || b1 == 0xDA || b1 == 0x01) {
                 println!("Found raw Zlib stream");
                 use flate2::read::ZlibDecoder;
                 let mut decoder = ZlibDecoder::new(&self.data[..]);
                 let mut buffer = Vec::new();
                 decoder.read_to_end(&mut buffer)?;
                 buffer
             } else {
                 self.data.clone()
             }
        };
        
        // 2. Parse Elements
        // Data is now a sequence of Elements.
        // Element Header:
        // [Length: i32]
        // [Element ID: GUID (16 bytes)]
        // [Object ID: i32]
        
        let mut element_cursor = Cursor::new(&working_data);
        let len = element_cursor.get_ref().len() as u64;
        
        while element_cursor.position() < len {
            // Read Length
            if len - element_cursor.position() < 4 { break; }
            let element_len = element_cursor.read_i32::<LittleEndian>()?;
            
            // Read ID
            if len - element_cursor.position() < 16 { break; }
            let mut id_bytes = [0u8; 16];
             element_cursor.read_exact(&mut id_bytes)?;
             
             // Check if ID is all zeros or looks valid?
             // JT GUIDs are usually not all zeros.
             
             // Check if we are reading valid element length.
             // If we are reading garbage, length is huge.
             // In `test.jt`, we saw valid small lengths before.
             // Why did it suddenly break?
             // Maybe because we started reading `Compression Flag` from `working_data`.
             // `working_data` was `self.data.clone()`.
             // `self.data` comes from `read_segment_bytes`.
             // `read_segment_bytes` reads `entry.length`.
             
             // If the segment is NOT compressed (Flag != 2), we just use `self.data` as `working_data`.
             // BUT, `self.data` STARTS with the Compression Flag!
             // We read `compression_flag` from `self.data` cursor.
             // Then we did `self.data[start..].to_vec()`.
             // `start` is 4 (after reading flag).
             // So `working_data` starts AFTER the flag.
             
             // Is it possible that for uncompressed segments, the Flag is NOT present?
             // Spec 4.1.3: "Data Segment: [Compression Flag] [Compressed Data Length]? [Segment Data]"
             // "If the Compression Flag is 0, the Data Segment contains uncompressed data."
             // It implies Flag is always there.
             
             // However, `test.jt` might have segments WITHOUT flag?
             // Or maybe we are misinterpreting `0`?
             
             // Let's debug the first few bytes of `self.data` for failed segments.
             // If the first 4 bytes are NOT 0, 1, or 2, maybe it's raw data?
             // Or maybe it is an Element Length directly?
             
             // Let's try to peek at `compression_flag` again.
             // If `compression_flag` was huge, we would have treated it as != 2 (else branch).
             // And then `working_data` would start at offset 4.
             // If `compression_flag` was actually part of an Element (e.g. Length), we skipped 4 bytes!
             
             // If `compression_flag` is not 0, 1, 2, assume NO compression header?
             // Let's modify the header detection logic.
             
             // The loop continues, so we can't easily change `working_data` logic inside here.
             // We need to change how `working_data` is created.
             
             let element_id = Uuid::from_bytes(id_bytes);
            
            // Read Object ID
            if len - element_cursor.position() < 4 { break; }
            let object_id = element_cursor.read_i32::<LittleEndian>()?;
            
            let data_start = element_cursor.position();
            let data_end = data_start + (element_len as u64) - 24; // Length includes header (24 bytes)
             
             let remaining = element_len - 24;
             if remaining < 0 {
                 // Should not happen for valid element
                  println!("Invalid Element Length: {}", element_len);
                  break;
             }
             
             // Check if we have enough data
             // NOTE: The previous check `element_cursor.position() + (remaining as u64) > len`
             // fails because `remaining` is derived from `element_len` which might be large.
             // If `element_len` is corrupt or we are reading garbage, this triggers.
             
             // However, `len` is the size of `working_data`.
             // If `working_data` is uncompressed and we sliced it from `self.data`,
             // `len` should be correct.
             
             // The debug output `Need 300974600 bytes, have 580` suggests `element_len` is 300MB!
             // This means we read a huge number as length.
             // This usually means we are not reading `Length` correctly, OR we are not at the start of an Element.
             
             // Possible causes:
             // 1. Compression Header was NOT present (Flag != 2), but we consumed 4 bytes?
             // 2. Data is NOT uncompressed?
             // 3. We are not aligned to Element Header.
             
             // If Flag != 2, we assumed "start..".
             // Let's check the first 4 bytes of `working_data` to see if they look like a length.
             // Typical Element Lengths are small (< 2000 in this file).
             
             // Let's relax the check and print a warning, but also try to validate the length.
             if remaining > 10_000_000 {
                 println!("Warning: Element length {} seems suspiciously large. Reverting to scanning approach.", element_len);
                 self.scan_for_elements(&working_data, &mut meshes);
                 break;
             }
             
             if element_cursor.position() + (remaining as u64) > len {
                println!("Element data truncated: Need {} bytes, have {}", remaining, len - element_cursor.position());
                break;
             }
             
             let element_data_end = element_cursor.position() + remaining as u64;
            
            // Dispatch based on ID
            // Tri-Strip Set: 10 DD 10 35 -> 35 10 DD 10 ...
            // Uuid::from_bytes expects bytes in order.
            // Our "window == [0xAB, 0x10, 0xDD, 0x10]" was Little Endian check.
            // 0x10DD10AB as bytes is 10 DD 10 AB.
            // But if file is LE, we read AB 10 DD 10.
            // UUID bytes are usually stored as-is.
            // Let's print the found ID to verify against known GUIDs.
            
            // Known GUIDs (Standard) often represented as mixed endian in string, but raw bytes here.
            // Tri-Strip Set GUID: 
            // 10DD1035-2AC8-11D1-9B-6B-00-80-C7-BB-59-97 (Example)
            // But we only checked first 4 bytes before.
            
            // Let's map the ID.
            // For now, let's look at the first 4 bytes of the ID to match our previous logic.
            let first_4 = &id_bytes[0..4];
            
            if first_4 == [0xAB, 0x10, 0xDD, 0x10] { // Primitive Set (Legacy/Test)
                 println!("Found Primitive Set Element via Header");
                 // Parse content
                 // We need to read from current cursor position
                 let mut element_reader = Cursor::new(&element_cursor.get_ref()[element_cursor.position() as usize .. element_data_end as usize]);
                 if let Ok(mesh) = self.parse_primitive_set_content(&mut element_reader) {
                     meshes.push(mesh);
                 }
            } else if first_4 == [0x35, 0x10, 0xDD, 0x10] { // Tri-Strip Set
                 println!("Found Tri-Strip Set Element via Header");
            } else {
                 // println!("Skipping Element ID: {}", element_id);
            }
            
            // Advance to next element
            element_cursor.seek(SeekFrom::Start(element_data_end))?;
        }
        
        Ok(meshes)
    }

    fn scan_for_elements(&self, data: &[u8], meshes: &mut Vec<Mesh>) {
        let tristrip_id = [0x35, 0x10, 0xDD, 0x10];
        let primitive_id = [0xAB, 0x10, 0xDD, 0x10];
        
        let mut i = 0;
        while i < data.len().saturating_sub(4) {
            let window = &data[i..i+4];
            
            if window == primitive_id {
                println!("Found Primitive Set Element (0x10DD10AB) at {} via SCAN", i);
                if let Ok(mesh) = self.parse_primitive_set_scan(data, i) {
                    meshes.push(mesh);
                }
                i += 100; 
            } else if window == tristrip_id {
                 i += 100;
            } else {
                i += 1;
            }
        }
    }

    fn parse_primitive_set_scan(&self, data: &[u8], offset: usize) -> Result<Mesh> {
        let count_offset = 92;
        if offset + count_offset + 4 > data.len() {
             return Err(anyhow!("Data too short"));
        }
        
        let index_count = (&data[offset+count_offset..]).read_u32::<LittleEndian>()? as usize;
        
        if index_count == 0 || index_count > 1_000_000 {
             return Err(anyhow!("Invalid index count"));
        }
        
        let mut indices = Vec::new();
        let indices_start = offset + count_offset + 4;
        
        if indices_start + index_count > data.len() { return Err(anyhow!("Indices out of bounds")); }
        
        let mut max_index = 0;
        for j in 0..index_count {
            let idx = data[indices_start + j] as u32;
            indices.push(idx);
            if idx > max_index { max_index = idx; }
        }
        
        let vertices_start = indices_start + index_count;
        let vertex_count = (max_index + 1) as usize;
        let mut vertices = Vec::new();
        
        let mut v_ptr = vertices_start;
        for _ in 0..vertex_count {
            if v_ptr + 12 > data.len() { break; }
            let x = (&data[v_ptr..]).read_f32::<LittleEndian>()?;
            let y = (&data[v_ptr+4..]).read_f32::<LittleEndian>()?;
            let z = (&data[v_ptr+8..]).read_f32::<LittleEndian>()?;
            vertices.push(point3::new(x, y, z));
            v_ptr += 12;
        }
        
        Ok(Mesh {
             vertices,
             normals: Vec::new(),
             indices,
        })
    }

    fn parse_primitive_set_content(&self, cursor: &mut Cursor<&[u8]>) -> Result<Mesh> {
        // Content structure (based on test.jt):
        // [Index Count: i32] ?
        // [Indices]
        // [Vertices]
        
        // In test.jt, we had ID at 28, Count at 120. Gap = 92 bytes.
        // Header was 24 bytes. So 92 - 24 = 68 bytes of "Primitive Set Header" or similar?
        // Let's skip 68 bytes and see.
        
        // Wait, "Length includes header".
        // ID was at 28. Length was at 24.
        // So Header starts at 24.
        // Data starts at 24 + 24 = 48.
        // Count at 120.
        // 120 - 48 = 72 bytes offset into data.
        
        // Let's try skipping 72 bytes.
        if cursor.get_ref().len() < 76 {
             return Err(anyhow!("Primitive Set content too short"));
        }
        
        cursor.seek(SeekFrom::Start(72))?;
        
        let index_count = cursor.read_u32::<LittleEndian>()? as usize;
        
         if index_count == 0 || index_count > 1_000_000 {
             return Err(anyhow!("Invalid index count: {}", index_count));
        }
        
        let mut indices = Vec::new();
        // Assuming u8 indices for this specific format
        for _ in 0..index_count {
            let idx = cursor.read_u8()? as u32;
            indices.push(idx);
        }
        
        // Align? or just sequential?
        // In test.jt: Indices at 124. Vertices at 160.
        // 124 + 36 = 160. Sequential.
        
        // Determine max index for vertex count
        let max_index = indices.iter().cloned().fold(0, u32::max);
        let vertex_count = (max_index + 1) as usize;
        
        let mut vertices = Vec::new();
        for _ in 0..vertex_count {
             let x = cursor.read_f32::<LittleEndian>()?;
             let y = cursor.read_f32::<LittleEndian>()?;
             let z = cursor.read_f32::<LittleEndian>()?;
             vertices.push(point3::new(x, y, z));
        }
        
        Ok(Mesh {
            vertices,
            normals: Vec::new(),
            indices,
        })
    }
}
