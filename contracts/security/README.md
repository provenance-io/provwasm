# Security Contract
The purpose of this contract is to provide the Provenance Blockchain with the ability to associate assets with a security type. It provides the ability
to map an address to a supported security type and enable efficient lookups by address or security types.

## Concepts
In the security contract, there are a few concepts that should be understood to correctly use the contract.

### Owner
The owner of the contract is the only address that can run the contract's execution end points. This address will remain as the owner, unless they decide
to revoke their ownership and pass it on to another address.

### Asset
Assets are markers and scopes that have been recorded on the Provenance Blockchain.

### Security Type
A Security Type is a grouping of data that contains a category and, optionally, a name for a security. It may be difficult to initially come up with a name for a finanacial instrument, but
it allows the Provenance Blockchain to effectively categorize assets.

## Transactions
The security contract enables all three transaction entry points. These transactions allow the user to manipulate contract and blockchain state with messages. 

### [Instantiation]
These message variants are utilized in the construction of the contract and to activate the instantiate entry point.

#### [Default]
A default instantiation message that provides and demonstrates commonly used setup functionality.

#### Request Parameters
- owner: The address of the account that will own the contract.
- security_types: An initial list of security types that can be linked to an asset.

#### Emitted Attributes
- action: This will always have a value of "instantiate".

#### Emitted Events
This transaction does not emit any events.

#### Request Sample
```
{
    "default": {
        "owner": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr",
        "security_types": [
            {
                "category": "category_name",
                "name": "instrument_name",
            },
            {
                "category": "category_name",
            },
        ]
    }
}
```

### [Execute]
The execute message variants encompass the various methods through which users can interact with and modify the contract. These variants activate the execute entry point.

#### [Change Owner]
This transaction will change the owner of the contract to the provided address.

##### Note
This message variant will fail if the sender is not the current owner of the contract.

#### Request Parameters
- new_owner: The address of the contract's new owner.

#### Emitted Attributes
- action: This will always have the value of "change_owner".

#### Emitted Events
- change_owner:
    - previous_owner: The address of the previous owner.
    - new_owner: The address of the new owner.

#### Request Sample
```
{
    "change_owner": {
        "new_owner": "tp1ek77wghn0n9lc7x2uycgh697sjc7fvy995keun"
    }
}
```

#### [Add Security Types]
This transaction will update the contract to allow additional security types to be linked against an asset.

##### Note
This message variant will fail if the sender is not the current owner of the contract. The category and name, if supplied, must have
a length of at least 1.

#### Request Parameters
- security_types: The security types that the contract should add to the list of supported types.

#### Emitted Attributes
- action: This will always have the value of "add_security_types".

#### Emitted Events
- update_security_types

#### Request Sample
```
{
    "add_security_types": {
        "security_types": [
            {
                "category": "category_name",
                "name": "instrument_name",
            },
            {
                "category": "category_name",
            },
        ]
    }
}
```

#### [Remove Security Types]
This transaction will update the contract to remove security types and prevent assets from being linked against them.

##### Note
This message variant will fail if the sender is not the current owner of the contract. If any provided security type is linked
against an asset, then the transaction will fail.

#### Request Parameters
- security_types: The security types that the contract should remove from the list of supported types.

#### Emitted Attributes
- action: This will always have the value of "remove_security_types".

#### Emitted Events
- remove_security_types

#### Request Sample
```
{
    "remove_security_types": {
        "security_types": [
            {
                "category": "category_name",
                "name": "instrument_name",
            },
            {
                "category": "category_name",
            },
        ]
    }
}
```

#### [Set Security]
This transaction will link an asset to a security type, and replace any pre-existing link.

##### Note
This message variant will fail if the sender is not the current owner of the contract. The asset that is supplied
must be either a marker or a scope. The provided security type must also be supported by the contract.

#### Request Parameters
- asset_addr: The address of the asset to be set.
- security: The type to link against the asset.

#### Emitted Attributes
- action: This will always have the value of "set_security".

#### Emitted Events
- set_security:
  - asset_address: The address of the asset that was linked.
  - security: The type of security linked to the asset.

#### Request Sample
```
{
    "set_security": {
        "asset_addr": "tp1ek77wghn0n9lc7x2uycgh697sjc7fvy995keun",
        "security": {
            "category": "category_name",
            "name": "instrument_name",
        }
    }
}
```

#### [Set Security Multiple]
This transaction will link multiple assets to the same security type, and replace any pre-existing links.

##### Note
This message variant will fail if the sender is not the current owner of the contract. All assets supplied
must be either a marker or a scope, and the provided security type must be supported by the contract.

#### Request Parameters
- assets: The addresses of the asset to be set.
- security: The type to link against the asset.

#### Emitted Attributes
- action: This will always have the value of "set_security_multiple".

#### Emitted Events
- set_security:
  - num_assets: The number of assets linked.
  - security: The type of security linked to the assets.

