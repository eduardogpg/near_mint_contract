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

Set environments $ID

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

```
near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "title": "Simple NFT Title", "description": "Simple NFT Description", "media": "https://bafkreibgkz7xlvik7sdbu6jow2qmdvdyle5k3q3tjagjt2muyalae4wc7a.ipfs.nftstorage.link/", "hash": "JlZ/ddUK/IYaeS62oMHUeFk6rcNzSAyZ6ZTAFgJywvg=" }'  --accountId $ID --deposit 0.1
```