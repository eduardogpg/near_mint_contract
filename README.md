### Server setup

```sh
source $HOME/.cargo/env
```

### Build contract

```
cargo build --target wasm32-unknown-unknown --release
```

### Deploy contract

```
near login
```

Set environment $ID

```
export ID=<username>.testnet
```

```
near deploy --wasmFile target/wasm32-unknown-unknown/release/custome_nft.wasm --accountId $ID
```

Initiaize the contract.

```
near call $ID new_default_meta '{"owner_id": "'$ID'", "name": "<Name>", "symbol": "<Symbol>"}' --accountId $ID
```

Mint the custome NFT!

```
near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "token_metadata": '"`cat token.json`}" --accountId $ID --deposit 0.1
```