# SharingShard

Using the following instructions to deploy the contract

https://docs.near.org/docs/develop/contracts/rust/intro#compile-the-code

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown

Create an account for the contract
-- we use: ss2022_jc.testnet

To test the operation we use 
near-cli

First we need to install it:

    npm install -g near-cli

Compile the programa
    cargo build --target wasm32-unknown-unknown --release

Deploy the contract
near deploy --wasmFile target/wasm32-unknown-unknown/release/near_sharingshard.wasm --accountId ss2022_jc.testnet

Invoking the method
near call ss2022_jc.testnet increment --accountId ss2022_jc.testnet
