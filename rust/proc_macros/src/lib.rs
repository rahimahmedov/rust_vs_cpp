// Procedural Macro Crate — proc_macros
//
// A proc-macro crate compiles to a shared library loaded by the compiler itself.
// It receives and returns `TokenStream` — a sequence of tokens that the compiler
// uses as source code.
//
// Three kinds of proc macro this crate exports:
//   #[proc_macro]           — invoked as a function-like macro: name!(...)
//   #[proc_macro_attribute] — invoked as an attribute: #[name(...)]
//   #[proc_macro_derive]    — invoked via #[derive(Name)]
//
// All operate on `proc_macro::TokenStream`, which is:
//   - An iterator of `TokenTree` variants
//   - `Ident`   — an identifier:  x, foo, String, my_var
//   - `Literal` — a literal:      42, "hello", 3.14, b'A'
//   - `Punct`   — a punctuation:  + - * / : ; , . ! = < > & | ^ ~ # @ $ ?
//   - `Group`   — delimited subtree: (...) [...] {...}

use proc_macro::{Delimiter, TokenStream, TokenTree};
use std::str::FromStr;

// ── Shared helpers ─────────────────────────────────────────────────────────────

/// Recursively describe every TokenTree in a TokenStream.
/// This is the core of the educational `token_info!` macro.
fn describe_stream(ts: &TokenStream) -> String {
    ts.clone()
        .into_iter()
        .map(|tt| describe_tree(&tt))
        .collect::<Vec<_>>()
        .join(" ")
}

fn describe_tree(tt: &TokenTree) -> String {
    match tt {
        // An identifier: a plain word that is not a literal.
        TokenTree::Ident(id) => format!("Ident({})", id),

        // A literal: number, string, char, bool, byte.
        // .to_string() includes the delimiters, e.g. `"hello"` or `42`.
        TokenTree::Literal(lit) => format!("Literal({})", lit),

        // A punctuation character. Spacing::Joint means it is immediately
        // followed by another Punct with no whitespace (e.g. `::`, `&&`).
        TokenTree::Punct(p) => {
            let spacing = match p.spacing() {
                proc_macro::Spacing::Joint => "·",
                proc_macro::Spacing::Alone => " ",
            };
            format!("Punct({}{})", p.as_char(), spacing)
        }

        // A delimited group wraps a nested TokenStream.
        // Delimiter::None is used for macro-generated invisible groups.
        TokenTree::Group(g) => {
            let (open, close) = match g.delimiter() {
                Delimiter::Parenthesis => ("(", ")"),
                Delimiter::Brace      => ("{", "}"),
                Delimiter::Bracket    => ("[", "]"),
                Delimiter::None       => ("⟨", "⟩"),
            };
            let inner = describe_stream(&g.stream());
            if inner.is_empty() {
                format!("Group{}{}", open, close)
            } else {
                format!("Group{}{}{}{}",
                    open, inner,
                    if inner.ends_with(' ') { "" } else { " " },
                    close)
            }
        }
    }
}

