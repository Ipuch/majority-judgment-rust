//! # Majority Judgment
//! These are the functions to calculate the majority judgment of a poll.
//! All sub-functions are private and are not exposed to the user.
//! The user only needs to call the majority_judgment function.
use std::collections::BTreeMap;

/// Function that checks that all the lengths of the polls are the same otherwise it throws an error
/// # Arguments
/// * `poll_data`: a BTreeMap with the poll data
///
/// # Returns
/// * `Result<(), &str>`: an empty result or an error message
///
/// # Example (no panic)
/// use std::collections::BTreeMap;
/// let mut poll_data = BTreeMap::new();
/// poll_data.insert("Pizza", vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
/// poll_data.insert("Chips", vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
/// check_poll_length(&poll_data);
///
/// # Example (panic)
/// use std::collections::BTreeMap;
/// let mut poll_data = BTreeMap::new();
/// poll_data.insert("Pizza", vec![0, 2, 3]);
/// poll_data.insert("Chips", vec![0, 3, 2, 3, 4]);
/// check_poll_length(&poll_data);
///
fn check_poll_length(poll_data: &BTreeMap<String, Vec<u8>>) -> Result<(), &str> {
    let first_poll_length = poll_data.values().next().unwrap().len();
    for poll in poll_data.values() {
        if poll.len() != first_poll_length {
            panic!("The polls have different lengths!")
        }
    }
    Ok(())

}

/// Function that calculates the majority judgment of a poll
/// # Arguments
/// * `poll_data`: a BTreeMap<String, Vec<u8>> with the poll data
///
/// # Returns
/// * `Vec<(&String, usize)>`: a vector of tuple with the candidate and its rank
pub fn majority_judgment(poll_data: &BTreeMap<String, Vec<u8>>) -> Vec<(&String, usize)> {

    let _ = check_poll_length(&poll_data);

    let mut majority_values = BTreeMap::new();
    for (item, grades) in poll_data {
        majority_values.insert(item, compute_majority_values(grades.to_vec()));
    }

    let mut majority_values_vec: Vec<(&&String, &Vec<u32>)> = majority_values.iter().collect();
    majority_values_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    let mut final_ranking:Vec<(&String, usize)> = Vec::new();
    for (rank, (item, _)) in majority_values_vec.iter().enumerate() {
        final_ranking.push((item, rank));
    }

    return final_ranking
}

/// This function computes the median grades, when each time withdrawing the median grade.
/// It provides a simple efficient way to rank candidates even if the initial median grade is the same.
/// # Arguments
/// * grades: Vec<u8> all the collected grades unsorted
///
/// # Returns
/// * Vec<u32> The consecutive median grades when withdrawing the previous one
fn compute_majority_values(grades: Vec<u8>) -> Vec<u32> {

    let tally = compute_frequency_of_grades(grades.clone());

    let keys = tally.keys().collect::<Vec<&u8>>();
    let mut values = tally.values().collect::<Vec<&u32>>().iter().map(|&x| *x).collect::<Vec<u32>>();
    let total_votes = grades.len() as u32;

    let mut majority_values : Vec<u32> = Vec::new();

    for _ in 0..total_votes {
        let total: u32 = values.clone().into_iter().sum();
        let total_f32 = total as f32;

        let values_f32: Vec<f32> = values.clone().into_iter().map(|x| x as f32).collect();
        let cumsum: Vec<f32> = values_f32.clone().into_iter().scan(0.0, |sum, val| {
            *sum += val / total_f32;
            Some(*sum)
        }).collect();


        let idx: u32 = median_grade(cumsum);

        // extra safeguard to prevent panic because no key found at the given index.
        if let Some(key) = keys.get(idx as usize) {
            let key_clone = (**key).clone();
            majority_values.push(key_clone.try_into().unwrap());
        } else {
            println!("No key found at index {}", idx);
        }

        // remove median grade of the total count of votes
        // by changing removing a counted vote in the value vector at index idx
        values = values.into_iter().enumerate().map(|(i, x)| {
            if i == idx as usize {
                x - 1
            } else {
                x
            }
        }).collect::<Vec<_>>();
    }
    return majority_values
}

/// Function that compute the frequency of each grade in BTreeMap structure
///
/// # Arguments
/// * `grades`:  Vec<u8> unsorted numbers representing the grades
///
/// # Returns
/// * BTreeMap<u8, u32>, first is the grade, the second is the number of time, it has been given
///
fn compute_frequency_of_grades(mut grades: Vec<u8>) -> BTreeMap<u8, u32> {
    let mut tally: BTreeMap<u8, u32> = BTreeMap::new();

    grades.sort();
    let grades_group = group_by(grades);

    for grades in grades_group.iter() {
        tally.insert( grades[0]
                      , grades.len().try_into().unwrap());
    }
    return tally
}
/// Function that group the sorted vector in to a vector of sub vectors
/// I couldn't replicate the group_by function of python, so I reimplemented an equivalent
///
/// # Arguments
/// * `vector`:  Vec<T> of sorted number
///
/// # Returns
/// * Vec<Vec<T>> vector of vectors, each vector contains the number
///
fn group_by<T: PartialEq + Clone>(vector: Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();

    for item in vector.iter() {
        if let Some(group) = result.last_mut() {
            if group[0] == *item {
                group.push(item.clone());
                continue;
            }
        }

        result.push(vec![item.clone()]);
    }

    result
}

