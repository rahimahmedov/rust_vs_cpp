// Rust Macros and Metaprogramming Demo
//
// Sections:
//   1 — macro_rules!: declarative macros (pattern matching on syntax)
//   2 — Built-in and standard library macros
//   3 — #[derive]: compiler-generated trait implementations
//   4 — const fn and const generics: compile-time computation
//   5 — Patterns and antipatterns

// ── Proc macro imports ────────────────────────────────────────────────────────
// proc_macros is a separate crate compiled as a shared library loaded by rustc.
// It must live in its own crate (proc-macro = true); it cannot be in the same
// crate as the code that uses it.
use proc_macros::{inspect, route, token_info, Describe};

// ── Module-level items used by section 6 ─────────────────────────────────────

// #[route] is an attribute proc macro.  rustc calls our route() function with:
//   attr = TokenStream for  "/api/users"
//   item = TokenStream for  fn handle_users() -> String { ... }
// The macro emits: original fn  +  PATH const  +  register_ fn
#[route("/api/users")]
fn handle_users() -> String { String::from("alice, bob, carol") }

#[route("/api/health")]
fn handle_health() -> String { String::from("ok") }

#[route("/api/version")]
fn handle_version() -> String { String::from("1.0.0") }

// #[derive(Describe)] is a derive proc macro.  rustc calls derive_describe()
// with the full struct TokenStream; the macro emits only the new impl block.
// The struct itself is always preserved automatically by the derive machinery.
#[derive(Debug, Describe)]
struct Server {
    host: String,
    port: u16,
}

#[derive(Debug, Describe)]
struct Endpoint {
    method: String,
    path:   String,
}

// ── Macros defined at module level (must appear before use) ──────────────────

// 1. Simple replacement
macro_rules! square {
    ($x:expr) => { $x * $x };
}

// 2. Multiple patterns (overloading by shape)
macro_rules! log {
    // one-arg variant — no context label
    ($msg:expr) => {
        println!("[LOG] {}", $msg);
    };
    // two-arg variant — label: message
    ($label:expr, $msg:expr) => {
        println!("[LOG] {}: {}", $label, $msg);
    };
    // variadic: label + format string + args
    ($label:expr, $fmt:literal, $($arg:expr),+) => {
        println!(concat!("[LOG] {}: ", $fmt), $label, $($arg),+);
    };
}

// 3. Repetition: collect any number of key=value pairs into a Vec<(&str, i32)>
macro_rules! kv_pairs {
    ( $( $k:expr => $v:expr ),* ) => {{
        vec![ $( ($k, $v) ),* ]
    }};
}

// 4. Recursive macro: sum of a list
macro_rules! sum {
    // base case
    ($x:expr) => { $x };
    // recursive case: first + rest
    ($x:expr, $($rest:expr),+) => { $x + sum!($($rest),+) };
}

// 5. tt-muncher: build a runtime Vec from a DSL-style list
macro_rules! build_vec {
    // accumulate into internal buffer, then emit
    (@inner [$($acc:expr),*]) => { vec![$($acc),*] };
    (@inner [$($acc:expr),*] $x:expr, $($rest:tt)*) => {
        build_vec!(@inner [$($acc,)* $x] $($rest)*)
    };
    (@inner [$($acc:expr),*] $x:expr) => {
        build_vec!(@inner [$($acc,)* $x])
    };
    ($($tt:tt)*) => { build_vec!(@inner [] $($tt)*) };
}

// 6. Macro generating functions
macro_rules! make_adder_fn {
    ($name:ident, $n:expr) => {
        fn $name(x: i32) -> i32 { x + $n }
    };
}
make_adder_fn!(add_ten,  10);
make_adder_fn!(add_hundred, 100);

// 7. assert_approx — domain-specific assertion
macro_rules! assert_approx {
    ($a:expr, $b:expr, $eps:expr) => {{
        let diff = ($a - $b).abs();
        assert!(
            diff < $eps,
            "assert_approx failed: |{} - {}| = {} >= {}",
            $a, $b, diff, $eps
        );
    }};
}

