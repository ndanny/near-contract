# near-smart-contract

A smart contract on the NEAR protocol that implements a counter. The counter is easy to implement but the stored on the near blockchain.

### Test & compile

`cargo test -- --nocapture`

`cargo build --target wasm32-unknown-unknown --release`

### Deploying

`near login`
`near deploy --wasmFile target/wasm32-unknown-unknown/release/counter.wasm --accountId <acc.testnet>`

### Commands

#### Increment Counter

`near call <acc.testnet> increment --accountId <acc.testnet>`

#### Decrement Counter

`near call <acc.testnet> decrement --accountId <acc.testnet>`

#### Reset Counter

`near call <acc.testnet> reset --accountId <acc.testnet>`

#### Get Count

`near view <acc.testnet> get_count --accountId <acc.testnet>`

#### Output

If the transaction was successful, the output should look something like the one below. The transaction fee was 0.00055 NEAR.

```
Scheduling a call: <acc.testnet>.increment()
Doing account.functionCall()
Receipt: 8UtEHSo6UkQmopK4HYUpnB4hoCekHBASMUxu6iEyYBKE
	Log [pw.testnet]: Increased counter to 1
Transaction Id Z4zpez2CoaiUk6TZ5HQEZsu64MFxxWmyU9xe86Ld3am
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Z4zpez2CoaiUk6TZ5HQEZsu64MFxxWmyU9xe86Ld3am
```
