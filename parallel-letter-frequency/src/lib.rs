use std::collections::HashMap;
use std::thread;

// test bench_large_parallel   ... bench:     165,609 ns/iter (+/- 6,886)
// test bench_large_sequential ... bench:     539,659 ns/iter (+/- 46,748)
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut thread_pool = vec![];
    let mut letters: HashMap<char, usize> = HashMap::new();

    for lines in input.chunks(input.len() / worker_count + 1) {
        // we need to create a new string to pass into the thread which is guaranteed to live as long as the thread execution
        let string = lines.join("");

        let worker = thread::spawn(move || {
            let mut letters: HashMap<char, usize> = HashMap::new();
            for c in string
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                if c.is_alphabetic() {
                    *letters.entry(c).or_default() += 1;
                }
            }
            letters
        });

        thread_pool.push(worker);
    }

    for thread in thread_pool {
        let response = thread.join().unwrap();
        for (key, val) in response.iter() {
            *letters.entry(*key).or_default() += val;
        }
    }

    letters
}
