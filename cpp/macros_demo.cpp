// C++ Macros and Metaprogramming Demo
//
// Sections:
//   1 — Preprocessor macros: #define, stringification, token pasting
//   2 — Templates: function templates, class templates, specialisation
//   3 — Template metaprogramming (TMP): type traits, if constexpr, SFINAE
//   4 — constexpr / consteval: compile-time computation
//   5 — Patterns and antipatterns
//
// C++17 throughout; C++20 concepts noted where applicable.
// clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2

#include <algorithm>
#include <array>
#include <cassert>
#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

static void section1_preprocessor();
static void section2_templates();
static void section3_tmp_traits();
static void section4_constexpr();
static void section5_patterns_antipatterns();

int main() {
    section1_preprocessor();
    section2_templates();
    section3_tmp_traits();
    section4_constexpr();
    section5_patterns_antipatterns();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Preprocessor Macros
// ─────────────────────────────────────────────────────────────────────────────

// Object-like macro (constant replacement)
#define MAX_CONNECTIONS 128
#define PI_APPROX       3.14159265358979

// Function-like macro — no type safety, no scoping
#define SQUARE(x)       ((x) * (x))
#define MAX(a, b)       ((a) > (b) ? (a) : (b))

// Stringification: # turns a token into a string literal
#define STRINGIFY(x)    #x
#define TO_STRING(x)    STRINGIFY(x)   // indirection needed for macro args

// Token pasting: ## glues two tokens into one identifier
#define MAKE_FN(prefix, suffix)  prefix ## _ ## suffix
int MAKE_FN(compute, area)(int w, int h) { return w * h; }
int MAKE_FN(compute, perimeter)(int w, int h) { return 2 * (w + h); }

// Variadic macro (C99 / C++11) — requires at least one arg after fmt
#define LOG(fmt, ...)  std::printf("[LOG] " fmt "\n", __VA_ARGS__)

// Multi-line macro using do { } while(0) idiom
#define SWAP(a, b)  do { auto _tmp = (a); (a) = (b); (b) = _tmp; } while(0)

// Conditional compilation
#ifdef NDEBUG
#  define DCHECK(cond)  ((void)0)
#else
#  define DCHECK(cond)  do { if (!(cond)) { \
       std::fprintf(stderr, "DCHECK failed: %s  [%s:%d]\n", #cond, __FILE__, __LINE__); \
       std::abort(); } } while(0)
#endif

// Include guard pattern (shown as comment — actual guard is at top of header)
// #pragma once           -- modern alternative, non-standard but universally supported
// #ifndef MY_HEADER_H    -- traditional double-include guard
// #define MY_HEADER_H
// ...
// #endif

