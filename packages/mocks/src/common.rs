use cosmwasm_std::{Binary, ContractResult, QuerierResult, StdResult, SystemError, SystemResult};
use std::fs::File;
use std::io::Read;

// Helper for returning mock query success cases.
pub fn query_result(bin: StdResult<Binary>) -> QuerierResult {
    SystemResult::Ok(ContractResult::Ok(bin.unwrap()))
}

// Helper for returning mock query error cases.
pub fn query_error(error: &str, bin: StdResult<Binary>) -> QuerierResult {
    SystemResult::Err(SystemError::InvalidRequest {
        error: String::from(error),
        request: bin.unwrap(),
    })
}

/// A helper function for mock testing. This function allows tests to read a file in CosmWasm binary
/// format. This function will panic on any error. Because of this, it should only be used in tests.
pub fn must_read_binary_file(filename: &str) -> Binary {
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            Binary::from(content.as_bytes())
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}
