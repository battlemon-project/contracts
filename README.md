# Battlemon NFT token and marketplace

## NFT Token Methods

### `init`

> Method for initialization smart-contract.

#### Arguments:

- `owner_id` - NEAR account.

**Example**

```bash
near call $CONTRACT_NAME init '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```

---

### `mint`

> It's a payable method that mints Battlemon's NFT token whether for contract owner or arbitrary account.
> The method can be invoked only by the contract owner.
> Attached deposit must cover costs to store token's data on-chain.
> Any extra attached deposit didn't use for storage will be returned.

#### Arguments:

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
near call $CONTRACT_NAME  mint '{"token_id": "1", "token_metadata": {"title": "Title for token 1", "description": "some description for batllemon nft token", "media": "blabla", "properties": {"option": "on_sale", "century": "our_time", "type": "light", "lemon_gen": "nakamoto", "background": "red", "top": "headdress", "cyber_suit": "metallic", "expression": "brooding", "eyes": "open", "hair": "bob_marley", "accessory": "cigar", "winrate": 14, "rarity": 12}}}' --accountId $CONTRACT_NAME --amount 0.1
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

#### Arguments:

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

## Marketplace Methods

### `init`

> Method for initialization smart-contract.

#### Arguments:

- `nft_id` - NEAR account of the NFT token.

**Example**

```bash
near call $CONTRACT_NAME init '{"nft_id": "'$NFT_CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```

---

### `list_asks`

> View method to list all _asks_.

**Example**

```bash
near view list_asks $CONTRACT_NAME '{}'
```

---
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

### `buy`

> It's a payable method that uses for buying particular token.

#### Arguments:

- `token_id` - id of NFT token.

**Example**

```bash
near call $CONTRACT_NAME buy '{"token_id": "2"}' --depositYocto 10 --gas 40000000000000 --accountId $NEW_OWNER_ID
```

- `depositYocto` - price of the token with `token_id`.
- `gas` - attached gas for method execution. The current amount can be changed in the future.

---