static void section1_preprocessor() {
    std::cout << "=== Section 1: Preprocessor Macros ===\n";
    std::cout << "The C preprocessor runs BEFORE the compiler — pure text substitution.\n";
    std::cout << "  No type checking, no scoping, no hygiene, no debugger visibility.\n";
    std::cout << "  Modern C++ replaces most uses with constexpr, inline, templates.\n\n";

    std::cout << "  1a. Object-like macros (constants):\n";
    std::cout << "    MAX_CONNECTIONS = " << MAX_CONNECTIONS << "\n";
    std::cout << "    PI_APPROX       = " << PI_APPROX << "\n";
    std::cout << "    Prefer: constexpr int kMaxConnections = 128; (typed, scoped)\n\n";

    std::cout << "  1b. Function-like macros:\n";
    std::cout << "    SQUARE(5)   = " << SQUARE(5) << "\n";
    std::cout << "    SQUARE(2+3) = " << SQUARE(2+3) << "  (safe: outer parens)\n";
    int a = 3, b = 7;
    std::cout << "    MAX(3, 7)   = " << MAX(a, b) << "\n";
    std::cout << "    Danger: MAX(++a, ++b) increments TWICE — UB\n\n";

    std::cout << "  1c. Stringification (#) and token pasting (##):\n";
    std::cout << "    STRINGIFY(hello world) = \"" << STRINGIFY(hello world) << "\"\n";
    std::cout << "    TO_STRING(MAX_CONNECTIONS) = \"" << TO_STRING(MAX_CONNECTIONS) << "\"\n";
    std::cout << "    MAKE_FN(compute,area)(3,4) = " << compute_area(3, 4) << "\n";
    std::cout << "    MAKE_FN(compute,perimeter)(3,4) = " << compute_perimeter(3, 4) << "\n\n";

    std::cout << "  1d. Variadic macro:\n";
    LOG("server started on port %d", 8080);
    LOG("request: %s %s", "GET", "/index");
    std::cout << "\n";

    std::cout << "  1e. SWAP using do{}while(0) — safe in if-else:\n";
    int x = 10, y = 20;
    SWAP(x, y);
    std::cout << "    after SWAP(10,20): x=" << x << " y=" << y << "\n";
    std::cout << "    do{{}}while(0) prevents dangling-else bugs in macro body\n\n";

    std::cout << "  1f. __FILE__, __LINE__, __func__ — built-in macros:\n";
    std::cout << "    __FILE__ = " << __FILE__ << "\n";
    std::cout << "    __LINE__ = " << __LINE__ << "\n";
    std::cout << "    __func__ = " << __func__ << "\n\n";

    std::cout << "  1g. Rust comparison:\n";
    std::cout << "    #define SQUARE(x) ((x)*x)  ↔  macro_rules! square {{ ($x:expr) => {{ $x*$x }} }}\n";
    std::cout << "    C macro: text replacement, unhygienic, no types.\n";
    std::cout << "    Rust macro: token-tree aware, hygienic, pattern-matched.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Templates
// ─────────────────────────────────────────────────────────────────────────────

// Function template — instantiated for each concrete type at the call site
template<typename T>
T tmax(T a, T b) { return a > b ? a : b; }

// Template with multiple type params
template<typename T, typename U>
auto add(T a, U b) -> decltype(a + b) { return a + b; }

// Class template
template<typename T>
class Stack {
public:
    void push(T val) { data_.push_back(std::move(val)); }
    T    pop()       { T v = std::move(data_.back()); data_.pop_back(); return v; }
    bool empty()     const { return data_.empty(); }
    std::size_t size() const { return data_.size(); }
private:
    std::vector<T> data_;
};

// Full specialisation: Stack<bool> stores bits, not bytes
template<>
class Stack<bool> {
public:
    void push(bool val) { data_.push_back(val); }
    bool pop()          { bool v = data_.back(); data_.pop_back(); return v; }
    bool empty()        const { return data_.empty(); }
    std::size_t size()  const { return data_.size(); }
    void print_internals() const {
        std::cout << "    Stack<bool> specialisation: ";
        for (bool b : data_) std::cout << b;
        std::cout << "\n";
    }
private:
    std::vector<bool> data_;   // specialised storage
};

// Variadic template — type-safe printf equivalent
template<typename... Args>
std::string build_message(Args&&... args) {
    std::ostringstream oss;
    (oss << ... << std::forward<Args>(args));   // fold expression (C++17)
    return oss.str();
}

// Non-type template parameter
template<std::size_t N>
std::array<int, N> iota_array() {
    std::array<int, N> arr{};
    for (std::size_t i = 0; i < N; ++i) arr[i] = static_cast<int>(i);
    return arr;
}

static void section2_templates() {
    std::cout << "=== Section 2: Templates ===\n";
    std::cout << "Templates are instantiated at compile time for each concrete type.\n";
    std::cout << "  Duck typing: any T that satisfies the operations used will compile.\n";
    std::cout << "  Error messages are notoriously verbose (C++20 concepts improve this).\n\n";

    std::cout << "  2a. Function template — tmax<T>:\n";
    std::cout << "    tmax(3, 7)          = " << tmax(3, 7) << "  (T=int, deduced)\n";
    std::cout << "    tmax(3.14, 2.71)    = " << tmax(3.14, 2.71) << "  (T=double)\n";
    std::cout << "    tmax<int>(3, 7)     = " << tmax<int>(3, 7) << "  (explicit)\n";
    std::cout << "    tmax(\"abc\",\"xyz\")   = " << tmax(std::string("abc"), std::string("xyz")) << "  (T=string)\n\n";

    std::cout << "  2b. Multi-type function template with decltype:\n";
    std::cout << "    add(3, 4.5)   = " << add(3, 4.5) << "  (int+double=double)\n";
    std::cout << "    add(2, 3)     = " << add(2, 3) << "  (int+int=int)\n\n";

    std::cout << "  2c. Class template — Stack<T>:\n";
    {
        Stack<std::string> s;
        s.push("first"); s.push("second"); s.push("third");
        std::cout << "    Stack<string> size=" << s.size() << ", pop: " << s.pop() << "\n";
    }
    {
        Stack<bool> bs;
        bs.push(true); bs.push(false); bs.push(true);
        bs.print_internals();
    }
    std::cout << "\n";

    std::cout << "  2d. Variadic template + fold expression (C++17):\n";
    std::string msg = build_message("items=", 42, " price=", 9.99, " ok=", true);
    std::cout << "    build_message(\"items=\",42,...) = \"" << msg << "\"\n\n";

    std::cout << "  2e. Non-type template parameter — iota_array<N>:\n";
    auto a5 = iota_array<5>();
    auto a8 = iota_array<8>();
    std::cout << "    iota_array<5>: ";
    for (int v : a5) std::cout << v << " ";
    std::cout << "\n    iota_array<8>: ";
    for (int v : a8) std::cout << v << " ";
    std::cout << "\n\n";

    std::cout << "  Rust comparison:\n";
    std::cout << "    C++ templates: duck typing, errors at instantiation.\n";
    std::cout << "    Rust generics: trait bounds checked at definition — earlier, clearer.\n";
    std::cout << "    fn tmax<T: PartialOrd>(a: T, b: T) -> T  ← constraint is explicit\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Template Metaprogramming and Type Traits
// ─────────────────────────────────────────────────────────────────────────────

// Classic TMP: compile-time factorial via recursion
template<unsigned N>
struct Factorial { static constexpr unsigned long value = N * Factorial<N-1>::value; };
template<>
struct Factorial<0> { static constexpr unsigned long value = 1; };

// Compile-time GCD
template<unsigned A, unsigned B>
struct GCD { static constexpr unsigned value = GCD<B, A % B>::value; };
template<unsigned A>
struct GCD<A, 0> { static constexpr unsigned value = A; };

// Type list: compile-time list of types
template<typename... Ts>
struct TypeList {
    static constexpr std::size_t size = sizeof...(Ts);
};
using NumericTypes = TypeList<int, float, double, long>;

// SFINAE (Substitution Failure Is Not An Error): enable overload only for integral types
template<typename T>
typename std::enable_if<std::is_integral<T>::value, std::string>::type
describe_type(T) { return "integral"; }

template<typename T>
typename std::enable_if<std::is_floating_point<T>::value, std::string>::type
describe_type(T) { return "floating-point"; }

// if constexpr (C++17) — cleaner than SFINAE for branching on type properties
template<typename T>
std::string smart_to_string(T val) {
    if constexpr (std::is_same_v<T, bool>) {
        return val ? "true" : "false";
    } else if constexpr (std::is_integral_v<T>) {
        return "int:" + std::to_string(val);
    } else if constexpr (std::is_floating_point_v<T>) {
        return "fp:" + std::to_string(val);
    } else {
        return "other";
    }
}

// std::void_t detection idiom: does T have a .size() method?
template<typename T, typename = void>
struct has_size : std::false_type {};
template<typename T>
struct has_size<T, std::void_t<decltype(std::declval<T>().size())>> : std::true_type {};

// Generic print: uses size() if available
template<typename T>
void smart_print(const T& v) {
    if constexpr (has_size<T>::value) {
        std::cout << "    [has .size()=" << v.size() << "] ";
    } else {
        std::cout << "    [scalar] ";
    }
    std::cout << v << "\n";
}

static void section3_tmp_traits() {
    std::cout << "=== Section 3: Template Metaprogramming and Type Traits ===\n";
    std::cout << "TMP turns the C++ template system into a compile-time functional language.\n";
    std::cout << "  Types are values; template specialisation is pattern matching.\n";
    std::cout << "  C++17 if constexpr and std:: type traits reduce TMP boilerplate.\n\n";

    std::cout << "  3a. Compile-time recursion — Factorial<N>:\n";
    std::cout << "    Factorial<0>::value = " << Factorial<0>::value << "\n";
    std::cout << "    Factorial<5>::value = " << Factorial<5>::value << "\n";
    std::cout << "    Factorial<10>::value = " << Factorial<10>::value << "\n";
    std::cout << "    (computed entirely at compile time — no runtime loop)\n\n";

    std::cout << "  3b. Compile-time GCD<A,B>:\n";
    std::cout << "    GCD<48,18>::value = " << GCD<48,18>::value << "\n";
    std::cout << "    GCD<100,75>::value = " << GCD<100,75>::value << "\n\n";

    std::cout << "  3c. Type traits from <type_traits>:\n";
    std::cout << "    is_integral<int>     = " << std::is_integral_v<int>      << "\n";
    std::cout << "    is_integral<double>  = " << std::is_integral_v<double>   << "\n";
    std::cout << "    is_pointer<int*>     = " << std::is_pointer_v<int*>      << "\n";
    std::cout << "    is_same<int,int>     = " << std::is_same_v<int,int>      << "\n";
    std::cout << "    is_same<int,long>    = " << std::is_same_v<int,long>     << "\n";
    std::cout << "    is_const<const int>  = " << std::is_const_v<const int>   << "\n";
    std::cout << "    is_base_of<...>      — checks inheritance relationships\n\n";

    std::cout << "  3d. SFINAE — Substitution Failure Is Not An Error:\n";
    std::cout << "    describe_type(42)    = " << describe_type(42)    << "\n";
    std::cout << "    describe_type(3.14)  = " << describe_type(3.14)  << "\n";
    std::cout << "    (wrong overload silently dropped, not a compile error)\n\n";

    std::cout << "  3e. if constexpr (C++17) — cleaner type-based branching:\n";
    std::cout << "    smart_to_string(true)   = " << smart_to_string(true)    << "\n";
    std::cout << "    smart_to_string(42)     = " << smart_to_string(42)      << "\n";
    std::cout << "    smart_to_string(3.14f)  = " << smart_to_string(3.14f)   << "\n";
    std::cout << "    smart_to_string('x')    = " << smart_to_string('x')     << "\n\n";

    std::cout << "  3f. void_t detection idiom — does T have .size():\n";
    std::cout << "    has_size<std::string>::value = " << has_size<std::string>::value << "\n";
    std::cout << "    has_size<int>::value         = " << has_size<int>::value         << "\n";
    smart_print(std::string("hello"));
    smart_print(99);
    std::cout << "\n";

    std::cout << "  3g. Type transformations:\n";
    std::cout << "    remove_const<const int>   -> int:    "
              << std::is_same_v<std::remove_const_t<const int>, int> << "\n";
    std::cout << "    remove_reference<int&>    -> int:    "
              << std::is_same_v<std::remove_reference_t<int&>, int> << "\n";
    std::cout << "    add_pointer<int>          -> int*:   "
              << std::is_same_v<std::add_pointer_t<int>, int*> << "\n";
    std::cout << "    decay<int[3]>             -> int*:   "
              << std::is_same_v<std::decay_t<int[3]>, int*> << "\n\n";

    std::cout << "  Rust comparison:\n";
    std::cout << "    C++ TMP: encode logic in template specialisation (recursive structs).\n";
    std::cout << "    Rust:    const fn for computation; trait bounds for type constraints.\n";
    std::cout << "    C++ SFINAE  ↔  Rust where clause / trait bounds\n";
    std::cout << "    C++ if constexpr  ↔  Rust if (inside const fn, limited)\n";
    std::cout << "    C++ <type_traits>  ↔  Rust std::any::TypeId, marker traits\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — constexpr and consteval
// ─────────────────────────────────────────────────────────────────────────────

// constexpr function: evaluated at compile time when args are compile-time constants
constexpr unsigned long factorial_cx(unsigned n) {
    return n <= 1 ? 1 : n * factorial_cx(n - 1);
}

constexpr unsigned gcd_cx(unsigned a, unsigned b) {
    return b == 0 ? a : gcd_cx(b, a % b);
}

// consteval (C++20): ALWAYS evaluated at compile time; errors if runtime call is attempted
// consteval int square_ce(int x) { return x * x; }   // C++20 only, shown as comment

// constexpr class
class Rational {
public:
    constexpr Rational(int num, int den)
        : num_(num / gcd_cx(static_cast<unsigned>(num < 0 ? -num : num),
                            static_cast<unsigned>(den))),
          den_(den / gcd_cx(static_cast<unsigned>(num < 0 ? -num : num),
                            static_cast<unsigned>(den))) {}
    constexpr int num() const { return num_; }
    constexpr int den() const { return den_; }
    constexpr Rational operator+(Rational o) const {
        return Rational(num_ * o.den_ + o.num_ * den_, den_ * o.den_);
    }
    constexpr bool operator==(Rational o) const {
        return num_ == o.num_ && den_ == o.den_;
    }
private:
    int num_, den_;
};

// Compile-time lookup table: FNV-1a hash of single characters
constexpr std::uint32_t fnv1a_byte(std::uint32_t hash, unsigned char byte) {
    return (hash ^ byte) * 16777619u;
}
constexpr std::uint32_t hash_string(const char* s, std::uint32_t h = 2166136261u) {
    return (*s == '\0') ? h : hash_string(s + 1, fnv1a_byte(h, static_cast<unsigned char>(*s)));
}

// constexpr array
constexpr auto make_squares_table() {
    std::array<int, 10> t{};
    for (int i = 0; i < 10; ++i) t[static_cast<std::size_t>(i)] = i * i;
    return t;
}
constexpr auto SQUARES = make_squares_table();

static void section4_constexpr() {
    std::cout << "=== Section 4: constexpr and consteval ===\n";
    std::cout << "constexpr functions run at compile time when all arguments are\n";
    std::cout << "  compile-time constants; otherwise fall back to runtime.\n";
    std::cout << "  consteval (C++20) mandates compile-time evaluation — always.\n\n";

    std::cout << "  4a. constexpr functions:\n";
    constexpr unsigned long F5  = factorial_cx(5);
    constexpr unsigned long F10 = factorial_cx(10);
    constexpr unsigned      G   = gcd_cx(48, 18);
    std::cout << "    factorial_cx(5)  = " << F5  << "  (compile time)\n";
    std::cout << "    factorial_cx(10) = " << F10 << "  (compile time)\n";
    std::cout << "    gcd_cx(48,18)    = " << G   << "  (compile time)\n";
    std::cout << "    factorial_cx(7)  = " << factorial_cx(7) << "  (can be runtime too)\n\n";

    std::cout << "  4b. constexpr as array size:\n";
    constexpr std::size_t BUF = factorial_cx(3);
    std::array<std::uint8_t, BUF> buf{};
    std::cout << "    constexpr BUF = factorial_cx(3) = " << BUF
              << "  → std::array<uint8_t, " << buf.size() << ">\n\n";

    std::cout << "  4c. constexpr class — Rational arithmetic:\n";
    constexpr Rational half(1, 2);
    constexpr Rational third(1, 3);
    constexpr Rational five_sixths = half + third;
    std::cout << "    half       = " << half.num()         << "/" << half.den()         << "\n";
    std::cout << "    third      = " << third.num()        << "/" << third.den()        << "\n";
    std::cout << "    half+third = " << five_sixths.num()  << "/" << five_sixths.den()  << "\n";
    static_assert(five_sixths == Rational(5, 6), "compile-time check");
    std::cout << "    static_assert(half+third == 5/6) passed\n\n";

    std::cout << "  4d. Compile-time hash and switch:\n";
    constexpr std::uint32_t h_get  = hash_string("GET");
    constexpr std::uint32_t h_post = hash_string("POST");
    auto route = [&](const char* method) {
        std::uint32_t h = hash_string(method);
        if      (h == h_get)  std::cout << "    route(" << method << ") → GET handler\n";
        else if (h == h_post) std::cout << "    route(" << method << ") → POST handler\n";
        else                  std::cout << "    route(" << method << ") → 405\n";
    };
    route("GET"); route("POST"); route("DELETE");
    std::cout << "\n";

    std::cout << "  4e. Compile-time lookup table:\n";
    std::cout << "    SQUARES[0..9]: ";
    for (int v : SQUARES) std::cout << v << " ";
    std::cout << "\n    (table computed at compile time, stored as read-only data)\n\n";

    std::cout << "  4f. static_assert — compile-time assertions:\n";
    static_assert(sizeof(int) >= 4,  "int must be at least 32 bits");
    static_assert(factorial_cx(5) == 120, "factorial check");
    static_assert(GCD<48,18>::value == 6, "TMP GCD check");
    std::cout << "    static_assert(factorial_cx(5)==120) passed\n";
    std::cout << "    static_assert(GCD<48,18>==6) passed\n\n";

    std::cout << "  Rust comparison:\n";
    std::cout << "    C++ constexpr fn  ↔  Rust const fn  (both compile-time)\n";
    std::cout << "    C++ consteval     ↔  Rust const fn (all const fn in Rust must be\n";
    std::cout << "                         usable in const contexts; no runtime fallback)\n";
    std::cout << "    C++ static_assert ↔  Rust const { assert!(...) } / static_assert\n";
    std::cout << "    C++ constexpr var ↔  Rust const X: T = expr\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Patterns and Antipatterns
// ─────────────────────────────────────────────────────────────────────────────

static void section5_patterns_antipatterns() {
    std::cout << "=== Section 5: Patterns and Antipatterns ===\n\n";

    std::cout << "  ── PATTERNS (good) ──────────────────────────────────────────\n\n";

    std::cout << "  P1. Replace #define constants with constexpr:\n";
    std::cout << "    #define MAX 100          // no type, no scope, no debugger\n";
    std::cout << "    constexpr int kMax = 100; // typed, scoped, visible in debugger\n\n";

    std::cout << "  P2. Replace function-like macros with inline/template functions:\n";
    std::cout << "    #define SQUARE(x) ((x)*(x))        // evaluates x twice\n";
    std::cout << "    template<typename T>                // evaluated once, typed\n";
    std::cout << "    constexpr T square(T x) { return x*x; }\n\n";

    std::cout << "  P3. Use if constexpr instead of SFINAE for branching:\n";
    std::cout << "    if constexpr (std::is_integral_v<T>) { ... }  // readable\n";
    std::cout << "    enable_if<is_integral<T>::value>              // obscure\n\n";

    std::cout << "  P4. Use std:: type traits instead of hand-rolled TMP structs:\n";
    std::cout << "    std::is_same_v<T, U>, std::remove_cv_t<T>, std::conditional_t\n";
    std::cout << "    Cover nearly all common type queries — prefer over custom TMP.\n\n";

    std::cout << "  P5. Use static_assert to document and enforce type requirements:\n";
    std::cout << "    template<typename T>\n";
    std::cout << "    void process(T val) {\n";
    std::cout << "        static_assert(std::is_trivially_copyable_v<T>,\n";
    std::cout << "                      \"T must be trivially copyable\");\n";
    std::cout << "    }\n\n";

    std::cout << "  P6. Use concepts (C++20) to constrain templates with clear errors:\n";
    std::cout << "    template<std::integral T>   // C++20\n";
    std::cout << "    T popcount(T x) { return __builtin_popcountll(x); }\n";
    std::cout << "    Errors say 'T does not satisfy std::integral' — not 3 pages.\n\n";

    std::cout << "  P7. do{{}}while(0) in multi-statement macros:\n";
    std::cout << "    #define ASSERT(c) do {{ if(!(c)) abort(); }} while(0)\n";
    std::cout << "    Prevents dangling-else when macro used in if without braces.\n\n";

    std::cout << "  ── ANTIPATTERNS (avoid) ─────────────────────────────────────\n\n";

    std::cout << "  A1. #define for constants — leaks everywhere, no type:\n";
    std::cout << "    #define PI 3.14  // global, untypes, no namespace, no debugger\n\n";

    std::cout << "  A2. Function-like macro evaluating args multiple times:\n";
    std::cout << "    #define MAX(a,b) ((a)>(b)?(a):(b))\n";
    std::cout << "    MAX(++x, ++y)  → x and y incremented TWICE (UB)\n\n";

    std::cout << "  A3. TMP factorial / fibonacci when constexpr fn exists:\n";
    std::cout << "    template<int N> struct Fib {{ ... }};  // 20 lines\n";
    std::cout << "    constexpr int fib(int n) {{ ... }}      // 5 lines, debuggable\n\n";

    std::cout << "  A4. Overusing SFINAE — prefer if constexpr or concepts:\n";
    std::cout << "    typename enable_if<is_integral<T>::value && !is_same<T,bool>::value\n";
    std::cout << "      && sizeof(T)>=4, T>::type  // near-unreadable\n\n";

    std::cout << "  A5. Macros that generate identifiers unpredictably:\n";
    std::cout << "    #define REGISTER(cls) void _reg_##cls() {{ ... }}\n";
    std::cout << "    Name collisions are silent — no compiler warning.\n\n";

    std::cout << "  A6. Include guards via #define that clash with identifiers:\n";
    std::cout << "    #define UTILS_H  — collides if user defines the same name.\n";
    std::cout << "    Use #pragma once (universally supported) or long unique names.\n\n";

    std::cout << "  A7. Using void* + macros instead of templates for generics:\n";
    std::cout << "    void qsort(void* base, size_t n, size_t sz, int(*cmp)(void*,void*))\n";
    std::cout << "    No type safety, no inlining. Use std::sort<T> instead.\n\n";

    std::cout << "  Summary — C++ metaprogramming layers:\n";
    std::cout << "    1. Preprocessor (#define)     — avoid for new code\n";
    std::cout << "    2. Templates                  — generic code, type deduction\n";
    std::cout << "    3. TMP + type_traits           — compile-time type logic\n";
    std::cout << "    4. constexpr / consteval       — compile-time values and functions\n";
    std::cout << "    5. Concepts (C++20)            — readable, first-class constraints\n\n";
    std::cout << "  Rust comparison summary:\n";
    std::cout << "    C++ #define                   ↔  (no equivalent — use const fn)\n";
    std::cout << "    C++ function template         ↔  Rust generic fn with trait bounds\n";
    std::cout << "    C++ class template            ↔  Rust generic struct + impl\n";
    std::cout << "    C++ SFINAE / concepts         ↔  Rust where T: Trait\n";
    std::cout << "    C++ constexpr fn              ↔  Rust const fn\n";
    std::cout << "    C++ TMP recursive struct      ↔  Rust const fn (no TMP needed)\n";
    std::cout << "    C++ static_assert             ↔  Rust static_assert! / const {{ assert! }}\n";
    std::cout << "    C++ <type_traits>             ↔  Rust std::mem, marker traits\n";
}
