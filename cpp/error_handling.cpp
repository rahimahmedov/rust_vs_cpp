// C++ Error Handling Demo
//
// Sections:
//   1 — try / catch / throw basics
//   2 — Exception hierarchy and std::exception
//   3 — RAII and exception safety
//   4 — Custom exception classes and noexcept
//   5 — Patterns and antipatterns (+ std::optional)
//
// C++17; compiled with: clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2

#include <cassert>
#include <functional>
#include <iostream>
#include <memory>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

static void section1_try_catch_throw();
static void section2_exception_hierarchy();
static void section3_raii_exception_safety();
static void section4_custom_exceptions_noexcept();
static void section5_patterns_antipatterns();

int main() {
    section1_try_catch_throw();
    section2_exception_hierarchy();
    section3_raii_exception_safety();
    section4_custom_exceptions_noexcept();
    section5_patterns_antipatterns();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — try / catch / throw
// ─────────────────────────────────────────────────────────────────────────────

static double divide(double a, double b) {
    if (b == 0.0)
        throw std::invalid_argument("division by zero");
    return a / b;
}

static int parse_int(const std::string& s) {
    std::size_t pos{};
    int result = std::stoi(s, &pos);        // throws std::invalid_argument or
    if (pos != s.size())                    // std::out_of_range on failure
        throw std::invalid_argument("trailing characters in: " + s);
    return result;
}

static void section1_try_catch_throw() {
    std::cout << "=== Section 1: try / catch / throw ===\n";
    std::cout << "C++ uses exceptions for error signalling:\n";
    std::cout << "  throw <expr>   — unwinds the stack searching for a handler\n";
    std::cout << "  try { }        — marks a guarded region\n";
    std::cout << "  catch(T& e) { }— handler for exception of type T (or subclass)\n";
    std::cout << "  Unlike Rust, the compiler does NOT force you to handle errors.\n";
    std::cout << "\n";

    // Basic catch
    std::cout << "  1a. Basic try/catch:\n";
    try {
        double r = divide(10.0, 3.0);
        std::cout << "    10 / 3  = " << r << "  (ok)\n";
        divide(5.0, 0.0);                   // throws
        std::cout << "    (this line is unreachable)\n";
    } catch (const std::invalid_argument& e) {
        std::cout << "    caught: " << e.what() << "\n";
    }
    std::cout << "\n";

    // Catching by reference vs value
    std::cout << "  1b. Catch by REFERENCE (correct) vs by VALUE (slices):\n";
    std::cout << "    catch(const std::exception& e)  // correct — no slicing\n";
    std::cout << "    catch(std::exception e)          // wrong  — copies, loses subtype\n";
    std::cout << "\n";

    // Multiple catch blocks
    std::cout << "  1c. Multiple catch blocks — most-derived first:\n";
    auto try_parse = [](const std::string& s) {
        try {
            int n = parse_int(s);
            std::cout << "    parse(\"" << s << "\") = " << n << "\n";
        } catch (const std::out_of_range&) {
            std::cout << "    parse(\"" << s << "\") → out_of_range\n";
        } catch (const std::invalid_argument& e) {
            std::cout << "    parse(\"" << s << "\") → invalid_argument: " << e.what() << "\n";
        }
    };
    try_parse("42");
    try_parse("abc");
    try_parse("99999999999999");
    std::cout << "\n";

    // Catch-all: use sparingly
    std::cout << "  1d. Catch-all catch(...):\n";
    try {
        throw std::runtime_error("something unexpected");
    } catch (...) {
        std::cout << "    caught unknown exception (type information is lost!)\n";
        std::cout << "    use catch(...) only as a last-resort safety net\n";
    }
    std::cout << "\n";

    // Re-throwing
    std::cout << "  1e. Re-throwing with bare throw;:\n";
    try {
        try {
            throw std::runtime_error("original error");
        } catch (const std::exception&) {
            std::cout << "    inner: logging, then re-throwing\n";
            throw;  // preserves original exception type and message
        }
    } catch (const std::exception& e) {
        std::cout << "    outer: caught re-thrown: " << e.what() << "\n";
    }
    std::cout << "\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Exception Hierarchy
// ─────────────────────────────────────────────────────────────────────────────

static void section2_exception_hierarchy() {
    std::cout << "=== Section 2: Exception Hierarchy ===\n";
    std::cout << "std::exception is the conventional base for all C++ exceptions.\n";
    std::cout << "\n";

    std::cout << "  std::exception\n";
    std::cout << "  ├─ std::logic_error       — violations of logical preconditions\n";
    std::cout << "  │  ├─ std::invalid_argument\n";
    std::cout << "  │  ├─ std::domain_error\n";
    std::cout << "  │  ├─ std::length_error\n";
    std::cout << "  │  └─ std::out_of_range\n";
    std::cout << "  └─ std::runtime_error     — errors detectable only at runtime\n";
    std::cout << "     ├─ std::range_error\n";
    std::cout << "     ├─ std::overflow_error\n";
    std::cout << "     └─ std::underflow_error\n";
    std::cout << "\n";

    std::cout << "  2a. Catching at base-class level:\n";
    auto demo = [](auto fn_) {
        try { fn_(); }
        catch (const std::logic_error& e)   { std::cout << "    [logic_error]   " << e.what() << "\n"; }
        catch (const std::runtime_error& e) { std::cout << "    [runtime_error] " << e.what() << "\n"; }
        catch (const std::exception& e)     { std::cout << "    [exception]     " << e.what() << "\n"; }
    };

    demo([] { throw std::invalid_argument("bad arg"); });
    demo([] { throw std::out_of_range("index 99"); });
    demo([] { throw std::overflow_error("counter wrapped"); });
    std::cout << "\n";

    std::cout << "  2b. Exception objects are copied during throw:\n";
    std::cout << "    throw expr;  — creates a copy on a special 'exception storage'\n";
    std::cout << "    Avoid throwing large objects; prefer std::string message types.\n";
    std::cout << "\n";

    std::cout << "  2c. Non-class types can be thrown (but avoid them):\n";
    try { throw 42; }
    catch (int n) { std::cout << "    caught int: " << n << " (not recommended!)\n"; }
    std::cout << "    Prefer types derived from std::exception for .what() support.\n";
    std::cout << "\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — RAII and Exception Safety
// ─────────────────────────────────────────────────────────────────────────────

struct TrackedResource {
    std::string label_;
    explicit TrackedResource(std::string label) : label_(std::move(label)) {
        std::cout << "    [ACQUIRE] " << label_ << "\n";
    }
    ~TrackedResource() {
        std::cout << "    [RELEASE] " << label_ << "  (destructor called)\n";
    }
    // Non-copyable — mirrors Box<T> in Rust
    TrackedResource(const TrackedResource&) = delete;
    TrackedResource& operator=(const TrackedResource&) = delete;
};

static void section3_raii_exception_safety() {
    std::cout << "=== Section 3: RAII and Exception Safety ===\n";
    std::cout << "RAII (Resource Acquisition Is Initialization) ensures resources\n";
    std::cout << "  are released even when exceptions unwind the stack.\n";
    std::cout << "  Destructors run deterministically — just like Rust's Drop trait.\n";
    std::cout << "\n";

    std::cout << "  3a. Stack unwinding frees RAII objects:\n";
    try {
        TrackedResource r1("outer-resource");
        {
            TrackedResource r2("inner-resource");
            std::cout << "    throwing inside inner scope...\n";
            throw std::runtime_error("deliberate throw");
            // r2 destructor fires before r1 (LIFO)
        }
    } catch (const std::exception& e) {
        std::cout << "    caught: " << e.what() << "\n";
    }
    // Both destructors ran before the catch block executed
    std::cout << "\n";

    std::cout << "  3b. unique_ptr for heap resources — never leaks on throw:\n";
    try {
        auto p = std::make_unique<TrackedResource>("heap-resource");
        std::cout << "    unique_ptr holds heap-resource, throwing...\n";
        throw std::runtime_error("heap test");
    } catch (const std::exception& e) {
        std::cout << "    caught: " << e.what()
                  << "  (unique_ptr destructor freed heap-resource above)\n";
    }
    std::cout << "\n";

    std::cout << "  3c. Early release: reset() mirrors Rust's drop():\n";
    {
        auto p = std::make_unique<TrackedResource>("early-release");
        std::cout << "    calling p.reset() before end of scope...\n";
        p.reset();  // releases now, not at end of block
        std::cout << "    (resource already freed; p is now null)\n";
    }
    std::cout << "\n";

    // NEVER throw from a destructor
    std::cout << "  3d. NEVER throw from a destructor:\n";
    std::cout << "    If a destructor throws while another exception is in flight,\n";
    std::cout << "    std::terminate() is called — program aborts immediately.\n";
    std::cout << "    Destructors must be noexcept (they are by default in C++11+).\n";
    std::cout << "    Swallow exceptions in destructors or use a try/catch inside.\n";
    std::cout << "\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Custom Exceptions and noexcept
// ─────────────────────────────────────────────────────────────────────────────

enum class ErrorCode { NotFound, PermissionDenied, ConnectionFailed };

class DbException : public std::runtime_error {
public:
    explicit DbException(ErrorCode code, const std::string& msg)
        : std::runtime_error(msg), code_(code) {}

    ErrorCode code() const noexcept { return code_; }

private:
    ErrorCode code_;
};

static std::string fetch_user(unsigned id) {
    switch (id) {
        case 1:  return "Alice";
        case 2:  return "Bob";
        case 0:  throw DbException(ErrorCode::PermissionDenied, "access denied");
        default: throw DbException(ErrorCode::NotFound, "user " + std::to_string(id) + " not found");
    }
}

// noexcept: promises no exception will escape; allows compiler optimisations
static int safe_add(int a, int b) noexcept { return a + b; }

static void section4_custom_exceptions_noexcept() {
    std::cout << "=== Section 4: Custom Exceptions and noexcept ===\n";
    std::cout << "Custom exceptions should inherit from std::exception (or a subclass).\n";
    std::cout << "  Add structured data (error codes) via member fields.\n";
    std::cout << "  noexcept marks functions that will never throw — enables\n";
    std::cout << "  move-constructor optimisations and smaller stack-unwind tables.\n";
    std::cout << "\n";

    std::cout << "  4a. Catching by error code:\n";
    for (unsigned id : {1u, 2u, 0u, 99u}) {
        try {
            std::string name = fetch_user(id);
            std::cout << "    id=" << id << ": user = " << name << "\n";
        } catch (const DbException& e) {
            std::string kind = (e.code() == ErrorCode::NotFound)
                ? "not found"
                : (e.code() == ErrorCode::PermissionDenied ? "permission denied" : "other");
            std::cout << "    id=" << id << ": [" << kind << "] " << e.what() << "\n";
        }
    }
    std::cout << "\n";

    std::cout << "  4b. noexcept — marks functions that guarantee no throw:\n";
    std::cout << "    safe_add(3, 4) = " << safe_add(3, 4) << "\n";
    std::cout << "    If noexcept function throws anyway → std::terminate()\n";
    std::cout << "    Move constructors should be noexcept for STL container safety.\n";
    std::cout << "    Destructors are implicitly noexcept in C++11.\n";
    std::cout << "\n";

    std::cout << "  4c. Exception specifications (history):\n";
    std::cout << "    throw(T)   — C++03 dynamic spec, removed in C++17\n";
    std::cout << "    throw()    — equivalent to noexcept, deprecated C++11, removed C++17\n";
    std::cout << "    noexcept   — C++11 replacement; use this\n";
    std::cout << "    noexcept(expr) — conditional: noexcept only if expr is true\n";
    std::cout << "\n";

    std::cout << "  Rust comparison:\n";
    std::cout << "    Rust has NO exceptions. Equivalent patterns:\n";
    std::cout << "    throw DbException(NotFound) ↔ return Err(DbError::NotFound(id))\n";
    std::cout << "    noexcept                    ↔ functions returning () always succeed\n";
    std::cout << "    catch(DbException& e)       ↔ match result { Err(DbError::NotFound(n)) }\n";
    std::cout << "    The key difference: Rust errors are VALUES in the return type,\n";
    std::cout << "    checked by the compiler. C++ exceptions are invisible to the type system.\n";
    std::cout << "\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Patterns and Antipatterns
// ─────────────────────────────────────────────────────────────────────────────

static void section5_patterns_antipatterns() {
    std::cout << "=== Section 5: Patterns and Antipatterns ===\n";
    std::cout << "\n";

    // ── PATTERNS ─────────────────────────────────────────────────────────────
    std::cout << "  ── PATTERNS (good) ──────────────────────────────────────────\n";
    std::cout << "\n";

    std::cout << "  P1. Always catch by const reference:\n";
    std::cout << "    catch(const std::exception& e)  // correct: no copy, no slicing\n";
    std::cout << "    catch(std::exception e)          // wrong: slices derived type\n";
    std::cout << "\n";

    std::cout << "  P2. Use std::optional for 'value or nothing' (C++17):\n";
    auto find_even = [](const std::vector<int>& v) -> std::optional<int> {
        for (int n : v)
            if (n % 2 == 0) return n;
        return std::nullopt;
    };
    auto a = find_even({1, 3, 4, 7});
    auto b = find_even({1, 3, 5, 7});
    std::cout << "    find_even({1,3,4,7}) = " << (a ? std::to_string(*a) : "nullopt") << "\n";
    std::cout << "    find_even({1,3,5,7}) = " << (b ? "some" : "nullopt") << "\n";
    std::cout << "    std::optional is zero-overhead and type-safe — prefer over\n";
    std::cout << "    returning -1, nullptr, or throwing for expected absence.\n";
    std::cout << "\n";

    std::cout << "  P3. Use RAII wrappers — never raw new/delete:\n";
    std::cout << "    std::unique_ptr<T>  — sole ownership, auto-freed\n";
    std::cout << "    std::shared_ptr<T>  — shared ownership, ref-counted\n";
    std::cout << "    std::lock_guard     — mutex locked/unlocked via RAII\n";
    std::cout << "    std::ifstream       — file closed in destructor\n";
    std::cout << "\n";

    std::cout << "  P4. Prefer error codes / std::optional over exceptions for\n";
    std::cout << "      expected (non-exceptional) failures:\n";
    std::cout << "    // good: no exception for an expected 'not found'\n";
    std::cout << "    std::optional<User> find_user(int id);\n";
    std::cout << "    // noisy: throws for normal 'miss'\n";
    std::cout << "    User& get_user(int id);  // throws if not found\n";
    std::cout << "\n";

    std::cout << "  P5. Provide strong exception guarantee in operators:\n";
    std::cout << "    Copy-and-swap idiom: do all work on a copy, then swap.\n";
    std::cout << "    T& operator=(T rhs) { swap(*this, rhs); return *this; }\n";
    std::cout << "    If the copy throws, *this is untouched — strong guarantee.\n";
    std::cout << "\n";

    std::cout << "  P6. Document noexcept contracts on move constructors:\n";
    std::cout << "    MyClass(MyClass&&) noexcept;\n";
    std::cout << "    STL containers use std::move only if noexcept — otherwise\n";
    std::cout << "    they copy (expensive) to maintain the strong guarantee.\n";
    std::cout << "\n";

    // ── ANTIPATTERNS ──────────────────────────────────────────────────────────
    std::cout << "  ── ANTIPATTERNS (avoid) ─────────────────────────────────────\n";
    std::cout << "\n";

    std::cout << "  A1. Empty catch blocks — silently swallow errors:\n";
    std::cout << "    try { risky(); } catch (...) { }  // what went wrong?\n";
    std::cout << "    At minimum: log e.what() or set an error flag.\n";
    std::cout << "\n";

    std::cout << "  A2. Throwing from destructors:\n";
    std::cout << "    ~MyClass() { cleanUp(); }  // if cleanUp() throws → terminate()\n";
    std::cout << "    Wrap destructor body in try/catch and swallow or log.\n";
    std::cout << "\n";

    std::cout << "  A3. Using raw pointers / manual delete:\n";
    std::cout << "    int* p = new int(5);\n";
    std::cout << "    risky();               // if this throws, delete never runs → leak\n";
    std::cout << "    delete p;\n";
    std::cout << "    Fix: auto p = std::make_unique<int>(5);\n";
    std::cout << "\n";

    std::cout << "  A4. Throwing std::string or int instead of exception types:\n";
    std::cout << "    throw std::string(\"error\");  // caller cannot catch by base class\n";
    std::cout << "    throw 42;                    // caller must know the type exactly\n";
    std::cout << "    Always throw types derived from std::exception.\n";
    std::cout << "\n";

    std::cout << "  A5. Using exceptions for normal control flow:\n";
    std::cout << "    // slow: exceptions carry stack-unwind tables\n";
    std::cout << "    for (auto& s : strings) {\n";
    std::cout << "        try { result.push_back(std::stoi(s)); }\n";
    std::cout << "        catch (...) { /* skip */ }\n";
    std::cout << "    }\n";
    std::cout << "    // better: use std::optional or check before converting\n";
    std::cout << "\n";

    std::cout << "  A6. Catching std::exception by value (slicing):\n";
    std::cout << "    catch(std::exception e) { ... }  // slices to base\n";
    std::cout << "    e.what() returns base-class message, derived info is lost.\n";
    std::cout << "\n";

    std::cout << "  A7. Forgetting std::current_exception() when re-throwing:\n";
    std::cout << "    catch(const std::exception& e) {\n";
    std::cout << "        throw e;   // BAD: throws a copy of e (slices!)\n";
    std::cout << "        throw;     // GOOD: re-throws original exception object\n";
    std::cout << "    }\n";
    std::cout << "\n";

    std::cout << "  Summary:\n";
    std::cout << "    C++ exceptions are invisible to the type system — callers may\n";
    std::cout << "    not handle them and the compiler won't warn. This contrasts\n";
    std::cout << "    sharply with Rust's Result<T,E>, which is part of the return\n";
    std::cout << "    type and cannot be silently ignored. Use RAII to guarantee\n";
    std::cout << "    cleanup, std::optional for expected absence, and typed\n";
    std::cout << "    exception hierarchies for truly exceptional conditions.\n";
}
