> What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter) contract    - Manage smart-contracts: deploy code, call functions
> Choose a contract action: call-function   - Execute function (contract method)
> Сhoose action for account: as-transaction  - Calling a change method
> What is the contract account ID? quizzigame.testnet
> What is the name of the function? submit_answers
> How would you like to pass the function arguments? json-args    - Valid JSON arguments (e.g. {"token_id": "42"} or {} if no arguments)
> Enter the arguments to this function: {"game_id":"quiz1","responses":{"0":"Blockchain Openrating System","1":"Rust"}}

> Enter gas for function call: 100 TeraGas

> Enter deposit for a function call (example: 10NEAR or 0.5near or 10000yoctonear): 0 NEAR
> What is the signer account ID? vbiquizzi.testnet
> What is the name of the network? testnet

Unsigned transaction:

signer_id:    vbiquizzi.testnet
receiver_id:  quizzigame.testnet
actions:
   -- function call:      
                   method name:  submit_answers
                   args:         {
                                   "game_id": "quiz1",
                                   "responses": {
                                     "0": "Blockchain Openrating System",
                                     "1": "Rust"
                                   }
                                 }
                   gas:          100.0 Tgas
                   deposit:      0 NEAR

> Select a tool for signing the transaction: sign-with-keychain               - Sign the transaction with a key saved in the secure keychain

Your transaction was signed successfully.
Public key: ed25519:61yG1u4ykZXBajBxNLvnhnRFLhtVD4Nb9zwFRnvNPbXz
Signature: ed25519:LX84bUmZUneu4ENsLkztv2g1MxAMzNaaBu9cD4oMKUuYzDVQNYaqDge3sRKRd8oYWr8uEcTA4Fr9J1pLe39mChL
> How would you like to proceed? send      - Send the transaction to the network
Transaction sent ...
--- Logs ---------------------------
Logs [quizzigame.testnet]:   No logs
Logs [vbiquizzi.testnet]:   No logs
Failed transaction

Here is your console command if you need to script it or re-run:
near contract call-function as-transaction quizzigame.testnet submit_answers json-args '{"game_id":"quiz1","responses":{"0":"Blockchain Openrating System","1":"Rust"}}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as vbiquizzi.testnet network-config testnet sign-with-keychain send
Error: 
   0: Error: An error occurred during a `FunctionCall` Action, parameter is debug message.
      ExecutionError("Smart contract panicked: panicked at src/lib.rs:116:5:\nassertion failed: env::block_timestamp() < quiz.end_time")

Location:
   src/transaction_signature_options/mod.rs:144

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
