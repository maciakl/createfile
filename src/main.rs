use clap::{Parser, ValueEnum};

/// Create a file of arbitrary size
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name or path to the file
    #[arg(short, long)]
    name: String,

    /// Size of the file
    #[arg(short, long, default_value_t = 5)]
    size: usize,


    /// Unit of measurement
    #[arg(value_enum, short, long, default_value_t = Unit::M)]
    unit: Unit,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Unit {
    /// Bytes
    B,
    /// Kilobytes
    K,
    /// Megabytes
    M,
    /// Gigabytes
    G
}


fn main() {

    let args = Args::parse();

    let name = args.name;
    let size = args.size;
    let unit = args.unit;

    let multiplier:usize = match unit {
        Unit::B => 1,
        Unit::K => 1024,
        Unit::M => 1024*1024,
        Unit::G => 1024*1024*1024

    };

    let size_in_bytes = (size * multiplier).try_into().unwrap();

    println!("creating {} {}{:?}", name, size, unit);

    create_file(name, size_in_bytes);
}






use std::fs::File;
use std::io::{BufWriter, Write};
use std::cmp;
use rand::Rng;

fn create_file(name:String, size:usize) {
    

    if let Ok(f) = File::create(name) {
        let mut writer = BufWriter::new(f);
        
        let mut rng = rand::thread_rng();
        let mut buffer = [0; 1024];
        let mut remaining_size = size;
        
        while remaining_size > 0 {

            let to_write = cmp::min(remaining_size, buffer.len());
            let buffer=  &mut buffer[..to_write];
            rng.fill(buffer);
            writer.write(buffer).unwrap();
            
            remaining_size -= to_write;
        }
    }
    else {
        eprintln!("Could not create file");
        std::process::exit(1);
    }

}

