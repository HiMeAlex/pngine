use std::env;
use std::fs;
use std::str::from_utf8;
use std::collections::HashMap;

const CRITICAL_CHUNKS: [&str; 4] = ["IHDR", "PLTE", "IDAT", "IEND"];
const ANCILLARY_CHUNKS: [&str; 18] = ["bKGD","cHRM","dSIG","eXIf","gAMA","hIST",
                                      "iCCP","iTXt","pHYs","sBIT","sPLT","sRGB",
                                      "sTER","tEXt","tEXt","tIME","tRNS","zTXt"];

fn read_file(file_path: &str) -> Vec<u8> {
    let file_res: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
    if file_res.is_err() {
        panic!("File could not be read, check that path is correct and is owned by your user.");
    } else {        
        let bytes = file_res.unwrap(); 
        bytes
    }
}

fn is_start_of_chunk(index:usize, list: Vec<u8>) -> bool {
    if index+8 >= list.len() {
        return false;
    }
    let chunk_res =  from_utf8(&list[(index + 4)..(index + 8)]);
    if chunk_res.is_err() {
        false
    } else if CRITICAL_CHUNKS.contains(&chunk_res.unwrap()) || ANCILLARY_CHUNKS.contains(&chunk_res.unwrap()) {
        true
    } else {
        false
    }
}

fn bytes_as_u32(array: &[u8]) -> u32 {
    // from: https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

// TODO: meta data from IHDR (and whatever ancillary chunks store metadata)
// return hashmap containing metadata key/ value pairs
// fn get_text_metadata(chunk: Vec<u8>) -> HashMap<String, String> {

// }
// fn get_numerical_metadata(chunk: Vec<u8>) -> HashMap<String, u32> {
// }
// TODO: IDAT decoder
// return vector consisting of arrays of color data
//(0-255 so u8 and RGB so arrays of length 3 )
// fn idat_decoder(chunk: Vec<u8>) -> Vec<Vec<u8>> {

// }

fn interpret_chunks(byte_data: Vec<u8>) -> HashMap<usize, String> {
    if byte_data[0..8] == [137, 80, 78, 71, 13, 10, 26, 10] {
        let mut chunk_map  = HashMap::new();  
        let mut i = 0;
        while i < byte_data.len() {
            if is_start_of_chunk(i, byte_data) {
                let chunk_type = String::from(from_utf8(&byte_data[(i+4)..(i+8)]).unwrap());
                chunk_map.insert(i, chunk_type);
            }
            i += 12;
        }
        chunk_map
    } else {
        panic!("File does not contain PNG signature. Try a PNG file?")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        let path_arg = &args[1];
        let file_bytes = read_file(path_arg);
        print!("{:?}", interpret_chunks(file_bytes));
    } else {
        panic!("No arguements provided.")
    }
}