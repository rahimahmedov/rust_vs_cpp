// Rust Iterators Demo
//
// Sections:
//   1 — Iterator trait: lazy evaluation and the pull model
//   2 — map: transform each element
//   3 — flat_map / flatten: flatMap
//   4 — for_each: forEach (side effects)
//   5 — fold / rfold: foldLeft and foldRight

fn main() {
    section1_iterator_trait();
    section2_map();
    section3_flat_map();
    section4_for_each();
    section5_fold();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — The Iterator Trait: Lazy by Default
// ─────────────────────────────────────────────────────────────────────────────

// The core trait:
//   trait Iterator {
//       type Item;
//       fn next(&mut self) -> Option<Self::Item>;
//   }
//
// Adapters (.map(), .filter(), .take(), …) are LAZY — they return a new
// iterator struct that wraps the previous one. NO computation happens until
// a consumer (.collect(), .for_each(), .fold(), .sum(), …) drives the chain.
//
// C++17 STL algorithms (std::transform, std::accumulate) are EAGER.
// C++20 Ranges (std::views::transform, …) are lazy like Rust.

struct Counter {
    value: u32,
    limit: u32,
}

impl Counter {
    fn up_to(limit: u32) -> Self { Counter { value: 0, limit } }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.value < self.limit {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

fn section1_iterator_trait() {
    println!("=== Section 1: The Iterator Trait — Lazy Evaluation ===");
    println!("Rust iterators use a PULL model: values computed only when consumed.");
    println!("C++17 algorithms are EAGER (compute all then return).");
    println!("C++20 Ranges are lazy, like Rust.");
    println!();

    // Custom iterator
    println!("  Custom Counter iterator (implements Iterator trait):");
    let evens: Vec<u32> = Counter::up_to(5)
        .map(|x| x * 2)
        .collect();
    println!("    Counter::up_to(5).map(×2) = {:?}", evens);

    // Lazy evaluation: no work happens until collect()
    println!();
    println!("  Lazy evaluation demonstration:");
    println!("  Building a chain over 1..=1_000_000 but only computing 5 values:");
    let first_five_divisible_by_7: Vec<u32> = (1_u32..=1_000_000)
        .filter(|x| x % 7 == 0)  // lazy — no filtering yet
        .map(|x| x * x)           // lazy — no squaring yet
        .take(5)                   // lazy — pull only 5 items
        .collect();                // NOW the chain executes (only ~35 iterations)
    println!("    first 5 squares of multiples of 7: {:?}", first_five_divisible_by_7);

    // Iterating in different ways
    println!();
    println!("  Three ways to iterate a Vec:");
    let v = vec![10, 20, 30];
    let sum_refs:  i32 = v.iter().sum();        // iter()      → &i32
    let sum_owned: i32 = v.clone().into_iter().sum(); // into_iter() → i32 (moves)
    let sum_mut:   i32 = v.iter().copied().sum(); // iter().copied() → i32 (copies)
    println!("    iter() sum (borrows)  = {}", sum_refs);
    println!("    into_iter() sum (moves) = {}", sum_owned);
    println!("    iter().copied() sum   = {}", sum_mut);
    println!("    v still owned: {:?}", v);

    println!();
    println!("  C++ note: range-for is syntactic sugar for begin()/end() iteration.");
    println!("  C++20 std::ranges::views provides lazy adapters like Rust's.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — map
// ─────────────────────────────────────────────────────────────────────────────

fn section2_map() {
    println!("=== Section 2: map — Transform Each Element ===");
    println!("Rust: .map(|x| ...) — lazy adapter, returns a new iterator.");
    println!("C++17: std::transform (eager, writes to output container).");
    println!("C++20: std::views::transform (lazy, like Rust).");
    println!();

    let numbers = vec![1_i32, 2, 3, 4, 5];

    // Basic map
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("  doubled: {:?}", doubled);
    println!("  squared: {:?}", squared);

    // map with type conversion
    let as_strings: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    let as_floats:  Vec<f64>    = numbers.iter().map(|&x| x as f64 * 1.5).collect();
    println!("  as_strings: {:?}", as_strings);
    println!("  as_floats:  {:?}", as_floats);

    // Chained maps — each .map() wraps the previous iterator (still lazy)
    let chain_result: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)    // step 1: double
        .map(|x| x + 1)     // step 2: add 1
        .map(|x| x * x)     // step 3: square
        .collect();
    println!("  chained (×2, +1, ^2): {:?}", chain_result);

    // Mapping with index (enumerate)
    println!();
    println!("  map with index (enumerate):");
    let indexed: Vec<String> = numbers.iter()
        .enumerate()
        .map(|(i, &x)| format!("[{}]={}", i, x))
        .collect();
    println!("    {:?}", indexed);

    // Parsing strings to numbers
    let raw = vec!["42", "7", "100", "13"];
    let parsed: Vec<i32> = raw.iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!("  parsed strings: {:?}", parsed);

    println!();
    println!("  C++ equivalent:");
    println!("    std::vector<int> doubled(v.size());");
    println!("    std::transform(v.begin(), v.end(), doubled.begin(), [](int x) {{ return x*2; }});");
    println!("    C++20: auto doubled = v | std::views::transform([](int x){{return x*2;}});");
    println!("    KEY DIFFERENCE: C++17 transform needs pre-allocated output container.");
    println!("    Rust .map().collect() allocates exactly what's needed.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — flat_map / flatten
// ─────────────────────────────────────────────────────────────────────────────

fn section3_flat_map() {
    println!("=== Section 3: flat_map / flatten — FlatMap ===");
    println!("flat_map(f) = map(f).flatten()");
    println!("Maps each element to an iterator, then flattens all into one sequence.");
    println!("C++17: no direct equivalent; manual loop or back_inserter.");
    println!("C++23: std::views::transform | std::views::join (lazy flatMap).");
    println!();

    // ── 3a: flat_map on nested Vecs ──────────────────────────────────────────
    println!("  3a. Flatten nested Vecs:");
    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let flat: Vec<i32> = nested.iter()
        .flat_map(|inner| inner.iter().copied())
        .collect();
    println!("    nested={:?}", nested);
    println!("    flat  ={:?}", flat);

    // Equivalently using flatten() after map() — same result
    let flat2: Vec<i32> = nested.iter()
        .map(|inner| inner.iter().copied())
        .flatten()
        .collect();
    println!("    flat2 (map+flatten) = {:?}", flat2);

    // ── 3b: flat_map on strings ───────────────────────────────────────────────
    println!();
    println!("  3b. Words from sentences (flat_map over split_whitespace):");
    let sentences = vec!["hello world rust", "iterators are lazy", "flat map rocks"];
    let words: Vec<&str> = sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("    words: {:?}", words);

    let word_lengths: Vec<usize> = sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .map(|w| w.len())
        .collect();
    println!("    word lengths: {:?}", word_lengths);

    // ── 3c: flat_map to filter and transform together ─────────────────────────
    println!();
    println!("  3c. flat_map to combine filter + transform (Option as container):");
    // flat_map with Option<T> acts like filter_map:
    // Some(x) contributes one element; None contributes zero
    let raw = vec!["1", "two", "3", "four", "5", "6"];
    let parsed_only: Vec<i32> = raw.iter()
        .flat_map(|s| s.parse::<i32>())   // parse returns Ok(n) or Err — into Option via ok()
        .collect();
    // Note: flat_map on IntoIterator works; Result also implements IntoIterator
    println!("    parsing {:?}", raw);
    println!("    successfully parsed: {:?}", parsed_only);

    // The idiomatic way for Option filtering is filter_map:
    let filter_mapped: Vec<i32> = raw.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("    via filter_map (preferred for Option): {:?}", filter_mapped);

    // ── 3d: Product of two ranges (cartesian product via flat_map) ────────────
    println!();
    println!("  3d. Cartesian product via flat_map:");
    let pairs: Vec<(i32, i32)> = (1..=3)
        .flat_map(|x| (1..=3).map(move |y| (x, y)))
        .collect();
    println!("    (1..=3) × (1..=3) = {:?}", pairs);

    println!();
    println!("  C++ equivalent (C++17, manual):");
    println!("    for (auto& inner : nested) for (int x : inner) result.push_back(x);");
    println!("    C++23: nested | std::views::join  (lazy flatten)");
    println!("    C++23: no single-call flatMap — compose transform + join manually");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — for_each
// ─────────────────────────────────────────────────────────────────────────────

fn section4_for_each() {
    println!("=== Section 4: for_each — Consuming Iterator for Side Effects ===");
    println!("for_each(f) consumes the iterator, calling f on each element.");
    println!("Equivalent to a for loop, but composable in an iterator chain.");
    println!("C++ equivalent: std::for_each(begin, end, f).");
    println!();

    let numbers = vec![1_i32, 2, 3, 4, 5];

    // Basic for_each
    println!("  Basic for_each:");
    numbers.iter().for_each(|x| print!("    {} ", x));
    println!();

    // for_each vs for loop — identical semantics
    println!();
    println!("  for_each vs for loop (both equivalent):");
    println!("    for_each: ");
    (1..=5).for_each(|x| print!("{} ", x));
    println!();
    println!("    for loop: ");
    for x in 1..=5 { print!("{} ", x); }
    println!();

    // for_each at the END of a chain (cannot use `for` on a mid-chain iterator easily)
    println!();
    println!("  for_each at the end of a chain:");
    (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .for_each(|x| print!("    {} ", x));
    println!();
    println!("    (even numbers squared: 4 16 36 64 100)");

    // for_each with index using enumerate
    println!();
    println!("  for_each with enumerate (index + value):");
    vec!["alpha", "beta", "gamma", "delta"]
        .iter()
        .enumerate()
        .for_each(|(i, word)| println!("    [{}] = \"{}\"", i, word));

    // for_each with mutable external state (via captured &mut)
    println!();
    println!("  for_each accumulating into external mutable state:");
    let mut evens  = Vec::new();
    let mut odds   = Vec::new();
    (1..=10).for_each(|x| {
        if x % 2 == 0 { evens.push(x); } else { odds.push(x); }
    });
    println!("    evens: {:?}", evens);
    println!("    odds:  {:?}", odds);

    println!();
    println!("  C++ equivalent:");
    println!("    std::for_each(v.begin(), v.end(), [](int x) {{ std::cout << x; }});");
    println!("    C++20: std::ranges::for_each(v, f)  — range-based version");
    println!("    KEY DIFFERENCE: Rust for_each integrates naturally in lazy chains.");
    println!("    C++ for_each is standalone — can't chain with transform lazily (C++17).");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — fold / rfold: foldLeft and foldRight
// ─────────────────────────────────────────────────────────────────────────────

fn section5_fold() {
    println!("=== Section 5: fold / rfold — foldLeft and foldRight ===");
    println!("fold(init, f)  = foldLeft:  processes left to right");
    println!("rfold(init, f) = foldRight: processes right to left (requires DoubleEndedIterator)");
    println!("C++ equivalent: std::accumulate (left), reverse-iterator accumulate (right).");
    println!();

    let numbers = vec![1_i32, 2, 3, 4, 5];

    // ── 5a: Basic foldLeft (fold) ─────────────────────────────────────────────
    println!("  5a. fold (foldLeft) — left to right:");
    let sum     = numbers.iter().fold(0, |acc, &x| acc + x);
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    let max_val = numbers.iter().fold(i32::MIN, |acc, &x| acc.max(x));
    println!("    sum     (fold 0, +) = {}", sum);
    println!("    product (fold 1, ×) = {}", product);
    println!("    max     (fold MIN, max) = {}", max_val);

    // Trace the fold step-by-step to make it clear
    println!();
    println!("  Trace of sum fold: 0→+1→+2→+3→+4→+5");
    let sum_traced = numbers.iter().fold(0_i32, |acc, &x| {
        let new_acc = acc + x;
        println!("    acc={} + x={} → {}", acc, x, new_acc);
        new_acc
    });
    println!("    final sum = {}", sum_traced);

    // Subtraction: non-commutative — foldLeft vs foldRight give different results
    println!();
    println!("  Non-commutative fold (subtraction) — shows foldLeft vs foldRight difference:");
    // foldLeft:  ((((0 - 1) - 2) - 3) - 4) - 5  = -15
    let left_sub  = numbers.iter().fold(0_i32, |acc, &x| acc - x);
    println!("    foldLeft  (0,−): ((((0−1)−2)−3)−4)−5 = {}", left_sub);

    // ── 5b: rfold (foldRight) ─────────────────────────────────────────────────
    println!();
    println!("  5b. rfold (foldRight) — right to left:");
    // foldRight: 1-(2-(3-(4-(5-0)))) = 1-2+3-4+5 = 3
    let right_sub = numbers.iter().rfold(0_i32, |acc, &x| x - acc);
    println!("    foldRight (0,−): 1−(2−(3−(4−(5−0)))) = {}", right_sub);

    let sum_right = numbers.iter().rfold(0_i32, |acc, &x| acc + x);
    println!("    foldRight sum (commutative — same result): {}", sum_right);

    // ── 5c: fold to build complex structures ─────────────────────────────────
    println!();
    println!("  5c. fold to build complex structures:");

    // Build a comma-separated string
    let joined = numbers.iter().fold(String::new(), |mut acc, x| {
        if !acc.is_empty() { acc.push_str(", "); }
        acc.push_str(&x.to_string());
        acc
    });
    println!("    joined: \"{}\"", joined);

    // Build a HashMap from a list of pairs
    use std::collections::HashMap;
    let pairs = vec![("one", 1), ("two", 2), ("three", 3), ("four", 4)];
    let map: HashMap<&str, i32> = pairs.into_iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });
    println!("    fold to HashMap: {:?}", map);

    // Running totals (scan = fold that yields each intermediate value)
    let running: Vec<i32> = numbers.iter()
        .scan(0_i32, |acc, &x| { *acc += x; Some(*acc) })
        .collect();
    println!("    running sum (scan): {:?}", running);

    // ── 5d: Practical fold examples ───────────────────────────────────────────
    println!();
    println!("  5d. Practical fold examples:");

    // Count occurrences of each character
    let text = "hello world";
    let char_counts: HashMap<char, usize> = text.chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
    let mut sorted: Vec<(char, usize)> = char_counts.into_iter().collect();
    sorted.sort_by_key(|&(c, _)| c);
    println!("    char counts in {:?}: {:?}", text, sorted);

    // Check if all / any satisfy a condition (fold versions)
    let all_positive = numbers.iter().fold(true,  |acc, &x| acc && x > 0);
    let any_gt_three = numbers.iter().fold(false, |acc, &x| acc || x > 3);
    println!("    all_positive (fold): {}  [same as .all(|&x| x>0)]", all_positive);
    println!("    any_gt_three (fold): {}  [same as .any(|&x| x>3)]", any_gt_three);

    println!();
    println!("  C++ equivalents:");
    println!("    foldLeft:  std::accumulate(v.begin(), v.end(), 0, f)");
    println!("    foldRight: std::accumulate(v.rbegin(), v.rend(), 0, f)");
    println!("    C++17 std::reduce: parallel-friendly but unordered (not foldLeft)");
    println!("    KEY DIFFERENCE: Rust fold/rfold are lazy chain adapters.");
    println!("    C++ accumulate is a standalone eager algorithm — no chaining.");
}