/// Extract the string *value* from an attr TokenStream that is a single
/// string literal.  `"/api/users"` → `/api/users` (quotes stripped).
fn parse_str_literal(ts: TokenStream) -> String {
    // TokenStream::to_string() for a string literal includes surrounding
    // double-quotes, e.g.  "  \"/api/users\"  "  after trimming.
    let raw = ts.to_string();
    let trimmed = raw.trim();
    // Strip leading and trailing '"'
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Walk a TokenStream and return the identifier that follows the `fn` keyword.
fn extract_fn_name(ts: &TokenStream) -> Option<String> {
    let mut iter = ts.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        if let TokenTree::Ident(id) = &tt {
            if id.to_string() == "fn" {
                if let Some(TokenTree::Ident(name)) = iter.next() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// Walk a TokenStream and return the identifier that follows the `struct` keyword.
fn extract_struct_name(ts: &TokenStream) -> Option<String> {
    let mut iter = ts.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        if let TokenTree::Ident(id) = &tt {
            if id.to_string() == "struct" {
                if let Some(TokenTree::Ident(name)) = iter.next() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// Extract the names of named fields from a struct body `{ field: Type, ... }`.
///
/// Strategy: find the brace-delimited `Group` at the top level, then scan
/// inside for  Ident  ':'  pairs — each leading Ident before a ':' is a field.
/// We skip 'pub' (and its optional visibility group) before the field name.
fn extract_named_fields(ts: &TokenStream) -> Vec<String> {
    // Find the brace group that is the struct body
    for tt in ts.clone().into_iter() {
        if let TokenTree::Group(g) = tt {
            if g.delimiter() == Delimiter::Brace {
                let mut fields = Vec::new();
                let mut iter = g.stream().into_iter().peekable();

                while let Some(tt) = iter.next() {
                    if let TokenTree::Ident(id) = &tt {
                        let name = id.to_string();

                        // `pub` — skip it (and an optional `(crate)` group)
                        if name == "pub" {
                            if let Some(TokenTree::Group(_)) = iter.peek() {
                                iter.next(); // consume visibility group
                            }
                            continue;
                        }

                        // If the next token is ':', this is a field name
                        if let Some(TokenTree::Punct(p)) = iter.peek() {
                            if p.as_char() == ':' {
                                fields.push(name);
                            }
                        }
                    }
                }
                return fields;
            }
        }
    }
    Vec::new()
}

// ── Proc Macro 1: token_info!(expr) ───────────────────────────────────────────

/// `token_info!(expr)` — a function-like proc macro.
///
/// Returns a `&'static str` describing the TokenTree structure of `expr`.
/// The expression itself is NOT evaluated — this is purely a compile-time
/// inspection of what tokens the compiler sees.
///
/// Example:
///   token_info!(a + b * 2)
///   → "Ident(a) Punct(+ ) Ident(b) Punct(*·) Literal(2)"
///
/// How it works:
///   input  = the TokenStream between the parentheses
///   output = a string-literal TokenStream: "..."
///
/// This is the simplest kind of proc macro — it takes tokens and returns tokens.
#[proc_macro]
pub fn token_info(input: TokenStream) -> TokenStream {
    let description = describe_stream(&input);
    // Escape backslashes and double-quotes so we can embed the description
    // inside a Rust string literal in the generated code.
    let escaped = description
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    let output = format!("\"{}\"", escaped);
    // TokenStream::from_str parses raw Rust source into tokens.
    TokenStream::from_str(&output).expect("token_info: invalid output")
}

// ── Proc Macro 2: inspect!(expr) ──────────────────────────────────────────────

/// `inspect!(expr)` — a function-like proc macro.
///
/// Evaluates `expr` exactly once, prints "inspect!(expr) = value", and
/// returns the value. Works like `dbg!` but uses Display (`{}`) instead
/// of Debug (`{:?}`), and the expression is shown as written in source.
///
/// Expansion of  inspect!(x * 2):
///   {
///       let _v = { x * 2 };
///       println!("  inspect!(x * 2) = {}", _v);
///       _v
///   }
///
/// Key detail: we wrap the original token stream verbatim in `{ ... }`,
/// so the compiler sees exactly the same tokens it would for `x * 2`.
/// No risk of double-evaluation — `let _v = {...}` evaluates once.
#[proc_macro]
pub fn inspect(input: TokenStream) -> TokenStream {
    // to_string() gives back something close to the original source text
    let expr_text = input.to_string();
    let escaped = expr_text
        .replace('\\', "\\\\")
        .replace('"', "\\\"");

    // Generate a block expression that:
    //   1. Binds the result to _v  (single evaluation)
    //   2. Prints source text + runtime value
    //   3. Returns _v  (so inspect!() can be used inside larger expressions)
    let generated = format!(
        r#"{{ let _v = {{ {expr} }}; println!("  inspect!({esc}) = {{}}", _v); _v }}"#,
        expr = expr_text,
        esc  = escaped,
    );
    TokenStream::from_str(&generated).expect("inspect: invalid output")
}

// ── Proc Macro 3: #[route("/path")] ───────────────────────────────────────────

/// `#[route("/path")]` — an attribute macro.
///
/// Annotates a handler function to register it with a router.
///
/// Input:
///   attr = the TokenStream of the attribute argument  →  "/api/users"
///   item = the TokenStream of the annotated item      →  fn handle_users() { ... }
///
/// Output (what the caller sees after expansion):
///   fn handle_users() { ... }            ← original function, unchanged
///   pub const HANDLE_USERS_PATH: &str = "/api/users";   ← generated
///   pub fn register_handle_users() { ... }              ← generated
///
/// Unlike `macro_rules!`, we keep `item` as-is and just EXTEND the output
/// with new items — no need to re-parse or quote the original function body.
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path     = parse_str_literal(attr);
    let fn_name  = extract_fn_name(&item)
        .expect("#[route] must be placed on a fn item");
    let fn_upper = fn_name.to_ascii_uppercase();

    // Start the output with the original function token stream, unchanged.
    // This is idiomatic: mutate `item` rather than stringifying and re-parsing.
    let mut output = item;

    // Append the generated constant and registrar function.
    let appended = format!(
        r#"
        /// Path constant generated by #[route("{path}")]
        pub const {upper}_PATH: &str = "{path}";

        /// Registration helper generated by #[route("{path}")]
        pub fn register_{name}() {{
            println!("  [router] GET  {path:<20}  →  {name}()");
        }}
        "#,
        path  = path,
        upper = fn_upper,
        name  = fn_name,
    );
    output.extend(TokenStream::from_str(&appended).expect("route: invalid append"));
    output
}

// ── Proc Macro 4: #[derive(Describe)] ─────────────────────────────────────────

/// `#[derive(Describe)]` — a derive macro.
///
/// Generates a `describe(&self) -> String` method that formats the struct
/// name and all named field values.
///
/// For:
///   #[derive(Describe)]
///   struct Server { host: String, port: u16 }
///
/// Generates:
///   impl Server {
///       pub fn describe(&self) -> String {
///           format!("Server {{ host: {}, port: {} }}", self.host, self.port)
///       }
///   }
///
/// A derive macro receives the struct/enum definition and returns ONLY the
/// new items to add — the original struct is always preserved automatically.
/// (Unlike #[proc_macro_attribute], which replaces the item unless you re-emit it.)
#[proc_macro_derive(Describe)]
pub fn derive_describe(item: TokenStream) -> TokenStream {
    let struct_name = extract_struct_name(&item)
        .expect("#[derive(Describe)] must be on a struct");
    let fields = extract_named_fields(&item);

    // Build format string:  "Server { host: {}, port: {} }"
    let field_fmt = fields.iter()
        .map(|f| format!("{}: {{}}", f))
        .collect::<Vec<_>>()
        .join(", ");

    let fmt_str = if field_fmt.is_empty() {
        struct_name.clone()
    } else {
        format!("{} {{{{ {} }}}}", struct_name, field_fmt)
        // Note: {{{{ }}}} in format! becomes {{ }} in the *output* format string,
        // which the generated format!(...) then expands to literal { }.
    };

    // Build argument list:  "self.host, self.port"
    let field_args = fields.iter()
        .map(|f| format!("self.{}", f))
        .collect::<Vec<_>>()
        .join(", ");

    let generated = if field_args.is_empty() {
        format!(
            r#"impl {name} {{
                pub fn describe(&self) -> String {{ String::from("{fmt}") }}
            }}"#,
            name = struct_name,
            fmt  = fmt_str,
        )
    } else {
        format!(
            r#"impl {name} {{
                pub fn describe(&self) -> String {{
                    format!("{fmt}", {args})
                }}
            }}"#,
            name = struct_name,
            fmt  = fmt_str,
            args = field_args,
        )
    };

    TokenStream::from_str(&generated).expect("derive_describe: invalid output")
}
