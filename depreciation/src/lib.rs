//! Define an `Asset` and calculate its depreciation.
//!
//! # Diminishing Value Method
//! 
//! The diminishing value method tends to bias the depreciation amount in the earlier years.
//! 
//! The formula:
//! 
//! – assets from before 10 May 2006 : Base Value x (Days held / 365) x (150% / Effective life in
//! years) (Section 40-70)
//! 
//! – assets from on or after 10 May 2006: Base Value x (Days held / 365) x (200% / Effective life
//! in years) (Section 40-72)
//! 
//! The Base Value is initially the cost, but this can be modified by later improvements and the
//! forgiveness of commercial debts.
//!
//! (ref: https://atotaxrates.info/tax-deductions/ato-depreciation/#dep-methods)

use chrono::Duration;
use chrono::naive::NaiveDate;

pub struct Asset {
    pub base_value: f32,            // TODO: Use currency amount from some crate.
    pub purchase_date: String,
}

impl Asset {
    pub fn depreciation(&self, _fye: u32) -> f32 {
        let days_held = days_held(&self.purchase_date);
        let num_days = days_held.num_days();

        // 200% / effective life of 4 years for computers.
        self.base_value * (num_days as f32 / 365 as f32) * 0.5
    }
}

pub fn days_held(purchase_date: &str) -> Duration {
    let date = NaiveDate::parse_from_str(purchase_date, "%Y-%m-%d").expect("failed to parse date");
    let eofy = NaiveDate::parse_from_str("2022-06-30", "%Y-%m-%d").expect("failed to parse eofy");

    let duration = eofy.signed_duration_since(date);
    duration
}
