### Server setup

```sh
source $HOME/.cargo/env
```

### Build contract

```
cargo test -- --nocapture
```

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

### Example NFT 1

```
near call $ID new '{"owner_id": "'$ID'", "name": "Welcome", "symbol": "WLM"}' --accountId $ID
```

```
near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "title": "Simple NFT Title", "description": "Simple NFT Description", "media": "https://bafybeifgjhhyjoz3gco6n3vi2u26dr72na47ychoat53536irekwnzm2gq.ipfs.nftstorage.link/", "hash": "ab9zDvrmxT/QmYej/dlGth/XNZ+8FSuNv4UDYrVmU1o=" }'  --accountId $ID --deposit 0.1
```


### Example NFT 2

```
near call $ID new '{"owner_id": "'$ID'", "name": "Flower", "symbol": "FLW"}' --accountId $ID
```

```
near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "title": "Simple NFT Title", "description": "Simple NFT Description", "media": "https://bafkreibgkz7xlvik7sdbu6jow2qmdvdyle5k3q3tjagjt2muyalae4wc7a.ipfs.nftstorage.link/", "hash": "JlZ/ddUK/IYaeS62oMHUeFk6rcNzSAyZ6ZTAFgJywvg=" }'  --accountId $ID --deposit 0.1
```