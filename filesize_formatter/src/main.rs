use std::env;
use std::collections::HashMap;


fn format_size(size: f64, xb: &str) -> HashMap<&str, f64> {
    let size_bytes = match xb {
        "B" => size,
        "Kb" => size * 1_024.0,
        "Mb" => size * 1_024.0 * 1_024.0,
        "Gb" => size * 1_024.0 * 1_024.0 * 1_024.0,
        "Tb" => size * 1_024.0 * 1_024.0 * 1_024.0 * 1_024.0,
        _ => 0.0,
    };

    let mut data = HashMap::new();
    data.insert("terabytes", size_bytes / (1_024.0 * 1_024.0 * 1_024.0 * 1_024.0));
    data.insert("gigabytes", size_bytes / (1_024.0 * 1_024.0 * 1_024.0));
    data.insert("megabytes", size_bytes / (1_024.0 * 1_024.0));
    data.insert("kilobytes", size_bytes / 1_024.0);
    data.insert("bytes", size_bytes);
      
    return data;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let size_xb: &String = &args[1];
    let size_xb = size_xb.to_string();
    let size_xb: Vec<&str> = size_xb.split_whitespace().collect();
    let size = size_xb[0];
    let size: Result<f64, _> = size.parse();
    let size: f64 = size.unwrap();
    let xb = size_xb[1];
    println!("User input size: {}", size);
    println!("User input format: {}", xb);
    let format = format_size(size, xb);
    println!("sizes: {:?}", format);
}