// Rust Variable Lifetime Demo
//
// Sections:
//   1 — Basic lifetime: scope, drop order, and what "lifetime" means
//   2 — Lifetime annotations on functions (elision vs explicit)
//   3 — Extending lifetimes: moving to outer scope, 'static, Arc, Box::leak
//   4 — Structs that hold references (lifetime parameters)
//   5 — Why lifetimes are critical for the borrow checker

use std::sync::Arc;

fn main() {
    section1_basic_lifetime();
    section2_lifetime_annotations();
    section3_extending_lifetimes();
    section4_struct_lifetimes();
    section5_borrow_checker();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Basic Lifetime and Scope
// ─────────────────────────────────────────────────────────────────────────────

fn section1_basic_lifetime() {
    println!("=== Section 1: Basic Lifetime and Scope ===");
    println!("A variable's lifetime = the span of code for which it is valid.");
    println!("Lifetime starts at declaration; ends at the closing }} of its scope.");
    println!();

    let x: i32 = 10; // x's lifetime begins here
    println!("  x={} declared, addr={:p}", x, &x);

    {
        let y: i32 = 20; // y's lifetime begins here — nested scope
        println!("  y={} declared in inner scope, addr={:p}", y, &y);
        println!("  Both x and y are alive here");
    } // y is dropped here — its stack slot is reclaimed

    // y no longer accessible beyond this point
    println!("  Back in outer scope: x={} still alive, y no longer exists", x);
    println!();

    // A REFERENCE cannot outlive the value it refers to.
    // The borrow checker tracks and enforces this at compile time.
    {
        let owner = String::from("hello");
        let r = &owner; // r borrows owner; r's lifetime ⊆ owner's lifetime
        println!("  r refers to owner: \"{}\"", r);
        // r and owner both go out of scope here — fine
    }

    // Attempting to use a reference after its owner is dropped:
    /* COMPILE ERROR (uncomment to see):
        let dangling_ref;
        {
            let short_lived = String::from("temporary");
            dangling_ref = &short_lived;
        } // short_lived dropped here
        println!("{}", dangling_ref);
        // error[E0597]: `short_lived` does not live long enough
        // Rust refuses to compile this. C++ compiles it and causes UB.
    */

    println!("  [Reference lifetime must be a subset of the owner's lifetime]");
    println!("  [COMPILE ERROR if reference outlives owner — impossible in safe Rust]");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Lifetime Annotations on Functions
// ─────────────────────────────────────────────────────────────────────────────

// Lifetime ELISION: when there is exactly one input reference, Rust infers
// that the output lifetime equals the input's lifetime. No annotation needed.
fn first_word(s: &str) -> &str {
    // Compiler expands this to: fn first_word<'a>(s: &'a str) -> &'a str
    match s.find(' ') {
        Some(i) => &s[..i],
        None    => s,
    }
}

// Explicit lifetime annotation: the output is tied to BOTH inputs.
// 'a = the shorter (intersection) of x's and y's lifetimes.
// The caller cannot use the result beyond the point where either input expires.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn section2_lifetime_annotations() {
    println!("=== Section 2: Lifetime Annotations on Functions ===");
    println!("Annotations are NOT runtime information — they are compile-time constraints.");
    println!("They tell the borrow checker how input and output lifetimes relate.");
    println!();

    // Elision — single input reference, output lifetime inferred
    let sentence = String::from("hello world from Rust");
    let word = first_word(&sentence);
    println!("  first_word (elision):  \"{}\"  —  return tied to `sentence`", word);
    println!();

    // Explicit annotation — two inputs, result valid while BOTH are alive
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("  longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);
        // result used here — both s1 and s2 alive — OK
    } // s2 dropped here

    /* COMPILE ERROR — result used after s2 is dropped:
        let result_bad;
        {
            let s2 = String::from("xyz");
            result_bad = longest(s1.as_str(), s2.as_str());
        } // s2 dropped; result_bad's lifetime would exceed s2's
        println!("{}", result_bad);
        // error[E0597]: `s2` does not live long enough
        //
        // Even if longest() always returns s1 at runtime, the borrow checker
        // only reads the *signature* — not the body. The annotation says
        // result_bad lives at most as long as the shorter of s1 and s2.
    */

    println!("  [Annotation 'a = intersection of caller's lifetimes for x and y]");
    println!("  [Borrow checker only inspects the signature, never the body]");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Extending Lifetimes
// ─────────────────────────────────────────────────────────────────────────────

fn section3_extending_lifetimes() {
    println!("=== Section 3: Extending Lifetimes ===");
    println!("A REFERENCE cannot be stretched past its owner's scope.");
    println!("But you can extend the *owner's* lifetime using these patterns:");
    println!();

    // ── 3a: Move the owner to a larger scope ─────────────────────────────────
    println!("  3a. Move owner to an outer scope:");
    let name;
    {
        let temp = String::from("Alice");
        name = temp; // ownership moves OUT of the inner block
                     // 'temp' no longer exists; 'name' is the new owner
    } // inner block ends — but the String is still alive (owned by `name`)
    println!("  name=\"{}\" — moved to outer scope, outlives the inner block", name);
    println!();

    // ── 3b: 'static lifetime — lives for the whole program ───────────────────
    println!("  3b. 'static lifetime:");
    let greeting: &'static str = "Hello, world!"; // string literal in read-only binary
    static APP_VERSION: &str = "1.0.0";           // static variable, also 'static
    println!("  string literal:  \"{}\"  (type: &'static str)", greeting);
    println!("  static variable: \"{}\"  (lives for the entire program)", APP_VERSION);
    println!("  'static is the longest possible lifetime");
    println!();

    // ── 3c: Arc<T> — shared ownership; data lives until last Arc drops ────────
    println!("  3c. Arc<T> — extend lifetime via shared ownership:");
    let arc1 = Arc::new(String::from("shared data"));
    let arc2 = Arc::clone(&arc1); // second owner — data now has two owners
    {
        let arc3 = Arc::clone(&arc1); // third owner for the inner block
        println!("  Arc strong_count inside inner block: {} owners", Arc::strong_count(&arc1));
        println!("  arc3 reads: \"{}\"", arc3);
    } // arc3 dropped — count goes from 3 to 2; data still alive
    println!("  Arc strong_count after inner block: {} owners", Arc::strong_count(&arc1));
    println!("  arc2 still valid: \"{}\"", arc2);
    println!("  [Data freed only when the very last Arc owner is dropped]");
    println!();

    // ── 3d: Box::leak — intentionally escape to 'static ──────────────────────
    println!("  3d. Box::leak — deliberately produce a 'static reference:");
    let dynamic = Box::new(format!("built at runtime: {}", 42));
    let leaked: &'static str = Box::leak(dynamic); // heap allocation is never freed
    println!("  leaked: \"{}\"", leaked);
    println!("  [Use only for one-time program-wide initialisation; avoids Mutex<OnceLock>]");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Structs That Hold References
// ─────────────────────────────────────────────────────────────────────────────

// The lifetime parameter 'a says: "this struct may not outlive the
// data that `text` points into."
struct Excerpt<'a> {
    text: &'a str,
    line: usize,
}

impl<'a> Excerpt<'a> {
    // Elision: output lifetime = &self's lifetime
    fn content(&self) -> &str {
        self.text
    }

    // Explicit: output tied to 'a (the borrowed data), not to &self
    fn text_ref(&self) -> &'a str {
        self.text
    }
}

fn section4_struct_lifetimes() {
    println!("=== Section 4: Structs Holding References ===");
    println!("A struct containing &T must carry a lifetime parameter.");
    println!("This makes the constraint visible: struct cannot outlive the data.");
    println!();

    let document = String::from(
        "Rust guarantees memory safety. No dangling pointers. No data races.",
    );

    let end = document.find('.').unwrap_or(document.len());
    let first_sentence = &document[..end];

    let excerpt = Excerpt { text: first_sentence, line: 1 };
    println!("  Excerpt line {}: \"{}\"", excerpt.line, excerpt.content());
    println!("  text_ref: \"{}\"  (tied to `document`'s lifetime)", excerpt.text_ref());
    println!();

    // The compiler prevents Excerpt from outliving `document`:
    /* COMPILE ERROR (uncomment to see):
        let bad_excerpt;
        {
            let ephemeral = String::from("will be dropped");
            bad_excerpt = Excerpt { text: &ephemeral, line: 99 };
        } // ephemeral dropped here
        println!("{}", bad_excerpt.content()); // error: `ephemeral` does not live long enough
    */

    println!("  [Lifetime parameter 'a appears in the type: Excerpt<'a>]");
    println!("  [Without it the compiler would refuse to compile the struct definition]");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Why Lifetimes Are Critical for the Borrow Checker
// ─────────────────────────────────────────────────────────────────────────────

fn section5_borrow_checker() {
    println!("=== Section 5: Why Lifetimes Are Critical for the Borrow Checker ===");
    println!("The borrow checker uses lifetime information to rule out three classes");
    println!("of memory bugs that are silent undefined behaviour in C++:");
    println!("  (a) dangling references   (b) use-after-move   (c) iterator invalidation");
    println!();

    // ── 5a: Dangling reference — impossible in safe Rust ─────────────────────
    println!("  5a. Dangling reference — compile error in Rust, UB in C++:");

    /* COMPILE ERROR — fn dangle() -> &String:
        fn dangle() -> &String {
            let s = String::from("hello"); // s lives on this frame
            &s                             // ERROR: returning ref to local
        }                                  // s dropped here — reference would dangle
        // error[E0106]: missing lifetime specifier
        //
        // Rust asks: what lifetime would the returned &String have?
        // There is no input to tie it to, so it must be the local's lifetime — but
        // that ends when the function returns. Rust rejects this entirely.
    */

    // Correct fix: return the owned String
    fn no_dangle() -> String {
        let s = String::from("hello");
        s // ownership transferred to caller — nothing dangled
    }
    println!("  no_dangle() = \"{}\"  (ownership transferred, no reference)", no_dangle());
    println!();

    // ── 5b: Use-after-move — borrow checker tracks move as lifetime end ───────
    println!("  5b. Use-after-move — borrow checker treats move as lifetime end:");
    let original = String::from("important data");
    let moved = original; // original's lifetime ends here (ownership transferred)

    /* COMPILE ERROR:
        println!("{}", original);
        // error[E0382]: borrow of moved value: `original`
        // In C++: `original` would be in a valid-but-unspecified state — accessing
        // it is not UB per the standard, but the data is gone and the result is
        // unpredictable. Rust makes any access a compile error.
    */

    println!("  moved=\"{}\"  —  original is inaccessible (compile error if accessed)", moved);
    println!();

    // ── 5c: Iterator invalidation — mutation while borrowed ──────────────────
    println!("  5c. Iterator invalidation — Rust prevents mutation while borrowed:");
    let mut numbers = vec![1, 2, 3, 4, 5];
    let first = &numbers[0]; // immutable borrow starts here

    /* COMPILE ERROR — overlapping lifetimes:
        numbers.push(6);       // mutable borrow of `numbers`
        println!("{}", first); // ERROR: cannot borrow `numbers` as mutable because
                               //        it is also borrowed as immutable
        // In C++: push_back() may reallocate the vector's buffer, leaving `first`
        // pointing to freed memory. Classic iterator invalidation — silent UB.
        // Rust's borrow checker sees the lifetime overlap and refuses to compile.
    */

    println!("  first={} — numbers.push() forbidden while `first` borrow is active", first);
    // first's last use is the line above — its borrow ends here (NLL)
    numbers.push(6); // now safe: no active borrows of numbers
    println!("  numbers after push (borrow released): {:?}", numbers);
    println!();

    // ── 5d: Lifetime prevents use of invalidated slice ───────────────────────
    println!("  5d. String slice invalidated by mutation — caught at compile time:");
    let mut text = String::from("Hello world");
    let hello = &text[..5]; // slice borrows text; hello's lifetime ⊆ text's lifetime

    /* COMPILE ERROR:
        text.push_str("!!!");  // mutates text — potentially reallocates buffer
        println!("{}", hello); // ERROR: cannot borrow `text` as mutable because
                               //        it is also borrowed as immutable
        // The slice `hello` is a pointer into text's buffer.
        // If push_str reallocates, hello points to freed memory — UB in C++.
    */

    println!("  hello=\"{}\" — text.push_str() forbidden while slice is borrowed", hello);
    // hello's last use above — borrow ends
    text.push_str("!!!");
    println!("  text after push_str (borrow released): \"{}\"", text);
    println!();

    println!("  Summary — what lifetimes prevent:");
    println!("    Dangling reference       → reference lifetime > owner lifetime → ERROR");
    println!("    Use-after-move           → move ends the value's lifetime      → ERROR");
    println!("    Iterator invalidation    → mut borrow overlaps immut borrow    → ERROR");
    println!("    Slice after reallocation → same as iterator invalidation       → ERROR");
    println!("    All four are silent UB in C++ — Rust turns them into compile errors");
}
