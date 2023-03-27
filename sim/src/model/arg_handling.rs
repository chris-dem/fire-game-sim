use clap::Parser;
use clap::Arg;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MyArgs {
    /// File name to input the data. 
    /// Keep in mind, directory for data is at
    /// $EVAC_HOME/inputs/tests
    #[arg(short, long, default_value_t = String::from("base_input.json"))]
    pub file_name : String
}


// #[cfg(feature = "search")]
// #[derive(Debug, Parser)]
// #[command(author, version, about, long_about = None)]
// pub struct BayesianArgument {
//     /// Optimize over lc,ld
//     #[clap(short, long, action)]
//     pub lcld : bool, 
//     /// Optimize over aspiration
//     #[clap(short, long, action)]
//     pub asp : bool, 
    
//     /// Optimize over ratio
//     #[clap(short, long, action)]
//     pub ratio : bool, 

//     /// Optimize over reward
//     #[clap(short, long, action)]
//     pub reward : bool, 

//     /// Optimize over static influence
//     #[clap(short, long, action)]
//     pub static_infl : bool, 
    
//     /// Optimize over dynamic influence
//     #[clap(short, long, action)]
//     pub dynamic_infl : bool, 
// }

// #[cfg(feature = "bayesian")]
// impl BayesianArgument {
//     pub fn all(&self) -> bool {
//         self.lcld
//         && self.asp
//         && self.ratio
//         && self.reward
//         && self.static_infl
//         && self.dynamic_infl
//     }
// }


// #[cfg(feature = "search")]
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// pub struct ArgSearch {
//     /// enable lc
// }