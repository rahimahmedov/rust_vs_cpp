// Rust Concurrency Demo
//
// Sections:
//   1 — Spawning threads: std::thread::spawn and join
//   2 — Shared state: Arc<Mutex<T>> and Arc<RwLock<T>>
//   3 — Message passing: std::sync::mpsc channels
//   4 — Synchronisation primitives: Barrier, Condvar, atomic types
//   5 — Patterns and antipatterns

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    section1_thread_basics();
    section2_shared_state();
    section3_channels();
    section4_sync_primitives();
    section5_patterns_antipatterns();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Thread Basics
// ─────────────────────────────────────────────────────────────────────────────

fn section1_thread_basics() {
    println!("=== Section 1: Spawning Threads ===");
    println!("std::thread::spawn takes a closure and runs it on a new OS thread.");
    println!("  The closure must be 'static + Send — no dangling borrows.");
    println!("  spawn returns a JoinHandle<T>; .join() blocks until the thread ends.");
    println!();

    // Basic spawn + join
    println!("  1a. Basic spawn and join:");
    let handle = thread::spawn(|| {
        println!("    [child] hello from thread {:?}", thread::current().id());
        42  // thread return value
    });
    let result = handle.join().expect("thread panicked");
    println!("    [main]  child returned: {}", result);
    println!();

    // Move closure — take ownership of captured values
    println!("  1b. move closure — transfer ownership into thread:");
    let data = [1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        // `data` is moved into this closure; main can no longer use it
        let sum: i32 = data.iter().sum();
        println!("    [child] sum of moved vec = {}", sum);
    });
    handle.join().unwrap();
    // println!("{:?}", data);  // COMPILE ERROR: moved into thread
    println!("    (data ownership transferred — main cannot use it after move)");
    println!();

    // Spawn multiple threads, collect handles
    println!("  1c. Spawning N threads and waiting for all:");
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                println!("    [thread {}] running", i);
                i * i
            })
        })
        .collect();
    let squares: Vec<i32> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("    squares: {:?}", squares);
    println!();

    // Thread with name
    println!("  1d. Named thread:");
    let handle = thread::Builder::new()
        .name("worker-1".to_string())
        .spawn(|| {
            println!("    [{}] running", thread::current().name().unwrap_or("?"));
        })
        .unwrap();
    handle.join().unwrap();
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Shared State
// ─────────────────────────────────────────────────────────────────────────────

