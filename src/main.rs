use std::{collections::HashMap, thread, ops::Add};

fn frequency(input: &[& str], workers: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new(); // store for all results
    let joined_input = input.join("").to_lowercase(); // join all input string

    let worker_input_size = &joined_input.len() / workers; // compute the size of input to each worker
    let mut threads = Vec::new(); // store thread results

    let mut postion = 0;
    
    while postion < joined_input.len() {
        let start: usize = postion;
        let mut end: usize = postion.add(worker_input_size - 1);

        if end >= joined_input.len() - 1 {
            end = joined_input.len() - 1;
        }

        let current_input = joined_input.clone()[start..=end].to_owned();

        let thread = thread::spawn( move || frequency_helper(&current_input));
        threads.push(thread);

        postion = postion.add(&worker_input_size);
    }

    for thread in threads {
        let thread_result = thread.join().unwrap();
        for (key, value) in thread_result {
            if result.contains_key(&key) {
                let current_count = result.get(&key).unwrap();
                result.insert(key, value.add(current_count));
            }else{
                result.insert(key, value);
            }
        }
    }

    result
}

fn frequency_helper(input: &str) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for char in input.chars() {
        let check_count_result = result.get(&char);

        let count: usize = match check_count_result {
            Some(t) => *t,
            None => 0,
        };

        let count_updated = count.checked_add(1_usize).unwrap();
        result.insert(char, count_updated);
    }
    result
}

#[cfg(test)]
#[test]
fn no_input() {
    let res = frequency(&[""], 1);
    assert!(res.is_empty())
}

#[test]
fn one_input() {
    let res = frequency(&["a"], 1);
    assert_eq!(res.get(&'a').unwrap(), &1);
}

#[test]
fn two_input() {
    let res = frequency(&["a b", "b a b"], 4);
    assert_eq!(res.get(&'a').unwrap(), &2);
    assert_eq!(res.get(&'b').unwrap(), &3);
}

#[test]
fn random_input() {
    let res = frequency(&["ADKDKknskskdke", "Morjelslkdfj", "mdkdkAKDKDKO"], 2);
    assert_eq!(res.get(&'a').unwrap(), &2);
}

fn main() {
}
