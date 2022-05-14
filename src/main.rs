use std::error::Error;
use std::process;
use serde::Deserialize;
extern crate statrs;
use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
#[derive(Clone)]

struct Candle {
  // date: String,
  Open: f64,
  High: f64,
  Low: f64,
  Close: f64,
  Volume: f64,
}

// impl Candle {
//   fn print_candle(&self){
//     println!("Date:{}, Open:{}, High:{}, Low:{}, Close:{}, Volume:{}", 
//         self.date, self.open, self.high, self.low, self.close, self.volume
//     );
//   }
// }

fn read_file(file_path: &str) -> Result<Vec<Candle>, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(file_path)?;
  let candles = rdr.deserialize::<Candle>().collect::<Result<Vec<Candle>, _>>()?;
  return Ok(candles);
}

fn rolling_var(x: Vec<f64>, window: usize) -> Vec<f64>{
  let mut var_vec: Vec<f64> = Vec::<f64>::new();
  for i in 0..x.len()-window{
    var_vec.push((&x[i..i+window]).variance());
  }
  return var_vec
}

fn main() {
  let now = Instant::now();
  let dir_name = "Candles";
  let mut candles_map: HashMap<String,Vec<Candle>> = HashMap::new();
  let mut log_returns_map: HashMap<String, Vec<f64>> = HashMap::new();
  let mut variance_map: HashMap<String, Vec<f64>> = HashMap::new();
  let file_names = std::fs::read_dir(dir_name).unwrap()
    .filter_map(|entry| {
      entry.ok().and_then(|e| e.path().file_name()
      .and_then(|n| n.to_str().map(|s| String::from(s)))
      )
    });
  for file_name in file_names {
    let path_to_read = dir_name.to_owned() + "/" + &file_name;
    let candles = match read_file(&path_to_read) {
      Ok(candles) => candles,
      Err(err) => {
        println!("Error trying to read candles: {}", err);
        process::exit(1);
      }
    };
    candles_map.insert(file_name.replace(".csv",""), candles.clone());
    let mut log_returns = Vec::<f64>::new(); 
    (1..candles.len()).for_each(|i| {
      log_returns.push(candles[i].Close.log(f64::exp(1.0)) - candles[i-1].Close.log(f64::exp(1.0)));
    });
    log_returns_map.insert(file_name.replace(".csv", ""), log_returns.clone());
    variance_map.insert(file_name.replace(".csv", ""), rolling_var(log_returns, 30));
  } 
  println!("{}", now.elapsed().as_millis());
}