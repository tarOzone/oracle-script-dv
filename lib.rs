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

// bandcli tx oracle create-oracle-script \
// --name test-dv-oracle-2 \
// --description "Test DV oracle #2" \
// --script band_wasm_bg.wasm \
// --schema "{symbol:string,multiplier:u64}/{price:u64,sources:[{name:string,time:u64}]}" \
// --owner band1w8mmcw6y6gsptyae6zwl67t3cccxvnnwlmfkdj \
// --from band1wktqrckx4yx55wauf9x3n4z72rlhcvymztnxvl \
// --chain-id band-guanyu-testnet3 \
// --fees 60000uband --gas 600000
// --url 
