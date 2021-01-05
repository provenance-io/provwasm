use cosmwasm_std::{
    to_binary, Binary, ContractResult, HumanAddr, QuerierResult, StdResult, SystemResult,
};
use provwasm_std::{Attribute, AttributeQueryParams, AttributeValueType, Attributes};

use std::collections::HashMap;

/// A mock for testing provenance account metadata queries.
#[derive(Clone, Default)]
pub struct AttributeQuerier {
    records: HashMap<HumanAddr, Vec<Attribute>>,
}

// Helper function to convert stringt to attribute value type.
fn determine_attr_type(typ: &str) -> AttributeValueType {
    if typ == "json" {
        AttributeValueType::Json
    } else if typ == "bytes" {
        AttributeValueType::Bytes
    } else if typ == "uuid" {
        AttributeValueType::Uuid
    } else {
        AttributeValueType::String
    }
}

impl AttributeQuerier {
    pub fn new(address: &str, inputs: &[(&str, &str, &str)]) -> Self {
        let mut records = HashMap::new();
        let mut attrs = vec![];
        for (n, v, t) in inputs.iter() {
            let name = (*n).to_string();
            let value = (*v).to_string();
            attrs.push(Attribute {
                name,
                value: Binary::from(value.as_bytes()),
                value_type: determine_attr_type(t),
            });
        }
        records.insert(HumanAddr::from(address), attrs);
        AttributeQuerier { records }
    }

    fn query_result(&self, bin: StdResult<Binary>) -> QuerierResult {
        SystemResult::Ok(ContractResult::Ok(bin.unwrap()))
    }

    fn query_attributes(&self, address: &HumanAddr, name: &str) -> Vec<Attribute> {
        self.records
            .get(address)
            .map(|attrs| {
                attrs
                    .iter()
                    .filter(|r| r.name.to_lowercase() == *name.to_lowercase())
                    .cloned()
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    }

    pub fn query(&self, params: &AttributeQueryParams) -> QuerierResult {
        match params {
            AttributeQueryParams::GetAttributes { address, name } => {
                self.query_result(to_binary(&Attributes {
                    address: address.to_owned(),
                    attributes: self.query_attributes(address, name),
                }))
            }
            AttributeQueryParams::GetAllAttributes { address } => {
                let empty: Vec<Attribute> = vec![];
                self.query_result(to_binary(&Attributes {
                    address: address.to_owned(),
                    attributes: self.records.get(address).unwrap_or(&empty).to_vec(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::from_binary;

    #[test]
    fn get_attributes() {
        let querier = AttributeQuerier::new(
            "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync",
            &[
                ("id", "88202946-4447-401c-b38c-75c46d8b5692", "uuid"),
                ("profile", "dd152f91-e39f-4c17-9ebf-6eb534ec61e5", "uuid"),
            ],
        );

        let params = AttributeQueryParams::GetAttributes {
            address: HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync"),
            name: String::from("id"),
        };

        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Attributes = from_binary(&bin).unwrap();

        assert_eq!(
            rep.address,
            HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
        );
        assert_eq!(rep.attributes.len(), 1);
        assert_eq!(
            rep.attributes[0].value,
            Binary::from("88202946-4447-401c-b38c-75c46d8b5692".as_bytes())
        )
    }

    #[test]
    fn get_attributes_empty() {
        let querier = AttributeQuerier::new(
            "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync",
            &[
                ("id", "88202946-4447-401c-b38c-75c46d8b5692", "uuid"),
                ("profile", "dd152f91-e39f-4c17-9ebf-6eb534ec61e5", "uuid"),
            ],
        );

        let params = AttributeQueryParams::GetAttributes {
            address: HumanAddr::from("tp1fhdhzrnpq9rnnyp8r6xvm75t0cmdul3xqyp6sd"),
            name: String::from("uuid"),
        };

        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Attributes = from_binary(&bin).unwrap();

        // Assert there are no records, but not an error
        assert_eq!(rep.attributes.len(), 0);
    }

    #[test]
    fn get_all_attributes() {
        let querier = AttributeQuerier::new(
            "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync",
            &[
                ("id", "88202946-4447-401c-b38c-75c46d8b5692", "uuid"),
                ("profile", "dd152f91-e39f-4c17-9ebf-6eb534ec61e5", "uuid"),
                ("kyc", "8cd6ec14-3f5e-4a08-98c5-0ea1585f9b76", "uuid"),
            ],
        );

        let params = AttributeQueryParams::GetAllAttributes {
            address: HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync"),
        };

        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Attributes = from_binary(&bin).unwrap();

        assert_eq!(
            rep.address,
            HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
        );
        assert_eq!(rep.attributes.len(), 3);
    }

    #[test]
    fn get_attributes_dup_keys() {
        let querier = AttributeQuerier::new(
            "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync",
            &[
                ("id", "88202946-4447-401c-b38c-75c46d8b5692", "uuid"),
                ("id", "06c1494e-fdee-4396-b7e9-16688ff5966c", "uuid"),
                ("profile", "dd152f91-e39f-4c17-9ebf-6eb534ec61e5", "uuid"),
                ("kyc", "8cd6ec14-3f5e-4a08-98c5-0ea1585f9b76", "uuid"),
            ],
        );

        let params = AttributeQueryParams::GetAttributes {
            address: HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync"),
            name: String::from("id"),
        };

        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Attributes = from_binary(&bin).unwrap();

        assert_eq!(
            rep.address,
            HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
        );
        assert_eq!(rep.attributes.len(), 2);
    }

    #[test]
    fn get_all_attributes_empty() {
        let querier = AttributeQuerier::new(
            "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync",
            &[
                ("id", "88202946-4447-401c-b38c-75c46d8b5692", "uuid"),
                ("profile", "dd152f91-e39f-4c17-9ebf-6eb534ec61e5", "uuid"),
                ("kyc", "8cd6ec14-3f5e-4a08-98c5-0ea1585f9b76", "uuid"),
            ],
        );

        let params = AttributeQueryParams::GetAllAttributes {
            address: HumanAddr::from("tp1fhdhzrnpq9rnnyp8r6xvm75t0cmdul3xqyp6sd"),
        };

        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Attributes = from_binary(&bin).unwrap();

        // Assert there are no records, but not an error
        assert_eq!(rep.attributes.len(), 0);
    }
}
