use bech32::ToBase32;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::util::metadata_address::KeyType::{
    ContractSpecification, Record, RecordSpecification, Scope, ScopeSpecification, Session,
};

pub enum Error {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyType {
    Scope = 0x00,
    Session = 0x01,
    Record = 0x02,
    ContractSpecification = 0x03,
    ScopeSpecification = 0x04,
    RecordSpecification = 0x05,
}

impl KeyType {
    pub fn to_str(&self) -> &str {
        match self {
            Scope => "scope",
            Session => "session",
            Record => "record",
            ContractSpecification => "contractspec",
            ScopeSpecification => "scopespec",
            RecordSpecification => "recspec",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataAddress {
    pub bech32: String,
    pub bytes: Vec<u8>,
    pub key_type: KeyType,
}

impl MetadataAddress {
    pub fn contract_specification(
        contract_specification_uuid: Uuid,
    ) -> Result<MetadataAddress, ContractError> {
        let key_type_byte = ContractSpecification as u8;
        let bytes = [key_type_byte]
            .iter()
            .cloned()
            .chain(Self::hex_encode_uuid(contract_specification_uuid))
            .collect::<Vec<u8>>();

        let addr = Self::encode_bech32(ContractSpecification, &bytes).unwrap();

        Ok(MetadataAddress {
            bech32: addr,
            bytes,
            key_type: ContractSpecification,
        })
    }

    pub fn scope_specification(
        scope_specification_uuid: Uuid,
    ) -> Result<MetadataAddress, ContractError> {
        let key_type_byte = ScopeSpecification as u8;
        let bytes = [key_type_byte]
            .iter()
            .cloned()
            .chain(Self::hex_encode_uuid(scope_specification_uuid))
            .collect::<Vec<u8>>();

        let addr = Self::encode_bech32(ScopeSpecification, &bytes).unwrap();

        Ok(MetadataAddress {
            bech32: addr,
            bytes,
            key_type: ScopeSpecification,
        })
    }

    pub fn scope(scope_uuid: Uuid) -> Result<MetadataAddress, ContractError> {
        let key_type_byte = Scope as u8;
        let bytes = [key_type_byte]
            .iter()
            .cloned()
            .chain(Self::hex_encode_uuid(scope_uuid))
            .collect::<Vec<u8>>();

        let addr = Self::encode_bech32(Scope, &bytes).unwrap();

        Ok(MetadataAddress {
            bech32: addr,
            bytes,
            key_type: Scope,
        })
    }

    pub fn record(scope_uuid: Uuid, record_name: String) -> Result<MetadataAddress, ContractError> {
        let key_type_byte = Record as u8;
        let bytes = [key_type_byte]
            .iter()
            .cloned()
            .chain(Self::hex_encode_uuid(scope_uuid))
            .chain(Self::hash_bytes(record_name))
            .collect::<Vec<u8>>();

        let addr = Self::encode_bech32(Record, &bytes).unwrap();

        Ok(MetadataAddress {
            bech32: addr,
            bytes,
            key_type: Record,
        })
    }

    pub fn record_specification(
        contract_specification_uuid: Uuid,
        record_specification_name: String,
    ) -> Result<MetadataAddress, ContractError> {
        let key_type_byte = RecordSpecification as u8;
        let bytes = [key_type_byte]
            .iter()
            .cloned()
            .chain(Self::hex_encode_uuid(contract_specification_uuid))
            .chain(Self::hash_bytes(record_specification_name))
            .collect::<Vec<u8>>();

        let addr = Self::encode_bech32(RecordSpecification, &bytes).unwrap();

        Ok(MetadataAddress {
            bech32: addr,
            bytes,
            key_type: RecordSpecification,
        })
    }

    pub fn session(scope_uuid: Uuid, session_uuid: Uuid) -> Result<MetadataAddress, ContractError> {
        let key_type_byte = Session as u8;
        let bytes = [key_type_byte]
            .iter()
            .cloned()
            .chain(Self::hex_encode_uuid(scope_uuid))
            .chain(Self::hex_encode_uuid(session_uuid))
            .collect::<Vec<u8>>();

        let addr = Self::encode_bech32(Session, &bytes).unwrap();

        Ok(MetadataAddress {
            bech32: addr,
            bytes,
            key_type: Session,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bech32.as_str().as_bytes().to_vec()
    }

    fn hex_encode_uuid(uuid: Uuid) -> Vec<u8> {
        hex::decode(uuid.simple().encode_lower(&mut Uuid::encode_buffer())).unwrap()
    }

    fn encode_bech32(key_type: KeyType, bytes: &Vec<u8>) -> Result<String, bech32::Error> {
        bech32::encode(
            key_type.to_str(),
            bytes.to_base32(),
            bech32::Variant::Bech32,
        )
    }

    pub fn hash_bytes(data: String) -> Vec<u8> {
        let hash = Sha256::digest(data.trim().to_lowercase().as_bytes());
        hash[0..16].to_vec()
    }
}

#[cfg(test)]
pub mod test {
    use std::str::FromStr;

    use uuid::Uuid;

    use crate::util::metadata_address::{KeyType, MetadataAddress};

    #[test]
    pub fn new_contract_spec() {
        let meta_addr = MetadataAddress::contract_specification(
            Uuid::from_str("9fe17f9a-56e1-4158-a8af-450680ac9e60").unwrap(),
        )
        .unwrap();

        assert_eq!(
            meta_addr,
            MetadataAddress {
                bech32: "contractspec1qw07zlu62ms5zk9g4azsdq9vnesqy4dtgm".to_string(),
                bytes: vec![
                    3, 159, 225, 127, 154, 86, 225, 65, 88, 168, 175, 69, 6, 128, 172, 158, 96
                ],
                key_type: KeyType::ContractSpecification,
            }
        );
    }

    #[test]
    pub fn new_scope_spec() {
        let meta_addr = MetadataAddress::scope_specification(
            Uuid::from_str("9fe17f9a-56e1-4158-a8af-450680ac9e60").unwrap(),
        )
        .unwrap();

        assert_eq!(
            meta_addr,
            MetadataAddress {
                bech32: "scopespec1qj07zlu62ms5zk9g4azsdq9vnesqxcv7hd".to_string(),
                bytes: vec![
                    4, 159, 225, 127, 154, 86, 225, 65, 88, 168, 175, 69, 6, 128, 172, 158, 96
                ],
                key_type: KeyType::ScopeSpecification,
            }
        );
    }

    #[test]
    pub fn new_scope() {
        let meta_addr =
            MetadataAddress::scope(Uuid::from_str("9fe17f9a-56e1-4158-a8af-450680ac9e60").unwrap())
                .unwrap();

        assert_eq!(
            meta_addr,
            MetadataAddress {
                bech32: "scope1qz07zlu62ms5zk9g4azsdq9vnesqg74ssc".to_string(),
                bytes: vec![
                    0, 159, 225, 127, 154, 86, 225, 65, 88, 168, 175, 69, 6, 128, 172, 158, 96
                ],
                key_type: KeyType::Scope,
            }
        );
    }

    #[test]
    pub fn new_record_spec() {
        let meta_addr = MetadataAddress::record_specification(
            Uuid::from_str("9fe17f9a-56e1-4158-a8af-450680ac9e60").unwrap(),
            "nft_record_spec_name".to_string(),
        )
        .unwrap();

        assert_eq!(
            meta_addr,
            MetadataAddress {
                bech32: "recspec1qk07zlu62ms5zk9g4azsdq9vneswsu3m8wtqu0zfqu9edf8r7l4juadl3c0"
                    .to_string(),
                bytes: vec![
                    5, 159, 225, 127, 154, 86, 225, 65, 88, 168, 175, 69, 6, 128, 172, 158, 96,
                    232, 114, 59, 59, 150, 14, 60, 73, 7, 11, 150, 164, 227, 247, 235, 46
                ],
                key_type: KeyType::RecordSpecification,
            }
        );
    }

    #[test]
    pub fn new_record() {
        let meta_addr = MetadataAddress::record(
            Uuid::from_str("9fe17f9a-56e1-4158-a8af-450680ac9e60").unwrap(),
            "nft_record_spec_name".to_string(),
        )
        .unwrap();

        assert_eq!(
            meta_addr,
            MetadataAddress {
                bech32: "record1q207zlu62ms5zk9g4azsdq9vneswsu3m8wtqu0zfqu9edf8r7l4jugz3hcl"
                    .to_string(),
                bytes: vec![
                    2, 159, 225, 127, 154, 86, 225, 65, 88, 168, 175, 69, 6, 128, 172, 158, 96,
                    232, 114, 59, 59, 150, 14, 60, 73, 7, 11, 150, 164, 227, 247, 235, 46
                ],
                key_type: KeyType::Record,
            }
        );
    }
}
