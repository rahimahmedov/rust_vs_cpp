// Rust Stack vs Heap Memory Demo
//
// Sections:
//   1 — Stack memory: what it is and what lives there
//   2 — Heap memory: what it is and what lives there
//   3 — Comprehensive list: stack variables vs heap variables
//   4 — Freeing stack memory (automatic, scope-based)
//   5 — Freeing heap memory (ownership, drop, Box/Vec/String)

use std::collections::HashMap;

fn main() {
    section1_stack_basics();
    section2_heap_basics();
    section3_stack_vs_heap_list();
    section4_freeing_stack();
    section5_freeing_heap();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Stack Memory
// ─────────────────────────────────────────────────────────────────────────────

fn add(a: i32, b: i32) -> i32 {
    // a, b, and the return value all live on the stack frame of this call
    let result = a + b;
    println!("    inside add(): &a={:p}  &b={:p}  &result={:p}", &a, &b, &result);
    result
}

fn section1_stack_basics() {
    println!("=== Section 1: Stack Memory ===");
    println!("The stack is a contiguous region of memory managed automatically.");
    println!("  - LIFO (last in, first out): each function call pushes a 'frame'");
    println!("  - Frame is popped (freed) when the function returns");
    println!("  - Allocation is O(1): just decrement the stack pointer");
    println!("  - Size must be known at compile time");
    println!("  - Typical size: 8 MB per thread (OS default)");
    println!();

    // All of these live on the stack of section1_stack_basics()
    let integer: i32    = 42;
    let float:   f64    = 3.14;
    let boolean: bool   = true;
    let character: char = 'R';
    let array: [i32; 4] = [10, 20, 30, 40];   // fixed-size: size known at compile time
    let tuple: (i32, f64) = (1, 2.0);

    println!("  Stack variables and their addresses:");
    println!("    integer   i32         = {}  @ {:p}", integer,   &integer);
    println!("    float     f64         = {}  @ {:p}", float,     &float);
    println!("    boolean   bool        = {}  @ {:p}", boolean,   &boolean);
    println!("    character char        = {}  @ {:p}", character, &character);
    println!("    array     [i32;4][0]  = {}  @ {:p}", array[0],  &array[0]);
    println!("    tuple.0   i32         = {}  @ {:p}", tuple.0,   &tuple.0);
    println!();
    println!("  Stack frames across function calls:");
    let sum = add(3, 4);
    println!("    back in caller: sum={}  @ {:p}", sum, &sum);
    println!("    (notice addresses change as stack frames are pushed/popped)");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Heap Memory
// ─────────────────────────────────────────────────────────────────────────────

fn section2_heap_basics() {
    println!("=== Section 2: Heap Memory ===");
    println!("The heap is a large memory pool managed by an allocator.");
    println!("  - Size can be determined at runtime");
    println!("  - Allocation is slower (allocator must find a free block)");
    println!("  - In Rust: freed automatically when the owner goes out of scope");
    println!("  - The stack holds a 'fat pointer': (ptr, length, capacity)");
    println!();

    // The String variable on the stack holds: ptr (8 bytes) + len (8) + cap (8)
    // The actual characters live on the heap at the address `ptr` points to.
    let s = String::from("hello, heap!");
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let b: Box<i32> = Box::new(999); // Box puts a single value on the heap

    println!("  Heap allocations:");
    println!("    String  stack addr (fat ptr) : {:p}", &s);
    println!("    String  heap  addr (data)    : {:p}  value=\"{}\"", s.as_ptr(), s);
    println!("    Vec     stack addr (fat ptr) : {:p}", &v);
    println!("    Vec     heap  addr (data[0]) : {:p}  value={}", v.as_ptr(), v[0]);
    println!("    Box<i32> stack addr (ptr)    : {:p}", &b);
    println!("    Box<i32> heap  addr (value)  : {:p}  value={}", &*b, *b);
    println!();
    println!("  Stack addresses are close together (same stack frame).");
    println!("  Heap addresses are in a completely different memory region.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Stack vs Heap: Comprehensive List
// ─────────────────────────────────────────────────────────────────────────────

fn section3_stack_vs_heap_list() {
    println!("=== Section 3: Stack vs Heap — What Lives Where ===");
    println!();

    println!("  ┌──────────────────────────────────────────────────────────────┐");
    println!("  │                     STACK (Rust)                             │");
    println!("  ├──────────────────────────────────────────────────────────────┤");
    println!("  │ Type                    Example                              │");
    println!("  ├──────────────────────────────────────────────────────────────┤");
    println!("  │ i8, i16, i32, i64, i128  let x: i32 = 5;                   │");
    println!("  │ u8, u16, u32, u64, u128  let n: u64 = 100;                 │");
    println!("  │ isize, usize             let i: usize = 0;                  │");
    println!("  │ f32, f64                 let f: f64 = 1.5;                  │");
    println!("  │ bool                     let b: bool = true;                │");
    println!("  │ char                     let c: char = 'a';                 │");
    println!("  │ Fixed array [T; N]       let a: [i32; 3] = [1,2,3];        │");
    println!("  │ Tuple (T1, T2, ...)      let t = (1, 2.0, 'x');            │");
    println!("  │ Plain struct (no heap)   struct Point {{ x:f64, y:f64 }}   │");
    println!("  │ &T / &mut T (the ref)    let r = &x;  // pointer is stack  │");
    println!("  │ *const T / *mut T        raw pointer variable itself        │");
    println!("  │ Function args & locals   any local variable                 │");
    println!("  │ std::mem::size_of::<T>() known at compile time              │");
    println!("  └──────────────────────────────────────────────────────────────┘");
    println!();

    println!("  ┌──────────────────────────────────────────────────────────────┐");
    println!("  │                     HEAP (Rust)                              │");
    println!("  ├──────────────────────────────────────────────────────────────┤");
    println!("  │ Type                    Note                                 │");
    println!("  ├──────────────────────────────────────────────────────────────┤");
    println!("  │ String                  heap buffer; stack holds fat ptr     │");
    println!("  │ Vec<T>                  heap buffer; stack holds fat ptr     │");
    println!("  │ Box<T>                  heap value; stack holds thin ptr     │");
    println!("  │ Rc<T> / Arc<T>          heap value + ref-count               │");
    println!("  │ HashMap<K,V>            heap buckets                         │");
    println!("  │ HashSet<T>              heap buckets                         │");
    println!("  │ BTreeMap<K,V>           heap nodes                           │");
    println!("  │ Box<dyn Trait>          heap vtable + value                  │");
    println!("  │ Mutex<T>, RwLock<T>     heap interior                        │");
    println!("  │ Any type via Box::new() explicit heap allocation             │");
    println!("  │ Closure capturing heap  if closure owns a String/Vec/…       │");
    println!("  └──────────────────────────────────────────────────────────────┘");
    println!();

    // Demonstrate sizes: stack types have size known at compile time
    println!("  Compile-time sizes (all live on stack):");
    println!("    i32   : {} bytes", std::mem::size_of::<i32>());
    println!("    f64   : {} bytes", std::mem::size_of::<f64>());
    println!("    bool  : {} bytes", std::mem::size_of::<bool>());
    println!("    char  : {} bytes", std::mem::size_of::<char>());
    println!("    [i32;4]: {} bytes", std::mem::size_of::<[i32; 4]>());
    println!();
    println!("  Fat pointer sizes (stack part only):");
    println!("    String  fat ptr: {} bytes  (ptr + len + cap)", std::mem::size_of::<String>());
    println!("    Vec<i32> fat ptr: {} bytes  (ptr + len + cap)", std::mem::size_of::<Vec<i32>>());
    println!("    Box<i32> thin ptr: {} bytes", std::mem::size_of::<Box<i32>>());
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Freeing Stack Memory
// ─────────────────────────────────────────────────────────────────────────────

fn demonstrate_stack_scope() {
    println!("    [entering demonstrate_stack_scope]");
    let x: i32 = 100; // x pushed onto this function's stack frame
    let y: i32 = 200;
    println!("    x={} @ {:p},  y={} @ {:p}", x, &x, y, &y);
    println!("    [leaving demonstrate_stack_scope — x and y freed automatically]");
    // x and y cease to exist here; the stack pointer moves back up
}

fn section4_freeing_stack() {
    println!("=== Section 4: Freeing Stack Memory ===");
    println!("Stack memory is freed automatically — no action required.");
    println!("  - When a function returns, its entire stack frame is released");
    println!("  - When a block {{ }} ends, variables declared in it are dropped");
    println!("  - Order: LIFO — last declared is first dropped");
    println!();

    demonstrate_stack_scope();
    println!("    (x and y from above no longer exist in memory)");
    println!();

    // Inner block: variables freed at end of block, before outer scope ends
    println!("  Block scoping demo:");
    let outer = 1_i32;
    println!("    outer declared @ {:p}", &outer);
    {
        let inner = 2_i32;
        let also_inner = 3_i32;
        println!("    inner declared @ {:p}", &inner);
        println!("    also_inner declared @ {:p}", &also_inner);
        println!("    [end of inner block — inner and also_inner freed here]");
    }
    // inner and also_inner are gone; their stack space is reclaimed
    println!("    outer still valid: {}  @ {:p}", outer, &outer);
    println!();
    println!("  Key point: you CANNOT manually free stack memory.");
    println!("  You cannot call free() or drop() on a plain i32/f64/array.");
    println!("  The compiler handles it entirely.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Freeing Heap Memory
// ─────────────────────────────────────────────────────────────────────────────

struct TrackedResource {
    label: &'static str,
}
impl Drop for TrackedResource {
    fn drop(&mut self) {
        println!("    [DROP] TrackedResource \"{}\" — heap memory freed", self.label);
    }
}

fn section5_freeing_heap() {
    println!("=== Section 5: Freeing Heap Memory ===");
    println!("Rust frees heap memory automatically via ownership — no delete/free needed.");
    println!();

    // ── 5a: Automatic free when owner goes out of scope ──────────────────────
    println!("  5a. Automatic free at end of scope:");
    {
        let s = String::from("I live on the heap");
        let v: Vec<i32> = vec![10, 20, 30];
        println!("    String heap ptr: {:p}  value=\"{}\"", s.as_ptr(), s);
        println!("    Vec    heap ptr: {:p}  value={:?}", v.as_ptr(), v);
        println!("    [end of block — String and Vec heap buffers freed here]");
    }
    // Heap buffers of s and v are released; no leak possible
    println!();

    // ── 5b: Box<T> — explicit heap allocation, freed when Box is dropped ──────
    println!("  5b. Box<T> — single value on heap:");
    {
        let b: Box<i32> = Box::new(42);
        println!("    Box heap addr: {:p}  value={}", &*b, *b);
        println!("    [end of block — Box drops: heap i32 freed]");
    }
    println!();

    // ── 5c: Explicit early drop with std::mem::drop() ────────────────────────
    println!("  5c. Explicit early drop:");
    let early = Box::new(TrackedResource { label: "early-box" });
    println!("    early Box allocated, heap addr: {:p}", &*early as *const _);
    drop(early); // heap memory freed immediately here, not at end of function
    println!("    (heap was freed above — early is no longer accessible)");
    println!();

    // ── 5d: HashMap — heap-allocated, freed with owner ───────────────────────
    println!("  5d. HashMap — freed when owner goes out of scope:");
    {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("alpha", 1);
        map.insert("beta",  2);
        println!("    map has {} entries on heap", map.len());
        println!("    [end of block — HashMap buckets freed here]");
    }
    println!();

    // ── 5e: Rc<T> — freed when reference count reaches zero ──────────────────
    println!("  5e. Rc<T> — freed when last reference is dropped:");
    {
        use std::rc::Rc;
        let rc1 = Rc::new(TrackedResource { label: "shared-rc" });
        let rc2 = Rc::clone(&rc1); // second owner; ref count = 2
        println!("    Rc ref count after clone: {}", Rc::strong_count(&rc1));
        drop(rc2); // ref count → 1; heap NOT freed yet
        println!("    Rc ref count after drop(rc2): {}", Rc::strong_count(&rc1));
        println!("    [end of block — rc1 dropped, count → 0, heap freed]");
    }
    println!();

    println!("  Summary of heap-free mechanisms:");
    println!("    Ownership scope end  → compiler inserts drop automatically");
    println!("    drop(value)          → explicit early release");
    println!("    Box::new()           → freed when Box goes out of scope");
    println!("    Rc<T>/Arc<T>         → freed when strong_count reaches 0");
    println!("    Vec/String/HashMap   → buffer freed when owner is dropped");
    println!("    No manual delete/free() calls — memory leaks are impossible*");
    println!("    (* unless you use std::mem::forget() or Rc cycles deliberately)");
}
