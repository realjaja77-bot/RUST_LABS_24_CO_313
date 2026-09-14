use std::collections::HashMap;

// --- 4.2 Exercise A ---
fn stats(data: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mean, min, max)
}

// TODO 1: Compute median
fn compute_median(sorted_scores: &[f64]) -> f64 {
    let len = sorted_scores.len();
    if len % 2 == 0 {
        (sorted_scores[len / 2 - 1] + sorted_scores[len / 2]) / 2.0
    } else {
        sorted_scores[len / 2]
    }
}

// TODO 2: Compute variance and standard deviation
fn compute_variance_and_std_dev(scores: &[f64], mean: f64) -> (f64, f64) {
    let variance = scores.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / scores.len() as f64;
    (variance, variance.sqrt())
}

// --- 4.3 Exercise B ---
fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        if !clean.is_empty() {
            *freq.entry(clean).or_insert(0) += 1;
        }
    }
    freq
}

// TODO 3: Top N words sorted descending
fn top_n<'a>(freq: &'a HashMap<String, usize>, n: usize) -> Vec<(&'a String, &'a usize)> {
    let mut list: Vec<(&'a String, &'a usize)> = freq.iter().collect();
    list.sort_by(|a, b| b.1.cmp(a.1));
    list.into_iter().take(n).collect()
}

// --- 4.4 Exercise C ---
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// Helper for TODO 4b
fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    (2..=((n as f64).sqrt() as u32)).all(|i| n % i != 0)
}

// --- 4.5 Exercise D ---
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}

// TODO 6: Custom infinite Primes iterator
struct Primes {
    current: u64,
}

impl Primes {
    fn new() -> Self {
        Primes { current: 1 }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.current += 1;
            let n = self.current;
            if (2..=((n as f64).sqrt() as u64)).all(|i| n % i != 0) {
                return Some(n);
            }
        }
    }
}

fn main() {
    println!("--- 4.2 Vec Operations ---");
    let mut scores: Vec<f64> = vec![85.0, 92.0, 78.5, 95.0, 60.0, 88.0];
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (mean, min, max) = stats(&scores);
    let med = compute_median(&scores);
    let (var, std_dev) = compute_variance_and_std_dev(&scores, mean);
    println!("Sorted: {:?}", scores);
    println!("Mean={:.2}, Min={:.2}, Max={:.2}, Median={:.2}", mean, min, max, med);
    println!("Variance={:.2}, Std Dev={:.2}\n", var, std_dev);

    println!("--- 4.3 HashMap Top N ---");
    let text = "the quick brown fox jumps over the lazy dog the fox was very quick and the dog was lazy";
    let freq = word_frequency(text);
    for (word, count) in top_n(&freq, 5) {
        println!("{:10}: {}", word, count);
    }

    println!("\n--- 4.4 Iterator Tasks ---");
    // TODO 4a: Sum of squares of odd numbers from 1 to 99
    let sum_odd_sq: u64 = (1..=99).filter(|x| x % 2 != 0).map(|x| (x * x) as u64).sum();
    println!("Sum of squares of odd numbers (1..99): {}", sum_odd_sq);

    // TODO 4b: Collect primes up to 50
    let primes_up_to_50: Vec<u32> = (1..=50).filter(|&x| is_prime(x)).collect();
    println!("Primes up to 50: {:?}\n", primes_up_to_50);

    println!("--- 4.5 Custom Iterators ---");
    // TODO 5: First Fibonacci > 1,000,000
    let first_over_1m = Fibonacci::new().find(|&x| x > 1_000_000).unwrap();
    println!("First Fibonacci > 1,000,000: {}", first_over_1m);

    // TODO 6: Infinite Primes iterator test
    let first_10_primes: Vec<u64> = Primes::new().take(10).collect();
    println!("First 10 primes from infinite iterator: {:?}", first_10_primes);
}