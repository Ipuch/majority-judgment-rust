# Majority Judgment Library for Rust

[![MIT](https://img.shields.io/github/license/MieuxVoter/majority-judgment-library-python?style=for-the-badge)](./LICENSE)
[![Join the Discord chat at https://discord.gg/rAAQG9S](https://img.shields.io/discord/705322981102190593.svg?style=for-the-badge)](https://discord.gg/rAAQG9S)

Rust library to help deliberate using [Majority Judgment](https://mieuxvoter.fr/).

The goal is to be **scalable**, **reliable**, fast and extensible.
We therefore use a _score-based algorithm_ whatsoever.

## Example Usage

Collect the **votes** for each Candidate and provide them in the function `majority_judgment` as is:

```rust
// Declare a BTreeMap with the poll data
let mut poll_data : BTreeMap<String, Vec<i32> > = BTreeMap::new();

poll_data.insert("Pizza".to_string(), vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
poll_data.insert("Chips".to_string(), vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
poll_data.insert("Pasta".to_string(), vec![0, 1, 0, 1, 2, 1, 3, 2, 3, 3]);
poll_data.insert("Bread".to_string(), vec![0, 1, 2, 1, 1, 2, 1, 2, 2, 3]);

println!("Data: {:?}", poll_data);
println!("Results as a vector of tuple (Candidate, Rank): {:?}",majority_judgment(&poll_data));
```
This results in the following output:
 
```
Data: {"Bread": [0, 1, 2, 1, 1, 2, 1, 2, 2, 3], "Chips": [0, 1, 0, 2, 1, 2, 2, 3, 2, 3], "Pasta": [0, 1, 0, 1, 2, 1, 3, 2, 3, 3], "Pizza": [0, 0, 3, 0, 2, 0, 3, 1, 2, 3]}
Results as a vector of tuple (Candidate, Rank): [("Chips", 0), ("Pasta", 1), ("Bread", 2), ("Pizza", 3)]
```

## License
[MIT](./LICENSE)  →  _Do whatever you want except complain._

Majority Judgment itself is part of the Commons, obviously.


## Fund us

We'd love to invest more energy in Majority Judgment development.

Please consider funding us, every bit helps : https://www.paypal.com/donate/?hosted_button_id=QD6U4D323WV4S


