use cosmwasm_std::{
    to_binary, Binary, ContractResult, HumanAddr, QuerierResult, StdResult, SystemError,
    SystemResult,
};
use provwasm_std::{Name, NameQueryParams, Names};
use std::collections::HashMap;

/// A mock for testing provenance name module queries.
#[derive(Clone, Default)]
pub struct NameQuerier {
    records: HashMap<String, Name>,
}

impl NameQuerier {
    pub fn new(inputs: &[(&str, &str, bool)]) -> Self {
        let mut records = HashMap::new();
        for (n, a, r) in inputs.iter() {
            let name = (*n).to_string();
            records.insert(
                name.to_owned(),
                Name {
                    name,
                    address: HumanAddr::from(*a),
                    restricted: *r,
                },
            );
        }
        NameQuerier { records }
    }

    fn query_result(&self, bin: StdResult<Binary>) -> QuerierResult {
        SystemResult::Ok(ContractResult::Ok(bin.unwrap()))
    }

    fn query_error(&self, error: &str, bin: StdResult<Binary>) -> QuerierResult {
        SystemResult::Err(SystemError::InvalidRequest {
            error: error.into(),
            request: bin.unwrap(),
        })
    }

    pub fn query(&self, params: &NameQueryParams) -> QuerierResult {
        match params {
            NameQueryParams::Resolve { name } => {
                let maybe_resolved = self.records.get(name).map(|record| {
                    self.query_result(to_binary(&Names {
                        records: vec![record.to_owned()],
                    }))
                });
                match maybe_resolved {
                    Some(r) => r,
                    None => self.query_error("no address bound to name", to_binary(params)),
                }
            }
            NameQueryParams::Lookup { address } => self.query_result(to_binary(&Names {
                records: self
                    .records
                    .values()
                    .cloned()
                    .filter(|r| !r.address.is_empty() && r.address == *address)
                    .collect(),
            })),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::from_binary;

    #[test]
    fn resolve() {
        let querier = NameQuerier::new(&[
            ("foo.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
            ("bar.pb", "tp1238aw49q0nvz6nyj86mxgppn0wt60td5ngfhk9", false),
        ]);

        let params = NameQueryParams::Resolve {
            name: String::from("foo.pb"),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Names = from_binary(&bin).unwrap();

        assert_eq!(rep.records.len(), 1);
        assert_eq!(
            rep.records[0].address,
            HumanAddr::from("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
        )
    }

    #[test]
    fn resolve_name_not_bound_err() {
        let querier =
            NameQuerier::new(&[("foo.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false)]);

        let params = NameQueryParams::Resolve {
            name: String::from("bar.pb"),
        };
        let err = querier.query(&params).unwrap_err();

        match err {
            SystemError::InvalidRequest { error, .. } => {
                assert_eq!(error, "no address bound to name")
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn lookup() {
        let querier = NameQuerier::new(&[
            ("foo.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
            ("bar.pb", "tp1238aw49q0nvz6nyj86mxgppn0wt60td5ngfhk9", false),
        ]);

        let params = NameQueryParams::Lookup {
            address: HumanAddr::from("tp1238aw49q0nvz6nyj86mxgppn0wt60td5ngfhk9"),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Names = from_binary(&bin).unwrap();

        assert_eq!(rep.records.len(), 1);
        assert_eq!(rep.records[0].name, "bar.pb")
    }

    #[test]
    fn lookup_empty() {
        let querier = NameQuerier::new(&[
            ("foo.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
            ("bar.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
            ("baz.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
        ]);

        let params = NameQueryParams::Lookup {
            address: HumanAddr::from("tp1238aw49q0nvz6nyj86mxgppn0wt60td5ngfhk9"),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Names = from_binary(&bin).unwrap();

        // Assert there are no records, but not an error
        assert_eq!(rep.records.len(), 0);
    }
}
