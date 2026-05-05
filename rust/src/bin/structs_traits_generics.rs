// Rust: Structs, Traits, and Generics Demo
//
// Sections:
//   1 — Structs: data/behavior separation, impl blocks, derive macros
//   2 — Traits: contracts with default methods (vs C++ abstract classes)
//   3 — Multiple traits and composition (Rust has NO inheritance)
//   4 — Generics: bounded type parameters (vs C++ templates)
//   5 — Static dispatch vs dynamic dispatch (dyn Trait)

use std::fmt;

fn main() {
    section1_structs();
    section2_traits();
    section3_composition();
    section4_generics();
    section5_dispatch();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Structs
// ─────────────────────────────────────────────────────────────────────────────

// RUST: struct holds DATA ONLY. Behavior lives in a separate `impl` block.
// C++:  class/struct holds data AND methods together.
//
// `derive` auto-generates trait implementations — no boilerplate needed.
// C++ equivalent: write copy constructor, operator==, operator<< manually.
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width:  f64,   // private by default (unlike C++ struct which is public by default)
    height: f64,
}

// Methods live in an impl block — completely separate from the data definition.
// A type can have multiple impl blocks (useful for organizing large types).
impl Rectangle {
    // Associated function (no `self`) — C++ equivalent: static method
    fn new(width: f64, height: f64) -> Self {
        assert!(width > 0.0 && height > 0.0, "dimensions must be positive");
        Rectangle { width, height }   // field-init shorthand: width: width → width
    }

    // &self = immutable borrow — C++ equivalent: const method
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    // &mut self = mutable borrow — C++ equivalent: non-const method
    fn scale(&mut self, factor: f64) {
        self.width  *= factor;
        self.height *= factor;
    }

    // self (by value) = consuming method — transfers ownership to this method
    // C++ equivalent: move constructor / rvalue-qualified method
    fn into_square(self) -> Rectangle {
        let side = self.width.min(self.height);
        Rectangle { width: side, height: side }
    }
}

// Struct update syntax: copy specific fields, inherit the rest from another instance.
// C++ has no direct equivalent.
fn section1_structs() {
    println!("=== Section 1: Structs ===");
    println!("KEY DIFFERENCE: Rust separates DATA (struct) from BEHAVIOR (impl block).");
    println!("C++ mixes both inside the class definition.");
    println!();

    let r1 = Rectangle::new(10.0, 5.0);
    println!("  r1 = {:?}", r1);                     // Debug trait via #[derive(Debug)]
    println!("  area     = {:.1}", r1.area());
    println!("  perimeter= {:.1}", r1.perimeter());
    println!("  is_square= {}", r1.is_square());

    // Struct update syntax: r2 takes width=20, inherits height from r1
    let r2 = Rectangle { width: 20.0, ..r1.clone() };
    println!("\n  r2 (update syntax) = {:?}", r2);

    // Clone vs move
    let r3 = r1.clone();  // deep copy via Clone trait (#[derive(Clone)])
    println!("  r3 (clone of r1) = {:?}", r3);
    println!("  r1 == r3: {}  (PartialEq via #[derive])", r1 == r3);

    // Mutable method
    let mut r4 = Rectangle::new(3.0, 4.0);
    println!("\n  r4 before scale: {:?}", r4);
    r4.scale(2.0);
    println!("  r4 after scale(2.0): {:?}", r4);

    // Consuming method: r4 is MOVED into into_square; r4 no longer accessible after
    let sq = r4.into_square();
    println!("  sq (consumed r4): {:?}  is_square={}", sq, sq.is_square());

    println!();
    println!("  DERIVE MACROS — auto-generated for free:");
    println!("    #[derive(Debug)]     → {{:?}} formatting  (C++: write operator<< manually)");
    println!("    #[derive(Clone)]     → .clone()           (C++: copy constructor)");
    println!("    #[derive(PartialEq)] → == operator        (C++: operator==)");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Traits
// ─────────────────────────────────────────────────────────────────────────────

// RUST TRAIT: a set of method signatures + optional default implementations.
// C++ EQUIVALENT: abstract base class with pure virtual methods.
//
// KEY DIFFERENCES:
//   - `impl Trait for Type` is separate from the type — you can implement
//     a trait for a type defined in another crate (subject to orphan rule).
//   - C++: you must inherit from the abstract class at the point of definition.
//   - Traits can have default method bodies; no override keyword needed.
//   - C++: pure virtual = no default body; virtual = has default body.
trait Shape {
    // Required methods — every implementor MUST provide these
    fn area(&self)      -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self)      -> &'static str;

    // Default method — implementors MAY override, but don't have to.
    // C++ equivalent: virtual method with a body (non-pure virtual).
    fn describe(&self) -> String {
        format!(
            "{}: area={:.2}, perimeter={:.2}",
            self.name(), self.area(), self.perimeter()
        )
    }

    // Default method that calls other required methods — always consistent
    fn is_larger_than(&self, other: &dyn Shape) -> bool {
        self.area() > other.area()
    }
}

