use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MyArgs {
    /// File name to input the data. 
    /// Keep in mind, directory for data is at
    /// $EVAC_HOME/inputs/tests
    #[arg(short, long, default_value_t = String::from("base_input.json"))]
    pub file_name : String
}
