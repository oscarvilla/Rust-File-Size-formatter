use std::env;
use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
enum MySizeUnit {
    Bytes,
    Megabytes,
    Kilobytes,
    Gigabytes,
    Terabytes,
    Error,
}

impl fmt::Display for MySizeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MySizeUnit::Bytes => "B",
            MySizeUnit::Kilobytes => "KB",
            MySizeUnit::Megabytes => "MB",
            MySizeUnit::Gigabytes => "GB",
            MySizeUnit::Terabytes => "TB",
            MySizeUnit::Error => "Invalid",
        })
    }
}

#[derive(Debug)]
struct FileSize {
    unit: MySizeUnit,
    value: f64, 
}

fn format_size(size: f64, xb: MySizeUnit) -> Vec<(MySizeUnit, f64)> {
    let size_bytes = match xb {
        MySizeUnit::Bytes => size,
        MySizeUnit::Kilobytes => size * 1_024.0,
        MySizeUnit::Megabytes => size * 1_024.0 * 1_024.0,
        MySizeUnit::Gigabytes => size * 1_024.0 * 1_024.0 * 1_024.0,
        MySizeUnit::Terabytes => size * 1_024.0 * 1_024.0 * 1_024.0 * 1_024.0,
        _ => 0.0,
    };

    let sizes = vec![
        (MySizeUnit::Terabytes, size_bytes / (1_024.0 * 1_024.0 * 1_024.0 * 1_024.0)),
        (MySizeUnit::Gigabytes, size_bytes / (1_024.0 * 1_024.0 * 1_024.0)),
        (MySizeUnit::Megabytes, size_bytes / (1_024.0 * 1_024.0)),
        (MySizeUnit::Kilobytes, size_bytes / 1_024.0),
        (MySizeUnit::Bytes, size_bytes),
    ];

    sizes
}

fn check_condition(value: &MySizeUnit) -> Result<(), String> {
    match value {
        MySizeUnit::Error => panic!("Units should be \"B\", \"kb\", \"Mb\", \"Gb\" or \"Tb\""),
        _ => Ok(()),
    }
  }

fn main() {
    // Get from CLI arguments
    let args: Vec<String> = env::args().collect();
    // Check if there are no extra arguments
    if args.len() < 2 {
        println!("Call command line with size and units as arguments in one string, ie \"64 mb\"");
        return;
    }

    // Split the argument string
    let size_xb: Vec<&str> = args[1].split_whitespace().collect();
    // The number as float 64 and the units as string
    let size = size_xb[0].parse::<f64>().unwrap();
    let xb = size_xb[1].to_string().to_lowercase();
    println!("User input size: {}", size);
    println!("User input format: {}", xb);

    let unit = match xb.as_str() {
        "b" => MySizeUnit::Bytes,
        "kb" => MySizeUnit::Kilobytes,
        "mb" => MySizeUnit::Megabytes,
        "gb" => MySizeUnit::Gigabytes,
        "tb" => MySizeUnit:: Terabytes,
        _ => MySizeUnit::Error,
    };

    let _checking = check_condition(&unit);
    let file1 = FileSize {
        unit: unit,
        value: size,
    };

    println!("My struct: {:?}", file1);

    let format = format_size(file1.value, file1.unit);
    println!("sizes: {:?}", format);
}