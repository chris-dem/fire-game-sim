// use krabmaga::OpenOptions;
// use serde::Serialize;

// use std::io::prelude::*;
// use std::time::{SystemTime, UNIX_EPOCH};

// #[derive(Debug, Default)]
// pub struct FileHandler {
//     name: String,
//     pub curr_line: Line,
//     lines: Vec<String>,
// }

// impl FileHandler {
//     pub fn update_file(&self, iteration: u16) {
//         let mut options = OpenOptions::new();
//         options.create(true);
//         options.append(true);
//         let mut f = options
//             .open(format!("output/{}_{}.json", self.name, iteration))
//             .expect("file creation failed");
//         for line in self.lines.iter() {
//             writeln!(f, "{}", line).expect("should write to file");
//         }
//     }
//     pub fn new(file_name: &str) -> Self {
//         Self {
//             name: format!(
//                 "{}_{}.json",
//                 file_name,
//                 SystemTime::now()
//                     .duration_since(UNIX_EPOCH)
//                     .unwrap()
//                     .as_secs()
//             ),
//             ..Default::default()
//         }
//     }

//     pub fn add_line(&mut self) {
//         let string = serde_json::to_string(&self.curr_line).expect("should work");
//         self.lines.push(string);
//         self.curr_line.reset();
//     }

//     pub fn reset(&mut self) {
//         *self = Self {
//             name: self.name.to_owned(),
//             ..Default::default()
//         };
//     }
// }

// #[derive(Debug, Clone, Default, Serialize)]
// pub struct Line {
//     pub lno: usize,
//     pub ratio: AvgVar,  // finished
//     pub asp: f32,       // finished
//     pub all_cnt: usize, // finished
//     pub abo_cnt: usize, // finished
//     pub no_cnt: usize,  // finished
//     pub reward: AvgVar, // finished
//     pub escaped: usize,
//     pub dead: usize,
//     // pub flow: Option<f32>,
// }

// #[derive(Debug, Clone, Copy, Default, Serialize)]
// pub struct AvgVar {
//     sum: f32,
//     sumq: f32,
//     n: usize,
// }

// impl AvgVar {
//     pub fn update(&mut self, val: f32) {
//         self.sum += val;
//         self.sumq += val * val;
//         self.n += 1;
//     }
// }

// #[derive(Debug, Clone, Copy, Default, Serialize)]
// struct Couple(f32, u32);

// impl Line {
//     fn reset(&mut self) {
//         *self = Self {
//             lno: self.lno + 1,
//             ..Default::default()
//         };
//     }
// }
