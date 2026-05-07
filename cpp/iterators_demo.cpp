// C++ Iterators Demo — C++17 (with C++20 notes)
// Mirrors the Rust iterators_demo: same 5 sections, analogous patterns.
//
// Sections:
//   1 — Iterator concepts: begin/end pairs, range-for, eagerness vs laziness
//   2 — map     → std::transform (C++17 eager) / std::views::transform (C++20 lazy)
//   3 — flat_map → manual loop / C++23 views::join
//   4 — for_each → std::for_each / std::ranges::for_each
//   5 — fold    → std::accumulate (foldLeft) / reverse-iterator accumulate (foldRight)

#include <algorithm>
#include <functional>
#include <iostream>
#include <iterator>
#include <map>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

// Helper: print a vector
template <typename T>
static void print_vec(const std::string& label, const std::vector<T>& v) {
    std::cout << "    " << label << ": [";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "]\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — The C++ Iterator Concept
// ─────────────────────────────────────────────────────────────────────────────

// Custom forward iterator — equivalent to implementing Rust's Iterator trait
class Counter {
public:
    class iterator {
        unsigned value_;
    public:
        using value_type        = unsigned;
        using difference_type   = std::ptrdiff_t;
        using pointer           = const unsigned*;
        using reference         = const unsigned&;
        using iterator_category = std::input_iterator_tag;

        iterator(unsigned v, unsigned) : value_(v) {}
        unsigned  operator*()  const { return value_; }
        iterator& operator++()       { ++value_; return *this; }
        bool operator!=(const iterator& o) const { return value_ != o.value_; }
    };

    explicit Counter(unsigned limit) : limit_(limit) {}
    iterator begin() const { return { 1, limit_ }; }
    iterator end()   const { return { limit_ + 1, limit_ }; }

private:
    unsigned limit_;
};

static void section1_iterator_concept() {
    std::cout << "=== Section 1: The C++ Iterator Concept ===\n";
    std::cout << "C++17: iterator PAIRS (begin/end) — external iteration.\n";
    std::cout << "       STL algorithms are EAGER — compute everything immediately.\n";
    std::cout << "C++20: Ranges library adds LAZY adapters (like Rust).\n";
    std::cout << "Rust:  Iterator trait (single next() method) — PULL model, LAZY.\n\n";

    // Custom range (Counter) — needs begin()/end() pair
    std::cout << "  Custom Counter (implements begin/end):\n";
    Counter c(5);
    std::vector<unsigned> doubled;
    for (unsigned x : c) doubled.push_back(x * 2);
    print_vec("Counter(5), doubled", doubled);
    std::cout << "    C++ range-for = syntactic sugar for begin()/end() iteration\n";
    std::cout << "    Rust for x in iter = syntactic sugar for Iterator::next()\n\n";

    // Eagerness demonstration
    std::cout << "  Eagerness vs Laziness:\n";
    std::vector<int> source = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    // C++17 eager: transform creates a full new vector
    std::vector<int> doubled_all(source.size());
    std::transform(source.begin(), source.end(), doubled_all.begin(),
                   [](int x) { return x * 2; });
    print_vec("std::transform (eager, all 10)", doubled_all);

    // C++17 manual lazy simulation (loop with early exit)
    std::cout << "    Manual lazy C++17 (first 3 doubled-evens):\n    [";
    int found = 0;
    for (int x : source) {
        if (found == 3) break;
        if (x % 2 == 0) { std::cout << (found ? ", " : "") << x * 2; ++found; }
    }
    std::cout << "]\n";
    std::cout << "    Rust: source.iter().filter(even).map(×2).take(3).collect()\n";
    std::cout << "          — zero extra allocation, lazy, reads only what's needed\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — map: std::transform
// ─────────────────────────────────────────────────────────────────────────────

static void section2_transform() {
    std::cout << "=== Section 2: map — std::transform ===\n";
    std::cout << "C++17: std::transform(first, last, output, unary_op) — eager.\n";
    std::cout << "C++20: std::views::transform — lazy range adapter.\n";
    std::cout << "Rust:  .map(|x| ...) — lazy, zero-cost adapter.\n\n";

    std::vector<int> numbers = {1, 2, 3, 4, 5};

    // Basic std::transform
    std::vector<int> doubled(numbers.size()), squared(numbers.size());
    std::transform(numbers.begin(), numbers.end(), doubled.begin(),
                   [](int x) { return x * 2; });
    std::transform(numbers.begin(), numbers.end(), squared.begin(),
                   [](int x) { return x * x; });
    print_vec("doubled", doubled);
    print_vec("squared", squared);

    // Type-converting transform
    std::vector<std::string> as_strings(numbers.size());
    std::transform(numbers.begin(), numbers.end(), as_strings.begin(),
                   [](int x) { return std::to_string(x); });
    std::cout << "    as_strings: [";
    for (size_t i = 0; i < as_strings.size(); ++i)
        std::cout << (i ? ", " : "") << "\"" << as_strings[i] << "\"";
    std::cout << "]\n";

    // Chaining transforms: C++17 requires intermediate vectors
    std::vector<int> step1(numbers.size()), step2(numbers.size()), step3(numbers.size());
    std::transform(numbers.begin(), numbers.end(), step1.begin(), [](int x){return x*2;});
    std::transform(step1.begin(),   step1.end(),   step2.begin(), [](int x){return x+1;});
    std::transform(step2.begin(),   step2.end(),   step3.begin(), [](int x){return x*x;});
    print_vec("chained (×2, +1, ^2)", step3);

    // Indexed: use iota + lambda over indices
    std::cout << "\n  map with index (no built-in; use enumerate manually):\n";
    std::vector<std::string> indexed(numbers.size());
    for (size_t i = 0; i < numbers.size(); ++i)
        indexed[i] = "[" + std::to_string(i) + "]=" + std::to_string(numbers[i]);
    std::cout << "    [";
    for (size_t i = 0; i < indexed.size(); ++i) std::cout << (i?", ":"") << "\"" << indexed[i] << "\"";
    std::cout << "]\n";
    std::cout << "    Rust: .enumerate().map(|(i, &x)| format!(\"[{}]={}\", i, x))\n";

    std::cout << "\n  KEY DIFFERENCES from Rust .map():\n";
    std::cout << "    C++17 std::transform: needs pre-allocated output vector\n";
    std::cout << "    Rust .map(): lazy adapter, no allocation until .collect()\n";
    std::cout << "    C++17 chaining: requires intermediate vectors at each step\n";
    std::cout << "    Rust chaining: zero-cost — each .map() wraps the previous lazily\n";
    std::cout << "    C++20 views::transform: lazy, chains without intermediate allocations\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — flat_map: manual loop / join
// ─────────────────────────────────────────────────────────────────────────────

static void section3_flat_map() {
    std::cout << "=== Section 3: flat_map — Manual Loop / std::views::join ===\n";
    std::cout << "C++17: no built-in flatMap; use nested loops + back_inserter.\n";
    std::cout << "C++20: std::views::transform | std::views::join (lazy flatMap).\n";
    std::cout << "C++23: std::views::join_with.\n";
    std::cout << "Rust:  .flat_map(|x| ...) — lazy, composable.\n\n";

    // ── 3a: Flatten nested vectors ─────────────────────────────────────────────
    std::cout << "  3a. Flatten nested vectors (C++17 manual):\n";
    std::vector<std::vector<int>> nested = {{1, 2, 3}, {4, 5}, {6, 7, 8, 9}};
    std::vector<int> flat;
    for (const auto& inner : nested) {
        flat.insert(flat.end(), inner.begin(), inner.end());
    }
    print_vec("flattened", flat);
    std::cout << "    Rust: nested.iter().flatten().collect()\n\n";

    // ── 3b: Words from sentences ────────────────────────────────────────────────
    std::cout << "  3b. Words from sentences (flatMap over split):\n";
    std::vector<std::string> sentences = {"hello world rust", "iterators are lazy", "flat map rocks"};
    std::vector<std::string> words;
    for (const auto& sentence : sentences) {
        std::istringstream iss(sentence);
        std::string word;
        while (iss >> word) words.push_back(word);
    }
    std::cout << "    words: [";
    for (size_t i = 0; i < words.size(); ++i) std::cout << (i?", ":"") << "\"" << words[i] << "\"";
    std::cout << "]\n";
    std::cout << "    Rust: sentences.iter().flat_map(|s| s.split_whitespace()).collect()\n\n";

    // ── 3c: flat_map to filter+transform (Option equivalent) ────────────────────
    std::cout << "  3c. flat_map to parse only valid integers (filter_map equivalent):\n";
    std::vector<std::string> raw = {"1", "two", "3", "four", "5", "6"};
    std::vector<int> parsed_only;
    for (const auto& s : raw) {
        try {
            parsed_only.push_back(std::stoi(s));
        } catch (...) {
            // skip non-numeric strings
        }
    }
    print_vec("parsed only", parsed_only);
    std::cout << "    Rust: raw.iter().filter_map(|s| s.parse::<i32>().ok()).collect()\n\n";

    // ── 3d: Cartesian product ─────────────────────────────────────────────────
    std::cout << "  3d. Cartesian product (1..=3) × (1..=3):\n";
    std::vector<std::pair<int,int>> pairs;
    for (int x = 1; x <= 3; ++x)
        for (int y = 1; y <= 3; ++y)
            pairs.push_back({x, y});
    std::cout << "    [";
    for (size_t i = 0; i < pairs.size(); ++i)
        std::cout << (i?", ":"") << "(" << pairs[i].first << "," << pairs[i].second << ")";
    std::cout << "]\n";
    std::cout << "    Rust: (1..=3).flat_map(|x| (1..=3).map(move |y| (x, y))).collect()\n\n";

    std::cout << "  KEY DIFFERENCES from Rust flat_map:\n";
    std::cout << "    C++17: no built-in — always requires manual nested loops\n";
    std::cout << "    Rust:  .flat_map() is a lazy adapter — no intermediate allocation\n";
    std::cout << "    C++20: views::transform + views::join = lazy flatMap\n";
    std::cout << "    Rust flat_map with Option = automatic filtering (None = skip)\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — for_each: std::for_each
// ─────────────────────────────────────────────────────────────────────────────

static void section4_for_each() {
    std::cout << "=== Section 4: for_each — std::for_each ===\n";
    std::cout << "C++: std::for_each(begin, end, f) — applies f to each element.\n";
    std::cout << "C++20: std::ranges::for_each(range, f) — range-based version.\n";
    std::cout << "Rust: .for_each(|x| ...) — integrates in lazy iterator chains.\n\n";

    std::vector<int> numbers = {1, 2, 3, 4, 5};

    // Basic std::for_each
    std::cout << "  Basic std::for_each:\n    ";
    std::for_each(numbers.begin(), numbers.end(), [](int x) { std::cout << x << " "; });
    std::cout << "\n\n";

    // for_each vs range-for — equivalent results
    std::cout << "  std::for_each vs range-for (equivalent):\n";
    std::cout << "    for_each: ";
    std::for_each(numbers.begin(), numbers.end(), [](int x) { std::cout << x << " "; });
    std::cout << "\n    range-for:";
    for (int x : numbers) std::cout << " " << x;
    std::cout << "\n";

    // Accumulating external state via captured reference
    std::cout << "\n  std::for_each accumulating into external mutable state:\n";
    std::vector<int> evens, odds;
    std::for_each(numbers.begin(), numbers.end(), [&evens, &odds](int x) {
        (x % 2 == 0 ? evens : odds).push_back(x);
    });
    print_vec("evens", evens);
    print_vec("odds", odds);

    // With enumerate (index): no built-in — use a counter
    std::cout << "\n  for_each with index (manual counter — C++ has no enumerate):\n";
    std::vector<std::string> fruits = {"apple", "banana", "cherry", "date"};
    int idx = 0;
    std::for_each(fruits.begin(), fruits.end(), [&idx](const std::string& s) {
        std::cout << "    [" << idx++ << "] = \"" << s << "\"\n";
    });

    // Chain simulation (can't chain directly in C++17 — must break into steps)
    std::cout << "\n  for_each at end of multi-step pipeline (C++17, three passes):\n";
    std::vector<int> step1(10), step2;
    std::iota(step1.begin(), step1.end(), 1);               // 1..=10
    std::copy_if(step1.begin(), step1.end(), std::back_inserter(step2),
                 [](int x) { return x % 2 == 0; });         // filter evens
    std::vector<int> step3(step2.size());
    std::transform(step2.begin(), step2.end(), step3.begin(),
                   [](int x) { return x * x; });             // square
    std::cout << "    (1..=10).filter(even).map(^2) = [";
    std::for_each(step3.begin(), step3.end(),
                  [first = true](int x) mutable {
                      std::cout << (first ? "" : ", ") << x; first = false;
                  });
    std::cout << "]\n";
    std::cout << "    Rust: (1..=10).filter(|x| x%2==0).map(|x| x*x).for_each(|x| print!(\"{} \",x))\n";
    std::cout << "    — single lazy chain, no intermediate vectors\n\n";

    std::cout << "  KEY DIFFERENCES from Rust .for_each():\n";
    std::cout << "    C++17: std::for_each is a standalone algorithm — not chainable\n";
    std::cout << "    Rust:  .for_each() is the terminal of a lazy chain\n";
    std::cout << "    C++17 pipeline needs N intermediate vectors for N steps\n";
    std::cout << "    Rust pipeline: zero intermediate allocations\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — fold: std::accumulate (foldLeft and foldRight)
// ─────────────────────────────────────────────────────────────────────────────

static void section5_fold() {
    std::cout << "=== Section 5: fold — std::accumulate (foldLeft / foldRight) ===\n";
    std::cout << "C++: std::accumulate(first, last, init, binary_op) — foldLeft.\n";
    std::cout << "     std::accumulate(rbegin, rend, init, binary_op) — foldRight.\n";
    std::cout << "Rust: .fold(init, |acc, x| ...)  = foldLeft.\n";
    std::cout << "      .rfold(init, |acc, x| ...) = foldRight (DoubleEndedIterator).\n\n";

    std::vector<int> numbers = {1, 2, 3, 4, 5};

    // ── 5a: Basic foldLeft ────────────────────────────────────────────────────
    std::cout << "  5a. foldLeft (std::accumulate, left to right):\n";
    int sum     = std::accumulate(numbers.begin(), numbers.end(), 0,
                                  [](int acc, int x) { return acc + x; });
    int product = std::accumulate(numbers.begin(), numbers.end(), 1,
                                  [](int acc, int x) { return acc * x; });
    int max_val = std::accumulate(numbers.begin(), numbers.end(), INT_MIN,
                                  [](int acc, int x) { return std::max(acc, x); });
    std::cout << "    sum     (accumulate 0, +) = " << sum     << "\n";
    std::cout << "    product (accumulate 1, ×) = " << product << "\n";
    std::cout << "    max     (accumulate MIN)  = " << max_val << "\n";

    // Trace the fold step-by-step
    std::cout << "\n  Trace of sum accumulate: 0→+1→+2→+3→+4→+5\n";
    std::accumulate(numbers.begin(), numbers.end(), 0, [](int acc, int x) {
        int new_acc = acc + x;
        std::cout << "    acc=" << acc << " + x=" << x << " → " << new_acc << "\n";
        return new_acc;
    });

    // Non-commutative: subtraction shows foldLeft vs foldRight difference
    std::cout << "\n  Non-commutative fold (subtraction):\n";
    // foldLeft:  ((((0 - 1) - 2) - 3) - 4) - 5 = -15
    int left_sub = std::accumulate(numbers.begin(), numbers.end(), 0,
                                   [](int acc, int x) { return acc - x; });
    std::cout << "    foldLeft  (0,−): ((((0−1)−2)−3)−4)−5 = " << left_sub << "\n";

    // ── 5b: foldRight via reverse iterators ──────────────────────────────────
    std::cout << "\n  5b. foldRight via reverse iterators (rbegin/rend):\n";
    // foldRight: 1-(2-(3-(4-(5-0)))) = 3
    // NOTE: with reverse iterators, the lambda receives (acc, x) where x comes from right
    // We need to flip: foldRight f z [1..5] = f 1 (f 2 (f 3 (f 4 (f 5 z))))
    int right_sub = std::accumulate(numbers.rbegin(), numbers.rend(), 0,
                                    [](int acc, int x) { return x - acc; });
    std::cout << "    foldRight (0,−): 1−(2−(3−(4−(5−0)))) = " << right_sub << "\n";

    int right_sum = std::accumulate(numbers.rbegin(), numbers.rend(), 0,
                                    [](int acc, int x) { return acc + x; });
    std::cout << "    foldRight sum (commutative — same): " << right_sum << "\n";
    std::cout << "    Rust: .rfold(0, |acc, &x| x - acc) — same semantics\n";

    // ── 5c: fold to build complex structures ─────────────────────────────────
    std::cout << "\n  5c. fold to build complex structures:\n";

    // Comma-separated string
    std::string joined = std::accumulate(
        numbers.begin(), numbers.end(), std::string{},
        [](const std::string& acc, int x) {
            return acc.empty() ? std::to_string(x) : acc + ", " + std::to_string(x);
        });
    std::cout << "    joined: \"" << joined << "\"\n";

    // Build a map from pairs
    std::vector<std::pair<std::string, int>> kvs = {{"one",1},{"two",2},{"three",3},{"four",4}};
    auto m = std::accumulate(kvs.begin(), kvs.end(), std::map<std::string,int>{},
        [](std::map<std::string,int> acc, const std::pair<std::string,int>& kv) {
            acc.insert(kv);
            return acc;
        });
    std::cout << "    fold to map: {";
    bool first = true;
    for (const auto& [k,v] : m) {
        std::cout << (first?"":", ") << k << ":" << v; first = false;
    }
    std::cout << "}\n";

    // Running totals (scan — partial_sum equivalent)
    std::vector<int> running(numbers.size());
    std::partial_sum(numbers.begin(), numbers.end(), running.begin());
    print_vec("running sum (partial_sum)", running);
    std::cout << "    Rust: .scan(0, |acc, &x| {{ *acc += x; Some(*acc) }}).collect()\n";

    // ── 5d: Practical fold examples ───────────────────────────────────────────
    std::cout << "\n  5d. Practical: character frequency count:\n";
    std::string text = "hello world";
    auto char_counts = std::accumulate(text.begin(), text.end(),
        std::map<char, int>{},
        [](std::map<char,int> acc, char c) { ++acc[c]; return acc; });
    std::cout << "    char counts in \"" << text << "\": {";
    first = true;
    for (const auto& [c, cnt] : char_counts) {
        if (c == ' ') continue;
        std::cout << (first?"":", ") << "'" << c << "':" << cnt; first = false;
    }
    std::cout << "}\n";

    std::cout << "\n  KEY DIFFERENCES from Rust fold/rfold:\n";
    std::cout << "    C++: std::accumulate is a standalone eager function\n";
    std::cout << "    Rust: .fold() is a lazy chain consumer — composes with map/filter\n";
    std::cout << "    C++: foldRight needs rbegin/rend — awkward, easy to get wrong\n";
    std::cout << "    Rust: .rfold() is first-class on DoubleEndedIterator\n";
    std::cout << "    C++17 std::reduce: parallel-friendly but UNORDERED (not foldLeft!)\n";
    std::cout << "    Rust .fold(): always ordered, left-to-right, deterministic\n";
}

// ─────────────────────────────────────────────────────────────────────────────

int main() {
    section1_iterator_concept();
    section2_transform();
    section3_flat_map();
    section4_for_each();
    section5_fold();
    return 0;
}
