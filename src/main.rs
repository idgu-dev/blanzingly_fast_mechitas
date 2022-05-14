use std::error::Error;
use std::process;
use serde::Deserialize;
extern crate statrs;
use statrs::statistics;
use statrs::distribution;
use statrs::statistics::Statistics;

#[derive(Debug, Deserialize)]

struct Candle {
  date: String,
  open: f64,
  high: f64,
  low: f64,
  close: f64,
  volume: f64,
}

impl Candle {
  fn print_candle(&self){
    println!("Date:{}, Open:{}, High:{}, Low:{}, Close:{}, Volume:{}", 
        self.date, self.open, self.high, self.low, self.close, self.volume
    );
  }
}

fn read_file(file_path: &str) -> Result<Vec<Candle>, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(file_path)?;
  let candles = rdr.deserialize::<Candle>().collect::<Result<Vec<Candle>, _>>()?;
  return Ok(candles);
}

fn rolling_var(x: Vec<f64>, window: usize) -> Vec<f64>{
  let mut var_vec: Vec<f64> = Vec::<f64>::new();
  for i in 0..x.len()-window{
      // let avg = x[i..i+window].iter().sum::<f64>() as f64 / window as f64;
      // var_vec.push(f64::sqrt(((x[i+window-1] - avg).powf(2.0)) / window as f64));
      var_vec.push((&x[i..i+window]).variance());
  }
  return var_vec
}

fn main() {
  let candles = match read_file("BTC_2015.csv") {
    Ok(candles) => candles,
    Err(err) => {
        println!("Error trying to read candles: {}", err);
        process::exit(1);
    }
  };

  let mut log_returns = Vec::<f64>::new(); 
  (1..candles.len()).for_each(|i| {
    log_returns.push(candles[i].close.log(f64::exp(1.0)) - candles[i-1].close.log(f64::exp(1.0)));
  });

  let variance = rolling_var(log_returns, 30);
  for v in variance.iter(){
    println!("{}", v);
  }

}