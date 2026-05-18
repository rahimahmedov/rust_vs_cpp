// Rust Error Handling Demo
//
// Sections:
//   1 — Result<T, E>: the core error type
//   2 — Option<T>: representing absence
//   3 — The ? operator: propagating errors ergonomically
//   4 — Custom error types via std::error::Error
//   5 — Patterns and antipatterns

use std::fmt;
use std::num::ParseIntError;

fn main() {
    section1_result_basics();
    section2_option_basics();
    section3_question_mark();
    section4_custom_errors();
    section5_patterns_antipatterns();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Result<T, E>
// ─────────────────────────────────────────────────────────────────────────────

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn section1_result_basics() {
    println!("=== Section 1: Result<T, E> ===");
    println!("Result<T, E> is Rust's primary mechanism for recoverable errors.");
    println!("  enum Result<T, E> {{ Ok(T), Err(E) }}");
    println!("  No exceptions, no surprise control-flow jumps.");
    println!("  The caller is forced to acknowledge the error at compile time.");
    println!();

    // match: exhaustive — must handle both arms
    println!("  1a. Exhaustive match:");
    match divide(10.0, 3.0) {
        Ok(v)  => println!("    10 / 3  = {:.4}  (Ok)", v),
        Err(e) => println!("    Error: {}", e),
    }
    match divide(5.0, 0.0) {
        Ok(v)  => println!("    5 / 0   = {}  (Ok)", v),
        Err(e) => println!("    5 / 0   → Err(\"{}\")", e),
    }
    println!();

    // Combinators: transform without unwrapping
    println!("  1b. Combinators — map, map_err, and_then, unwrap_or:");
    let doubled = divide(10.0, 4.0).map(|v| v * 2.0);
    println!("    divide(10,4).map(*2)        = {:?}", doubled);

    let stringified = divide(10.0, 4.0).map(|v| format!("{:.2}", v));
    println!("    divide(10,4).map(format)    = {:?}", stringified);

    let fallback = divide(1.0, 0.0).unwrap_or(f64::NAN);
    println!("    divide(1,0).unwrap_or(NaN)  = {}", fallback);

    let chained = divide(20.0, 4.0).and_then(|v| divide(v, 2.0));
    println!("    divide(20,4).and_then(/2)   = {:?}", chained);

    let mapped_err = divide(1.0, 0.0)
        .map_err(|e| format!("Math error: {}", e));
    println!("    divide(1,0).map_err(wrap)   = {:?}", mapped_err);
    println!();

    // is_ok / is_err predicates
    println!("  1c. Predicates:");
    println!("    divide(10,3).is_ok()  = {}", divide(10.0, 3.0).is_ok());
    println!("    divide(1,0).is_err()  = {}", divide(1.0, 0.0).is_err());
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Option<T>
// ─────────────────────────────────────────────────────────────────────────────

fn find_first_even(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().find(|n| n % 2 == 0)
}

fn section2_option_basics() {
    println!("=== Section 2: Option<T> ===");
    println!("Option<T> represents a value that may or may not be present.");
    println!("  enum Option<T> {{ Some(T), None }}");
    println!("  Rust has NO null pointer. Option replaces null entirely.");
    println!("  The type system forces you to handle the None case.");
    println!();

    println!("  2a. Basic usage:");
    let evens_present = find_first_even(&[1, 3, 4, 7]);
    let evens_absent  = find_first_even(&[1, 3, 5, 7]);
    println!("    first even in [1,3,4,7] = {:?}", evens_present);
    println!("    first even in [1,3,5,7] = {:?}", evens_absent);
    println!();

    println!("  2b. Combinators:");
    let doubled = find_first_even(&[1, 3, 6, 9]).map(|n| n * 2);
    println!("    find(6).map(*2)          = {:?}", doubled);

    let fallback = find_first_even(&[1, 3, 5]).unwrap_or(0);
    println!("    find(none).unwrap_or(0)  = {}", fallback);

    let filtered = find_first_even(&[2, 4, 6]).filter(|&n| n > 3);
    println!("    find(2).filter(>3)       = {:?}", filtered);  // None: 2 <= 3

    let chained: Option<String> = find_first_even(&[1, 8, 9])
        .and_then(|n| if n > 5 { Some(format!("big:{}", n)) } else { None });
    println!("    find(8).and_then(big?)   = {:?}", chained);
    println!();

    println!("  2c. Option <-> Result conversions:");
    let as_result: Result<i32, &str> = find_first_even(&[1, 3, 5])
        .ok_or("no even number found");
    println!("    find(none).ok_or(msg)    = {:?}", as_result);

    let as_option: Option<f64> = divide(10.0, 2.0).ok();
    println!("    divide(10,2).ok()        = {:?}", as_option);
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — The ? operator
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Logic(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Logic(s) => write!(f, "logic error: {}", s),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_and_double(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.trim().parse()?;  // ParseIntError auto-converts via From impl
    if n < 0 {
        return Err(AppError::Logic(format!("{} is negative", n)));
    }
    Ok(n * 2)
}

fn multi_step(a: &str, b: &str) -> Result<i32, AppError> {
    let x = parse_and_double(a)?;  // propagate immediately on Err
    let y = parse_and_double(b)?;  // same
    Ok(x + y)
}

fn section3_question_mark() {
    println!("=== Section 3: The ? Operator ===");
    println!("The ? operator is syntactic sugar for early return on Err/None.");
    println!("  expr?  ≡  match expr {{ Ok(v) => v, Err(e) => return Err(e.into()) }}");
    println!("  It also calls .into() — enabling automatic error type conversion");
    println!("  via From<E> implementations.");
    println!();

    println!("  3a. Single ? in parse_and_double:");
    println!("    parse_and_double(\"21\")  = {:?}", parse_and_double("21"));
    println!("    parse_and_double(\"abc\") = {:?}", parse_and_double("abc"));
    println!("    parse_and_double(\"-5\")  = {:?}", parse_and_double("-5"));
    println!();

    println!("  3b. Chained ? in multi_step:");
    println!("    multi_step(\"3\", \"7\")    = {:?}", multi_step("3", "7"));
    println!("    multi_step(\"3\", \"bad\")  = {:?}", multi_step("3", "bad"));
    println!("    multi_step(\"-1\", \"7\")   = {:?}", multi_step("-1", "7"));
    println!();

    println!("  3c. Without ?: verbose equivalent:");
    println!("    let n = match \"42\".parse::<i32>() {{");
    println!("        Ok(v)  => v,");
    println!("        Err(e) => return Err(AppError::from(e)),");
    println!("    }};");
    println!("    // ? collapses this to: let n: i32 = \"42\".parse()?;");
    println!();

    println!("  3d. ? on Option:");
    fn first_char(s: &str) -> Option<char> {
        let c = s.chars().next()?;  // returns None if empty
        Some(c.to_ascii_uppercase())
    }
    println!("    first_char(\"hello\") = {:?}", first_char("hello"));
    println!("    first_char(\"\")      = {:?}", first_char(""));
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Custom Error Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum DbError {
    NotFound(u32),
    ConnectionFailed(String),
    PermissionDenied,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound(id)          => write!(f, "record {} not found", id),
            DbError::ConnectionFailed(msg) => write!(f, "connection failed: {}", msg),
            DbError::PermissionDenied      => write!(f, "permission denied"),
        }
    }
}

impl std::error::Error for DbError {}

fn fetch_user(id: u32) -> Result<String, DbError> {
    match id {
        1 => Ok(String::from("Alice")),
        2 => Ok(String::from("Bob")),
        0 => Err(DbError::PermissionDenied),
        _ => Err(DbError::NotFound(id)),
    }
}

fn fetch_username_upper(id: u32) -> Result<String, DbError> {
    let name = fetch_user(id)?;
    Ok(name.to_uppercase())
}

// A "root cause" holder: wraps any error via Box<dyn Error>
fn fetch_user_dynamic(id: u32) -> Result<String, Box<dyn std::error::Error>> {
    let name = fetch_user(id)?;  // DbError implements Error, so ? boxes it
    Ok(name)
}

fn section4_custom_errors() {
    println!("=== Section 4: Custom Error Types ===");
    println!("Idiomatic Rust: define an enum covering all failure modes,");
    println!("  implement Display (human message) and std::error::Error (trait object).");
    println!("  Add From<OtherError> impls to enable ? across error type boundaries.");
    println!();

    println!("  4a. Matching error variants:");
    for id in [1u32, 2, 0, 99] {
        match fetch_user(id) {
            Ok(name)                       => println!("    id={}: user = {}", id, name),
            Err(DbError::NotFound(n))      => println!("    id={}: not found (id={})", id, n),
            Err(DbError::PermissionDenied) => println!("    id={}: access denied", id),
            Err(DbError::ConnectionFailed(m)) => println!("    id={}: conn err: {}", id, m),
        }
    }
    println!();

    println!("  4b. Chaining with ?:");
    println!("    fetch_username_upper(1)  = {:?}", fetch_username_upper(1));
    println!("    fetch_username_upper(99) = {:?}", fetch_username_upper(99));
    println!();

    println!("  4c. Box<dyn Error> — type-erased error (useful in main / scripts):");
    println!("    fetch_user_dynamic(2)   = {:?}", fetch_user_dynamic(2));
    println!("    fetch_user_dynamic(0)   = {:?}", fetch_user_dynamic(0));
    println!();

    println!("  4d. Display vs Debug:");
    let e_not_found = DbError::NotFound(42);
    let e_conn = DbError::ConnectionFailed("timeout after 30s".to_string());
    println!("    Display (user-facing): {}", e_not_found);
    println!("    Debug   (developer):   {:?}", e_not_found);
    println!("    Display ConnectionFailed: {}", e_conn);
    println!("    Debug   ConnectionFailed: {:?}", e_conn);
    println!();

    println!("  4e. panic! vs Result — when to use each:");
    println!("    panic!  → unrecoverable bugs: index out of bounds, broken invariants");
    println!("    Result  → expected failures: I/O, parsing, network, user input");
    println!("    Rule: if the caller can do something useful about the error, use Result.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Patterns and Antipatterns
// ─────────────────────────────────────────────────────────────────────────────

fn section5_patterns_antipatterns() {
    println!("=== Section 5: Patterns and Antipatterns ===");
    println!();

    // ── PATTERNS ─────────────────────────────────────────────────────────────
    println!("  ── PATTERNS (good) ──────────────────────────────────────────────");
    println!();

    println!("  P1. Propagate with ? instead of manual match:");
    println!("    // good");
    println!("    fn read_count(s: &str) -> Result<usize, ParseIntError> {{");
    println!("        Ok(s.trim().parse::<usize>()?)");
    println!("    }}");
    println!();

    println!("  P2. Use map_err to add context:");
    let result = "abc".parse::<i32>()
        .map_err(|e| format!("failed to parse config value 'timeout': {}", e));
    println!("    \"abc\".parse().map_err(context) = {:?}", result);
    println!();

    println!("  P3. Collect Results — fail-fast on first error:");
    let strings = vec!["1", "2", "bad", "4"];
    let parsed: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("    collect [\"1\",\"2\",\"bad\",\"4\"] = {:?}", parsed);
    let ok_strings = vec!["1", "2", "3"];
    let ok_parsed: Result<Vec<i32>, _> = ok_strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("    collect [\"1\",\"2\",\"3\"]        = {:?}", ok_parsed);
    println!();

    println!("  P4. filter_map to skip errors silently:");
    let mixed = vec!["1", "two", "3", "four", "5"];
    let valid: Vec<i32> = mixed.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("    filter_map parse [\"1\",\"two\",\"3\",...] = {:?}", valid);
    println!();

    println!("  P5. if let / while let for ergonomic None/Err handling:");
    if let Some(v) = find_first_even(&[3, 5, 8]) {
        println!("    if let Some(v): found even = {}", v);
    }
    println!();

    println!("  P6. Return Box<dyn Error> from main for scripts:");
    println!("    fn main() -> Result<(), Box<dyn std::error::Error>> {{");
    println!("        let n: i32 = std::env::args().nth(1)");
    println!("            .ok_or(\"missing argument\")?.parse()?;");
    println!("        println!(\"got {{}}\", n);");
    println!("        Ok(())");
    println!("    }}");
    println!();

    // ── ANTIPATTERNS ──────────────────────────────────────────────────────────
    println!("  ── ANTIPATTERNS (avoid) ─────────────────────────────────────────");
    println!();

    println!("  A1. .unwrap() in production code → panics on None/Err:");
    println!("    let x: Option<i32> = None;");
    println!("    x.unwrap();  // PANIC: called `Option::unwrap()` on a `None` value");
    println!("    // Use .unwrap_or(), .unwrap_or_else(), or ? instead.");
    println!();

    println!("  A2. .expect(msg) is better than .unwrap() but still panics:");
    println!("    file.read_to_string(&mut buf).expect(\"read failed\");");
    println!("    // Reserve .expect() for cases you KNOW cannot fail at runtime,");
    println!("    // or in tests/prototypes. Prefer ? in library code.");
    println!();

    println!("  A3. Swallowing errors with .ok() and ignoring the Option:");
    println!("    some_result.ok();  // discards the error silently — why?");
    println!("    // If you want to ignore: let _ = some_result; and leave a comment.");
    println!();

    println!("  A4. Using panic! for expected/recoverable failures:");
    println!("    fn parse_port(s: &str) -> u16 {{");
    println!("        let n: u16 = s.parse().unwrap();  // panics on bad input");
    println!("        n");
    println!("    }}");
    println!("    // Better: return Result<u16, ParseIntError>");
    println!();

    println!("  A5. Overusing Box<dyn Error> in library code:");
    println!("    // Box<dyn Error> erases the error type — callers cannot match variants.");
    println!("    // Use a typed enum for library APIs; Box<dyn Error> for app main/scripts.");
    println!();

    println!("  A6. Nesting match instead of using ? or combinators:");
    println!("    match outer() {{");
    println!("        Ok(x) => match inner(x) {{");
    println!("            Ok(y) => /* ... */,");
    println!("            Err(e) => return Err(e),");
    println!("        }},");
    println!("        Err(e) => return Err(e),");
    println!("    }}");
    println!("    // Better: let y = inner(outer()?)?;");
    println!();

    println!("  Summary:");
    println!("    Rust has NO exceptions — error handling is explicit, zero-cost,");
    println!("    and checked at compile time. Use Result for recoverable errors,");
    println!("    panic! for unrecoverable bugs. The ? operator keeps code concise");
    println!("    without hiding control flow. Custom error enums with Display +");
    println!("    std::error::Error provide structured, matchable, human-readable errors.");
}
