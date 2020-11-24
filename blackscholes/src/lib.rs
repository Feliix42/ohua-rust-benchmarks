use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

const INV_SQRT_2PI: f32 = 0.39894228040143270286;

/// Type of a stock option
#[derive(PartialEq, Clone)]
pub enum OptionType {
    Put,
    Call,
}

impl From<&str> for OptionType {
    fn from(inp: &str) -> Self {
        match inp {
            "P" => Self::Put,
            "C" => Self::Call,
            _ => panic!("Incorrect Option type (expected P or C)"),
        }
    }
}

#[derive(Clone)]
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
        let line_count = usize::from_str(buf.trim()).unwrap();

        let mut options = Vec::with_capacity(line_count);

        for line in reader.lines() {
            let line: String = line?;
            options.push(OptionData::from(line.as_str()));
        }

        Ok(options)
    }

    pub fn calculate_black_scholes(&self) -> f32 {
        // just the 1:1 copyover of the calculation in the C version
        let x_sqrt_time = self.time.sqrt();
        let x_log_term = (self.spot / self.strike).ln();

        let x_power_term = self.volatility.powi(2) * 0.5;

        let mut x_d1 = (self.interest + x_power_term) * self.time + x_log_term;

        let x_den = self.volatility * x_sqrt_time;
        x_d1 /= x_den;
        let x_d2 = x_d1 - x_den;

        let no_fx_d1 = cndf(x_d1);
        let no_fx_d2 = cndf(x_d2);

        let future_value_x = self.strike * (-self.interest * self.time).exp();

        let option_price = if self.ty == OptionType::Call {
            (self.spot * no_fx_d1) - (future_value_x * no_fx_d2)
        } else {
            // PUT option
            let neg_no_fx_d1 = 1.0 - no_fx_d1;
            let neg_no_fx_d2 = 1.0 - no_fx_d2;

            (future_value_x * neg_no_fx_d2) - (self.spot * neg_no_fx_d1)
        };

        option_price
    }

    /// Returns `true` when the divergence between the computed price and the DerivaGem Reference Value is within the acceptable tolerance of 0.0001.
    #[inline(always)]
    fn verify(&self, result: f32) -> bool {
        let delta = (self.ref_val - result).abs();
        delta < 0.0001
    }
}

/// Cumulative Normal Distribution Function
///
/// https://en.wikipedia.org/wiki/Cumulative_distribution_function
fn cndf(input_x: f32) -> f32 {
    let invert_output = input_x.is_sign_negative();
    let x_input = input_x.abs();

    let x_n_primeof_x = (-0.5_f32 * x_input.powi(2)).exp() * INV_SQRT_2PI;

    let x_k2 = 1.0 / (1.0 + (0.2316419 * x_input));
    let x_k2_2 = x_k2.powi(2);
    let x_k2_3 = x_k2 * x_k2_2;
    let x_k2_4 = x_k2 * x_k2_3;
    let x_k2_5 = x_k2 * x_k2_4;

    let x_local1 = x_k2 * 0.319381530;
    let mut x_local2 = x_k2_2 * (-0.356563782);
    let mut acc = x_k2_3 * 1.781477937;
    x_local2 += acc;
    acc = x_k2_4 * (-1.821255978);
    x_local2 += acc;
    acc = x_k2_5 * 1.330274429;
    x_local2 += acc;

    let x_local = 1.0 - ((x_local2 + x_local1) * x_n_primeof_x);

    if invert_output {
        1.0 - x_local
    } else {
        x_local
    }
}

/// Verifies the results calculated.
///
/// Both slices must have the same ordering of data and be equally long.
pub fn verify_all_results(options: &[OptionData], results: &[f32]) -> usize {
    assert_eq!(options.len(), results.len());

    let mut err_count = 0;
    for idx in 0..options.len() {
        if !options[idx].verify(results[idx]) {
            err_count += 1;
        }
    }

    err_count
}