// Implementing Shape for Rectangle — completely outside the struct definition
impl Shape for Rectangle {
    fn area(&self)      -> f64    { self.width * self.height }
    fn perimeter(&self) -> f64    { 2.0 * (self.width + self.height) }
    fn name(&self)      -> &'static str { "Rectangle" }
    // `describe` and `is_larger_than` use the defaults — no override needed
}

#[derive(Debug, Clone)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self)      -> f64    { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64    { 2.0 * std::f64::consts::PI * self.radius }
    fn name(&self)      -> &'static str { "Circle" }

    // Override the default describe
    fn describe(&self) -> String {
        format!("Circle with radius={:.2} (area={:.2})", self.radius, self.area())
    }
}

#[derive(Debug, Clone)]
struct Triangle {
    a: f64, b: f64, c: f64,   // three sides
}

impl Triangle {
    fn new(a: f64, b: f64, c: f64) -> Self {
        assert!(a + b > c && b + c > a && a + c > b, "invalid triangle");
        Triangle { a, b, c }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0; // Heron's formula
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64    { self.a + self.b + self.c }
    fn name(&self)      -> &'static str { "Triangle" }
}

fn section2_traits() {
    println!("=== Section 2: Traits ===");
    println!("TRAIT = required method signatures + optional default implementations.");
    println!("C++ = abstract base class (pure virtual + optional virtual methods).");
    println!();
    println!("  KEY DIFFERENCES from C++ abstract classes:");
    println!("    - impl is OUTSIDE the struct — no inheritance coupling");
    println!("    - You can impl a trait for ANY type, even one from another library");
    println!("    - Default methods work across all implementors automatically");
    println!("    - No 'override' keyword — just provide the method");
    println!("    - No virtual destructor needed — Drop trait handles cleanup");
    println!();

    let rect  = Rectangle::new(6.0, 4.0);
    let circ  = Circle   { radius: 3.0 };
    let tri   = Triangle::new(3.0, 4.0, 5.0);

    // describe(): rect and tri use default; circle uses its override
    println!("  describe() — default vs overridden:");
    println!("    rect:  {}", rect.describe());   // default
    println!("    circ:  {}", circ.describe());   // overridden
    println!("    tri:   {}", tri.describe());    // default

    println!();
    println!("  is_larger_than() — default method using other required methods:");
    println!("    rect.is_larger_than(circ): {}", rect.is_larger_than(&circ));
    println!("    circ.is_larger_than(tri):  {}", circ.is_larger_than(&tri));
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Multiple Traits and Composition (No Inheritance)
// ─────────────────────────────────────────────────────────────────────────────

// RUST HAS NO INHERITANCE. You compose behavior through:
//   (a) implementing multiple traits on one type
//   (b) embedding other structs (composition)
//   (c) trait default methods for shared behavior

trait Drawable {
    fn draw(&self);
}

trait Resizable {
    fn resize(&mut self, factor: f64);
}

// Implement multiple independent traits on the same type.
// C++ would require multiple inheritance (with its diamond-problem risks).
impl Drawable for Rectangle {
    fn draw(&self) {
        println!("    [drawing rectangle {}×{}]", self.width, self.height);
    }
}

impl Resizable for Rectangle {
    fn resize(&mut self, factor: f64) {
        self.scale(factor);
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("    [drawing circle r={}]", self.radius);
    }
}

// Composition: ColoredShape wraps any Shape and adds color.
// Rust idiom for "extending" a type without inheritance.
struct ColoredShape<S: Shape + Drawable> {
    shape: S,
    color: String,
}

impl<S: Shape + Drawable> ColoredShape<S> {
    fn new(shape: S, color: &str) -> Self {
        ColoredShape { shape, color: color.to_string() }
    }
    fn draw_colored(&self) {
        print!("    color={} ", self.color);
        self.shape.draw();
    }
}

// Delegate the Shape trait to the inner shape (forwarding)
impl<S: Shape + Drawable> Shape for ColoredShape<S> {
    fn area(&self)      -> f64    { self.shape.area() }
    fn perimeter(&self) -> f64    { self.shape.perimeter() }
    fn name(&self)      -> &'static str { self.shape.name() }
}

// fmt::Display is a standard library trait — implement it for any type
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({}×{})", self.width, self.height)
    }
}

