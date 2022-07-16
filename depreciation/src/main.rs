//! Calculate deprecation claims.
//!

use depreciation::Asset;

fn main() {
    let data = Asset {
        base_value: 498.61,
        purchase_date: "2021-09-14".to_string(),
    };

    let depreciation = data.depreciation(2022);
    println!("depreciation: {}", depreciation);
}
