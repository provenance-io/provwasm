use crate::common::{query_error, query_result};
use cosmwasm_std::{to_binary, Addr, QuerierResult};
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
                name.clone(),
                Name {
                    name,
                    address: Addr::unchecked(*a),
                    restricted: *r,
                },
            );
        }
        NameQuerier { records }
    }

    fn get_names(&self, name: &str) -> Option<Names> {
        self.records.get(name).map(|record| Names {
            records: vec![record.to_owned()],
        })
    }

    fn lookup_names(&self, address: &Addr) -> Names {
        Names {
            records: self
                .records
                .values()
                .cloned()
                .filter(|r| !r.address.to_string().is_empty() && r.address == *address)
                .collect(),
        }
    }

    pub fn query(&self, params: &NameQueryParams) -> QuerierResult {
        match params {
            NameQueryParams::Resolve { name } => match self.get_names(name) {
                Some(names) => query_result(to_binary(&names)),
                None => query_error("no address bound to name", to_binary(params)),
            },
            NameQueryParams::Lookup { address } => {
                let names = self.lookup_names(address);
                query_result(to_binary(&names))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{from_binary, SystemError};

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
            Addr::unchecked("tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
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
            address: Addr::unchecked("tp1238aw49q0nvz6nyj86mxgppn0wt60td5ngfhk9"),
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
            address: Addr::unchecked("tp1238aw49q0nvz6nyj86mxgppn0wt60td5ngfhk9"),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let rep: Names = from_binary(&bin).unwrap();

        // Assert there are no records, but not an error
        assert_eq!(rep.records.len(), 0);
    }
}
