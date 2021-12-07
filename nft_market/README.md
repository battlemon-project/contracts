# Marketplace Contract

## Overview

[`init`](#init)

[`list_asks`](#list_asks)

[`list_trade_history_by_token_id`](#list_trade_history_by_token_id)

[`buy`](#buy)

[`bid`](#bid)

[`list_bids`](#bid)

## Marketplace Methods

### `init`

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

```
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

### `bid`

> It's a payable method that bids for existing token. An attached deposit will be used as the bid's price. If the token's owner already sells the token and the attached deposit equals or more than the token's price, it will be automatically processed. The difference between the attached deposit and the token's price will be refunded to the caller.

**Arguments:**

- `token_id` - id of NFT token

**Example:**

```bash
near call $CONTRACT_NAME bid '{"token_id": "1"}' --depositYocto 10 --gas 200000000000000 --accountId $NEW_OWNER_ID
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
null
```

</p>
</details>

---

### `list_bids`

> It's a view method that list bids for tokens.

**Arguments:**

- `token_id` - id of NFT token

**Example:**

```bash
near view $CONTRACT_NAME list_bids ''
```

<details>
<summary> <strong>Example Response</strong> </summary>
<p>

```
[
  [
    '1',
    [
      {
        token_id: '1',
        bidder_id: 'dev-1638882173121-36557120766090',
        price: '510000000000000000000'
      },
      {
        token_id: '1',
        bidder_id: 'bob.dev-1638882173121-36557120766090',
        price: '520000000000000000000'
      }
    ]
  ],
  [
    '2',
    [
      {
        token_id: '2',
        bidder_id: 'bob.dev-1638882173121-36557120766090',
        price: '10000000000000000000'
      }
    ]
  ]
]

```

</p>
</details>

---