fn section3_composition() {
    println!("=== Section 3: Multiple Traits and Composition ===");
    println!("Rust has NO inheritance. Use:");
    println!("  (a) multiple trait impls on one type");
    println!("  (b) struct composition (embed one struct inside another)");
    println!("  (c) trait default methods for shared logic");
    println!();

    let mut rect = Rectangle::new(4.0, 3.0);

    // Rectangle implements Shape, Drawable, Resizable, Display — all independently
    println!("  Rectangle implements Shape + Drawable + Resizable + Display:");
    println!("    area={:.1}  perimeter={:.1}", rect.area(), rect.perimeter());
    rect.draw();
    println!("    Display trait: {}", rect);   // fmt::Display
    rect.resize(2.0);
    println!("    after resize(2.0): {}", rect);

    println!();
    println!("  Composition — ColoredShape<Rectangle>:");
    let colored = ColoredShape::new(Rectangle::new(5.0, 2.0), "red");
    colored.draw_colored();
    println!("    area (delegated to inner shape): {:.1}", colored.area());

    println!();
    println!("  C++ equivalent would be multiple inheritance:");
    println!("    class ColoredRect : public Shape, public Drawable, public Resizable {{...}}");
    println!("    Rust avoids the diamond problem entirely — traits carry no state.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Generics (Rust's Templates)
// ─────────────────────────────────────────────────────────────────────────────

// RUST GENERICS vs C++ TEMPLATES:
//
//   C++ template: type checked at INSTANTIATION — errors appear at call site,
//                 often with pages of cryptic messages.
//   Rust generic: type checked at DEFINITION — errors appear where the generic
//                 is written; callers are guaranteed to compile if bounds are met.
//
//   C++20 concepts are the closest C++ analog to Rust trait bounds.

// Generic function: T must implement PartialOrd (comparable with <, >)
// C++ equivalent: template<typename T> requires std::totally_ordered<T>
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    assert!(!list.is_empty(), "list must not be empty");
    let mut biggest = &list[0];
    for item in list.iter() {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

// Generic struct — works with any type T
// C++ equivalent: template<typename T, typename U> struct Pair { T first; U second; };
#[derive(Debug)]
struct Pair<T, U> {
    first:  T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

// Conditional impl: cmp_display only when BOTH fields are the SAME type T.
// PartialOrd<Rhs = Self> compares T with T — comparing T with a different U
// would require a separate bound T: PartialOrd<U>.
// This `impl<T> Pair<T, T>` is a Rust-specific feature: an impl block that
// only activates for the specialised case where both type params are equal.
// C++ equivalent: template specialization or a requires clause.
impl<T> Pair<T, T>
where
    T: fmt::Display + PartialOrd,
{
    fn cmp_display(&self) {
        println!(
            "    first={} {} second={}",
            self.first,
            if self.first >= self.second { ">" } else { "<" },
            self.second
        );
    }
}

// Generic function with multiple bounds using `+` syntax
// The compiler monomorphizes this — generates a concrete copy per type used (like C++)
fn print_shape_info<T>(shape: &T)
where
    T: Shape + fmt::Debug,
{
    println!("    {:?}  →  area={:.2}", shape, shape.area());
}

// Returning `impl Trait` — static dispatch, caller doesn't need to know concrete type.
// C++ equivalent (C++20): auto return type with concept constraints.
fn make_unit_circle() -> impl Shape {
    Circle { radius: 1.0 }
}

fn section4_generics() {
    println!("=== Section 4: Generics (Rust's Templates) ===");
    println!("GENERIC = parameterized over types, with bounds checked at DEFINITION.");
    println!("C++ TEMPLATE = checked at INSTANTIATION (duck typing until C++20 concepts).");
    println!();

    // Generic function over different types
    let integers = vec![34, 50, 25, 100, 65];
    let floats   = vec![7.2, 4.5, 9.1, 3.3];
    let words    = vec!["apple", "orange", "banana", "cherry"];
    println!("  largest() — same generic function, three types:");
    println!("    largest(&[i32]) = {}", largest(&integers));
    println!("    largest(&[f64]) = {}", largest(&floats));
    println!("    largest(&[&str]) = {}", largest(&words));

    println!();
    println!("  Generic struct Pair<T, U>:");
    let p1 = Pair::new(10_i32, 20_i32);
    let p2 = Pair::new("hello", "world");
    let p3 = Pair::new(1.5_f64, 0.5_f64);
    p1.cmp_display();
    p2.cmp_display();
    p3.cmp_display();
    println!("    p1 = {:?}", p1);

    println!();
    println!("  print_shape_info<T: Shape + Debug>:");
    print_shape_info(&Rectangle::new(3.0, 4.0));
    print_shape_info(&Circle { radius: 2.0 });
    print_shape_info(&Triangle::new(3.0, 4.0, 5.0));

    println!();
    let unit = make_unit_circle();
    println!("  make_unit_circle() -> impl Shape  area={:.4}", unit.area());

    println!();
    println!("  KEY DIFFERENCES from C++ templates:");
    println!("    C++: template<typename T> void f(T x) {{ x.foo(); }}");
    println!("         Error only if T doesn't have .foo() — at CALL SITE");
    println!("    Rust: fn f<T: Foo>(x: T) {{ x.foo(); }}");
    println!("         Error if T doesn't impl Foo — at DEFINITION SITE");
    println!("    Both generate monomorphized machine code (zero abstraction overhead).");
    println!("    C++20 `requires` / `concept` is the closest C++ analog to trait bounds.");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Static Dispatch vs Dynamic Dispatch
// ─────────────────────────────────────────────────────────────────────────────

// STATIC DISPATCH via generic bound: the compiler generates a separate copy
// of the function for each concrete type. No runtime overhead — like C++ templates.
fn total_area_static<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// DYNAMIC DISPATCH via `dyn Trait`: a vtable pointer is used at runtime.
// C++ equivalent: virtual method calls through a base-class pointer.
// Required when mixing different types in one collection.
fn total_area_dynamic(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn print_all(shapes: &[Box<dyn Shape>]) {
    for s in shapes {
        println!("    {}", s.describe());
    }
}

fn section5_dispatch() {
    println!("=== Section 5: Static Dispatch vs Dynamic Dispatch ===");
    println!();

    // ── Static dispatch ───────────────────────────────────────────────────────
    println!("  Static dispatch — &T where T: Shape:");
    println!("    Compiler creates a concrete version of total_area_static for each T.");
    println!("    Zero runtime overhead — identical to C++ template instantiation.");
    println!();

    let rects = vec![
        Rectangle::new(3.0, 4.0),
        Rectangle::new(5.0, 2.0),
        Rectangle::new(1.0, 1.0),
    ];
    println!("    total_area_static([Rectangle×3]) = {:.2}", total_area_static(&rects));

    // ── Dynamic dispatch ──────────────────────────────────────────────────────
    println!();
    println!("  Dynamic dispatch — &dyn Shape / Box<dyn Shape>:");
    println!("    A fat pointer: (data pointer, vtable pointer).");
    println!("    Method calls go through the vtable — one indirection per call.");
    println!("    C++ equivalent: virtual method through base-class pointer.");
    println!("    Required when mixing different concrete types in one collection.");
    println!();

    // Mixed collection — only possible with dyn Trait
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(3.0, 4.0)),
        Box::new(Circle { radius: 2.0 }),
        Box::new(Triangle::new(3.0, 4.0, 5.0)),
    ];
    print_all(&shapes);
    println!("    total_area_dynamic (mixed vec) = {:.2}", total_area_dynamic(&shapes));

    println!();
    println!("  Choosing between static and dynamic dispatch:");
    println!("    Static  (impl Trait / <T: Trait>): prefer when types are known at");
    println!("            compile time; zero cost; enables inlining.");
    println!("    Dynamic (dyn Trait / Box<dyn Trait>): needed for heterogeneous");
    println!("            collections or when the concrete type varies at runtime.");
    println!();
    println!("  C++ NOTE: all virtual calls are dynamic dispatch.");
    println!("    Rust lets you choose per call site — static by default,");
    println!("    dynamic only when you explicitly write `dyn`.");
}
