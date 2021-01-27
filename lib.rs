use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm::{execute_entry_point, ext, oei, prepare_entry_point};

// Input Parameters
#[derive(OBIDecode, OBISchema)]
struct Input {
    symbol: String,
    multiplier: u64,
}

// Output Parameters
#[derive(OBIEncode, OBISchema)]
struct Output {
    volume: u64,
}

// Preparation Phase
#[no_mangle]
fn prepare_impl(input: Input) {
    // Coingecko volume data source
    oei::ask_external_data(1, 9, &input.symbol.as_bytes());
}

// Execution Phase
#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let avg: Option<f64> = ext::load_average(1);
    Output { volume: (avg.unwrap() * input.multiplier as f64) as u64 }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
