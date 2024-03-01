> What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter) contract    - Manage smart-contracts: deploy code, call functions
> Choose a contract action: call-function   - Execute function (contract method)
> Сhoose action for account: as-transaction  - Calling a change method
> What is the contract account ID? quizzigame.testnet
> What is the name of the function? add_questions
> How would you like to pass the function arguments? json-args    - Valid JSON arguments (e.g. {"token_id": "42"} or {} if no arguments)
> Enter the arguments to this function: {"game_id":"quiz1","questions":[["What is BOS?",["Blockchain Openrating System", "Blockchain Opensource system"],"Blockchain Operating System", 5},["What language is near smartcontract?",["Rust or JS","Solidity or JS"],"Rust or JS", 5]]}

Here is your console command if you need to script it or re-run:
near contract call-function as-transaction quizzigame.testnet add_questions json-args '{"game_id":"quiz1","questions":[["What is BOS?",["Blockchain Openrating System", "Blockchain Opensource system"],"Blockchain Operating System", 5},["What language is near smartcontract?",["Rust or JS","Solidity or JS"],"Rust or JS", 5]]}'
Error: 
   0: Data not in JSON format!
   1: expected `,` or `]` at line 1 column 146

Location:
   src/commands/contract/call_function/call_function_args_type/mod.rs:90

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
