//! # Majority Judgment
//! This is a simple implementation of the Majority Judgment voting system.
//! The Majority Judgment is a voting system that was proposed by Michel Balinski and Rida Laraki.
//! It is a single-winner voting system that selects the candidate who has the highest median grade.

use std::collections::BTreeMap;
use majority_judgement_rust::majority_judgment;


fn main() {

    // Declare a BTreeMap with the poll data
    let mut poll_data : BTreeMap<String, Vec<u8> > = BTreeMap::new();

    poll_data.insert("Pizza".to_string(), vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
    poll_data.insert("Chips".to_string(), vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
    poll_data.insert("Pasta".to_string(), vec![0, 1, 0, 1, 2, 1, 3, 2, 3, 3]);
    poll_data.insert("Bread".to_string(), vec![0, 1, 2, 1, 1, 2, 1, 2, 2, 3]);

    println!("Data: {:?}", poll_data);
    println!("Results as a vector of tuple (Candidate, Rank): {:?}",majority_judgment(&poll_data));

}
