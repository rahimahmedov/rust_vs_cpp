// C++ Closures (Lambdas) Demo — C++17
// Mirrors the Rust closures_demo: same 5 sections, analogous patterns.
//
// Sections:
//   1 — Basic syntax
//   2 — Capture modes: by value, by reference, mutable, C++14 init-capture
//   3 — Lambda types: anonymous functors, std::function — NO Fn/FnMut/FnOnce
//   4 — Lambdas as arguments: template (static) vs std::function (dynamic)
//   5 — Returning lambdas and thread captures

#include <functional>
#include <iostream>
#include <memory>
#include <string>
#include <thread>
#include <vector>

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Basic Syntax
// ─────────────────────────────────────────────────────────────────────────────

static void section1_basic_syntax() {
    std::cout << "=== Section 1: Basic Syntax ===\n";
    std::cout << "C++ lambda:   [capture](params) -> rettype { body }\n";
    std::cout << "Rust closure: |params| expression  or  |params| { block }\n";
    std::cout << "Both compile to anonymous structs/objects under the hood.\n\n";

    auto add     = [](int x, int y) { return x + y; };      // return type inferred
    auto square  = [](int x) { return x * x; };
    auto greet   = [](const std::string& name) { return "Hello, " + name + "!"; };
    auto multi   = [](int x) -> int {                        // explicit return type
        int doubled = x * 2;
        return doubled + 1;
    };
    auto no_args = []() { return 42; };

    std::cout << "  add(3, 4)          = " << add(3, 4)           << "\n";
    std::cout << "  square(7)          = " << square(7)           << "\n";
    std::cout << "  greet(\"Alice\")     = " << greet("Alice")     << "\n";
    std::cout << "  multi(10)          = " << multi(10)           << "\n";
    std::cout << "  no_args()          = " << no_args()           << "\n";

    // Generic lambdas (C++14): `auto` parameters
    auto generic_add = [](auto x, auto y) { return x + y; };
    std::cout << "  generic_add(1, 2)        = " << generic_add(1, 2)         << "\n";
    std::cout << "  generic_add(1.5, 2.5)    = " << generic_add(1.5, 2.5)    << "\n";
    std::cout << "  generic_add(\"hi\", \"!\")  = " << generic_add(std::string("hi"), std::string("!")) << "\n";

    std::cout << "\n  Syntax differences from Rust:\n";
    std::cout << "    C++:  [](int x) { return x * x; }\n";
    std::cout << "    Rust: |x: i32| x * x\n";
    std::cout << "    C++: capture list [] is REQUIRED (even if empty)\n";
    std::cout << "    C++: `return` keyword needed in multi-statement body\n";
    std::cout << "    Rust: last expression is the return value automatically\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Capture Modes
// ─────────────────────────────────────────────────────────────────────────────

static void section2_capture_modes() {
    std::cout << "=== Section 2: Capture Modes ===\n";
    std::cout << "C++: explicit capture list — you specify exactly what and how to capture.\n";
    std::cout << "Rust: implicit — compiler infers the minimum capture needed.\n\n";

    // ── Capture by value [x] ─────────────────────────────────────────────────
    std::cout << "  [var] — capture by value (COPY made at lambda creation time):\n";
    std::string name = "Alice";
    auto greet_by_val = [name]() {
        std::cout << "    lambda sees: \"" << name << "\"\n";
        // Cannot modify `name` here (const copy) without `mutable`
    };
    name = "Bob";             // change original AFTER capture
    greet_by_val();           // prints "Alice" — lambda captured the copy at creation
    std::cout << "    original `name` now: \"" << name << "\"\n";
    std::cout << "    KEY: [name] captured a COPY at lambda definition — not a live view\n";
    std::cout << "    Rust default: borrows &name — always sees the current value\n\n";

    // ── Capture by reference [&x] ────────────────────────────────────────────
    std::cout << "  [&var] — capture by reference (live view, modifies original):\n";
    int count = 0;
    auto inc_ref = [&count]() { count++; };
    inc_ref(); inc_ref(); inc_ref();
    std::cout << "    count after 3 increments via [&count]: " << count << "\n";
    std::cout << "    Same semantics as Rust FnMut capturing &mut count\n\n";

    // ── mutable keyword: modify a by-value capture ───────────────────────────
    std::cout << "  [count] + mutable — captures a copy, mutates the COPY only:\n";
    int counter = 0;
    auto inc_copy = [counter]() mutable { counter++; return counter; };
    std::cout << "    inc_copy(): " << inc_copy() << "\n";
    std::cout << "    inc_copy(): " << inc_copy() << "\n";
    std::cout << "    inc_copy(): " << inc_copy() << "\n";
    std::cout << "    original counter = " << counter << "  ← UNCHANGED (lambda has its own copy)\n";
    std::cout << "    Rust has no equivalent ambiguity: move means own a copy,\n";
    std::cout << "    &mut means borrow the original. No `mutable` keyword confusion.\n\n";

    // ── Capture all by value [=] ─────────────────────────────────────────────
    std::cout << "  [=] — capture all locals by value:\n";
    int x = 10, y = 20;
    auto sum_all = [=]() { return x + y; };
    std::cout << "    sum_all() = " << sum_all() << "  (captured copies of x and y)\n\n";

    // ── Capture all by reference [&] ─────────────────────────────────────────
    std::cout << "  [&] — capture all locals by reference (DANGEROUS for async/threads):\n";
    int a = 5, b = 10;
    auto sum_ref = [&]() { return a + b; };
    a = 100;
    std::cout << "    sum_ref() after a=100: " << sum_ref() << "  (sees live value)\n";
    std::cout << "    DANGER: [&] in a thread or stored lambda = dangling reference risk\n\n";

    // ── C++14 init-capture: std::move into lambda ─────────────────────────────
    std::cout << "  C++14 init-capture — move data into lambda (Rust `move` equivalent):\n";
    std::vector<int> data = {1, 2, 3, 4, 5};
    // [data = std::move(data)] moves data INTO the lambda
    auto owns_data = [data = std::move(data)]() {
        int sum = 0;
        for (int v : data) sum += v;
        return sum;
    };
    std::cout << "    owns_data() = " << owns_data() << "\n";
    std::cout << "    data after std::move: size=" << data.size() << " (moved-from, unspecified)\n";
    std::cout << "    HOWEVER: C++ does NOT prevent you from using `data` after this!\n";
    std::cout << "    Rust `move`: accessing `data` after the closure is a COMPILE ERROR.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Lambda Types and std::function
// ─────────────────────────────────────────────────────────────────────────────

static void section3_lambda_types() {
    std::cout << "=== Section 3: Lambda Types — No Fn/FnMut/FnOnce ===\n";
    std::cout << "C++: lambdas are anonymous functor classes with operator().\n";
    std::cout << "     Two identical-looking lambdas have DIFFERENT types.\n";
    std::cout << "     No type-system distinction between 'callable once' vs 'many times'.\n";
    std::cout << "Rust: closures implement Fn / FnMut / FnOnce — enforced by compiler.\n\n";

    // Every lambda has a unique anonymous type — even identical-looking ones
    auto lam1 = [](int x) { return x * 2; };
    auto lam2 = [](int x) { return x * 2; };  // same body — different type!
    // decltype(lam1) != decltype(lam2) — they are distinct types
    std::cout << "  lam1(5) = " << lam1(5) << "\n";
    std::cout << "  lam2(5) = " << lam2(5) << "\n";
    std::cout << "  lam1 and lam2 have DIFFERENT types (both compile to unique structs)\n\n";

    // std::function: type-erased wrapper — stores any callable with matching signature
    std::cout << "  std::function<int(int)> — type-erased, stores any callable:\n";
    std::function<int(int)> f;
    f = [](int x) { return x * 2; };    std::cout << "    f([](x){{x*2}})(5) = " << f(5) << "\n";
    f = [](int x) { return x * x; };    std::cout << "    f([](x){{x*x}})(5) = " << f(5) << "\n";
    f = [](int x) { return x + 10; };   std::cout << "    f([](x){{x+10}})(5) = " << f(5) << "\n";

    std::cout << "\n  std::function OVERHEAD:\n";
    std::cout << "    - Type erasure via virtual dispatch (like Rust Box<dyn Fn>)\n";
    std::cout << "    - May heap-allocate if the closure is too large for SSO buffer\n";
    std::cout << "    - Cannot be inlined by the compiler\n";
    std::cout << "    Rust impl Fn (static) = zero overhead, inlineable.\n";
    std::cout << "    Rust Box<dyn Fn>      = same overhead as std::function.\n\n";

    // Demonstrating the lack of FnOnce semantics in C++
    std::cout << "  C++ CANNOT enforce 'callable only once' — Rust FnOnce does:\n";
    std::string message = "one-time message";
    // This lambda SHOULD only be called once (it moves message out),
    // but C++ has no type-system way to enforce this.
    auto one_time = [msg = std::move(message)]() mutable {
        std::string result = std::move(msg);  // 'consume' the message
        return result;
    };
    std::cout << "    first  call: \"" << one_time() << "\"\n";
    std::cout << "    second call: \"" << one_time() << "\"  ← empty! msg was consumed\n";
    std::cout << "    C++ allowed the second call — no compile error.\n";
    std::cout << "    Rust FnOnce: second call is a COMPILE ERROR.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Lambdas as Arguments
// ─────────────────────────────────────────────────────────────────────────────

// Template (static dispatch) — zero overhead, can be inlined
// Rust equivalent: fn apply<F: Fn(int) -> int>(f: F, ...)
template <typename F>
static std::vector<int> apply_template(const std::vector<int>& v, F f) {
    std::vector<int> result;
    result.reserve(v.size());
    for (int x : v) result.push_back(f(x));
    return result;
}

// std::function (dynamic dispatch) — type-erased, overhead
// Rust equivalent: fn apply_dynamic(f: &dyn Fn(i32) -> i32, ...)
static std::vector<int> apply_dynamic(
    const std::vector<int>& v, std::function<int(int)> f)
{
    std::vector<int> result;
    result.reserve(v.size());
    for (int x : v) result.push_back(f(x));
    return result;
}

static void print_vec(const std::string& label, const std::vector<int>& v) {
    std::cout << "    " << label << ": [";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "]\n";
}

static void section4_as_arguments() {
    std::cout << "=== Section 4: Lambdas as Arguments ===\n";
    std::cout << "Template (static):     zero-overhead, compiler knows the type.\n";
    std::cout << "std::function (dynamic): type-erased, flexible but slower.\n";
    std::cout << "Rust equivalent: impl Fn (static) vs &dyn Fn / Box<dyn Fn> (dynamic).\n\n";

    std::vector<int> nums = {1, 2, 3, 4, 5};

    // Template — static dispatch (preferred in C++)
    print_vec("template doubled", apply_template(nums, [](int x) { return x * 2; }));
    print_vec("template squared", apply_template(nums, [](int x) { return x * x; }));
    print_vec("template +10",     apply_template(nums, [](int x) { return x + 10; }));

    // std::function — dynamic dispatch (needed for heterogeneous storage)
    std::cout << "\n";
    std::vector<std::function<int(int)>> transforms = {
        [](int x) { return x * 2; },
        [](int x) { return x * x; },
        [](int x) { return x + 10; },
        [](int x) { return x - 1;  },
    };
    std::vector<std::string> names = {"*2", "^2", "+10", "-1"};
    for (size_t i = 0; i < transforms.size(); ++i) {
        print_vec("dynamic(" + names[i] + ")", apply_dynamic(nums, transforms[i]));
    }

    std::cout << "\n  KEY COMPARISON:\n";
    std::cout << "    C++: template<F> apply(v, F f) — static, zero cost\n";
    std::cout << "    Rust: fn apply<F: Fn(i32)->i32>(v: &[i32], f: F) — same\n";
    std::cout << "    C++: std::function<int(int)> — dynamic, type-erased\n";
    std::cout << "    Rust: &dyn Fn(i32)->i32 or Box<dyn Fn(i32)->i32> — same\n";
    std::cout << "    IMPORTANT: C++ std::function can heap-allocate large closures.\n";
    std::cout << "    Rust impl Fn is ALWAYS stack/inline — no hidden allocation.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Returning Lambdas and Thread Captures
// ─────────────────────────────────────────────────────────────────────────────

// Return a lambda by capturing a value (C++14 auto return)
auto make_adder(int n) {
    return [n](int x) { return x + n; };  // n captured by value — safe
}

// Return via std::function (needed when C++ can't deduce the return type in all cases)
std::function<int(int)> make_transform(const std::string& kind) {
    if (kind == "double") return [](int x) { return x * 2; };
    if (kind == "square") return [](int x) { return x * x; };
    if (kind == "negate") return [](int x) { return -x; };
    return [](int x) { return x; };
}

// Stateful closure returned by std::function
std::function<int()> make_counter() {
    int count = 0;
    return [count]() mutable { return ++count; };
}

static void section5_move_and_return() {
    std::cout << "=== Section 5: Returning Lambdas and Thread Captures ===\n\n";

    // ── Returning lambdas ─────────────────────────────────────────────────────
    std::cout << "  Returning lambdas:\n";
    auto add5  = make_adder(5);
    auto add10 = make_adder(10);
    std::cout << "    make_adder(5)(3)  = " << add5(3)  << "\n";
    std::cout << "    make_adder(10)(3) = " << add10(3) << "\n";

    auto counter = make_counter();
    std::cout << "    counter(): " << counter()
              << ", " << counter()
              << ", " << counter() << "\n";

    std::cout << "\n  Returning std::function by kind:\n";
    for (const auto& kind : {"double", "square", "negate", "unknown"}) {
        auto f = make_transform(kind);
        std::cout << "    make_transform(\"" << kind << "\")(4) = " << f(4) << "\n";
    }

    // ── Thread captures: value vs reference ──────────────────────────────────
    std::cout << "\n  Thread captures — the critical difference from Rust:\n";

    std::vector<int> data = {10, 20, 30, 40, 50};

    // Capture by value — safe (thread gets its own copy)
    std::thread t_copy([data]() {
        int sum = 0;
        for (int x : data) sum += x;
        std::cout << "    t_copy: sum = " << sum << "  (worked on a copy)\n";
    });
    t_copy.join();
    std::cout << "    original data still accessible after copy-capture: size=" << data.size() << "\n";

    // C++14 move-capture — data moved into thread (Rust `move` closure equivalent)
    std::thread t_move([data = std::move(data)]() {
        int sum = 0;
        for (int x : data) sum += x;
        std::cout << "    t_move: sum = " << sum << "  (owns moved data)\n";
    });
    t_move.join();
    // C++ does NOT prevent using `data` here — it's in a valid-but-unspecified state
    std::cout << "    data after std::move: size=" << data.size()
              << "  (unspecified — C++ lets you access it anyway!)\n";

    // Dangerous by-reference capture — DO NOT call this lambda past `dangerous_data`'s scope
    // (not demonstrated at runtime to avoid UB)
    std::cout << "\n  DANGEROUS pattern (not invoked — shown for education):\n";
    std::cout << "    std::vector<int> local = {1,2,3};\n";
    std::cout << "    std::thread t([&local]() { /* reads local */ });\n";
    std::cout << "    // If main() returns before t finishes: DANGLING REFERENCE, UB.\n";
    std::cout << "    // C++ COMPILES this without warning.\n";
    std::cout << "    // Rust: [&] closure cannot cross thread boundary.\n";
    std::cout << "    //       thread::spawn requires Send + 'static — rejects borrows.\n";
    std::cout << "    //       This bug is IMPOSSIBLE in safe Rust.\n\n";

    std::cout << "  Summary of key differences in captures:\n";
    std::cout << "    C++ [v]:            copy — original unchanged, lambda has its own\n";
    std::cout << "    C++ [&v]:           reference — live view, dangling-ref risk\n";
    std::cout << "    C++ [v=move(v)]:    move — v left in unspecified state (no compile guard)\n";
    std::cout << "    C++ [v] mutable:    mutable copy — confusingly named, mutates the copy only\n";
    std::cout << "    Rust (default):     minimal borrow — &v for reads, &mut v for writes\n";
    std::cout << "    Rust `move`:        owned — v moved in, accessing v after is compile error\n";
    std::cout << "    Rust `move` thread: required for spawn — compiler enforces it\n";
}

// ─────────────────────────────────────────────────────────────────────────────

int main() {
    section1_basic_syntax();
    section2_capture_modes();
    section3_lambda_types();
    section4_as_arguments();
    section5_move_and_return();
    return 0;
}
