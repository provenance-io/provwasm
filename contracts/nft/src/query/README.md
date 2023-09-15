### Supported CW721 Query Methods

- Approval
  ```rust
  /// Returns the operator approval details for a given token
  #[returns(cw721::ApprovalResponse)]
  Approval {
      token_id: String,
      spender: String,
      include_expired: Option<bool>,
  }
  ```

- Approvals
  ```rust
  /// Returns all operator approvals details for a given token
  #[returns(cw721::ApprovalsResponse)]
  Approvals {
      /// Token ID in UUID format (Scope UUID)
      token_id: String,
      include_expired: Option<bool>,
  }
  ```

- Mint
  ```rust
  /// Returns the `minter`
  #[returns(MinterResponse)]
  Minter {}
  ```

- Owner Of
  ```rust
  /// Returns the owner of the given token, error if token does not exist
  #[returns(cw721::OwnerOfResponse)]
  OwnerOf {
      /// Token ID in UUID format (Scope UUID)
      token_id: String,
      /// unset or false will filter out expired approvals, you must set to true to see them
      include_expired: Option<bool>,
  }
  ```

- Operator
  ```rust
  /// Returns the operator approval details for all tokens of a given owner or an error if not set
  #[returns(cw721::OperatorResponse)]
  Operator {
    owner: String,
    operator: String,
    include_expired: Option<bool>,
  }
  ```

- All Operators
  ```rust
  /// Returns all operator approvals details for all tokens of a given owner
  #[returns(cw721::OperatorsResponse)]
  AllOperators {
    owner: String,
    include_expired: Option<bool>,
    start_after: Option<String>,
    limit: Option<u32>,
  }
  ```

- Number of Tokens
  ```rust
  /// Total number of tokens issued
  #[returns(cw721::NumTokensResponse)]
  NumTokens {}
  ```

### Metadata Query Methods
- Contract Info
  ```rust
  /// MetaData Extension
  /// Returns metadata about the contract
  #[returns(cw721::ContractInfoResponse)]
  ContractInfo {}
  ```

- Contract Version
  ```rust
  /// MetaData Extension
  /// Returns the contract version information
  #[returns(ContractVersionResponse)]
  ContractVersion {}
  ```

- NFT Info
  ```rust
  /// MetaData Extension
  /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
  /// but directly from the contract
  #[returns(cw721::NftInfoResponse<ScopeResponse>)]
  NftInfo { token_id: String }
  ```
  
- All NFT Info
  ```rust
  /// MetaData Extension
  /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
  /// for clients
  #[returns(cw721::AllNftInfoResponse<ScopeResponse>)]
  AllNftInfo {
    /// Token ID in UUID format (Scope UUID)
    token_id: String,
    /// unset or false will filter out expired approvals, you must set to true to see them
    include_expired: Option<bool>,
  }
  ```
  
### Paginated Query Methods

Pagination is achieved via `start_after` and `limit`. `Limit`, if unset, will use `DefaultLimit` (suggested 10). If set, it will be used up to a `MaxLimit` value (suggested 30).

If `start_after` is unset, the query returns the first results, ordered lexicographically by `token_id`. If `start_after` is set, then it returns the first limit tokens after the given one.

- Tokens
  ```rust
  /// Enumerable extension
  /// Returns all tokens owned by the given address, [] if unset.
  #[returns(cw721::TokensResponse)]
  Tokens {
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
  }
  ```

- All Tokens
  ```rust
  /// Enumerable extension
  /// Requires pagination. Lists all token_ids controlled by the contract.
  #[returns(cw721::TokensResponse)]
  AllTokens {
    start_after: Option<String>,
    limit: Option<u32>,
  }
  ```