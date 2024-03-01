pich@pich-VirtualBox:~/quizzi-rs/quizzi-bos-rs$ near contract call-function as-transaction quizzigame.testnet add_questions json-args '{"game_id":"quiz1","questions":[{"question_text":"What is BOS?","possible_answers":["Blockchain Openrating System","Blockchain Opensource system"
],"correct_answer":"Blockchain Operating System", "score": 5},{"question_text":"What language is near smartcontract?","possible_answers":["R
ust or JS","Solidity or JS"],"correct_answer":"Rust or JS", "score": 5}]}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as quizzi
game.testnet network-config testnet sign-with-keychain send

Unsigned transaction:

signer_id:    quizzigame.testnet
receiver_id:  quizzigame.testnet
actions:
   -- function call:      
                   method name:  add_questions
                   args:         {
                                   "game_id": "quiz1",
                                   "questions": [
                                     {
                                       "correct_answer": "Blockchain Operating System",
                                       "possible_answers": [
                                         "Blockchain Openrating System",
                                         "Blockchain Opensource system"
                                       ],
                                       "question_text": "What is BOS?",
                                       "score": 5
                                     },
                                     {
                                       "correct_answer": "Rust or JS",
                                       "possible_answers": [
                                         "Rust or JS",
                                         "Solidity or JS"
                                       ],
                                       "question_text": "What language is near smartcontract?",
                                       "score": 5
                                     }
                                   ]
                                 }
                   gas:          100.0 Tgas
                   deposit:      0 NEAR


Your transaction was signed successfully.
Public key: ed25519:9BdmRQZY2s7J7AmTVvbfdXqBkyZjpHPJG7HNXF9SbPsb
Signature: ed25519:3B8httVtvBtRiJini1XDgCpvKnwXgPyuCcC62zGCg9yeytidya1cei8GNifF3tf6u9YbiCifGN7bynnAwwr4NdQe
Transaction sent ...
--- Logs ---------------------------
Logs [quizzigame.testnet]:   No logs
Logs [quizzigame.testnet]:   No logs
Failed transaction

Here is your console command if you need to script it or re-run:
near contract call-function as-transaction quizzigame.testnet add_questions json-args '{"game_id":"quiz1","questions":[{"question_text":"What is BOS?","possible_answers":["Blockchain Openrating System","Blockchain Opensource system"],"correct_answer":"Blockchain Operating System", "score": 5},{"question_text":"What language is near smartcontract?","possible_answers":["Rust or JS","Solidity or JS"],"correct_answer":"Rust or JS", "score": 5}]}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as quizzigame.testnet network-config testnet sign-with-keychain send
Error: 
   0: Error: An error occurred during a `FunctionCall` Action, parameter is debug message.
      ExecutionError("Smart contract panicked: panicked at src/lib.rs:72:1:\nFailed to deserialize input from JSON.: Error(\"invalid type: map, expected a tuple of size 4\", line: 1, column: 32)")

Location:
   src/transaction_signature_options/mod.rs:144

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
