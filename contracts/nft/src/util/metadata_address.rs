use crate::core::error::ContractError;
use crate::util::metadata_address::KeyType::ContractSpecification;
use bech32::ToBase32;
use uuid::Uuid;

pub enum Error {}

pub enum KeyType {
    ContractSpecification = 0x03,
}

pub struct MetadataAddress {
    pub bech32: String,
    pub key_type: KeyType,
    pub uuid: Uuid,
}

impl MetadataAddress {
    pub fn new(key_type: KeyType, uuid: Uuid) -> Result<MetadataAddress, ContractError> {
        match key_type {
            KeyType::ContractSpecification => {
                let addr = bech32::encode(
                    "contractspec",
                    [ContractSpecification as u8]
                        .iter()
                        .cloned()
                        .chain(
                            hex::decode(uuid.simple().encode_lower(&mut Uuid::encode_buffer()))
                                .unwrap()
                                .into_iter(),
                        )
                        .collect::<Vec<u8>>()
                        .to_base32(),
                    bech32::Variant::Bech32,
                )
                .unwrap();

                Ok(MetadataAddress {
                    bech32: addr,
                    key_type,
                    uuid,
                })
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bech32.as_str().as_bytes().to_vec()
    }
}

#[cfg(test)]
pub mod test {
    use crate::util::metadata_address::{KeyType, MetadataAddress};
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    pub fn new_contract_spec() {
        let meta_addr = MetadataAddress::new(
            KeyType::ContractSpecification,
            Uuid::from_str("9fe17f9a-56e1-4158-a8af-450680ac9e60").unwrap(),
        )
        .unwrap();

        assert_eq!(
            meta_addr.bech32,
            "contractspec1qw07zlu62ms5zk9g4azsdq9vnesqy4dtgm"
        );
    }
}
