use std::{env, collections::HashMap, str::FromStr};

use num::Complex;

fn main() {
    let args_raw = env::args().into_iter().collect::<Vec<_>>();
    let mut args: HashMap<String, String> = parse(args_raw);

    let bounds = args.entry("-size".to_string())
        .or_default()
        .parse::<Bound>()
        .expect("cant find -size INTxINT");

    let num = split(args.entry("-num".to_string())
        .or_default(), ",")
        .expect("cant find -num FLOAT,FLOAT");
    
    let num: Complex<f64> = Complex {re: num.0, im: num.1};

    println!("{:?}, {:?}", bounds, num)
}

fn parse(args: Vec<String>) -> HashMap<String, String> {
    args.chunks(2)
    .filter_map(|pair| {
        if pair.len() == 2 {
            Some((pair[0].to_string(),pair[1].to_string()))
        } else {
            None
        }
    })
    .collect::<HashMap<String, String>>()
}

#[derive(Debug)]
struct Bound {
    width: usize,
    height: usize,
}

impl FromStr for Bound {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = split(s, "x")?;
        Ok(Bound { width: pair.0, height: pair.1 })
    }
}

fn split<T: FromStr>(s: &str, sep: &str) -> Result<(T,T), String> {
    let vals = s.split(sep).into_iter().collect::<Vec<&str>>();
    if vals.len() != 2 {
        return Err("invalid length, expected 2".to_string());
    }
    let first = vals.get(0)
        .ok_or("cant val 0".to_string())?
        .parse::<T>().map_err(|_| "error parsing num".to_string())?;
    
    let second = vals.get(1)
        .ok_or("cant val 1".to_string())?
        .parse::<T>().map_err(|_| "error parsing num".to_string())?;

    Ok((first, second))
}

