# Battlemon NFT token and marketplace

## Overview

_Click on a method for more information and examples._

**NFT Token Methods:**

[`init`](#nft_init)

[`mint`](#mint)

[`nft_approve`](#nft_approve)

[`nft_token`](#nft_token)

[`nft_tokens`](#nft_tokens)

[`nft_total_supply`](#nft_total_supply)

[`nft_supply_for_owner`](#nft_supply_for_owner)

[`nft_tokens_for_owner`](#nft_tokens_for_owner)

**Marketplace Methods:**

[`init`](#marketplace_init)

[`list_asks`](#list_asks)

[`list_trade_history_by_token_id`](#list_trade_history_by_token_id)

[`buy`](#buy)

## NFT Token Methods

### <a name="nft_init"></a>`init`

> Method for initialization smart-contract.

**Arguments:**

- `owner_id` - NEAR account.

**Example:**

```bash
near call $CONTRACT_NAME init '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```

---

### `mint`

> It's a payable method that mints Battlemon's NFT token whether for contract owner or arbitrary account.
> The method can be invoked only by the contract owner.
> Attached deposit must cover costs to store token's data on-chain.
> Any extra attached deposit didn't use for storage will be returned.

**Arguments:**

- `token_id`: An unique token id. Note that token IDs for NFTs are strings on NEAR. It's still fine to use
  auto-incrementing numbers as unique IDs if desired, but they should be stringified.
- `token_metadata`: The fields format must be according
  to [NEP-177](https://nomicon.io/Standards/NonFungibleToken/Metadata.html) standard. Additionally, it must contain
  mandatory data with token properties.
- `owner_id` (Optional): An owner of a minted NFT token must be a valid NEAR account. If the argument is omitted, the
  owner of the token will be the contract owner.

#### Battlemon NFT Token Properties:

- `"option"`: `["on_sale", "auction", "for_rent", "lemon_gen"]`
- `"century"`: `["ancient", "ourtime", "future", "otherworldly"]`
- `"type"`: `["light", "medium", "heavy"]`
- `"lemon_gen"`: `["nakamoto", "buterin", "mask", "jobs"]`
- `"background"`: `["red", "purple", "black", "yellow", "green"]`
- `"top"`: `["headdress", "hairstyle", "classical"]`
- `"cyber_suit"`: `["black", "metallic", "blue", "gold"]`
- `"expression"`: `["brooding", "merry", "angry", "tense", "relaxed", "mask"]`
- `"eyes"`: `["open", "close", "medium"]`
- `"hair"`: `["elvis", "bob_marley", "pubkkez", "disco"]`
- `"accessory"`: `["cigar", "toothpick", "tattoo", "scar"]`
- `"winrate"`(Optional): `0 - 100`
- `"rarity"`: `0 - 100`

**Example:**

```bash
near call $CONTRACT_NAME  mint '{"token_id": "1", "token_metadata": {"title": "Title for token 1", "description": "some description for battlemon nft token", "media": "blabla", "properties": {"option": "on_sale", "century": "our_time", "type": "light", "lemon_gen": "nakamoto", "background": "red", "top": "headdress", "cyber_suit": "metallic", "expression": "brooding", "eyes": "open", "hair": "bob_marley", "accessory": "cigar", "winrate": 14, "rarity": 12}}}' --accountId $CONTRACT_NAME --amount 0.1
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
{
  token_id: '2',
  owner_id: 'dev-1636434873070-41725363371137',
  metadata: {
    title: 'Title for token 1',
    description: 'some description for batllemon nft token',
    media: 'blabla',
    media_hash: null,
    copies: null,
    issued_at: null,
    expires_at: null,
    starts_at: null,
    updated_at: null,
    extra: null,
    reference: null,
    reference_hash: null
  },
  properties: {
    option: 'on_sale',
    century: 'our_time',
    type: 'light',
    lemon_gen: 'nakamoto',
    background: 'red',
    top: 'headdress',
    cyber_suit: 'metallic',
    expression: 'brooding',
    eyes: 'open',
    hair: 'bob_marley',
    accessory: 'cigar',
    winrate: 14,
    rarity: 12
  },
  approved_account_ids: {}
}
```

</p>
</details>

---

### `nft_approve`

> According to [NEP-178](https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html)
> It's a payable method that gives for particular account id approvals to manage the token.
> In our cases we must provide `account_id` of our market's contract, with proper `msg`.
> Attached deposit must be at least 1 yoctoNEAR and cover costs to store request's data on-chain.
> Gas must be at least than 30000000000000 yoctoNEAR.
> Any extra attached deposit and gas didn't use for storage will be returned.

**Arguments:**

- `token_id`: the token id for which to add an approval
- `account_id`: the account that will be approved for managing the NFT token.
- `msg` (Optional): the message that will be deserialized by callback call and processed in the proper way that depends
  on provided data.

#### `msg` format:

`"msg": "{\"price\":\"2\"}` - stringified json, price must be measured in yoctoNEAR.

**Example:**

```bash
 near call $NFT_CONTRACT_NAME nft_approve '{"token_id": "1", "account_id": "'$MARKET_CONTRACT_NAME'", "msg": "{\"price\":\"2\"}"}' --accountId $OWNER_NAME --depositYocto 440000000000000000000 --gas 30000000000000
```

- `$NFT_CONTRACT_NAME` - the account id of Battlemon NFT contract.
- `$MARKET_CONTRACT_NAME` - the account id of Battlemon Marketplace contract.
- `$OWNER_NAME` - the account id of NFT token's owner.

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```json
{
  "status": true,
  "message": "token 1 with price 2 was added to market"
}
```

</p>
</details>

---

### `nft_token`

> It's a view method that return the token with the given `token_id` or `null` if no such token.

**Arguments:**

- `token_id`: token's id in string representation.

**Example:**

```bash
near view $CONTRACT_NAME nft_token '{"token_id": "2"}'
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
{
  token_id: '2',
  owner_id: 'dev-1636641321126-54010839869553',
  metadata: {
    title: 'Title for token 2',
    description: 'some description for battlemon nft token',
    media: 'blabla',
    media_hash: null,
    copies: null,
    issued_at: null,
    expires_at: null,
    starts_at: null,
    updated_at: null,
    extra: null,
    reference: null,
    reference_hash: null
  },
  properties: {
    option: 'on_sale',
    century: 'our_time',
    type: 'light',
    lemon_gen: 'nakamoto',
    background: 'red',
    top: 'headdress',
    cyber_suit: 'metallic',
    expression: 'brooding',
    eyes: 'open',
    hair: 'bob_marley',
    accessory: 'cigar',
    winrate: 14,
    rarity: 12
  },
  approved_account_ids: {}
}

```

</p>
</details>

---

### `nft_tokens`

> It's a view method that return collection of minted tokens and an empty collection if there are no tokens.

**Arguments:**

- `from_index` (Optional): a string representing an unsigned 128-bit integer, representing the starting index of tokens
  to return. If it's omitted it will return collection with starting index equals zero.
- `limit` (Optional): the maximum number of tokens to return. If it's omitted it will return unlimited collection.

**Example:**

```bash
near view $CONTRACT_NAME nft_tokens ''
```

```bash
near view $CONTRACT_NAME nft_tokens '{"from_index": "1"}'
```

```bash
near view $CONTRACT_NAME nft_tokens '{"from_index": "2", limit: 3}'
```

```bash
near view $CONTRACT_NAME nft_tokens '{"limit": 5}'
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
[
  {
    token_id: '1',
    owner_id: 'dev-1636550205831-72164784084299',
    metadata: {
      title: 'Title for token 1',
      description: 'some description for battlemon nft token',
      media: 'blabla',
      media_hash: null,
      copies: null,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    },
    properties: {
      option: 'on_sale',
      century: 'our_time',
      type: 'light',
      lemon_gen: 'nakamoto',
      background: 'red',
      top: 'headdress',
      cyber_suit: 'metallic',
      expression: 'brooding',
      eyes: 'open',
      hair: 'bob_marley',
      accessory: 'cigar',
      winrate: 14,
      rarity: 12
    },
    approved_account_ids: {}
  },
  {
    token_id: '10',
    owner_id: 'dev-1636550205831-72164784084299',
    metadata: {
      title: 'Title for token 10',
      description: 'some description for battlemon nft token',
      media: 'blabla',
      media_hash: null,
      copies: null,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    },
    properties: {
      option: 'on_sale',
      century: 'our_time',
      type: 'light',
      lemon_gen: 'nakamoto',
      background: 'red',
      top: 'headdress',
      cyber_suit: 'metallic',
      expression: 'brooding',
      eyes: 'open',
      hair: 'bob_marley',
      accessory: 'cigar',
      winrate: 14,
      rarity: 12
    },
    approved_account_ids: {}
  },
  {
    token_id: '2',
    owner_id: 'dev-1636550205831-72164784084299',
    metadata: {
      title: 'Title for token 2',
      description: 'some description for battlemon nft token',
      media: 'blabla',
      media_hash: null,
      copies: null,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    },
    properties: {
      option: 'on_sale',
      century: 'our_time',
      type: 'light',
      lemon_gen: 'nakamoto',
      background: 'red',
      top: 'headdress',
      cyber_suit: 'metallic',
      expression: 'brooding',
      eyes: 'open',
      hair: 'bob_marley',
      accessory: 'cigar',
      winrate: 14,
      rarity: 12
    },
    approved_account_ids: {}
  },
  {
    token_id: '3',
    owner_id: 'dev-1636550205831-72164784084299',
    metadata: {
      title: 'Title for token 3',
      description: 'some description for battlemon nft token',
      media: 'blabla',
      media_hash: null,
      copies: null,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    },
    properties: {
      option: 'on_sale',
      century: 'our_time',
      type: 'light',
      lemon_gen: 'nakamoto',
      background: 'red',
      top: 'headdress',
      cyber_suit: 'metallic',
      expression: 'brooding',
      eyes: 'open',
      hair: 'bob_marley',
      accessory: 'cigar',
      winrate: 14,
      rarity: 12
    },
    approved_account_ids: {}
  }
]
```

</p>
</details>

---

### `nft_total_supply`

> It's a view method. Returns the total supply of non-fungible tokens and `0` if there are no tokens.

**Example:**

```bash
near view $CONTRACT_NAME nft_total_supply ''
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
'10'
```

</p>
</details>

---

### `nft_supply_for_owner`

> It's a view method. Returns the number of non-fungible tokens owned by given `account_id`.

#### Arguments

- `account_id` - a valid NEAR account

**Example:**

```bash
near view $CONTRACT_NAME nft_supply_for_owner '{"account_id": "'$OWNER_NAME'"}'
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
'7'
```

</p>
</details>

---

### `nft_tokens_for_owner`

> It's a view method. Returns the collection of non-fungible tokens owned by given `account_id`.

#### Arguments

- `account_id` - a valid NEAR account
- `from_index` (Optional): a string representing an unsigned 128-bit integer, representing the starting index of tokens
  to return. If it's omitted it will return collection with starting index equals zero.
- `limit` (Optional): the maximum number of tokens to return. If it's omitted it will return unlimited collection.

**Example:**

```bash
near view $CONTRACT_NAME nft_tokens_for_owner '{"account_id": "'$OWNER_NAME'"}'
```

```bash
near view $CONTRACT_NAME nft_tokens_for_owner '{"account_id": "'$OWNER_NAME'", "from_index": "2"}'
```

```bash
near view $CONTRACT_NAME nft_tokens_for_owner '{"account_id": "'$OWNER_NAME'", "limit": 5}'
```

```bash
near view $CONTRACT_NAME nft_tokens_for_owner '{"account_id": "'$OWNER_NAME'", "from_index": "2", "limit": 5}'
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
[
  {
    token_id: '3',
    owner_id: 'dev-1636550205831-72164784084299',
    metadata: {
      title: 'Title for token 3',
      description: 'some description for battlemon nft token',
      media: 'blabla',
      media_hash: null,
      copies: null,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    },
    properties: {
      option: 'on_sale',
      century: 'our_time',
      type: 'light',
      lemon_gen: 'nakamoto',
      background: 'red',
      top: 'headdress',
      cyber_suit: 'metallic',
      expression: 'brooding',
      eyes: 'open',
      hair: 'bob_marley',
      accessory: 'cigar',
      winrate: 14,
      rarity: 12
    },
    approved_account_ids: {}
  },
  {
    token_id: '4',
    owner_id: 'dev-1636550205831-72164784084299',
    metadata: {
      title: 'Title for token 4',
      description: 'some description for battlemon nft token',
      media: 'blabla',
      media_hash: null,
      copies: null,
      issued_at: null,
      expires_at: null,
      starts_at: null,
      updated_at: null,
      extra: null,
      reference: null,
      reference_hash: null
    },
    properties: {
      option: 'on_sale',
      century: 'our_time',
      type: 'light',
      lemon_gen: 'nakamoto',
      background: 'red',
      top: 'headdress',
      cyber_suit: 'metallic',
      expression: 'brooding',
      eyes: 'open',
      hair: 'bob_marley',
      accessory: 'cigar',
      winrate: 14,
      rarity: 12
    },
    approved_account_ids: {}
  }
]
```

</p>
</details>

---

## Marketplace Methods

### <a name="marketplace_init"></a> `init`

> Method for initialization smart-contract.

**Arguments:**

- `nft_id` - NEAR account of the NFT token.

**Example:**

```bash
near call $CONTRACT_NAME init '{"nft_id": "'$NFT_CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```

---

### `list_asks`

> View method to list all _asks_.

**Example:**

```bash
near view $CONTRACT_NAME list_asks '{}'
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```json lines
[
  {
    owner_id: 'nft.dev-1636529128471-59911444209733',
    token_id: '1',
    approval_id: 3,
    price: '2'
  },
  {
    owner_id: 'nft.dev-1636529128471-59911444209733',
    token_id: '2',
    approval_id: 1,
    price: '10'
  }
]
```

</p>
</details>

---

### `list_trade_history_by_token_id`

> It's a view method that list trade history for particular token.

**Arguments:**

- `token_id` - id of NFT token

**Example:**

```bash
near view $CONTRACT_NAME list_trade_history_by_token_id '{"token_id": "1"}'
```
<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
[
  {
    prev_owner: 'dev-1637162308296-49398331322990',
    curr_owner: 'alice.dev-1636529128471-59911444209733',
    price: '10',
    date: 1637162365804594000,
    type: 'sell'
  }
]
```

</p>
</details>

---

### `buy`

> It's a payable method that uses for buying particular token.

**Arguments:**

- `token_id` - id of NFT token.

**Example:**

```bash
near call $CONTRACT_NAME buy '{"token_id": "2"}' --depositYocto 10 --gas 40000000000000 --accountId $NEW_OWNER_ID
```

- `depositYocto` - price of the token with `token_id`.
- `gas` - attached gas for method execution. The current amount can be changed in the future.

---