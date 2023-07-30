use std::{env, collections::HashMap, str::FromStr, fs::File, io::{BufWriter, Write}};

use num::Complex;

fn main() {
    let args_raw = env::args().into_iter().collect::<Vec<_>>();
    let mut args: HashMap<String, String> = parse(args_raw);

    println!("provided args {:?}", args);

    let bounds = args.entry("-size".to_string())
        .or_default()
        .parse::<Bound>()
        .expect("cant find -size INTxINT");

    let start = split(args.entry("-start".to_string())
        .or_default(), ",")
        .expect("cant find -start FLOAT,FLOAT");
    let start: Complex<f64> = Complex {re: start.0, im: start.1};

    let end = split(args.entry("-end".to_string())
        .or_default(), ",")
        .expect("cant find -end FLOAT,FLOAT");
    let end: Complex<f64> = Complex {re: end.0, im: end.1};

    println!("{:?}, {:?}, {:?}", bounds, start, end);

    let img = calculate_mandelbrot(&bounds, start, end);
    render(&bounds, img);
}

fn parse(mut args: Vec<String>) -> HashMap<String, String> {
    args.remove(0);
    
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

fn calculate_mandelbrot(bound: &Bound, start: Complex<f64>, end: Complex<f64>) -> Vec<u8> {
    let mut out = vec![0; bound.height*bound.width];

    for row in 0..bound.height {
        for col in 0..bound.width {
            let point = pixel_to_point(&bound, &start, &end, (row,col));
            out[row * bound.height + col] = mandel(point).unwrap_or_default();
        }
    }
    out
}

fn pixel_to_point(bound: &Bound, start: &Complex<f64>, end: &Complex<f64>, point: (usize, usize)) -> Complex<f64> {
    let w = end.re - start.re;
    let h = start.im - end.im;

    Complex { 
        re: start.re + point.0 as f64 * w / bound.width as f64, 
        im: start.im - point.1 as f64 * h / bound.height as f64 
    }
}

fn mandel(c: Complex<f64>) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0};
    for i in 0..255 {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn render(bound: &Bound, pixels: Vec<u8>) {
    let mut data = String::from("P2\n");
    data += format!("{} {}\n", bound.height, bound.width).as_str();

    for (i, pix) in pixels.iter().enumerate() {
        data += format!("{pix}").as_str();

        if i != 0 && i % bound.width == 0 {
            data += "\n";
        } else {
            data += " ";
        }
    }

    let f = File::create("mandel.PGM").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}