fn section2_shared_state() {
    println!("=== Section 2: Shared State ===");
    println!("Rust prevents data races at compile time via the type system:");
    println!("  Arc<T>        — thread-safe reference counting (like shared_ptr)");
    println!("  Mutex<T>      — mutual exclusion; .lock() returns a guard");
    println!("  RwLock<T>     — multiple readers OR one writer at a time");
    println!("  The borrow checker ensures you cannot share &mut T across threads.");
    println!();

    // Arc<Mutex<T>> — the classic shared counter
    println!("  2a. Arc<Mutex<T>> — shared mutable counter:");
    let counter = Arc::new(Mutex::new(0_u64));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut guard = counter.lock().unwrap();
                *guard += 1;
                // guard dropped here → mutex released
            })
        })
        .collect();
    for h in handles { h.join().unwrap(); }
    println!("    counter after 8 increments = {}", *counter.lock().unwrap());
    println!();

    // Arc<RwLock<T>> — many readers, one writer
    println!("  2b. Arc<RwLock<T>> — shared config (many readers, one writer):");
    let config = Arc::new(RwLock::new(vec!["host=localhost", "port=8080"]));

    // Spawn 3 readers concurrently
    let readers: Vec<_> = (0..3)
        .map(|i| {
            let cfg = Arc::clone(&config);
            thread::spawn(move || {
                let guard = cfg.read().unwrap();
                println!("    [reader {}] entries: {}", i, guard.len());
            })
        })
        .collect();
    for h in readers { h.join().unwrap(); }

    // One writer
    {
        let mut guard = config.write().unwrap();
        guard.push("timeout=30");
        println!("    [writer]  added entry; total: {}", guard.len());
    }
    println!();

    // Deadlock illustration (avoided)
    println!("  2c. Lock ordering — deadlock avoidance:");
    println!("    Deadlock occurs when two threads each hold a lock the other needs.");
    println!("    Prevention: always acquire multiple locks in the SAME order.");
    let lock_a = Arc::new(Mutex::new("resource A"));
    let lock_b = Arc::new(Mutex::new("resource B"));

    let (la, lb) = (Arc::clone(&lock_a), Arc::clone(&lock_b));
    let t1 = thread::spawn(move || {
        let _a = la.lock().unwrap();  // always A first
        thread::sleep(Duration::from_millis(1));
        let _b = lb.lock().unwrap();
        println!("    [t1] acquired A then B — safe");
    });
    let (la, lb) = (Arc::clone(&lock_a), Arc::clone(&lock_b));
    let t2 = thread::spawn(move || {
        let _a = la.lock().unwrap();  // also A first — consistent order → no deadlock
        let _b = lb.lock().unwrap();
        println!("    [t2] acquired A then B — safe");
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Channels
// ─────────────────────────────────────────────────────────────────────────────

fn section3_channels() {
    println!("=== Section 3: Message Passing via Channels ===");
    println!("std::sync::mpsc — multi-producer, single-consumer channel.");
    println!("  mpsc::channel()        — unbounded (asynchronous, non-blocking send)");
    println!("  mpsc::sync_channel(n)  — bounded (send blocks when buffer is full)");
    println!("  Sender<T>: clone-able, Send. Receiver<T>: single owner.");
    println!("  Channel transfers OWNERSHIP of the message — no sharing needed.");
    println!();

    // Basic send / recv
    println!("  3a. Simple producer → consumer:");
    {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let producer = thread::spawn(move || {
            for word in ["hello", "from", "the", "thread"] {
                tx.send(word.to_string()).unwrap();
            }
            // tx dropped here — signals end of stream to receiver
        });
        producer.join().unwrap();
        // Drain all messages
        for msg in rx {
            print!("    recv: \"{}\"  ", msg);
        }
        println!();
    }
    println!();

    // Multiple producers (clone Sender)
    println!("  3b. Multiple producers, one consumer:");
    {
        let (tx, rx) = std::sync::mpsc::channel::<(usize, i64)>();
        let handles: Vec<_> = (0..3)
            .map(|i| {
                let tx = tx.clone();
                thread::spawn(move || {
                    let result: i64 = (1..=100).map(|n| n as i64).sum();
                    tx.send((i, result)).unwrap();
                })
            })
            .collect();
        drop(tx); // drop original sender so channel closes when all clones are gone
        for h in handles { h.join().unwrap(); }
        let mut results: Vec<_> = rx.iter().collect();
        results.sort();
        for (id, val) in results {
            println!("    producer {} computed sum = {}", id, val);
        }
    }
    println!();

    // Bounded / sync channel — back-pressure
    println!("  3c. Bounded channel — back-pressure (capacity = 2):");
    {
        let (tx, rx) = std::sync::mpsc::sync_channel::<i32>(2);
        let producer = thread::spawn(move || {
            for n in 0..5 {
                println!("    [producer] sending {}", n);
                tx.send(n).unwrap(); // blocks when buffer is full
            }
        });
        thread::sleep(Duration::from_millis(5));
        for n in rx {
            println!("    [consumer] received {}", n);
        }
        producer.join().unwrap();
    }
    println!();

    // Work-queue pattern: distribute tasks across worker threads
    println!("  3d. Work-queue pattern:");
    {
        let (tx, rx) = std::sync::mpsc::channel::<u64>();
        let rx = Arc::new(Mutex::new(rx));
        let (result_tx, result_rx) = std::sync::mpsc::channel::<u64>();

        // Spawn 3 workers
        let workers: Vec<_> = (0..3)
            .map(|id| {
                let rx = Arc::clone(&rx);
                let result_tx = result_tx.clone();
                thread::spawn(move || {
                    loop {
                        let task = { rx.lock().unwrap().recv() };
                        match task {
                            Ok(n) => {
                                let r = n * n;
                                println!("    [worker {}] {}² = {}", id, n, r);
                                result_tx.send(r).unwrap();
                            }
                            Err(_) => break, // channel closed
                        }
                    }
                })
            })
            .collect();
        drop(result_tx);

        // Send tasks
        for n in [2u64, 3, 5, 7, 11] { tx.send(n).unwrap(); }
        drop(tx); // close channel → workers exit loop

        for w in workers { w.join().unwrap(); }
        let mut results: Vec<u64> = result_rx.iter().collect();
        results.sort();
        println!("    sorted squares: {:?}", results);
    }
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Synchronisation Primitives
// ─────────────────────────────────────────────────────────────────────────────

fn section4_sync_primitives() {
    println!("=== Section 4: Synchronisation Primitives ===");
    println!();

    // Barrier — coordinate N threads to a rendezvous point
    println!("  4a. Barrier — rendezvous point:");
    {
        let barrier = Arc::new(Barrier::new(3));
        let handles: Vec<_> = (0..3)
            .map(|i| {
                let b = Arc::clone(&barrier);
                thread::spawn(move || {
                    println!("    [thread {}] phase 1 work done", i);
                    b.wait(); // blocks until all 3 reach here
                    println!("    [thread {}] phase 2 started", i);
                })
            })
            .collect();
        for h in handles { h.join().unwrap(); }
    }
    println!();

    // Condvar — wait for a condition
    println!("  4b. Condvar — producer/consumer with condition variable:");
    {
        let pair = Arc::new((Mutex::new(VecDeque::<i32>::new()), Condvar::new()));
        let pair_prod = Arc::clone(&pair);
        let pair_cons = Arc::clone(&pair);

        let producer = thread::spawn(move || {
            let (lock, cvar) = &*pair_prod;
            for n in 1..=4 {
                let mut q = lock.lock().unwrap();
                q.push_back(n);
                println!("    [producer] pushed {}", n);
                cvar.notify_one();
            }
        });

        let consumer = thread::spawn(move || {
            let (lock, cvar) = &*pair_cons;
            for _ in 0..4 {
                let mut q = lock.lock().unwrap();
                while q.is_empty() {
                    q = cvar.wait(q).unwrap();
                }
                let v = q.pop_front().unwrap();
                println!("    [consumer] received {}", v);
            }
        });

        producer.join().unwrap();
        consumer.join().unwrap();
    }
    println!();

    // Atomic types — lock-free counter
    println!("  4c. Atomic types — lock-free shared counter:");
    {
        let atomic_count = Arc::new(AtomicUsize::new(0));
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = Arc::clone(&atomic_count);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        c.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();
        for h in handles { h.join().unwrap(); }
        println!("    atomic counter after 4×1000 increments = {}",
                 atomic_count.load(Ordering::SeqCst));
        println!("    (no Mutex needed — hardware atomic instruction)");
    }
    println!();

    println!("  4d. Ordering semantics (brief):");
    println!("    Relaxed    — no ordering guarantee; cheapest; ok for counters");
    println!("    Acquire    — loads before this point cannot be reordered after");
    println!("    Release    — stores after this point cannot be reordered before");
    println!("    AcqRel     — both Acquire and Release");
    println!("    SeqCst     — total sequential consistency; most expensive");
    println!();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Patterns and Antipatterns
// ─────────────────────────────────────────────────────────────────────────────

fn section5_patterns_antipatterns() {
    println!("=== Section 5: Patterns and Antipatterns ===");
    println!();

    println!("  ── PATTERNS (good) ──────────────────────────────────────────────");
    println!();
    println!("  P1. Prefer channels over shared state for decoupling:");
    println!("    Don't share mutable data — send owned messages instead.");
    println!("    mpsc channels are zero-copy when T is already on the heap.");
    println!();
    println!("  P2. Minimise lock scope — hold guard only as long as needed:");
    println!("    let value = {{ counter.lock().unwrap().clone() }};");
    println!("    // guard dropped; do heavy work with `value` outside the lock");
    println!();
    println!("  P3. Use atomic types for simple counters/flags:");
    println!("    AtomicBool for a 'shutdown' flag, AtomicUsize for metrics.");
    println!("    Avoids Mutex overhead for single-value updates.");
    println!();
    println!("  P4. Consistent lock ordering prevents deadlock:");
    println!("    Always acquire mutexes in the same global order across threads.");
    println!();
    println!("  P5. Use Arc::clone explicitly to share ownership:");
    println!("    let shared = Arc::clone(&data);  // clear, no implicit copies");
    println!();
    println!("  P6. drop(tx) to signal channel closure:");
    println!("    Dropping all Senders causes rx.recv() to return Err,");
    println!("    letting worker threads exit their loops cleanly.");
    println!();

    println!("  ── ANTIPATTERNS (avoid) ─────────────────────────────────────────");
    println!();
    println!("  A1. Holding a Mutex guard across an await or long operation:");
    println!("    let _guard = m.lock().unwrap();");
    println!("    expensive_io();  // other threads blocked the whole time");
    println!("    // Solution: clone data out, release guard, then do work.");
    println!();
    println!("  A2. Calling .unwrap() on a poisoned Mutex:");
    println!("    If a thread panics while holding a lock, the Mutex is 'poisoned'.");
    println!("    .lock().unwrap() then propagates the panic to all other threads.");
    println!("    Handle with: .lock().unwrap_or_else(|p| p.into_inner())");
    println!();
    println!("  A3. Arc without Mutex for shared mutation:");
    println!("    Arc<Vec<i32>> does NOT allow mutation — Arc gives shared &T.");
    println!("    You need Arc<Mutex<Vec<i32>>> for thread-safe mutation.");
    println!("    The compiler will reject the unsafe attempt.");
    println!();
    println!("  A4. Using Rc<T> or RefCell<T> across threads:");
    println!("    Rc is not Send; RefCell is not Sync.");
    println!("    Use Arc and Mutex/RwLock instead — the compiler enforces this.");
    println!();
    println!("  A5. Spawning threads without joining — silent failures:");
    println!("    thread::spawn(|| risky());  // panic is swallowed if not joined");
    println!("    Always store and join handles, or use a scoped thread pool.");
    println!();
    println!("  A6. Busy-waiting instead of Condvar or channel:");
    println!("    while !*flag.lock().unwrap() {{ thread::yield_now(); }}");
    println!("    Burns CPU and still needs synchronisation. Use Condvar::wait().");
    println!();
    println!("  Summary:");
    println!("    Rust's type system makes data races impossible at compile time.");
    println!("    Send — a type that is safe to transfer to another thread.");
    println!("    Sync — a type whose reference is safe to share across threads.");
    println!("    Arc<Mutex<T>>: share + mutate.  mpsc channels: communicate.");
    println!("    The compiler rejects every unsafe sharing pattern — 'fearless concurrency'.");
}
