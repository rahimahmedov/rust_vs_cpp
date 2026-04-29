// C++ Ownership & Borrowing Demo (C++17)
// Mirrors the Rust demo: same 5 sections, analogous patterns.
// Shows where C++ gives the same guarantees and where it diverges.

#include <iostream>
#include <memory>
#include <string>
#include <utility>

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Ownership and Move Semantics
// ─────────────────────────────────────────────────────────────────────────────

static void take_ownership(std::unique_ptr<std::string> p) {
    std::cout << "  function owns: \"" << *p << "\"\n";
} // p (and the string it owns) is destroyed here

static void section1_ownership_and_move() {
    std::cout << "=== Section 1: Ownership and Move Semantics ===\n";
    std::cout << "unique_ptr models exclusive ownership; std::move() transfers it.\n\n";

    // unique_ptr: only one pointer owns the heap object at any time
    auto p1 = std::make_unique<std::string>("hello");
    auto p2 = std::move(p1); // ownership transferred; p1 is now null

    // DANGEROUS (commented out — unlike Rust this is NOT a compile error):
    // std::cout << *p1;  // undefined behaviour: p1 is null after move
    //                    // Rust prevents this at compile time; C++ does not.

    std::cout << "  p2 owns: \"" << *p2 << "\"  (p1 is null — accessing it is UB)\n";

    // std::string move: s1 enters valid-but-unspecified state
    std::string s1 = "world";
    std::string s2 = std::move(s1); // move constructor — s1 becomes empty
    std::cout << "  s2=\"" << s2 << "\", s1=\"" << s1
              << "\"  (s1 valid-but-unspecified; Rust forbids any use of s1)\n";

    take_ownership(std::move(p2)); // p2 null after this line

    std::cout << "  p2 was moved into the function and freed there.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Copy Types vs Move Types
// ─────────────────────────────────────────────────────────────────────────────

// Move-only type: analogous to a Rust struct that does NOT implement Copy or Clone
struct MoveOnly {
    int value;
    explicit MoveOnly(int v) : value(v) {}
    MoveOnly(const MoveOnly&) = delete;            // no copy
    MoveOnly& operator=(const MoveOnly&) = delete; // no copy-assign
    MoveOnly(MoveOnly&&) = default;                // move OK
    MoveOnly& operator=(MoveOnly&&) = default;
};

static void section2_copy_vs_move() {
    std::cout << "=== Section 2: Copy Types vs Move Types ===\n";
    std::cout << "int/POD: copied by assignment (like Rust Copy types).\n";
    std::cout << "std::string: COPIED by default in C++ (unlike Rust String which moves).\n";
    std::cout << "Explicit std::move() required to trigger the move constructor.\n\n";

    // int: trivially copyable — both a and b are independent
    int a = 42, b = a;
    std::cout << "  [COPY] int: a=" << a << ", b=" << b
              << "  — both valid, a was copied\n";

    // std::string: copy by default (KEY DIFFERENCE from Rust)
    std::string s1 = "hello";
    std::string s2 = s1; // copy constructor — s1 still valid!
    std::cout << "  [COPY] std::string: s1=\"" << s1 << "\", s2=\"" << s2
              << "\"  — BOTH valid (C++ copies by default)\n";

    // Explicit move with std::move()
    std::string s3 = std::move(s1); // s1 now unspecified
    std::cout << "  [MOVE] after std::move: s3=\"" << s3
              << "\", s1=\"" << s1 << "\"  (s1 unspecified)\n";

    // Move-only type: enforces move semantics like a Rust non-Copy struct
    MoveOnly m1{99};
    MoveOnly m2 = std::move(m1); // forced move — copy is deleted
    // MoveOnly m3 = m2;  // COMPILE ERROR: copy constructor is deleted
    std::cout << "  [MOVE-ONLY struct] m2.value=" << m2.value
              << "  — copy deleted, only move allowed\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Shared References / const References
// ─────────────────────────────────────────────────────────────────────────────

static void print_length(const std::string& s) {
    // const ref: read-only borrow — does not copy or take ownership
    std::cout << "  borrowed length: " << s.size() << "\n";
} // ref expires; nothing freed

static void section3_shared_references() {
    std::cout << "=== Section 3: Shared References / Const References ===\n";
    std::cout << "const T&: read-only reference; any number may coexist.\n\n";

    std::string s = "borrow me";

    const std::string& r1 = s; // const ref — read-only view
    const std::string& r2 = s; // second const ref — fine
    std::cout << "  r1=\"" << r1 << "\", r2=\"" << r2
              << "\"  — two const refs coexist\n";

    print_length(s);

    std::cout << "  s still valid: \"" << s << "\"\n";
    std::cout << "  [Multiple const refs coexist — same rule as Rust &T]\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Mutable References / Non-const References
// ─────────────────────────────────────────────────────────────────────────────

static void append_exclamation(std::string& s) {
    s += "!"; // mutates through the reference
}

static void section4_mutable_references() {
    std::cout << "=== Section 4: Mutable References / Non-const References ===\n";
    std::cout << "T&: mutable reference. C++ imposes NO exclusivity rule.\n";
    std::cout << "Rust &mut T: exactly one at a time, never alongside &T.\n\n";

    std::string s = "hello";
    append_exclamation(s);
    std::cout << "  after mutation: \"" << s << "\"\n";

    // C++ allows multiple mutable references simultaneously (Rust forbids this)
    std::string& r1 = s;
    std::string& r2 = s; // second mutable ref to the same object — C++ allows it
    r1 += " via-r1";
    r2 += " via-r2";
    std::cout << "  after two mutable refs: \"" << s << "\"\n";
    std::cout << "  [C++ allows multiple T& to the same object]\n";
    std::cout << "  [Rust &mut T is EXCLUSIVE — only one at a time, enforced at compile time]\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — RAII and Destructors (deterministic destruction)
// ─────────────────────────────────────────────────────────────────────────────

struct Resource {
    std::string name;
    explicit Resource(std::string n) : name(std::move(n)) {
        std::cout << "  [ACQUIRE] Resource \"" << name << "\" created\n";
    }
    ~Resource() { // RAII: destructor = Rust's Drop trait
        std::cout << "  [DTOR]    Resource \"" << name << "\" freed\n";
    }
    // Non-copyable (unique ownership, like Rust)
    Resource(const Resource&) = delete;
    Resource& operator=(const Resource&) = delete;
};

static void section5_raii_and_drop() {
    std::cout << "=== Section 5: RAII / Destructors and Scope ===\n";
    std::cout << "Objects are destroyed deterministically at end of scope, LIFO order.\n";
    std::cout << "This is the pattern Rust's Drop trait is modeled on.\n\n";

    {
        Resource outer{"outer"};
        {
            Resource inner{"inner"};
            std::cout << "  -- inside inner scope --\n";
        } // inner.~Resource() called here
        std::cout << "  -- back in outer scope --\n";
    } // outer.~Resource() called here

    std::cout << "\n";

    // Explicit early release with unique_ptr::reset() — analogous to Rust drop()
    auto early = std::make_unique<Resource>("early");
    early.reset(); // destructor runs immediately — ownership released
    std::cout << "  early resource was already freed above\n\n";
    std::cout << "  [Destructors run deterministically — no GC, no finalizer uncertainty]\n";
}

// ─────────────────────────────────────────────────────────────────────────────

int main() {
    section1_ownership_and_move();
    section2_copy_vs_move();
    section3_shared_references();
    section4_mutable_references();
    section5_raii_and_drop();
    return 0;
}