fn main() {
    section1_macro_rules();
    section2_builtin_macros();
    section3_derive();
    section4_const_generics();
    section5_patterns_antipatterns();
    section6_proc_macros();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — macro_rules!
// ─────────────────────────────────────────────────────────────────────────────

fn section1_macro_rules() {
    println!("=== Section 1: macro_rules! — Declarative Macros ===");
    println!("macro_rules! matches on SYNTAX (token trees), not values.");
    println!("  Expanded at compile time — zero runtime cost.");
    println!("  Hygienic: macro-internal variables don't leak into caller scope.");
    println!("  Unlike C macros: no text substitution, full token-tree awareness.");
    println!();

    println!("  1a. Simple expression macro:");
    println!("    square!(5)   = {}", square!(5));
    println!("    square!(2+3) = {}", square!(2 + 3));  // expands to (2+3)*(2+3) = 25
    println!();

    println!("  1b. Multiple patterns (overloaded by argument count):");
    log!("server started");
    log!("request", "GET /index");
    log!("response", "status={}, size={} bytes", 200, 1024);
    println!();

    println!("  1c. Repetition ($(...),*)  — variadic input:");
    let pairs = kv_pairs!["alpha" => 1, "beta" => 2, "gamma" => 3];
    for (k, v) in &pairs {
        println!("    {} => {}", k, v);
    }
    println!();

    println!("  1d. Recursive macro — sum!:");
    println!("    sum!(1)           = {}", sum!(1));
    println!("    sum!(1,2,3)       = {}", sum!(1, 2, 3));
    println!("    sum!(10,20,30,40) = {}", sum!(10, 20, 30, 40));
    println!();

    println!("  1e. tt-muncher — advanced internal accumulation:");
    let v = build_vec![10, 20, 30, 40, 50];
    println!("    build_vec![10,20,30,40,50] = {:?}", v);
    println!();

    println!("  1f. Code-generating macro — make_adder_fn!:");
    println!("    add_ten(5)     = {}", add_ten(5));
    println!("    add_hundred(5) = {}", add_hundred(5));
    println!("    (two functions generated by one macro invocation)");
    println!();

    println!("  1g. Domain-specific assertion macro:");
    let pi_approx: f64 = 355.0 / 113.0;  // classic rational approx, not the PI constant
    assert_approx!(pi_approx, std::f64::consts::PI, 0.001);
    println!("    assert_approx!(355/113, PI, 0.001) passed");
    println!();

    println!("  Metavariable fragment specifiers:");
    println!("    expr  — any expression");
    println!("    ident — identifier (variable/function name)");
    println!("    ty    — type");
    println!("    pat   — pattern");
    println!("    stmt  — statement");
    println!("    block — {{ }} block");
    println!("    tt    — single token tree (most flexible)");
    println!("    literal, lifetime, vis, path, item, meta");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Built-in and Standard Library Macros
// ─────────────────────────────────────────────────────────────────────────────

fn section2_builtin_macros() {
    println!("=== Section 2: Built-in and Standard Library Macros ===");
    println!("Rust's standard library is full of macros you use every day.");
    println!();

    // println! / format! / eprintln!
    println!("  2a. Formatting macros:");
    let s = format!("pi ≈ {:.4}", std::f64::consts::PI);
    println!("    format!(...)           = \"{}\"", s);
    let padded = format!("{:>10}", "right");
    println!("    format!({{:>10}})       = \"{}\"", padded);
    let hex = format!("0x{:08X}", 255_u32);
    println!("    format!(0x{{:08X}},255) = {}", hex);
    println!();

    // vec! / array construction
    println!("  2b. Collection macros:");
    let v: Vec<i32>        = vec![0; 5];
    let v2: Vec<i32>       = vec![1, 2, 3];
    println!("    vec![0; 5]  = {:?}", v);
    println!("    vec![1,2,3] = {:?}", v2);
    println!();

    // dbg! — prints file/line/value, returns the value
    println!("  2c. dbg! — debug printing that returns its argument:");
    let x = dbg!(2 + 3);      // prints [src/bin/macros_demo.rs:N] 2 + 3 = 5
    println!("    dbg!(2+3) returned: {}", x);
    println!();

    // assert! / assert_eq! / assert_ne!
    println!("  2d. Assertion macros:");
    let two = 1 + 1;
    let four = 2 * 2;
    assert!(two == 2);
    assert_eq!(four, 4, "math still works");
    assert_ne!(3_i32, 4);
    println!("    assert!, assert_eq!, assert_ne! all passed");
    println!();

    // todo! / unimplemented! / unreachable! — compile-time placeholders
    println!("  2e. Placeholder macros:");
    println!("    todo!()         — marks unfinished code, panics at runtime");
    println!("    unimplemented!()— marks intentionally missing impl");
    println!("    unreachable!()  — marks logically impossible branches");
    println!("    panic!(\"msg\")  — explicit unrecoverable error");
    println!();

    // include! / include_str! / include_bytes! — compile-time file embedding
    println!("  2f. Compile-time file embedding:");
    println!("    include_str!(\"file.txt\")   — embeds file as &'static str");
    println!("    include_bytes!(\"img.png\")  — embeds file as &'static [u8]");
    println!("    include!(\"generated.rs\")   — textually includes a Rust file");
    println!("    All resolved at compile time — no runtime I/O.");
    println!();

    // env! / option_env! — compile-time env vars
    println!("  2g. Compile-time environment:");
    let pkg = env!("CARGO_PKG_NAME");
    println!("    env!(\"CARGO_PKG_NAME\") = \"{}\"", pkg);
    println!("    option_env!(\"MY_VAR\")  = {:?}", option_env!("MY_VAR"));
    println!("    Baked into the binary at compile time — no std::env at runtime.");
    println!();

    // concat! / stringify!
    println!("  2h. Token-level macros:");
    let joined = concat!("hello", ", ", "world", "!");
    println!("    concat!(\"hello\",\", \",\"world\",\"!\") = \"{}\"", joined);
    macro_rules! show_expr {
        ($e:expr) => { println!("    stringify!({}) = \"{}\"", stringify!($e), stringify!($e)) };
    }
    show_expr!(1 + 2 * 3);
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — #[derive]: Compiler-Generated Trait Implementations
// ─────────────────────────────────────────────────────────────────────────────

// derive generates boilerplate implementations from the struct/enum definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// Default — generate a zero-value constructor
#[derive(Debug, Default)]
struct Config {
    width:  u32,
    height: u32,
    title:  String,
    debug:  bool,
}

fn section3_derive() {
    println!("=== Section 3: #[derive] — Procedural Macro Code Generation ===");
    println!("#[derive(...)] is a procedural macro that inspects the AST and");
    println!("  emits trait impl blocks. Zero runtime cost — pure code generation.");
    println!("  Available in stable std: Debug, Clone, Copy, PartialEq, Eq,");
    println!("  PartialOrd, Ord, Hash, Default, From (via From derive).");
    println!();

    println!("  3a. Debug — automatic pretty-printer:");
    let p = Point { x: 3, y: 4 };
    println!("    {:?}",  p);
    println!("    {:#?}", p);   // pretty-print
    println!();

    println!("  3b. Clone — deep copy:");
    let p2 = p.clone();
    println!("    original: {:?}", p);
    println!("    clone:    {:?}", p2);
    println!("    same?     {}", p == p2);   // PartialEq derived
    println!();

    println!("  3c. PartialOrd + Ord — lexicographic comparison:");
    let mut points = vec![
        Point { x: 2, y: 5 },
        Point { x: 1, y: 9 },
        Point { x: 2, y: 1 },
    ];
    points.sort();   // uses derived Ord (x first, then y)
    println!("    sorted points: {:?}", points);
    println!();

    println!("  3d. Default — zero-value construction:");
    let cfg = Config::default();
    println!("    Config::default() = {:?}", cfg);
    println!("    width={} height={} debug={}", cfg.width, cfg.height, cfg.debug);
    let custom = Config { width: 1920, height: 1080, ..Config::default() };
    println!("    struct update: width={} height={} title=\"{}\"",
             custom.width, custom.height, custom.title);
    println!();

    println!("  3e. Enum derive:");
    let colors = [Color::Red, Color::Green, Color::Blue, Color::Custom(255, 128, 0)];
    for c in &colors {
        println!("    {:?}", c);
    }
    let first = Color::Red;
    println!("    Red == Red?   {}", first == Color::Red);
    println!("    Red == Green? {}", first == Color::Green);
    println!();

    println!("  3f. What derive generates (conceptually) for PartialEq on Point:");
    println!("    impl PartialEq for Point {{");
    println!("        fn eq(&self, other: &Self) -> bool {{");
    println!("            self.x == other.x && self.y == other.y");
    println!("        }}");
    println!("    }}");
    println!("    Generated by inspecting field names and types at compile time.");
    println!();

    println!("  3g. External derive crates (ecosystem, not std):");
    println!("    serde::Serialize / Deserialize — JSON/TOML/etc serialisation");
    println!("    thiserror::Error               — structured error types");
    println!("    clap::Parser                   — CLI argument parsing");
    println!("    These require proc-macro crates; follow the same pattern.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — const fn and const Generics
// ─────────────────────────────────────────────────────────────────────────────

// const fn: evaluated at compile time when called in a const context
const fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// Const generic: array length is a compile-time parameter
fn array_sum<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

// A stack-allocated buffer whose capacity is a const generic
#[derive(Debug)]
struct FixedStack<T, const CAP: usize> {
    data: [Option<T>; CAP],
    top:  usize,
}

impl<T: Copy + std::fmt::Debug, const CAP: usize> FixedStack<T, CAP> {
    const fn new() -> Self {
        Self { data: [None; CAP], top: 0 }
    }
    fn push(&mut self, val: T) -> bool {
        if self.top >= CAP { return false; }
        self.data[self.top] = Some(val);
        self.top += 1;
        true
    }
    fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        self.data[self.top].take()
    }
}

// Trait with an associated const
trait Describe {
    const KIND: &'static str;
    fn describe(&self) -> String;
}
struct Meters(f64);
struct Kilograms(f64);
impl Describe for Meters {
    const KIND: &'static str = "length";
    fn describe(&self) -> String { format!("{} m  [{}]", self.0, Self::KIND) }
}
impl Describe for Kilograms {
    const KIND: &'static str = "mass";
    fn describe(&self) -> String { format!("{} kg [{}]", self.0, Self::KIND) }
}

fn section4_const_generics() {
    println!("=== Section 4: const fn and const Generics ===");
    println!("const fn: runs at compile time in const contexts (values, array sizes, ...).");
    println!("  const generics: types parameterised by compile-time integer values.");
    println!("  Both enable zero-cost abstraction with compile-time guarantees.");
    println!();

    // const fn called at compile time
    println!("  4a. const fn — compile-time evaluation:");
    const F5:  u64 = factorial(5);
    const F10: u64 = factorial(10);
    const G:   u64 = gcd(48, 18);
    println!("    factorial(5)  = {} (computed at compile time)", F5);
    println!("    factorial(10) = {} (computed at compile time)", F10);
    println!("    gcd(48, 18)   = {} (computed at compile time)", G);
    // Also usable at runtime:
    println!("    factorial(7) at runtime = {}", factorial(7));
    println!();

    // const as array size
    println!("  4b. const value as array size:");
    const BUF_SIZE: usize = factorial(3) as usize;  // = 6, compile-time
    let buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    println!("    BUF_SIZE = factorial(3) = {}  →  [u8; {}]", BUF_SIZE, buf.len());
    println!();

    // const generic function
    println!("  4c. Const generic function — N baked into the type:");
    let a3 = [10, 20, 30];
    let a5 = [1, 2, 3, 4, 5];
    println!("    array_sum([10,20,30])  = {}", array_sum(&a3));
    println!("    array_sum([1,2,3,4,5]) = {}", array_sum(&a5));
    // array_sum(&[1,2,3,4][..])  // would be a compile error: slice, not array
    println!();

    // FixedStack — CAP in the type, stack-allocated, no heap
    println!("  4d. FixedStack<T, CAP> — capacity in the type:");
    let mut stack: FixedStack<i32, 4> = FixedStack::new();
    println!("    push 1,2,3,4: {},{},{},{}",
        stack.push(1), stack.push(2), stack.push(3), stack.push(4));
    println!("    push 5 (overflow): {}", stack.push(5));
    println!("    pop: {:?}, {:?}", stack.pop(), stack.pop());
    println!();

    // Associated const in trait
    println!("  4e. Associated constants in traits:");
    let d = Meters(9.81);
    let m = Kilograms(70.0);
    println!("    Meters::KIND    = \"{}\"", Meters::KIND);
    println!("    Kilograms::KIND = \"{}\"", Kilograms::KIND);
    println!("    d.describe() = {}", d.describe());
    println!("    m.describe() = {}", m.describe());
    println!();

    println!("  4f. Compile-time vs runtime summary:");
    println!("    const X: T = expr      — must be const fn or literals");
    println!("    const fn f() -> T      — usable in both const and runtime contexts");
    println!("    struct Foo<const N: usize> — N must be known at call site");
    println!("    static LOOKUP: [u8; N] — zero-cost table embedded in binary");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Patterns and Antipatterns
// ─────────────────────────────────────────────────────────────────────────────

fn section5_patterns_antipatterns() {
    println!("=== Section 5: Patterns and Antipatterns ===");
    println!();

    println!("  ── PATTERNS (good) ──────────────────────────────────────────────");
    println!();

    println!("  P1. Use macros only when a function cannot do the job:");
    println!("    Macros are justified for: variadic args, syntactic extension,");
    println!("    code generation across multiple items, domain-specific languages.");
    println!("    If a generic function suffices — prefer the function.");
    println!();

    println!("  P2. Use #[derive] instead of manual boilerplate:");
    println!("    #[derive(Debug, Clone, PartialEq)]  // correct");
    println!("    impl Debug for Foo {{ ... }}         // only when derive is wrong");
    println!();

    println!("  P3. Document what a macro expands to:");
    println!("    // Expands to: vec![(k1,v1),(k2,v2),...]");
    println!("    macro_rules! kv_pairs {{ ... }}");
    println!("    Future maintainers can't step into macro expansions easily.");
    println!();

    println!("  P4. Use internal rules (@inner) to avoid name collisions:");
    println!("    macro_rules! my_macro {{");
    println!("        (@step ...) => {{ ... }};   // internal helper");
    println!("        ($($x:tt)*) => {{ my_macro!(@step ...) }};");
    println!("    }}");
    println!();

    println!("  P5. Prefer const fn over macros for computation:");
    println!("    const fn max_val(a: i32, b: i32) -> i32 {{ if a > b {{ a }} else {{ b }} }}");
    println!("    const MAX: i32 = max_val(10, 20);  // clear, type-safe, debuggable");
    println!();

    println!("  P6. Use const generics to encode array sizes in types:");
    println!("    fn dot<const N: usize>(a: &[f64;N], b: &[f64;N]) -> f64");
    println!("    Mismatched sizes are a compile error, not a runtime panic.");
    println!();

    println!("  ── ANTIPATTERNS (avoid) ─────────────────────────────────────────");
    println!();

    println!("  A1. Macro that should be a function:");
    println!("    macro_rules! double {{ ($x:expr) => {{ $x * 2 }} }}");
    println!("    // Better: fn double(x: i32) -> i32 {{ x * 2 }}");
    println!("    Functions are typed, testable, and show up in stack traces.");
    println!();

    println!("  A2. Evaluating expr arguments multiple times:");
    println!("    macro_rules! bad_max {{ ($a:expr,$b:expr) => {{ if $a > $b {{ $a }} else {{ $b }} }} }}");
    println!("    bad_max!(expensive(), expensive())  // expensive() called TWICE");
    println!("    Fix: bind to a let inside the macro expansion.");
    println!();

    println!("  A3. Leaking implementation details via non-hygienic ident naming:");
    println!("    Rust macros are hygienic — internal bindings don't clash with caller.");
    println!("    Never rely on generated names being accessible from outside.");
    println!();

    println!("  A4. Deep macro recursion causing long compile times:");
    println!("    Recursive macro_rules! is O(N) macro-expansion steps.");
    println!("    For large N, prefer const fn loops or build scripts.");
    println!();

    println!("  A5. Using macro_rules! to fake generics:");
    println!("    // Bad: macro that duplicates code for i32 and f64");
    println!("    // Good: generic fn or trait with blanket impl");
    println!();

    println!("  Summary:");
    println!("    Rust metaprogramming is layered:");
    println!("    1. const fn + const generics — zero-cost compile-time values");
    println!("    2. macro_rules! — syntax-level, hygienic, declarative");
    println!("    3. proc macros (#[derive], attribute, function-like)");
    println!("       — full AST access via the syn/quote crates (separate crate needed)");
    println!("    Unlike C++ TMP: no Turing-complete template tricks required.");
    println!("    Unlike C macros: hygienic, token-tree aware, no text substitution.");
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 6 — Procedural Macros and TokenStream
// ─────────────────────────────────────────────────────────────────────────────

fn section6_proc_macros() {
    println!("=== Section 6: Procedural Macros and TokenStream ===");
    println!("A proc macro is a compiler plugin: rustc calls your Rust function");
    println!("  at compile time, passing TokenStream in and expecting TokenStream out.");
    println!("  Three kinds:");
    println!("    #[proc_macro]           — function-like:  name!(...)");
    println!("    #[proc_macro_attribute] — attribute:      #[name(args)] fn/struct/...");
    println!("    #[proc_macro_derive]    — derive:         #[derive(Name)]");
    println!("  Proc macro crates must set  [lib] proc-macro = true  in Cargo.toml.");
    println!("  They cannot be in the same crate as the code that uses them.");
    println!();

    // ── 6a. What is a TokenStream? ────────────────────────────────────────────
    println!("  6a. TokenStream — what the compiler sees:");
    println!("  A TokenStream is an iterator of TokenTree variants:");
    println!("    Ident(x)        — identifier: a word that is not a keyword/literal");
    println!("    Literal(42)     — number, string, char, bool, byte literal");
    println!("    Punct(+)        — punctuation: + - * / : ; , . ! = < > & | ...");
    println!("    Group({{...}})   — a {{}} [] () delimited sub-stream");
    println!();

    // token_info!(expr) is our function-like proc macro.
    // It receives the tokens of the expression and returns a string literal
    // describing the token tree structure.  The expression is NOT evaluated.
    println!("  token_info!(expr) — inspect source tokens at compile time:");
    println!("    token_info!(a + b * 2)     = {}", token_info!(a + b * 2));
    println!("    token_info!(x.len())       = {}", token_info!(x.len()));
    println!("    token_info!(vec![1, 2, 3]) = {}", token_info!(vec![1, 2, 3]));
    println!("    token_info!(if c {{ 1 }} else {{ 0 }}) = {}",
             token_info!(if c { 1 } else { 0 }));
    println!();
    println!("  Note: 'Joint' Punct means no whitespace before the next token");
    println!("  (e.g. '::' is two Punct(: Joint) tokens, not Punct(: Alone)).");
    println!();

    // ── 6b. inspect! — function-like proc macro ───────────────────────────────
    println!("  6b. inspect!(expr) — evaluate + print source text + value:");
    let a = 6_i32;
    let b = 7_i32;
    let product = inspect!(a * b);
    println!("    returned value: {}", product);

    let name = String::from("world");
    let greeting = inspect!(format!("Hello, {name}!"));
    println!("    returned value: \"{}\"", greeting);
    println!();
    println!("  Expansion of  inspect!(a * b):");
    println!("    {{ let _v = {{ a * b }}; println!(\"  inspect!(a * b) = {{}}\", _v); _v }}");
    println!("  The original token stream  a * b  is inserted verbatim inside {{ }}.");
    println!("  Single evaluation — no double-call risk.");
    println!();

    // ── 6c. #[route] — attribute proc macro ──────────────────────────────────
    println!("  6c. #[route(\"/path\")] — attribute proc macro:");
    println!("  Defined at module level (attr macros annotate items, not expressions).");
    println!();
    println!("  Source code (what the programmer writes):");
    println!("    #[route(\"/api/users\")]");
    println!("    fn handle_users() -> String {{ String::from(\"alice, bob, carol\") }}");
    println!();
    println!("  What rustc sees after expansion:");
    println!("    fn handle_users() -> String {{ ... }}   // original, unchanged");
    println!("    pub const HANDLE_USERS_PATH: &str = \"/api/users\";");
    println!("    pub fn register_handle_users() {{ ... }}");
    println!();
    println!("  Generated PATH constants:");
    println!("    HANDLE_USERS_PATH  = \"{}\"", HANDLE_USERS_PATH);
    println!("    HANDLE_HEALTH_PATH = \"{}\"", HANDLE_HEALTH_PATH);
    println!("    HANDLE_VERSION_PATH = \"{}\"", HANDLE_VERSION_PATH);
    println!();
    println!("  Calling the generated registrar functions:");
    register_handle_users();
    register_handle_health();
    register_handle_version();
    println!();
    println!("  Calling the original handler functions (unchanged):");
    println!("    handle_users()   = \"{}\"", handle_users());
    println!("    handle_health()  = \"{}\"", handle_health());
    println!("    handle_version() = \"{}\"", handle_version());
    println!();

    // ── 6d. #[derive(Describe)] — derive proc macro ────────────────────────────
    println!("  6d. #[derive(Describe)] — derive proc macro:");
    println!("  Defined at module level on Server and Endpoint structs.");
    println!();
    println!("  Source code (what the programmer writes):");
    println!("    #[derive(Debug, Describe)]");
    println!("    struct Server {{ host: String, port: u16 }}");
    println!();
    println!("  What rustc sees after expansion (in addition to the struct):");
    println!("    impl Server {{");
    println!("        pub fn describe(&self) -> String {{");
    println!("            format!(\"Server {{ host: {{}}, port: {{}} }}\", self.host, self.port)");
    println!("        }}");
    println!("    }}");
    println!();
    let srv = Server { host: String::from("localhost"), port: 8080 };
    let ep  = Endpoint { method: String::from("GET"), path: String::from("/api") };
    println!("  Runtime output:");
    println!("    srv.describe() = \"{}\"", srv.describe());
    println!("    ep.describe()  = \"{}\"", ep.describe());
    println!();

    // ── 6e. How the proc_macros crate works ───────────────────────────────────
    println!("  6e. How proc_macros/src/lib.rs works (no syn/quote — pure proc_macro):");
    println!();
    println!("  TokenStream manipulation without syn:");
    println!("    ts.into_iter()         — iterate token trees");
    println!("    TokenTree::Ident(id)   — match an identifier, id.to_string()");
    println!("    TokenTree::Literal(l)  — match a literal,    l.to_string()");
    println!("    TokenTree::Punct(p)    — match punctuation,  p.as_char()");
    println!("    TokenTree::Group(g)    — match {{/(/[,         g.stream()");
    println!("    ts.extend(other_ts)    — append tokens to an existing stream");
    println!("    TokenStream::from_str(\"code\") — parse raw Rust source into tokens");
    println!();
    println!("  In real projects: use  syn  for structured AST parsing,");
    println!("    quote::quote!{{ ... }}  for ergonomic token generation.");
    println!("  This demo uses raw proc_macro only to make every step visible.");
    println!();
    println!("  Flow summary:");
    println!("    Your code        →   rustc tokenises   →   TokenStream");
    println!("    TokenStream      →   proc macro fn     →   new TokenStream");
    println!("    new TokenStream  →   rustc compiles    →   object code");
    println!("  All happens at compile time — zero runtime overhead.");
}
