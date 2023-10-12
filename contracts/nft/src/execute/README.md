### Supported CW721 Execute Methods

- Approve
  ```rust
  /// Allow operator to transfer / send the token from the owner's account.
  /// If expiration is set, then this allowance has a time/height limit
  Approve {
    spender: Addr,
    /// Token ID in UUID format (Scope UUID)
    token_id: String,
    expires: Option<Expiration>,
  },
  ```
- Approve All
  ```rust
  /// Allows operator to transfer / send any token from the owner's account.
  /// If expiration is set, then this allowance has a time/height limit
  ApproveAll {
    operator: Addr,
    expires: Option<Expiration>,
  },
  ```
- Revoke
  ```rust
  /// Remove previously granted Approval
  Revoke {
    spender: Addr,
    /// Token ID in UUID format (Scope UUID)
    token_id: String,
  },
  ```
- Revoke All
  ```rust
  /// Remove previously granted ApproveAll permission
  RevokeAll { operator: Addr },
  ```
- Send NFT
  ```rust 
  /// Transfer a token to a contract and trigger an action on the receiving contract.
  SendNft {
    /// Generated `UUID` to use for creating the `NFT`/`Scope`
    token_id: String,
    /// Generated `UUID` to use for creating the `Session` to write the new owner record
    session_uuid: String,
    /// Receiving Contract Address
    contract: String,
    /// Message to send to receiving contract
    msg: Binary,
  }
  ```
- Transfer NFT
  ```rust
  /// Transfer a token to another account without triggering actions
  TransferNft {
    /// Generated `UUID` to use for creating the `NFT`/`Scope`
    token_id: String,
    recipient: Addr,
    /// Generated `UUID` to use for creating the `Session` to write the new owner record
    session_uuid: String,
  },
  ```

### Additional Execution Methods

- Burn
  ```rust
  /// Burn an NFT the sender has access to
  Burn {
    /// Token ID in UUID format (Scope UUID)
    token_id: String,
  }
  ```
- Mint
  ```rust
  /// Mint a new NFT, can only be called by the contract minter
  Mint {
    /// Generated UUID to use for creating the NFT (Scope)
    scope_uuid: String,
    /// Generated UUID to use for creating the Session which encapsulates an owner data record
    session_uuid: String,
    /// Recipient of the NFT
    recipient: Addr,
  }
  ```
- Update Ownership
  ```rust
  UpdateOwnership {
    new_owner: String,
    expiry: Option<Expiration>,
  }
  
  AcceptOwnership
  
  RenounceOwnership
  ```