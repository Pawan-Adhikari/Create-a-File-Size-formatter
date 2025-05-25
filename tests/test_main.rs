// update this file with your own tests
#[derive(Debug)]
struct Sizes_struct {
    bytes: f64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    None,
}

impl Sizes_struct {
    fn new(bytes: f64, kilobytes: f64, megabytes: f64, gigabytes: f64) -> Sizes_struct {
        Sizes_struct { bytes, kilobytes, megabytes, gigabytes }
    }
}

fn format_size(size_str: String) -> Sizes_struct {
    let parts: Vec<&str> = size_str.trim().split_whitespace().collect();
    if parts.len() < 2 {
        panic!("Input must contain a size and a unit (e.g., '123 Bytes')");
    }

    let size: f64 = parts[0].parse().expect("Not a valid number");
    let size_type = parts[1];
    let file_type = match size_type.to_uppercase().as_str() {
        "BYTES" | "B" => FileSize::Bytes(size),
        "KILOBYTES" | "KB"=> FileSize::Kilobytes(size),
        "MEGABYTES" | "MB"=> FileSize::Megabytes(size),
        "GIGABYTES" | "GB"=> FileSize::Gigabytes(size),
        _ => FileSize::None,
    };

    let result = match file_type {
        FileSize::Bytes(b) => Sizes_struct::new(b, b / 1000.0, b / 1_000_000.0, b / 1_000_000_000.0),
        FileSize::Kilobytes(kb) => Sizes_struct::new(kb * 1000.0, kb, kb / 1000.0, kb / 1_000_000.0),
        FileSize::Megabytes(mb) => Sizes_struct::new(mb * 1_000_000.0, mb * 1000.0, mb, mb / 1000.0),
        FileSize::Gigabytes(gb) => Sizes_struct::new(gb * 1_000_000_000.0, gb / 1_000_000.0, gb * 1000.0, gb),
        FileSize::None => Sizes_struct::new(0.0, 0.0, 0.0, 0.0),
    };
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Enter a valid string.");
    let result = format_size(input);
    println!("{:?}", result);
}
