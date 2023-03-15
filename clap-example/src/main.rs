/// Imports
use clap::Parser;
use clap::ValueEnum;

/// Define the arguments for our cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// An argument
    #[arg(short, long)]
    name: String,

    /// An optional "DataType" to pass
    #[arg(short, long)]
    data_type: Option<DataType>,
}

/// We define an enum, Note that enum can contain data
#[derive(ValueEnum, Debug, Copy, Clone)]
enum DataType {
    HeartRate,
    SkinTemp,
    Eda,
    Accel,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}, using argument {:?}", args.name, args.data_type);
    match args.data_type {
        Some(DataType::HeartRate) => println!("We got HR {:?}", args.data_type),
        None => println!("We got nothing"),
        _ => println!("We don't care!"),
    }
}
