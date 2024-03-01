call create_quiz:

near contract call-function as-transaction quizzigame.testnet create_quiz json-args '{"game_id":"quiz1","name": "BOS bootcamp 2023 Test1","end_time":1661990400000000000, "reward_distribution":[5,3,2],"number_of_winners": 3,"ipfs_hash":"hashhere"}' prepaid-gas '100.0 Tgas' attached-deposit '10 NEAR' sign-as quizzigamebos.testnet network-config testnet sign-with-keychain send

call add_questions:

near contract call-function as-transaction quizzigame.testnet add_questions json-args "{\"game_id\":\"quiz1\",\"questions\":[[\"What is BOS?\",[\"Blockchain Openrating System\",\"Blockchain Opensource system\"],\"Blockchain Operating System\", 5],[\"What language is near smartcontract?\",[\"Rust or JS\",\"Solidity or JS\"],\"Rust or JS\", 5]]}"
