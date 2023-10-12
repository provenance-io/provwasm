use crate::core::constants::CONTRACT_INFO_KEY;
use crate::core::msg::ContractInfoResponse;
use cw_storage_plus::Item;

pub const CONTRACT_INFO: Item<ContractInfoResponse> = Item::new(CONTRACT_INFO_KEY);
