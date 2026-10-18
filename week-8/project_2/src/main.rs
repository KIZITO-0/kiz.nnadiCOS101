fn main() {
    // Vector of tuples: (name, years_of_experience)
    let candidates = vec![
        ("Alice", 5),
        ("Bob", 12),
        ("Charlie", 8),
        ("David", 15),
        ("Eve", 10),
    ];

    let mut most_experienced = candidates[0];

    for candidate in &candidates {
        if candidate.1 > most_experienced.1 {
            most_experienced = *candidate;
        }
    }

    println!(
        "Most experienced candidate: {} with {} years of experience",
        most_experienced.0,
        most_experienced.1
    );
}

