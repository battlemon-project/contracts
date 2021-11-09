# Battlemon NFT token and marketplace

## NFT Token Methods

### `mint`
> It's a payable method that mints Battlemon's NFT token whether for contract owner or arbitrary account.
> The method can be invoked only by the contract owner.
> Attached deposit must cover costs to store token's data on-chain.
> Any extra attached deposit didn't use for storage will be returned.  

#### Arguments:
- `token_id`: An unique token id. Note that token IDs for NFTs are strings on NEAR. It's still fine to use auto-incrementing numbers as unique IDs if desired, but they should be stringified.
- `token_metadata`: The fields format must be according to [NEP-177](https://nomicon.io/Standards/NonFungibleToken/Metadata.html) standard. Additionally, it must contain mandatory data with token properties.  
- `owner_id` (Optional): An owner of a minted NFT token must be a valid NEAR account. If the argument is omitted, the owner of the token will be the contract owner.

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