/// Evaluate the median grade from a cumulative sum of grades
/// # Arguments
/// * `cumsum_vec`:  Vec<f32> of cumulative sum of grades
///
/// # Returns
/// * u32, the index of the median grade
///
/// # Note
/// - This is not exactly the median grade, but the index of the median grade
///     if the number of element is even, it will return the index  (n/2 - 1)  and not the value of the median grade
/// - Plus, it is found based on a cumulative sum of grades,
///     so we always try to find the 0.5 value to return the median grade index
fn median_grade(cumsum_vec: Vec<f32>) -> u32 {
    // too strict when sometimes I get a 1.000001
    // verify the last element is a 1
    // if cumsum_vec.last() != Some(&1.0) {
    //     panic!("The last element of the cumulative sum vector is not 1.0. \
    //     Please normalize the vector before calling the function fn median_grade.")
    // }
    // verify all element are positive
    if cumsum_vec.iter().any(|&x| x < 0.0) {
        panic!("The cumulative sum vector contains negative values. \
        Please make sure all values are positive before calling the function fn median_grade.")
    }

    for (idx, &val) in cumsum_vec.iter().enumerate() {
        if val >= 0.5 {
            return idx.try_into().unwrap()
        }
    }
    return cumsum_vec.len() as u32 - 1u32
}


mod tests {
    use super::*;

    #[test]
    fn calling_check_poll_length() {
        let mut poll_data = BTreeMap::new();
        poll_data.insert("Pizza".to_string(), vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
        poll_data.insert("Chips".to_string(), vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
        let _ = check_poll_length(&poll_data);
    }

    #[test]
    fn calling_majority_judgment() {
        let mut poll_data: BTreeMap<String, Vec<u8>> = BTreeMap::new();

        poll_data.insert("Pizza".to_string(), vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
        poll_data.insert("Chips".to_string(), vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
        poll_data.insert("Pasta".to_string(), vec![0, 1, 0, 1, 2, 1, 3, 2, 3, 3]);
        poll_data.insert("Bread".to_string(), vec![0, 1, 2, 1, 1, 2, 1, 2, 2, 3]);

        let result = majority_judgment(&poll_data);
        assert_eq!(
            result,
            vec![(&"Chips".to_string(), 0),
                 (&"Pasta".to_string(), 1),
                 (&"Bread".to_string(), 2),
                 (&"Pizza".to_string(), 3)]);
    }

    #[test]
    fn calling_compute_majority_values() {
        let grades = vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3, 3, 3, 3, 3, 2, 1, 7 ,8];
        let result = compute_majority_values(grades);
        assert_eq!(result, vec![2, 3, 2, 3, 2, 3, 1, 3, 1, 3, 0, 3, 0, 3, 0, 7, 0, 8]);
    }

    #[test]
    fn calling_compute_frequency_of_grades() {
        let grades = vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3, 3, 3, 3, 3, 2, 1, 7 ,8];
        let result = compute_frequency_of_grades(grades);
        let mut expected = BTreeMap::new();
        expected.insert(0, 4);
        expected.insert(1, 2);
        expected.insert(2, 3);
        expected.insert(3, 7);
        expected.insert(7, 1);
        expected.insert(8, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn calling_group_by() {
        let grades = vec![0, 0, 0, 0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 7, 8];
        let result = group_by(grades);
        let expected = vec![vec![0, 0, 0, 0], vec![1, 1], vec![2, 2, 2], vec![3, 3, 3, 3, 3, 3, 3], vec![7], vec![8]];
        assert_eq!(result, expected);
    }

    #[test]
    fn calling_median_grade() {
        let cumsum_vec = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.51, 0.52, 0.6, 0.7, 0.8, 0.9, 1.0];
        let result = median_grade(cumsum_vec);
        assert_eq!(result, 5);

        let cumsum_vec = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.51, 0.52, 0.6, 0.7, 0.8, 0.9, 0.99, 1.0];
        let result = median_grade(cumsum_vec);
        assert_eq!(result, 5);

        let cumsum_vec = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.41, 0.43, 0.45, 0.5, 1.0];
        let result = median_grade(cumsum_vec);
        assert_eq!(result, 8);
    }
}