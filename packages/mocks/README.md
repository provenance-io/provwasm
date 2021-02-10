# Mocks

This project contains mocks that allow smart contract developers to write robust unit tests that
include native provenance module functionality.

## Example Usage

```rust
// Example unit test that uses provwasm mocks for a resolve name query.

// ref: contracts/name/src/msg.rs

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Resolve { name: String },
}

// ref: contracts/name/src/contract.rs

use cosmwasm_std::testing::mock_env;
use cosmwasm_std::from_binary;
use provwasm_mocks::mock_dependencies;
use provwasm_std::Name;

#[test]
fn query_resolve() {
    // Create provenance mock deps with a single bound name.
    let mut deps = mock_dependencies(&[]);
    deps.querier
        .with_names(&[("a.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false)]);

    // Call the smart contract query function to resolve the address for our test name.
    let bin = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Resolve {
            name: "a.pb".into(),
        },
    )
    .unwrap();

    // Ensure that we got the expected address.
    let rep: Name = from_binary(&bin).unwrap();
    assert_eq!(
        rep.address.as_str(),
        "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync"
    )
}
```
