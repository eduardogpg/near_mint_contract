### Server setup

```sh
source $HOME/.cargo/env
```

### Build contract

```
cargo build --target wasm32-unknown-unknown --release
```

### Deploy contract

IMAGE URL = https://bafybeifgjhhyjoz3gco6n3vi2u26dr72na47ychoat53536irekwnzm2gq.ipfs.nftstorage.link/
IMAGE HASH = ab9zDvrmxT/QmYej/dlGth/XNZ+8FSuNv4UDYrVmU1o=

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
near call $ID new '{"owner_id": "'$ID'", "name": "<Name>", "symbol": "<Symbol>"}' --accountId $ID
```

```
near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "title": "<Title>", "description": "<Description>", "media": "<Media>", "hash": "<Media Hash>", "copies": <N. Copies> }'  --accountId $ID --deposit 0.1
```

near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "title": "Flower", "description": "NFT Description", "media": "https://bafybeifgjhhyjoz3gco6n3vi2u26dr72na47ychoat53536irekwnzm2gq.ipfs.nftstorage.link/", "hash": "ab9zDvrmxT/QmYej/dlGth/XNZ+8FSuNv4UDYrVmU1o=" }'  --accountId $ID --deposit 0.1