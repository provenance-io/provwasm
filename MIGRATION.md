# Migrating Contracts

This guide provides information to assist in migrating contracts over major releases.

___There are also example [contracts](./contracts) that provide concrete examples using the current release.___

## v2.1.0/v2.2.0 -> 2.3.0

Cosmwasm is upgraded to 2.0.4 and Provenance to 1.19.0. These major upgrades are mostly handled within the library:

- The encoding for `Any` types has changed. This means if your contract has a query that expects a `MarkerAccount` in
  its response, it will fail as of `Provenance 1.19.0`. This is an internal change and only requires bumping the
  provwasm-std dependency.
- Provwasm now provides all provenance messages and queries.
  **There are 2 caveats**: not all queries will return valid responses (
  see [allowed queries](https://github.com/provenance-io/provenance/blob/7d6c507cab780bb6f0bdeef1e895c870cf4c7465/internal/provwasm/stargate_whitelist.go#L56)),
  and `provwasm-std` takes longer to build than before. This will improve with Grpc support.
- Follow the common [Cosmwasm migration path](https://github.com/cosmos/cosmos-rust). It is likely that your contract
  will not require many changes. `QueryRequest::Grpc` is not supported.

## v1.x.x -> 2.0.0

This major upgrade changes the message base type from `CosmosMsg::Custom<ProvenanceMsg>` to `CosmosMsg::Stargate`. This
allows all message types to be generated from the actual proto definitions in Provenance, making it much easier to add
and deprecate types. It will require updating all `provwasm` messages and queries. You will notice a `Deprecated`
warning until you replace the older types.

---

- Type:

  v1.x.x `Marker`:
  ```rust
  use crate::{AccessGrant, Marker, MarkerAccess, MarkerStatus, MarkerType};
  
  Marker{
      address: Addr::unchecked("tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u".to_string()),
      allow_forced_transfer: false,
      coins: vec![],
      account_number: 0,
      sequence: 0,
      manager: env.contract.address.to_string(),
      permissions: vec![AccessGrant{ permissions: vec![MarkerAccess::Admin], address: Addr::unchecked("tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz".to_string()) }],
      status: MarkerStatus::Active,
      denom: "nugz".to_string(),
      total_supply: Decimal(Uint128(420)),
      marker_type: MarkerType::Restricted,
      supply_fixed: false,
  }
  ```

  v2.0.0 `MarkerAccount`:
  ```rust
  use provwasm_std::types::cosmos::auth::v1beta1::BaseAccount;
  use provwasm_std::types::provenance::marker::v1::{AccessGrant, MarkerAccount, MarkerStatus};
  
  MarkerAccount{
      base_account: Some(
          BaseAccount {
              address: "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u".to_string(),
              pub_key: None,
              account_number: 10,
              sequence: 0,
          }
      ),
      manager: env.contract.address.to_string(),
      access_control: vec![AccessGrant {
          address: "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz".to_string(),
          permissions: vec![1, 2, 3, 4, 5, 6, 7],
      }],
      status: MarkerStatus::Active.into(),
      denom: "nugz".to_string(),
      supply: "420".to_string(),
      marker_type: 0,
      supply_fixed: false,
      allow_governance_control: false,
      allow_forced_transfer: false,
      required_attributes: vec![],
  }
  ```

---

- Message:
  ```rust
  use provwasm_std::types::provenance::marker::v1::MsgAddMarkerRequest;

  let msg = MsgAddMarkerRequest {
      amount: Some(coin),
      manager: validate_address(contract_address.clone())?.to_string(),
      from_address: validate_address(contract_address)?.to_string(),
      status: MarkerStatus::Proposed.into(),
      marker_type: marker_type.into(),
      access_list: vec![],
      supply_fixed: false,
      allow_governance_control: false,
      allow_forced_transfer,
      required_attributes: vec![],
  };

  let res = Response::new()
  .add_message(msg)
  .add_attribute("action", "provwasm.contracts.marker.create")
  .add_attribute("integration_test", "v2")
  .add_attribute("marker_supply", supply)
  .add_attribute("marker_denom", denom);
  ```

---  

- Query:
  ```rust
  let querier = MarkerQuerier::new(&deps.querier);
  let response = querier.marker(id)?;
  if let Some(marker) = response.marker {
      return MarkerAccount::try_from(marker)
  }
  ```

  ***`MarkerAccount` is the only special type that requires `::try_from()` since it is of type `Any`. Normally query
  responses are typed like the following:***

  ```rust
  let querier = NameQuerier::new(&deps.querier);
  let query_response = querier.resolve(name)?; // query_response is of type QueryResolveResponse
  ```

---

### Testing

- Setup
  ```diff
  -let mut deps = mock_dependencies(&[]);
  +let mut deps = mock_provenance_dependencies();
  ```

- Messages
  ```rust
  #[test]
    fn create_marker() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let contract_address = env.contract.address.to_string();

        // Expected marker supply
        let expected_msg: Binary = MsgAddMarkerRequest {
            ...
        }
        .try_into()
        .unwrap();

        // Create marker execute message
        let msg = ExecuteMsg::Create {
            ...
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created

        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgAddMarkerRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }
  ```

- Query

  ```rust
  #[test]
    fn query_marker() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();

        // Create a mock querier with our expected marker.
        let expected_marker = MarkerAccount {
            ...
        };

        let mock_marker_response = QueryMarkerResponse {
            marker: Some(Any {
                type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
                value: expected_marker.encode_to_vec(),
            }),
        };

        QueryMarkerRequest::mock_response(&mut deps.querier, mock_marker_response);

        // Query and ensure we got the expected marker
        let req = QueryMsg::GetByDenom {
            ...
        };

        let bin = query(deps.as_ref(), mock_env(), req).unwrap();

        let marker: Marker = from_binary(&bin).unwrap();
        assert_eq!(marker.marker_account, expected_marker);
        assert_eq!(
            marker.marker_account.base_account.unwrap().address,
            "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u"
        )
    }
  ```