// Rust Ownership & Borrowing Demo
// Each section demonstrates a core concept; intentional compile errors
// are shown inside block comments so the program still compiles and runs.

fn main() {
    section1_ownership_and_move();
    section2_copy_vs_move();
    section3_shared_references();
    section4_mutable_references();
    section5_raii_and_drop();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Ownership and Move Semantics
// ─────────────────────────────────────────────────────────────────────────────

fn take_ownership(s: String) {
    println!("  function owns: \"{}\"", s);
} // s is dropped here — heap memory freed

fn section1_ownership_and_move() {
    println!("=== Section 1: Ownership and Move Semantics ===");
    println!("Rule: every value has exactly one owner; assigning moves the value.");
    println!();

    // String is heap-allocated and non-Copy: assignment moves ownership.
    let s1 = String::from("hello");
    let s2 = s1; // ownership transferred from s1 to s2

    /* COMPILE ERROR (uncomment to see):
        println!("{}", s1);
        // error[E0382]: borrow of moved value: `s1`
        // s1 no longer owns the data — it was moved to s2.
    */

    println!("  s2 owns the string: \"{}\"  (s1 is no longer valid)", s2);

    // Moving into a function: function receives ownership, drops value on return
    take_ownership(s2);
    // s2 is no longer valid here either

    /* COMPILE ERROR (uncomment to see):
        println!("{}", s2);
        // error[E0382]: borrow of moved value: `s2`
    */

    println!("  s2 was moved into the function and freed there.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Copy Types vs Move Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn section2_copy_vs_move() {
    println!("=== Section 2: Copy Types vs Move Types ===");
    println!("Copy types (primitives, small structs): assignment duplicates bits.");
    println!("Move types (String, Vec, Box…): assignment transfers ownership.");
    println!();

    // i32 implements Copy: both a and b are valid after assignment
    let a: i32 = 42;
    let b = a; // bitwise copy — a is still valid
    println!("  [COPY] i32: a={}, b={}  — both valid, a was copied", a, b);

    // String does NOT implement Copy: assignment moves
    let s1 = String::from("move me");
    let s2 = s1; // s1 moved to s2
    println!("  [MOVE] String: s2=\"{}\"  — s1 was moved, it no longer exists", s2);

    /* COMPILE ERROR (uncomment to see):
        println!("{}", s1);
        // error[E0382]: borrow of moved value: `s1`
    */

    // Explicit deep copy with .clone()
    let original = String::from("original");
    let cloned = original.clone(); // two independent heap allocations
    println!(
        "  [CLONE] original=\"{}\", cloned=\"{}\"  — independent copies",
        original, cloned
    );

    // Custom struct: Clone but not Copy (must call .clone() explicitly)
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();
    println!(
        "  [CLONE struct] p1=({}, {}), p2=({}, {})  — explicit clone needed",
        p1.x, p1.y, p2.x, p2.y
    );
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Shared References (immutable borrows)
// ─────────────────────────────────────────────────────────────────────────────

fn print_length(s: &str) {
    // borrows s — does NOT take ownership
    println!("  borrowed length: {}", s.len());
} // borrow ends here; nothing is dropped

fn section3_shared_references() {
    println!("=== Section 3: Shared References / Immutable Borrows ===");
    println!("&T: any number of read-only references may coexist.");
    println!("The owner retains ownership; no data can be mutated through &T.");
    println!();

    let s = String::from("borrow me");

    let r1 = &s; // shared (immutable) borrow
    let r2 = &s; // second shared borrow — perfectly fine
    println!("  r1=\"{}\", r2=\"{}\"  — two borrows coexist", r1, r2);

    print_length(&s); // function borrows, does not own

    println!("  s still valid after borrows: \"{}\"", s);
    println!("  [Multiple &T borrows coexist — no mutation possible through them]");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Mutable References (exclusive borrows)
// ─────────────────────────────────────────────────────────────────────────────

fn append_exclamation(s: &mut String) {
    s.push_str("!"); // mutates through the mutable borrow
}

fn section4_mutable_references() {
    println!("=== Section 4: Mutable References / Exclusive Borrows ===");
    println!("&mut T: exactly ONE mutable reference at a time, with no &T active.");
    println!("This rule is enforced at compile time — no data races possible.");
    println!();

    let mut s = String::from("hello");

    append_exclamation(&mut s); // mutable borrow inside function
    println!("  after mutation: \"{}\"", s);

    // Demonstrate exclusivity via sequential (non-overlapping) borrows
    let r_immut = &s;
    println!("  immutable borrow active: \"{}\"", r_immut);
    // r_immut's last use is above — borrow ends here (Non-Lexical Lifetimes)

    let r_mut = &mut s;
    r_mut.push_str(" world");
    println!("  after second mutation: \"{}\"", r_mut);

    /* COMPILE ERROR (uncomment to see overlapping borrows):
        let r1 = &s;
        let r2 = &mut s;
        println!("{} {}", r1, r2);
        // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    */

    println!("  [Only one &mut at a time — and never alongside a &T]");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — RAII and Drop (deterministic destruction)
// ─────────────────────────────────────────────────────────────────────────────

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("  [ACQUIRE] Resource \"{}\" created", name);
        Resource { name: name.to_string() }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  [DROP]    Resource \"{}\" freed", self.name);
    }
}

fn section5_raii_and_drop() {
    println!("=== Section 5: RAII / Drop and Scope ===");
    println!("Values are dropped (freed) deterministically at end of scope, LIFO order.");
    println!();

    {
        let _outer = Resource::new("outer");
        {
            let _inner = Resource::new("inner");
            println!("  -- inside inner scope --");
        } // _inner dropped here
        println!("  -- back in outer scope --");
    } // _outer dropped here

    println!();

    // Explicit early drop (std::mem::drop)
    let early = Resource::new("early");
    drop(early); // ownership consumed — destructor runs immediately
    println!("  early resource was already freed above");
    println!();
    println!("  [Drop is guaranteed, deterministic, and always runs — even on panic]");
}
