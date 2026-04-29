// C++ Stack vs Heap Memory Demo (C++17)
// Mirrors the Rust memory_demo: same 5 sections, analogous patterns.
//
// Sections:
//   1 — Stack memory: what it is and what lives there
//   2 — Heap memory: what it is and what lives there
//   3 — Comprehensive list: stack variables vs heap variables
//   4 — Freeing stack memory (automatic, scope-based)
//   5 — Freeing heap memory (delete, unique_ptr, shared_ptr, RAII)

#include <array>
#include <iostream>
#include <map>
#include <memory>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Stack Memory
// ─────────────────────────────────────────────────────────────────────────────

static int add(int a, int b) {
    int result = a + b;
    std::cout << "    inside add(): &a=" << &a
              << "  &b=" << &b
              << "  &result=" << &result << "\n";
    return result;
} // stack frame for add() is popped here

static void section1_stack_basics() {
    std::cout << "=== Section 1: Stack Memory ===\n";
    std::cout << "The stack is a contiguous region of memory managed automatically.\n";
    std::cout << "  - LIFO (last in, first out): each function call pushes a 'frame'\n";
    std::cout << "  - Frame is popped (freed) when the function returns\n";
    std::cout << "  - Allocation is O(1): just decrement the stack pointer\n";
    std::cout << "  - Size must be known at compile time\n";
    std::cout << "  - Typical size: 8 MB per thread (OS default)\n\n";

    // All of these are stack variables
    int          integer   = 42;
    double       fp        = 3.14;
    bool         boolean   = true;
    char         character = 'C';
    int          array[4]  = {10, 20, 30, 40};  // fixed-size C array: on stack
    std::array<int, 3> stdarr = {1, 2, 3};       // std::array: also on stack

    std::cout << "  Stack variables and their addresses:\n";
    std::cout << "    integer   int     = " << integer   << "  @ " << &integer   << "\n";
    std::cout << "    fp        double  = " << fp        << "  @ " << &fp        << "\n";
    std::cout << "    boolean   bool    = " << boolean   << "  @ " << (void*)&boolean << "\n";
    std::cout << "    character char    = " << character << "  @ " << (void*)&character << "\n";
    std::cout << "    array[0]  int     = " << array[0]  << "  @ " << &array[0]  << "\n";
    std::cout << "    stdarr[0] int     = " << stdarr[0] << "  @ " << &stdarr[0] << "\n";
    std::cout << "\n";
    std::cout << "  Stack frames across function calls:\n";
    int sum = add(3, 4);
    std::cout << "    back in caller: sum=" << sum << "  @ " << &sum << "\n";
    std::cout << "    (notice addresses change as stack frames are pushed/popped)\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Heap Memory
// ─────────────────────────────────────────────────────────────────────────────

static void section2_heap_basics() {
    std::cout << "=== Section 2: Heap Memory ===\n";
    std::cout << "The heap is a large memory pool managed by the allocator.\n";
    std::cout << "  - Size can be determined at runtime\n";
    std::cout << "  - Allocation is slower (allocator must find a free block)\n";
    std::cout << "  - In C++: YOU must free it (delete/free) or use RAII smart pointers\n";
    std::cout << "  - The stack holds a pointer/fat-pointer to the heap data\n\n";

    // std::string: SSO may keep short strings on stack, but the pointer is always stack
    std::string s = "hello, heap from C++!";
    std::vector<int> v = {1, 2, 3, 4, 5};
    auto b = std::make_unique<int>(999); // unique_ptr on stack, int on heap

    // Raw new/delete — old-style, shown for clarity
    int* raw = new int(777);

    std::cout << "  Heap allocations:\n";
    std::cout << "    std::string  stack addr (obj)  : " << (void*)&s << "\n";
    std::cout << "    std::string  heap  addr (data)  : " << (void*)s.data()
              << "  value=\"" << s << "\"\n";
    std::cout << "    std::vector  stack addr (obj)   : " << &v << "\n";
    std::cout << "    std::vector  heap  addr (data[0]): " << v.data()
              << "  value=" << v[0] << "\n";
    std::cout << "    unique_ptr   stack addr (ptr)   : " << &b << "\n";
    std::cout << "    unique_ptr   heap  addr (value) : " << b.get()
              << "  value=" << *b << "\n";
    std::cout << "    raw new int  stack addr (ptr var): " << &raw << "\n";
    std::cout << "    raw new int  heap  addr (value) : " << raw
              << "  value=" << *raw << "\n";
    delete raw; // must free manually — NOT needed for unique_ptr/string/vector
    std::cout << "    raw int freed with delete\n\n";
    std::cout << "  Stack addresses cluster together; heap addresses are far away.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Stack vs Heap: Comprehensive List
// ─────────────────────────────────────────────────────────────────────────────

static void section3_stack_vs_heap_list() {
    std::cout << "=== Section 3: Stack vs Heap — What Lives Where ===\n\n";

    std::cout << "  +------------------------------------------------------------------+\n";
    std::cout << "  |                     STACK (C++)                                  |\n";
    std::cout << "  +------------------------------------------------------------------+\n";
    std::cout << "  | Type                      Example                                |\n";
    std::cout << "  +------------------------------------------------------------------+\n";
    std::cout << "  | int, long, short, char     int x = 5;                            |\n";
    std::cout << "  | unsigned variants           unsigned int n = 10;                 |\n";
    std::cout << "  | float, double, long double  double f = 1.5;                      |\n";
    std::cout << "  | bool                        bool b = true;                       |\n";
    std::cout << "  | Pointer variable            int* p = &x;  // p itself on stack  |\n";
    std::cout << "  | Reference variable          int& r = x;   // r itself on stack  |\n";
    std::cout << "  | C-style fixed array         int a[4];     // whole array stack  |\n";
    std::cout << "  | std::array<T, N>            std::array<int,4> a;                |\n";
    std::cout << "  | Plain struct/class          struct Point { double x, y; };       |\n";
    std::cout << "  | std::pair / std::tuple      std::pair<int,int> p{1,2};          |\n";
    std::cout << "  | Function args & locals      any local variable                  |\n";
    std::cout << "  | sizeof(T) known at compile  all of the above qualify            |\n";
    std::cout << "  +------------------------------------------------------------------+\n\n";

    std::cout << "  +------------------------------------------------------------------+\n";
    std::cout << "  |                     HEAP (C++)                                   |\n";
    std::cout << "  +------------------------------------------------------------------+\n";
    std::cout << "  | Type                      Note                                   |\n";
    std::cout << "  +------------------------------------------------------------------+\n";
    std::cout << "  | new T / new T[]           raw heap alloc; need delete/delete[]   |\n";
    std::cout << "  | malloc / calloc / realloc C-style; need free()                   |\n";
    std::cout << "  | std::string               heap buffer (beyond SSO threshold)     |\n";
    std::cout << "  | std::vector<T>            heap buffer                            |\n";
    std::cout << "  | std::unique_ptr<T>        heap T; freed by RAII on scope exit   |\n";
    std::cout << "  | std::shared_ptr<T>        heap T + ref-count block              |\n";
    std::cout << "  | std::deque / std::list     heap nodes                            |\n";
    std::cout << "  | std::map / std::set        heap tree nodes                       |\n";
    std::cout << "  | std::unordered_map/set     heap buckets                          |\n";
    std::cout << "  | std::function (capturing) may heap-allocate closure state       |\n";
    std::cout << "  | Variable-length array*     NOT standard C++; use vector instead  |\n";
    std::cout << "  +------------------------------------------------------------------+\n\n";

    // Demonstrate sizes
    std::cout << "  Compile-time sizes (stack types):\n";
    std::cout << "    int    : " << sizeof(int)    << " bytes\n";
    std::cout << "    double : " << sizeof(double) << " bytes\n";
    std::cout << "    bool   : " << sizeof(bool)   << " bytes\n";
    std::cout << "    char   : " << sizeof(char)   << " bytes\n";
    std::cout << "    int[4] : " << sizeof(int[4]) << " bytes\n";
    std::cout << "\n  Stack-side object sizes (heap types — stack part only):\n";
    std::cout << "    std::string          : " << sizeof(std::string)           << " bytes\n";
    std::cout << "    std::vector<int>     : " << sizeof(std::vector<int>)      << " bytes\n";
    std::cout << "    std::unique_ptr<int> : " << sizeof(std::unique_ptr<int>)  << " bytes\n";
    std::cout << "    std::shared_ptr<int> : " << sizeof(std::shared_ptr<int>)  << " bytes\n";
    std::cout << "    int* (raw pointer)   : " << sizeof(int*)                  << " bytes\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Freeing Stack Memory
// ─────────────────────────────────────────────────────────────────────────────

static void demonstrate_stack_scope() {
    std::cout << "    [entering demonstrate_stack_scope]\n";
    int x = 100;
    int y = 200;
    std::cout << "    x=" << x << " @ " << &x
              << ",  y=" << y << " @ " << &y << "\n";
    std::cout << "    [leaving demonstrate_stack_scope — x and y freed automatically]\n";
} // stack frame popped; x and y cease to exist

static void section4_freeing_stack() {
    std::cout << "=== Section 4: Freeing Stack Memory ===\n";
    std::cout << "Stack memory is freed automatically — no action required.\n";
    std::cout << "  - When a function returns, its entire stack frame is released\n";
    std::cout << "  - When a block { } ends, objects declared in it are destroyed (LIFO)\n";
    std::cout << "  - There is no operator to manually free a stack variable\n\n";

    demonstrate_stack_scope();
    std::cout << "    (x and y from above no longer exist in memory)\n\n";

    std::cout << "  Block scoping demo:\n";
    int outer = 1;
    std::cout << "    outer declared @ " << &outer << "\n";
    {
        int inner      = 2;
        int also_inner = 3;
        std::cout << "    inner declared @ " << &inner << "\n";
        std::cout << "    also_inner declared @ " << &also_inner << "\n";
        std::cout << "    [end of inner block — inner and also_inner destroyed here]\n";
    }
    std::cout << "    outer still valid: " << outer << "  @ " << &outer << "\n\n";
    std::cout << "  Key point: you CANNOT manually free a stack variable.\n";
    std::cout << "  delete/free() on a stack address is undefined behaviour.\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Freeing Heap Memory
// ─────────────────────────────────────────────────────────────────────────────

struct TrackedResource {
    std::string label;
    explicit TrackedResource(std::string l) : label(std::move(l)) {
        std::cout << "    [CTOR] TrackedResource \"" << label << "\" — heap allocated\n";
    }
    ~TrackedResource() {
        std::cout << "    [DTOR] TrackedResource \"" << label << "\" — heap freed\n";
    }
    TrackedResource(const TrackedResource&) = delete;
    TrackedResource& operator=(const TrackedResource&) = delete;
};

static void section5_freeing_heap() {
    std::cout << "=== Section 5: Freeing Heap Memory ===\n";
    std::cout << "C++ gives you THREE strategies; only RAII (smart pointers) is safe.\n\n";

    // ── 5a: Raw new/delete — manual, error-prone ──────────────────────────────
    std::cout << "  5a. Raw new / delete  [DANGEROUS — easy to forget or double-free]:\n";
    {
        int* p = new int(42);         // heap allocation
        std::cout << "    allocated int=" << *p << " @ " << p << "\n";
        delete p;                     // heap freed — MUST match every new
        p = nullptr;                  // null to prevent use-after-free
        std::cout << "    delete called — heap int freed\n";

        int* arr = new int[5]{1,2,3,4,5};
        std::cout << "    allocated int[5] @ " << arr << "\n";
        delete[] arr;                 // must use delete[] for arrays, not delete
        std::cout << "    delete[] called — heap array freed\n\n";
    }

    // ── 5b: unique_ptr — RAII, freed automatically on scope exit ─────────────
    std::cout << "  5b. std::unique_ptr  [PREFERRED — freed automatically by RAII]:\n";
    {
        auto up = std::make_unique<TrackedResource>("unique");
        std::cout << "    unique_ptr heap addr: " << up.get() << "\n";
        std::cout << "    [end of block — unique_ptr destroyed, resource freed]\n";
    } // ~TrackedResource() called here automatically
    std::cout << "\n";

    // ── 5c: Explicit early release with unique_ptr::reset() ──────────────────
    std::cout << "  5c. Explicit early release with unique_ptr::reset():\n";
    {
        auto early = std::make_unique<TrackedResource>("early");
        std::cout << "    early resource heap addr: " << early.get() << "\n";
        early.reset(); // destructor runs immediately; analogous to Rust drop()
        std::cout << "    early resource was freed above (reset() called)\n\n";
    }

    // ── 5d: shared_ptr — freed when reference count reaches zero ─────────────
    std::cout << "  5d. std::shared_ptr — freed when last owner releases it:\n";
    {
        auto sp1 = std::make_shared<TrackedResource>("shared");
        {
            auto sp2 = sp1; // shared ownership; ref count = 2
            std::cout << "    ref count after copy: " << sp1.use_count() << "\n";
            std::cout << "    [inner block ends — sp2 released, count -> 1]\n";
        } // sp2 destroyed; count = 1; heap NOT freed yet
        std::cout << "    ref count after sp2 scope: " << sp1.use_count() << "\n";
        std::cout << "    [outer block ends — sp1 released, count -> 0, heap freed]\n";
    } // ~TrackedResource() called here
    std::cout << "\n";

    // ── 5e: std::vector / std::string — RAII containers ──────────────────────
    std::cout << "  5e. std::vector and std::string freed automatically:\n";
    {
        std::vector<int> v = {10, 20, 30};
        std::string      s = "heap string buffer";
        std::cout << "    vector heap data @ " << v.data() << "\n";
        std::cout << "    string heap data @ " << (void*)s.data() << "\n";
        std::cout << "    [end of block — vector and string buffers freed]\n";
    }
    std::cout << "\n";

    std::cout << "  Summary of heap-free mechanisms:\n";
    std::cout << "    delete p           raw pointer — manual, must match every new\n";
    std::cout << "    delete[] p         raw array   — must use [] form for new[]\n";
    std::cout << "    free(p)            C-style malloc; paired with malloc/calloc\n";
    std::cout << "    unique_ptr scope   RAII: freed when unique_ptr goes out of scope\n";
    std::cout << "    unique_ptr.reset() explicit early release\n";
    std::cout << "    shared_ptr         freed when use_count() reaches 0\n";
    std::cout << "    vector/string dtor RAII: buffer freed by destructor\n";
    std::cout << "    Forgetting delete  = memory leak (Rust ownership prevents this)\n";
}

// ─────────────────────────────────────────────────────────────────────────────

int main() {
    section1_stack_basics();
    section2_heap_basics();
    section3_stack_vs_heap_list();
    section4_freeing_stack();
    section5_freeing_heap();
    return 0;
}
