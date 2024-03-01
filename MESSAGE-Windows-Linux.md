pich@pich-VirtualBox:~/quizzi-rs/quizzi-bos-rs$ cargo build --target wasm32-unknown-unknown --release
   Compiling quizzi-bos-rs v0.1.0 (/home/pich/quizzi-rs/quizzi-bos-rs)
error[E0277]: a value of type `near_sdk::collections::Vector<u128>` cannot be built from an iterator over elements of type `u128`
    --> src/lib.rs:61:111
     |
61   |         let rewards_in_yocto: Vector<Balance> = rewards.into_iter().map(|near_amount| near_amount * ONE_NEAR).collect();
     |                                                                                                               ^^^^^^^ value of type `near_sdk::collections::Vector<u128>` cannot be built from `std::iter::Iterator<Item=u128>`
     |
     = help: the trait `FromIterator<u128>` is not implemented for `near_sdk::collections::Vector<u128>`
note: the method call chain might not have had the expected associated types
    --> src/lib.rs:61:57
     |
61   |         let rewards_in_yocto: Vector<Balance> = rewards.into_iter().map(|near_amount| near_amount * ONE_NEAR).collect();
     |                                                 ------- ^^^^^^^^^^^ ----------------------------------------- `Iterator::Item` remains `u128` here
     |                                                 |       |
     |                                                 |       `Iterator::Item` is `u128` here
     |                                                 this expression has type `Vec<u128>`
note: required by a bound in `collect`
    --> /home/pich/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2049:19
     |
2049 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::collect`

error[E0308]: mismatched types
    --> src/lib.rs:125:35
     |
125  |         rankings.into_iter().take(self.quizzes.get(&game_id).expect("Quiz not found").rewards.len()).collect()
     |                              ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found `u64`
     |                              |
     |                              arguments to this method are incorrect
     |
note: method defined here
    --> /home/pich/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1411:8
     |
1411 |     fn take(self, n: usize) -> Take<Self>
     |        ^^^^
help: you can convert a `u64` to a `usize` and panic if the converted value doesn't fit
     |
125  |         rankings.into_iter().take(self.quizzes.get(&game_id).expect("Quiz not found").rewards.len().try_into().unwrap()).collect()
     |                                                                                                    ++++++++++++++++++++

error[E0614]: type `u128` cannot be dereferenced
   --> src/lib.rs:135:51
    |
135 |                 Promise::new(account_id).transfer(*reward_amount);
    |                                                   ^^^^^^^^^^^^^^

error[E0277]: the trait bound `Quiz: Serialize` is not satisfied
    --> src/lib.rs:46:1
     |
46   | #[near_bindgen]
     | ^^^^^^^^^^^^^^^ the trait `Serialize` is not implemented for `Quiz`
     |
     = help: the following other types implement trait `Serialize`:
               bool
               char
               isize
               i8
               i16
               i32
               i64
               i128
             and 173 others
     = note: required for `std::vec::Vec<Quiz>` to implement `Serialize`
note: required by a bound in `near_sdk::serde_json::to_vec`
    --> /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.114/src/ser.rs:2177:17
     |
2175 | pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
     |        ------ required by a bound in this function
2176 | where
2177 |     T: ?Sized + Serialize,
     |                 ^^^^^^^^^ required by this bound in `to_vec`
     = note: this error originates in the attribute macro `near_bindgen` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0277, E0308, E0614.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `quizzi-bos-rs` (lib) due to 4 previous errors
