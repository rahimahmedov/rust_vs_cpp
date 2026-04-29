// C++ Variable Lifetime Demo (C++17)
// Mirrors the Rust lifetime_demo: same 5 sections, analogous patterns.
//
// Sections:
//   1 — Basic lifetime: automatic storage duration, construction/destruction order
//   2 — Reference rules in C++ (no annotations; show what the language does and doesn't check)
//   3 — Extending lifetimes: moving to outer scope, static, const-ref-to-temp, shared_ptr
//   4 — Structs holding references (no enforcement; shows the danger)
//   5 — Dangling pointers and references — the silent UB that Rust's lifetimes prevent

#include <iostream>
#include <memory>
#include <string>
#include <vector>

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Basic Lifetime and Scope
// ─────────────────────────────────────────────────────────────────────────────

struct Scoped {
    std::string name;
    explicit Scoped(std::string n) : name(std::move(n)) {
        std::cout << "    [CTOR] Scoped(\"" << name << "\") — lifetime begins\n";
    }
    ~Scoped() {
        std::cout << "    [DTOR] Scoped(\"" << name << "\") — lifetime ends\n";
    }
    Scoped(const Scoped&) = delete;
    Scoped& operator=(const Scoped&) = delete;
};

static void section1_basic_lifetime() {
    std::cout << "=== Section 1: Basic Lifetime and Scope ===\n";
    std::cout << "A variable's lifetime = from its declaration to the end of its enclosing {}.\n";
    std::cout << "C++ calls this 'automatic storage duration' (stack variables).\n";
    std::cout << "Destructors run in LIFO order — last constructed, first destroyed.\n\n";

    int x = 10;
    std::cout << "  x=" << x << " declared, addr=" << &x << "\n";

    {
        int y = 20;
        std::cout << "  y=" << y << " declared in inner scope, addr=" << &y << "\n";
        std::cout << "  Both x and y are alive here\n";
    } // y destroyed here — stack slot reclaimed

    std::cout << "  Back in outer scope: x=" << x << " still alive, y no longer exists\n\n";

    // RAII objects make the lifetime visible through constructor/destructor calls:
    std::cout << "  RAII lifetime demonstration:\n";
    {
        Scoped outer{"outer"};
        {
            Scoped inner{"inner"};
            std::cout << "    Both outer and inner alive\n";
        } // inner destroyed (LIFO)
        std::cout << "    Only outer alive now\n";
    } // outer destroyed

    std::cout << "\n  KEY DIFFERENCE from Rust:\n";
    std::cout << "  C++ does NOT track reference lifetimes at compile time.\n";
    std::cout << "  A reference/pointer to a destroyed object compiles fine — it is UB.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — C++ Reference Rules (No Lifetime Annotations)
// ─────────────────────────────────────────────────────────────────────────────

// C++ has no lifetime annotations. The programmer is responsible.
// The compiler does NOT verify that a reference outlives what it refers to.

static const std::string& longer_string(const std::string& a, const std::string& b) {
    // Equivalent to Rust's fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    // But: C++ does NOT encode lifetime relationships in the type system.
    // The caller must ensure both a and b outlive the returned reference.
    return a.size() >= b.size() ? a : b;
}

static void section2_reference_rules() {
    std::cout << "=== Section 2: C++ Reference Rules — No Lifetime Annotations ===\n";
    std::cout << "C++ references must be initialized and cannot be rebound.\n";
    std::cout << "But the compiler does NOT check that a reference outlives its referent.\n\n";

    // Valid usage: both strings outlive the reference
    std::string s1 = "long string is long";
    std::string s2 = "xyz";
    const std::string& result = longer_string(s1, s2);
    std::cout << "  longer_string result: \"" << result << "\"\n\n";

    // C++ ALLOWS this (compiles), but it is dangerous:
    // Returning a reference to a local variable — the function is defined below
    // in the dangling section (5) and not called here to avoid UB.
    std::cout << "  Contrast with Rust:\n";
    std::cout << "    Rust fn longest<'a>(...) -> &'a str  encodes the lifetime contract.\n";
    std::cout << "    The borrow checker verifies callers respect it.\n";
    std::cout << "    C++ has no such contract — the programmer must be careful.\n\n";

    // References cannot be rebound (unlike pointers)
    int a = 1, b = 2;
    int& ra = a;
    // ra = b would ASSIGN b's value to a — not rebind ra to b
    ra = b; // a is now 2; ra still refers to a
    std::cout << "  After ra = b:  a=" << a << "  b=" << b
              << "  ra=" << ra << "  (ra still refers to a — assignment, not rebind)\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Extending Lifetimes
// ─────────────────────────────────────────────────────────────────────────────

static void section3_extending_lifetimes() {
    std::cout << "=== Section 3: Extending Lifetimes ===\n\n";

    // ── 3a: Move to outer scope ───────────────────────────────────────────────
    std::cout << "  3a. Move value to outer scope (same idea as Rust):\n";
    std::string name;
    {
        std::string temp = "Alice";
        name = std::move(temp); // temp's data moved to name; temp is now empty
    } // temp destroyed (empty); name owns the data and outlives the block
    std::cout << "  name=\"" << name << "\" — moved to outer scope, still alive\n\n";

    // ── 3b: Static local — lives for the entire program ──────────────────────
    std::cout << "  3b. static local variable — 'static lifetime equivalent:\n";
    static const std::string GREETING = "Hello from static storage";
    static int call_count = 0;
    ++call_count;
    std::cout << "  static GREETING: \"" << GREETING << "\"\n";
    std::cout << "  static call_count: " << call_count
              << "  (initialised once, lives until program exit)\n\n";

    // ── 3c: const T& binding to a temporary extends the temporary's lifetime ──
    std::cout << "  3c. const T& binding to temporary — C++11 lifetime extension rule:\n";
    {
        // Normally a temporary lives only to the end of the full-expression.
        // Binding it to a const reference extends it to the reference's scope.
        const std::string& ref_to_temp = std::string("I am a temporary");
        std::cout << "  ref_to_temp: \"" << ref_to_temp
                  << "\"  (temporary lives as long as ref_to_temp)\n";
        // ref_to_temp goes out of scope here — temporary destroyed here too
    }
    std::cout << "  [Non-const T& cannot bind to a temporary — const-only rule]\n\n";

    // ── 3d: shared_ptr — shared ownership; data lives until last owner drops ──
    std::cout << "  3d. std::shared_ptr — shared ownership (Rust Arc<T> equivalent):\n";
    auto sp1 = std::make_shared<std::string>("shared data");
    auto sp2 = sp1; // second owner
    {
        auto sp3 = sp1; // third owner
        std::cout << "  use_count inside inner block: " << sp1.use_count() << "\n";
        std::cout << "  sp3: \"" << *sp3 << "\"\n";
    } // sp3 destroyed; count → 2
    std::cout << "  use_count after inner block: " << sp1.use_count() << "\n";
    std::cout << "  sp2: \"" << *sp2 << "\"  (data still alive)\n";
    std::cout << "  [Data freed only when the last shared_ptr owner is destroyed]\n\n";

    // ── 3e: Heap allocation — data outlives the scope that allocated it ───────
    std::cout << "  3e. Heap allocation — data outlives its allocating scope:\n";
    std::unique_ptr<std::string> heap_str;
    {
        heap_str = std::make_unique<std::string>("heap-allocated, outlives this block");
    } // block ends, but heap_str (unique_ptr) moved out — data still alive
    std::cout << "  heap_str: \"" << *heap_str << "\"\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Structs Holding References
// ─────────────────────────────────────────────────────────────────────────────

// C++ struct holding a reference — NO lifetime enforcement.
// The programmer must ensure the struct never outlives the referred-to object.
struct Excerpt {
    const std::string& text; // reference member — dangerous if misused
    int line;

    Excerpt(const std::string& t, int l) : text(t), line(l) {}

    const std::string& content() const { return text; }
};

static void section4_struct_lifetimes() {
    std::cout << "=== Section 4: Structs Holding References ===\n";
    std::cout << "C++ allows structs with reference members — but provides NO safety net.\n";
    std::cout << "Rust requires a lifetime parameter 'a and enforces it at compile time.\n\n";

    std::string document = "C++ trusts the programmer. Rust trusts the compiler.";
    size_t dot = document.find('.');
    std::string first_sentence = document.substr(0, dot);

    Excerpt excerpt{first_sentence, 1};
    std::cout << "  Excerpt line " << excerpt.line
              << ": \"" << excerpt.content() << "\"\n\n";

    // THE DANGER C++ does NOT prevent (shown in comments to avoid UB):
    /*
        Excerpt* bad_excerpt = nullptr;
        {
            std::string ephemeral = "temporary text";
            bad_excerpt = new Excerpt{ephemeral, 99}; // reference to ephemeral
        } // ephemeral destroyed here — bad_excerpt->text is now dangling
        std::cout << bad_excerpt->content(); // UB: reads freed memory
        // May print garbage, crash, or appear to work — unpredictable.
        //
        // Rust equivalent: COMPILE ERROR
        //   let bad_excerpt: Excerpt;
        //   { let e = String::from("..."); bad_excerpt = Excerpt { text: &e, line: 99 }; }
        //   // error[E0597]: `e` does not live long enough
    */

    std::cout << "  The dangerous pattern (commented out to avoid UB):\n";
    std::cout << "    Excerpt holds a ref to a local string that goes out of scope.\n";
    std::cout << "    C++: compiles, runs, produces undefined behaviour.\n";
    std::cout << "    Rust: compile error — Excerpt<'a> cannot outlive its 'a.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Dangling Pointers and References (UB in C++, Errors in Rust)
// ─────────────────────────────────────────────────────────────────────────────

// These functions demonstrate dangerous patterns.
// They are DEFINED but NOT CALLED to avoid triggering UB at runtime.
// In Rust, the equivalent code does not compile at all.

// Suppress warnings for intentionally-dangerous educational functions.
// These are never called — they exist only to show what C++ permits that Rust refuses.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-stack-address"

[[maybe_unused]]
static int* dangling_pointer_factory() {
    int local = 42;
    return &local;
    // local destroyed on return — returned pointer is immediately dangling
    // Rust equivalent: fn dangle() -> &i32 { let x = 42; &x }
    //   error[E0106]: missing lifetime specifier — Rust refuses to compile this
}

[[maybe_unused]]
static std::string& dangling_ref_factory() {
    std::string s = "hello";
    return s;
    // s destroyed on return — returned reference is immediately dangling
}

#pragma clang diagnostic pop

static void section5_dangling_and_ub() {
    std::cout << "=== Section 5: Dangling References — C++ UB vs Rust Compile Errors ===\n";
    std::cout << "C++ compiles all four patterns below. Rust rejects all four at compile time.\n\n";

    // ── 5a: Dangling pointer from function ────────────────────────────────────
    std::cout << "  5a. Function returning pointer to local variable:\n";
    std::cout << "    int* p = dangling_pointer_factory();  // compiles; p is dangling\n";
    std::cout << "    *p = ...  // UB: reads/writes freed stack memory\n";
    std::cout << "    Rust equivalent: fn dangle() -> &i32 { &local }\n";
    std::cout << "    Rust error: missing lifetime specifier / does not live long enough\n\n";

    // ── 5b: Use-after-move ────────────────────────────────────────────────────
    std::cout << "  5b. Use-after-move — C++ allows it, Rust forbids it:\n";
    std::string original = "important data";
    std::string moved    = std::move(original); // original is valid-but-unspecified
    std::cout << "    moved=\"" << moved << "\"\n";
    std::cout << "    original=\"" << original
              << "\"  (valid-but-unspecified in C++ — may be empty or anything)\n";
    std::cout << "    In C++: accessing `original` after std::move is allowed (not UB per se),\n";
    std::cout << "            but the value is unspecified — practically useless and error-prone.\n";
    std::cout << "    In Rust: accessing the moved-from variable is a COMPILE ERROR.\n\n";

    // ── 5c: Iterator invalidation ─────────────────────────────────────────────
    std::cout << "  5c. Iterator invalidation — vector reallocation:\n";
    std::vector<int> v = {1, 2, 3};
    const int* first_ptr = v.data(); // raw pointer into vector's buffer
    std::cout << "    first_ptr points to v[0]=" << *first_ptr << " @ " << first_ptr << "\n";
    v.push_back(4);
    v.push_back(5);
    v.push_back(6); // may reallocate — first_ptr may now point to freed memory
    std::cout << "    v.data() after pushes @ " << v.data() << "\n";
    if (first_ptr != v.data()) {
        std::cout << "    Buffer was REALLOCATED — first_ptr is NOW DANGLING (UB to dereference)\n";
    } else {
        std::cout << "    Buffer was NOT reallocated this run (but cannot be relied upon)\n";
    }
    std::cout << "    Rust: compiler rejects `v.push_back()` while `first` borrow is active.\n\n";

    // ── 5d: String slice invalidation ────────────────────────────────────────
    std::cout << "  5d. String slice invalidated by mutation:\n";
    std::string text = "Hello world";
    const char* hello_ptr = text.data(); // pointer into text's buffer
    std::cout << "    hello_ptr -> \"Hello\" (first 5 chars) @ " << (void*)hello_ptr << "\n";
    text.append(" with a very long suffix that forces reallocation in most implementations");
    std::cout << "    text.data() after append @ " << (void*)text.data() << "\n";
    if (hello_ptr != text.data()) {
        std::cout << "    Buffer reallocated — hello_ptr is DANGLING (reading it is UB)\n";
    } else {
        std::cout << "    Buffer not reallocated this run (but the pointer is logically stale)\n";
    }
    std::cout << "    Rust: error — cannot mutate `text` while `hello` slice borrow is active.\n\n";

    std::cout << "  Summary — Four bugs C++ misses, Rust catches at compile time:\n";
    std::cout << "    Dangling return ref/ptr   C++ compiles  →  Rust: lifetime error\n";
    std::cout << "    Use-after-move            C++ compiles  →  Rust: moved value error\n";
    std::cout << "    Iterator invalidation     C++ compiles  →  Rust: borrow conflict\n";
    std::cout << "    Slice after reallocation  C++ compiles  →  Rust: borrow conflict\n";
    std::cout << "    All four are silent, context-dependent, hard-to-reproduce bugs.\n";
    std::cout << "    Rust's lifetime system eliminates the entire category.\n";
}

// ─────────────────────────────────────────────────────────────────────────────

int main() {
    section1_basic_lifetime();
    section2_reference_rules();
    section3_extending_lifetimes();
    section4_struct_lifetimes();
    section5_dangling_and_ub();
    return 0;
}
