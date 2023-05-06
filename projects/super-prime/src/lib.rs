use num::bigint::BigInt;
use num::traits::{Zero, One};

fn is_prime(n: &BigInt) -> bool {
    if n <= &BigInt::one() {
        return false;
    }

    for i in 2..=n.sqrt() {
        if n % i == BigInt::zero() {
            return false;
        }
    }

    true
}

fn find_sequences(start: &BigInt, max_len: usize) -> Vec<Vec<u32>> {
    let mut seqs = Vec::new();
    let mut queue = Vec::new();
    queue.push(vec![start.clone()]);

    while !queue.is_empty() {
        let seq = queue.remove(0);
        let last_digit = seq.last().unwrap().clone();
        if seq.len() > max_len {
            break;
        }

        for digit in 0..=9 {
            let mut new_seq = seq.clone();
            new_seq.push(last_digit * 10 + digit);
            if is_prime(&new_seq.last().unwrap()) {
                queue.push(new_seq.clone());
                if new_seq.len() > seqs.len() {
                    seqs.clear();
                    seqs.push(new_seq.iter().map(|n| n.to_u32().unwrap()).collect());
                } else if new_seq.len() == seqs.len() {
                    seqs.push(new_seq.iter().map(|n| n.to_u32().unwrap()).collect());
                }
            }
        }
    }

    seqs
}

fn main() {
    let start = BigInt::from(2);
    let max_len = 10;

    let sequences = find_sequences(&start, max_len);
    for seq in sequences {
        println!("{:?}", seq);
    }
}