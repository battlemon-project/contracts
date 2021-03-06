# Battlemon NFT Token Contract

## Battlemon NFT Token Properties

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

## Overview

_Click on a method for more information and examples._

### NFT Token Methods:

[`init`](#nft_init)

[`mint`](#mint)

[`nft_approve`](#nft_approve)

[`nft_token`](#nft_token)

[`nft_tokens`](#nft_tokens)

[`nft_total_supply`](#nft_total_supply)

[`nft_supply_for_owner`](#nft_supply_for_owner)

[`assemble_compound_nft`](#assemble_compound_nft)

[`compound_nft_token`](#compound_nft_token)

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
  to [NEP-177](https://nomicon.io/Standards/NonFungibleToken/Metadata.html) standard. Additionally, **it must contain
  mandatory data with [token properties](#Battlemon NFT Token Properties)**.
- `owner_id` (Optional): An owner of a minted NFT token must be a valid NEAR account. If the argument is omitted, the
  owner of the token will be the contract owner.

**Example:**

```bash
near call $CONTRACT_NAME  mint '{"token_id": "1", "token_metadata": {"title": "Title for token 1", "description": "some description for battlemon nft token", "media": "http://some-link-to-media.com", "model": {"lemon": {"option": "on_sale", "century": "our_time", "type": "light", "lemon_gen": "nakamoto", "background": "red", "top": "headdress", "cyber_suit": "metallic", "expression": "brooding", "eyes": "open", "hair": "bob_marley", "accessory": "cigar", "winrate": 14, "rarity": 12, "slots": []}}}}' --accountId $CONTRACT_NAME --amount 0.1
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
{
  token_id: '1',
  owner_id: 'dev-1640155522267-14107320700951',
  metadata: {
    title: 'Title for token 1',
    description: 'some description for battlemon nft token',
    media: 'http://some-link-to-media.com',
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
  model: {
    lemon: {
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
      rarity: 12,
      parent: null,
      slots: []
    }
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

Must be provided in stringified json.

* `"msg": "{\"sale_type\":\"selling\",\"price\":\"2\"}"` - Token's owner wants to sell his token. Price must be measured in yoctoNEAR.
* `"msg": "{\"sale_type\":\"accept_bid\"}"` - Token's owner wants to accept bid (send the token to bidder and get near for that).

**Example:**

```bash
 near call $NFT_CONTRACT_NAME nft_approve '{"token_id": "1", "account_id": "'$MARKET_CONTRACT_NAME'", "msg": "{\"sale_type\":\"selling\",\"price\":\"10\"}"}' --accountId $OWNER_NAME --depositYocto 440000000000000000000 --gas 30000000000000
 near call $NFT_CONTRACT_NAME nft_approve '{"token_id": "1", "account_id": "'$MARKET_CONTRACT_NAME'", "msg":  "{\"sale_type\":\"accept_bid\"}"}' --accountId $OWNER_NAME --depositYocto 440000000000000000000 --gas 30000000000000
```

- `$NFT_CONTRACT_NAME` - the account id of Battlemon NFT contract.
- `$MARKET_CONTRACT_NAME` - the account id of Battlemon Marketplace contract.
- `$OWNER_NAME` - the account id of NFT token's owner.

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
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
  token_id: '1',
  owner_id: 'dev-1640155522267-14107320700951',
  metadata: {
    title: 'Title for token 1',
    description: 'some description for battlemon nft token',
    media: 'http://some-link-to-media.com',
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
  model: {
    lemon: {
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
      rarity: 12,
      parent: null,
      slots: []
    }
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
    owner_id: 'dev-1640155937836-36922758196791',
    metadata: {
      title: 'Title for token 1',
      description: 'some description for battlemon nft token',
      media: 'http://some-link-to-media.com',
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
    model: {
      lemon: {
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
        rarity: 12,
        parent: null,
        slots: []
      }
    },
    approved_account_ids: {}
  },
  {
    token_id: '2',
    owner_id: 'dev-1640155937836-36922758196791',
    metadata: {
      title: 'Title for token 2',
      description: 'some description for battlemon nft token',
      media: 'http://some-link-to-media.com',
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
    model: {
      weapon: { level: 1, type: 'projection', parent: null, slots: [] }
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
    token_id: '1',
    owner_id: 'dev-1640155937836-36922758196791',
    metadata: {
      title: 'Title for token 1',
      description: 'some description for battlemon nft token',
      media: 'http://some-link-to-media.com',
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
    model: {
      lemon: {
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
        rarity: 12,
        parent: null,
        slots: []
      }
    },
    approved_account_ids: {}
  },
  {
    token_id: '2',
    owner_id: 'dev-1640155937836-36922758196791',
    metadata: {
      title: 'Title for token 2',
      description: 'some description for battlemon nft token',
      media: 'http://some-link-to-media.com',
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
    model: {
      weapon: { level: 1, type: 'projection', parent: null, slots: [] }
    },
    approved_account_ids: {}
  }
]

```

</p>
</details>

---

### `assemble_compound_nft`

> It's a payable method that assemble compatible tokens. The method checks tokens for compatibility, owners etc.

**Arguments:**

- `instructions`: list of token's ids.

**List's format:**
The format is `["1", "2", "2", "3"]` . It means, for token with id `"1"` we want to attach token with id `"2"`, then for token with id `"2"` we want to attach token with id `"3"`. One more example: `["1", "2", "1", "3"]`. It means, for token with id `"1"` we want to attach tokens with ids `"2"` and `"3"`.

**Example:**

```bash
near call dev-1640156382463-30807075638956 assemble_compound_nft '{"instructions": ["1", "2", "2", "3"]}' --accountId dev-1640156382463-30807075638956 --depositYocto 1
```
---

### `compound_nft_token`

> It's a view method that list models of nft token and all nested attachments.  

**Arguments:**

- `token_id`: token's id in string representation.

**Example:**

```bash
near view dev-1640156382463-30807075638956 compound_nft_token '{"token_id": "1"}'
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
[
  [
    '1',
    {
      lemon: {
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
        rarity: 12,
        parent: null,
        slots: [ '2' ]
      }
    }
  ],
  [
    '2',
    {
      weapon: { level: 1, type: 'projection', parent: '1', slots: [ '3' ] }
    }
  ],
  [ '3', { suppressor: { parent: '2', slots: [] } } ]
]
```

</p>
</details>

---
