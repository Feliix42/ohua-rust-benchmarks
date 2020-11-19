use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

/// Type of a stock option
pub enum OptionType {
    Put,
    Call
}

impl From<&str> for OptionType {
    fn from(inp: &str) -> Self {
        match inp {
            "P" => Self::Put,
            "C" => Self::Call,
            _ => panic!("Incorrect Option type (expected P or C)")
        }
    }
}

pub struct OptionData {
    /// Spot Price
    pub spot: f32,
    /// Strike Price
    pub strike: f32,
    /// Risk-free interest rate
    pub interest: f32,
    /// Dividend rate
    pub dividend_rate: f32,
    /// Volatility
    pub volatility: f32,
    /// Time to maturity or option expiration in years.
    /// (1yr = 1.0, 6mos = 0.5, 3mos = 0.25 etc)
    pub time: f32,
    /// Option type
    pub ty: OptionType,
    /// Dividend values (not used in this test)
    pub dividend_vals: f32,
    /// DerivaGem Reference Value
    pub ref_val: f32,
}

impl From<&str> for OptionData {
    fn from(inp: &str) -> Self {
        // input is a single line with all values listed in a row, separated by spaces
        let mut iter = inp.split_whitespace();

        Self {
            spot: f32::from_str(iter.next().unwrap()).unwrap(),
            strike: f32::from_str(iter.next().unwrap()).unwrap(),
            interest: f32::from_str(iter.next().unwrap()).unwrap(),
            dividend_rate: f32::from_str(iter.next().unwrap()).unwrap(),
            volatility: f32::from_str(iter.next().unwrap()).unwrap(),
            time: f32::from_str(iter.next().unwrap()).unwrap(),
            ty: OptionType::from(iter.next().unwrap()),
            dividend_vals: f32::from_str(iter.next().unwrap()).unwrap(),
            ref_val: f32::from_str(iter.next().unwrap()).unwrap(),
        }
    }
}

impl OptionData {
    pub fn load_from_file(path: &str) -> std::io::Result<Vec<OptionData>> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);

        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let line_count = usize::from_str(&buf).unwrap();

        let mut options = Vec::with_capacity(line_count);

        for line in reader.lines() {
            let line: String = line?;
            options.push(OptionData::from(line.as_str()));
        }

        Ok(options)
    }
}