#### Request Sample
```
{
    "set_security_multiple": {
        "assets": ["tp1ek77wghn0n9lc7x2uycgh697sjc7fvy995keun"],
        "security": {
            "category": "category_name",
            "name": "instrument_name",
        }
    }
}
```

#### [Remove Security]
This transaction will remove any linked security against an asset.

##### Note
This message variant will fail if the sender is not the current owner of the contract, or if the asset is not linked against a security.

#### Request Parameters
- asset_addr: The address of the asset to be removed and unlinked.

#### Emitted Attributes
- action: This will always have the value of "remove_security".

#### Emitted Events
- remove_security:
  - asset_address: The address of the asset that was removed.
  - security: The security that was linked against the asset.

#### Request Sample
```
{
    "remove_security": {
        "asset_addr": "tp1ek77wghn0n9lc7x2uycgh697sjc7fvy995keun",
    }
}
```

### [Migrate]
The migrate message is implemented to allow the contract to be migrated in the future.

#### [Default]
A default instantiation message that contains default migration functionality.

#### Request Parameters
- None

#### Emitted Attributes
- action: This will always have a value of "migrate".

#### Emitted Events
- None

#### Request Sample
```
{
    "default": {}
}
```

### [Query]
The query message variants allow the users to quickly obtain contract information.

### [Query Version]
The QueryVersion message will return the contract's current version.

#### Request Parameters
- None

#### Request Sample
```
{
    "query_version": {}
}
```

#### Response Sample
```
{
    "data": {
        "contract_version": {
            "contract": "template",
            "version": "1.0.0"
        }
    }
}
```

### [Query Owner]
The QueryVersion message will return the contract's current owner.

#### Request Parameters
- None

#### Request Sample
```
{
    "query_owner": {}
}
```

#### Response Sample
```
{
    "data": {
        "owner": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr"
    }
}
```

### [Query Security Types]
The QuerySecurityTypes message will return the contract's supported security types.

#### Request Parameters
- paginate: A pagination object to optionally limit the response.
  - limit: An optional uint64 that allows the user to limit the number of returned types.
  - start_after: An optional index that can be used to start obtaining security types after.

#### Request Sample
```
{
    "query_security_types": {
        "paginate": {
            "limit": "5",
            "start_after": {
                "category": "category_name",
                "name": "optional_name",
            }
        }
    }
}
```

#### Response Sample
```
{
    "data": {
        "securities": [
            {
                "category": "category_1",
                "name": "name"
            },
            {
                "category": "category_2",
                "name": "null"
            },
            {
                "category": "category_2",
                "name": "name_2"
            },
        ]
    }
}
```

### [Query Address]
The QueryAddress message will return the security type linked to an address.

#### Request Parameters
- asset_addr: The address to lookup the linked security of.

#### Request Sample
```
{
    "query_address": {
        "asset_addr": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr",
    }
}
```

#### Response Sample
```
{
    "data": {
        "security": {
            "category": "category_name",
            "name": "name",
        }
    }
}
```

### [Query Security Category]
The QuerySecurityCategory message will return all assets that have the provided category.

#### Request Parameters
- category: The category to search for.
- paginate: A pagination object to optionally limit the response.
  - limit: An optional uint64 that allows the user to limit the number of returned types.
  - start_after: An optional index that can be used to start obtaining assets after.

#### Request Sample
```
{
    "query_security_category": {
        "category": "category_name",
        "paginate": {
            "limit": "5",
            "start_after": {
                "name": "instrument_name",
                "asset": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr",
            }
        }
    }
}
```

#### Response Sample
```
{
    "data": {
        "assets": [
            {
                "asset": "scope1qptg2wnwv3yyk2u37fp4ztnfkafqrtem4y",
                "name": "name"
            },
            {
                "asset": "tp1pr93cqdh4kfnmrknhwa87a5qrwxw9k3dhkszp0",
                "name": "null"
            },
            {
                "asset": "tp1ts6zfw70xpntec49wsxsy9vwhvsda0vzk600qv",
                "name": "name_2"
            },
        ]
    }
}
```

### [Query Security]
The QuerySecurity message will return all assets that match the provided security exactly.

#### Request Parameters
- security: The security to search for.
- paginate: A pagination object to optionally limit the response.
  - limit: An optional uint64 that allows the user to limit the number of returned types.
  - start_after: An optional index that can be used to start obtaining assets after.

#### Request Sample
```
{
    "query_security": {
        "security": "category_name",
        "paginate": {
            "limit": "5",
            "start_after": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr",
        }
    }
}
```

#### Response Sample
```
{
    "data": {
        "assets": [
            "scope1qptg2wnwv3yyk2u37fp4ztnfkafqrtem4y",
            "tp1pr93cqdh4kfnmrknhwa87a5qrwxw9k3dhkszp0",
            "tp1ts6zfw70xpntec49wsxsy9vwhvsda0vzk600qv",
        ]
    }
}
```