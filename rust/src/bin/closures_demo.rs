// Rust Closures Demo
//
// Sections:
//   1 — Basic syntax: closures vs named functions
//   2 — Capture modes: by reference, by mutable reference, by move
//   3 — Fn / FnMut / FnOnce — Rust's three closure traits (NO C++ equivalent)
//   4 — Closures as arguments: static dispatch (impl Fn) vs dynamic (dyn Fn)
//   5 — Returning closures and move closures for threads

fn main() {
    section1_basic_syntax();
    section2_capture_modes();
    section3_fn_traits();
    section4_as_arguments();
    section5_move_and_return();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Basic Syntax
// ─────────────────────────────────────────────────────────────────────────────

fn section1_basic_syntax() {
    println!("=== Section 1: Basic Syntax ===");
    println!("Rust closure: |params| expression  or  |params| {{ block }}");
    println!("C++ lambda:   [capture](params) {{ body }}");
    println!("Both compile to anonymous structs/objects under the hood.");
    println!();

    // Types are almost always inferred from usage context
    let add      = |x: i32, y: i32| x + y;          // single expression
    let square   = |x: i32| x * x;
    let greet    = |name: &str| format!("Hello, {}!", name);
    let multi    = |x: i32| {                          // block body: last expr is returned
        let doubled = x * 2;
        doubled + 1      // no semicolon = return value (like regular fn)
    };
    let no_args  = || 42_i32;                          // zero parameters

    println!("  add(3, 4)          = {}", add(3, 4));
    println!("  square(7)          = {}", square(7));
    println!("  greet(\"Alice\")     = {}", greet("Alice"));
    println!("  multi(10)          = {}", multi(10));
    println!("  no_args()          = {}", no_args());

    // Closures can reference named functions and other closures
    fn double(x: i32) -> i32 { x * 2 }
    let apply = |f: fn(i32) -> i32, x: i32| f(x);
    println!("  apply(double, 5)   = {}", apply(double, 5));

    println!();
    println!("  Syntax differences from C++:");
    println!("    Rust:  |x: i32| x * x");
    println!("    C++:   [](int x) {{ return x * x; }}");
    println!("    Rust closure params: pipes |...|  (not parentheses)");
    println!("    Rust: last expression is the return value (no `return` needed)");
    println!("    Rust: type annotations optional (almost always inferred)");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Capture Modes
// ─────────────────────────────────────────────────────────────────────────────

fn section2_capture_modes() {
    println!("=== Section 2: Capture Modes ===");
    println!("Rust infers the MINIMUM capture needed (borrow checker enforces correctness).");
    println!("C++ requires explicit capture lists: [=] [&] [var] [&var].");
    println!();

    // ── 2a: Capture by shared reference (Rust default for reads) ─────────────
    println!("  2a. Capture by shared borrow (read-only, Rust default):");
    let greeting = String::from("Hello");
    let say_hi   = || println!("    closure reads greeting: \"{}\"", greeting); // &greeting
    say_hi();
    say_hi(); // multiple calls fine — shared borrow
    println!("    original still accessible: \"{}\"", greeting); // owner untouched

    println!();
    println!("    C++ equivalent: [greeting] or [&greeting] (must specify explicitly)");
    println!("    C++ [greeting]: copies the string into the lambda");
    println!("    C++ [&greeting]: references the string (same as Rust default)");
    println!("    KEY DIFFERENCE: Rust BORROWS by default (no copy); C++ [=] COPIES");

    // ── 2b: Capture by mutable reference ─────────────────────────────────────
    println!();
    println!("  2b. Capture by mutable borrow (FnMut closure):");
    let mut count = 0_i32;
    let mut inc   = || { count += 1; }; // borrows &mut count
    inc(); inc(); inc();
    drop(inc); // release the &mut borrow so we can use count again
    println!("    count after 3 increments: {}", count); // count IS 3 — same variable

    println!();
    println!("    C++ equivalent: [&count]() {{ count++; }}");
    println!("    Semantically similar (both modify the original).");
    println!("    BUT: C++ [count]() mutable {{ count++; }} captures a COPY.");
    println!("         Mutating the copy does NOT affect the original!");
    println!("    Rust has no such ambiguity: &mut = modifies original, move = owns a copy.");

    // ── 2c: Move capture ──────────────────────────────────────────────────────
    println!();
    println!("  2c. Move capture — closure OWNS the value (no shared borrow):");
    let data = vec![1, 2, 3, 4, 5];
    let show  = move || println!("    closure owns data: {:?}", data); // data MOVED in
    show();
    show(); // still callable — closure owns data, didn't consume it
    // println!("{:?}", data); // COMPILE ERROR: data was moved into the closure
    println!("    (data is no longer accessible outside the closure)");

    println!();
    println!("    C++ C++14 equivalent: [data = std::move(data)]() {{ /* owns data */ }}");
    println!("    C++17 has init-captures for this purpose.");
    println!("    KEY DIFFERENCE: Rust `move` is explicit and borrow-checked.");
    println!("                    C++ gives no compile error if you use `data` after.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Fn / FnMut / FnOnce Traits
// ─────────────────────────────────────────────────────────────────────────────

// Rust closures implement one or more of three traits based on HOW they capture:
//
//   FnOnce  — can be called at most ONCE (moves a captured value out)
//             Every closure implements FnOnce.
//   FnMut   — can be called multiple times, mutates captured state
//             Every FnMut also implements FnOnce.
//   Fn      — can be called any number of times, only borrows (immutably)
//             Every Fn also implements FnMut and FnOnce.
//
//   Hierarchy:   Fn ⊂ FnMut ⊂ FnOnce   (Fn is most restrictive)
//
//   C++ HAS NO EQUIVALENT. All C++ lambdas are callable objects with operator().
//   The callee cannot express "I will call this only once" in the type system.

fn call_once<F: FnOnce() -> String>(f: F) -> String {
    let result = f();
    // f(); // COMPILE ERROR: f was consumed by the first call
    result
}

fn call_fn_mut<F: FnMut(i32) -> i32>(mut f: F, values: &[i32]) -> Vec<i32> {
    values.iter().map(|&x| f(x)).collect()
}

fn call_fn<F: Fn(i32) -> i32>(f: &F, values: &[i32]) -> Vec<i32> {
    // &F because we're only borrowing — f can be shared across calls
    values.iter().map(|&x| f(x)).collect()
}

fn section3_fn_traits() {
    println!("=== Section 3: Fn / FnMut / FnOnce Traits ===");
    println!("RUST ONLY — C++ has no equivalent type-system encoding of call semantics.");
    println!("These traits determine how many times and in what context a closure can be used.");
    println!();

    // ── FnOnce: takes ownership of a captured value ───────────────────────────
    println!("  FnOnce — closure that can be called AT MOST ONCE:");
    let message = String::from("consumed message");
    let consume = move || {
        // `message` is moved OUT of the closure here — closure can only do this once
        format!("got: {}", message)
    };
    let result = call_once(consume);
    println!("    result: \"{}\"", result);
    // call_once(consume); // COMPILE ERROR: `consume` was moved into call_once
    println!("    [consume cannot be called again — ownership was transferred]");
    println!("    C++ cannot express this constraint; programmer must enforce it manually.");

    // ── FnMut: mutates captured state each call ───────────────────────────────
    println!();
    println!("  FnMut — closure that mutates captured state, callable many times:");
    let mut call_count = 0_usize;
    let counter = |x: i32| {
        call_count += 1; // mutates captured variable — requires FnMut
        x * call_count as i32
    };
    let results = call_fn_mut(counter, &[10, 20, 30]);
    println!("    results (each multiplied by call number): {:?}", results);
    drop(results); // just to be explicit
    println!("    call_count was captured by &mut — its final value is in the closure");
    println!("    C++ [&call_count] lambda achieves the same, but no type enforcement.");

    // ── Fn: immutable borrow, freely shareable ────────────────────────────────
    println!();
    println!("  Fn — closure with immutable borrows, shareable and re-entrant:");
    let multiplier = 3_i32;
    let triple = |x: i32| x * multiplier; // borrows &multiplier — Fn
    let r1 = call_fn(&triple, &[1, 2, 3]);
    let r2 = call_fn(&triple, &[4, 5, 6]); // can pass the SAME closure again
    println!("    triple([1,2,3]) = {:?}", r1);
    println!("    triple([4,5,6]) = {:?}", r2);
    println!("    [Fn closures can be passed by &ref and called concurrently]");

    println!();
    println!("  Hierarchy summary:");
    println!("    Fn  ⊂  FnMut  ⊂  FnOnce");
    println!("    Every Fn is also FnMut; every FnMut is also FnOnce.");
    println!("    A function accepting FnOnce is the most general (accepts all closures).");
    println!("    A function accepting Fn is the most restrictive (only pure closures).");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Closures as Arguments
// ─────────────────────────────────────────────────────────────────────────────

// Static dispatch: the compiler monomorphizes apply() for each concrete closure type.
// Zero overhead — the closure body can be inlined. Rust default.
fn apply_static<F: Fn(i32) -> i32>(values: &[i32], f: F) -> Vec<i32> {
    values.iter().map(|&x| f(x)).collect()
}

// Dynamic dispatch: a vtable is used at runtime.
// Required when the concrete closure type is not known at compile time,
// or when you need to store closures of different types in the same collection.
fn apply_dynamic(values: &[i32], f: &dyn Fn(i32) -> i32) -> Vec<i32> {
    values.iter().map(|&x| f(x)).collect()
}

// Storing closures of different types in a Vec — requires Box<dyn Fn>
fn section4_as_arguments() {
    println!("=== Section 4: Closures as Arguments ===");
    println!("Static dispatch: impl Fn / <F: Fn> — monomorphized, zero overhead.");
    println!("Dynamic dispatch: &dyn Fn / Box<dyn Fn> — vtable, needed for mixed types.");
    println!("C++ equivalent: template<F> (static) vs std::function<> (dynamic).");
    println!();

    let nums = vec![1, 2, 3, 4, 5];

    // Static dispatch — preferred when all closures are known at compile time
    let doubled  = apply_static(&nums, |x| x * 2);
    let squared  = apply_static(&nums, |x| x * x);
    let plus_ten = apply_static(&nums, |x| x + 10);
    println!("  apply_static: doubled  = {:?}", doubled);
    println!("  apply_static: squared  = {:?}", squared);
    println!("  apply_static: plus_ten = {:?}", plus_ten);

    // Dynamic dispatch — needed for heterogeneous collections of closures
    println!();
    let transforms: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
        Box::new(|x| x + 10),
        Box::new(|x| x - 1),
    ];
    let names = ["*2", "^2", "+10", "-1"];
    for (f, name) in transforms.iter().zip(names.iter()) {
        let result = apply_dynamic(&nums, f.as_ref());
        println!("  apply_dynamic({}): {:?}", name, result);
    }

    println!();
    println!("  C++ parallel:");
    println!("    template<typename F> auto apply(const std::vector<int>& v, F f)");
    println!("    → static dispatch (monomorphized, same as Rust impl Fn)");
    println!("    std::function<int(int)> f = [](int x) {{ return x*2; }};");
    println!("    → type-erased dynamic dispatch, std::function has heap-alloc overhead");
    println!("    KEY DIFFERENCE: Rust `impl Fn` is zero-cost; C++ std::function is not.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Move Closures and Returning Closures
// ─────────────────────────────────────────────────────────────────────────────

// Returning a closure: use `impl Fn` for static dispatch.
// The `move` is mandatory — the closure must OWN the captured value
// because the captured variable is otherwise dropped when the function returns.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // `n` is moved into the closure; closure owns it
}

fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0_i32;
    move || {
        count += 1;
        count
    }
}

// Returning a boxed closure for dynamic dispatch (when the concrete type must be hidden)
fn make_transform(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|x| x * 2),
        "square" => Box::new(|x| x * x),
        "negate" => Box::new(|x| -x),
        _        => Box::new(|x| x),
    }
}

fn section5_move_and_return() {
    println!("=== Section 5: Move Closures and Returning Closures ===");
    println!();

    // ── Returning closures ────────────────────────────────────────────────────
    println!("  Returning closures via impl Fn (static dispatch):");
    let add5  = make_adder(5);
    let add10 = make_adder(10);
    println!("    make_adder(5)(3)  = {}", add5(3));
    println!("    make_adder(10)(3) = {}", add10(3));

    let mut counter = make_counter();
    println!("    counter(): {}, {}, {}", counter(), counter(), counter());

    println!();
    println!("  Returning Box<dyn Fn> (dynamic dispatch by kind):");
    for kind in &["double", "square", "negate", "unknown"] {
        let f = make_transform(kind);
        println!("    make_transform(\"{}\") applied to 4 = {}", kind, f(4));
    }

    // ── Move closures for threads ─────────────────────────────────────────────
    println!();
    println!("  Move closures for multi-threading:");
    let shared_data = vec![10, 20, 30, 40, 50];

    // `move` is REQUIRED here — the thread may outlive the current stack frame.
    // Without `move`, the closure borrows `shared_data`, but the borrow cannot
    // cross a thread boundary (the thread could outlive `shared_data`'s scope).
    // Rust's type system enforces this: closures passed to thread::spawn must be
    // `Send + 'static`, which a borrow never satisfies.
    let handle = std::thread::spawn(move || {
        let sum: i32 = shared_data.iter().sum();
        println!("    thread: sum of moved data = {}", sum);
        // shared_data is fully owned by this thread now
    });
    handle.join().unwrap();
    // println!("{:?}", shared_data); // COMPILE ERROR: moved into thread
    println!("    [shared_data was moved into the thread — original is inaccessible]");

    println!();
    println!("  C++ thread equivalent:");
    println!("    std::thread t([data]() {{ /* owns a COPY */ }});");
    println!("    std::thread t([data = std::move(data)]() {{ /* owns via move */ }});");
    println!("    KEY DIFFERENCE: C++ does NOT prevent using `data` after thread launch.");
    println!("                    Rust enforces `move` semantics at compile time.");
    println!("                    C++ [&data] to a thread is a dangling-ref bug that");
    println!("                    compiles silently and crashes at runtime.");
}
