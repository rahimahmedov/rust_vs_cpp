// C++ Concurrency Demo
//
// Sections:
//   1 — Spawning threads: std::thread
//   2 — Shared state: std::mutex, std::shared_mutex
//   3 — Message passing: condition_variable queue and std::future/promise
//   4 — Synchronisation primitives: barrier, atomic, latch (C++20)
//   5 — Patterns and antipatterns
//
// C++17 (core); C++20 latch noted separately.
// clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2 -o concurrency_demo concurrency_demo.cpp

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <deque>
#include <future>
#include <iostream>
#include <mutex>
#include <numeric>
#include <queue>
#include <shared_mutex>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

// A mutex protecting std::cout so output lines don't interleave
static std::mutex g_print_mx;
#define PRINT(...) do { \
    std::lock_guard<std::mutex> _lg(g_print_mx); \
    std::cout << __VA_ARGS__ << "\n"; \
} while(0)

static void section1_thread_basics();
static void section2_shared_state();
static void section3_message_passing();
static void section4_sync_primitives();
static void section5_patterns_antipatterns();

int main() {
    section1_thread_basics();
    section2_shared_state();
    section3_message_passing();
    section4_sync_primitives();
    section5_patterns_antipatterns();
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 — Thread Basics
// ─────────────────────────────────────────────────────────────────────────────

static void section1_thread_basics() {
    std::cout << "=== Section 1: Spawning Threads ===\n";
    std::cout << "std::thread wraps an OS thread. Callable + args passed to constructor.\n";
    std::cout << "  Must call .join() or .detach() before thread object is destroyed.\n";
    std::cout << "  Unlike Rust, the compiler does NOT prevent data races at compile time.\n\n";

    // Basic spawn + join
    std::cout << "  1a. Basic thread and join:\n";
    {
        std::thread t([]() {
            PRINT("    [child] hello from thread " << std::this_thread::get_id());
        });
        t.join();
        std::cout << "    [main]  child finished\n\n";
    }

    // Thread with arguments
    std::cout << "  1b. Passing data to a thread (copy vs reference):\n";
    {
        int value = 42;
        // Pass by value — thread gets its own copy
        std::thread t_copy([](int v) {
            PRINT("    [thread-copy] v=" << v);
        }, value);
        // Pass by reference — UB if `value` goes out of scope before join!
        // Use std::ref to pass a reference safely:
        int result = 0;
        std::thread t_ref([](int v, int& out) {
            out = v * 2;
        }, value, std::ref(result));
        t_copy.join();
        t_ref.join();
        std::cout << "    result via ref = " << result << "\n\n";
    }

    // Spawn N threads
    std::cout << "  1c. Spawning N threads, waiting for all:\n";
    {
        std::vector<std::thread> threads;
        std::vector<int> squares(4);
        for (int i = 0; i < 4; ++i) {
            threads.emplace_back([i, &squares]() {
                squares[static_cast<std::size_t>(i)] = i * i;
                PRINT("    [thread " << i << "] computed " << i << "^2");
            });
        }
        for (auto& t : threads) t.join();
        std::cout << "    squares: ";
        for (int s : squares) std::cout << s << " ";
        std::cout << "\n\n";
    }

    // Named thread via hardware_concurrency hint
    std::cout << "  1d. Hardware concurrency hint:\n";
    std::cout << "    std::thread::hardware_concurrency() = "
              << std::thread::hardware_concurrency() << "\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 — Shared State
// ─────────────────────────────────────────────────────────────────────────────

static void section2_shared_state() {
    std::cout << "=== Section 2: Shared State ===\n";
    std::cout << "C++ shared state is NOT protected by the type system.\n";
    std::cout << "  std::mutex        — exclusive lock (like Rust Mutex<T>)\n";
    std::cout << "  std::shared_mutex — reader-writer lock (like Rust RwLock<T>)\n";
    std::cout << "  std::lock_guard   — RAII lock; released when guard goes out of scope\n";
    std::cout << "  std::unique_lock  — movable, deferrable, works with condition_variable\n\n";

    // Mutex + lock_guard — shared counter
    std::cout << "  2a. std::mutex — shared counter:\n";
    {
        std::mutex mx;
        unsigned long counter = 0;
        std::vector<std::thread> threads;
        for (int i = 0; i < 8; ++i) {
            threads.emplace_back([&mx, &counter]() {
                std::lock_guard<std::mutex> guard(mx);
                ++counter;
            });
        }
        for (auto& t : threads) t.join();
        std::cout << "    counter after 8 increments = " << counter << "\n\n";
    }

    // shared_mutex — many readers, one writer
    std::cout << "  2b. std::shared_mutex — readers and writer:\n";
    {
        std::shared_mutex rw_mx;
        std::vector<std::string> config = {"host=localhost", "port=8080"};

        std::vector<std::thread> readers;
        for (int i = 0; i < 3; ++i) {
            readers.emplace_back([i, &rw_mx, &config]() {
                std::shared_lock<std::shared_mutex> lock(rw_mx);
                PRINT("    [reader " << i << "] entries: " << config.size());
            });
        }
        for (auto& t : readers) t.join();

        {
            std::unique_lock<std::shared_mutex> lock(rw_mx);
            config.push_back("timeout=30");
            std::cout << "    [writer] added entry; total: " << config.size() << "\n";
        }
        std::cout << "\n";
    }

    // std::lock() — acquire multiple locks without deadlock
    std::cout << "  2c. std::lock() — deadlock-free multi-lock acquisition:\n";
    {
        std::mutex mx_a, mx_b;
        auto thread_fn = [&](const char* name) {
            std::unique_lock<std::mutex> la(mx_a, std::defer_lock);
            std::unique_lock<std::mutex> lb(mx_b, std::defer_lock);
            std::lock(la, lb);  // acquires both atomically — no fixed order needed
            PRINT("    [" << name << "] holds both locks safely");
        };
        std::thread t1(thread_fn, "t1");
        std::thread t2(thread_fn, "t2");
        t1.join(); t2.join();
    }
    std::cout << "\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 — Message Passing
// ─────────────────────────────────────────────────────────────────────────────

// Thread-safe bounded queue — mirrors Rust mpsc::sync_channel
template<typename T>
class Channel {
public:
    explicit Channel(std::size_t cap = 0) : cap_(cap) {}

    void send(T value) {
        std::unique_lock<std::mutex> lock(mx_);
        if (cap_ > 0)
            not_full_.wait(lock, [this] { return q_.size() < cap_ || closed_; });
        q_.push_back(std::move(value));
        not_empty_.notify_one();
    }

    // Returns false when channel is closed and empty
    bool recv(T& out) {
        std::unique_lock<std::mutex> lock(mx_);
        not_empty_.wait(lock, [this] { return !q_.empty() || closed_; });
        if (q_.empty()) return false;
        out = std::move(q_.front());
        q_.pop_front();
        if (cap_ > 0) not_full_.notify_one();
        return true;
    }

    void close() {
        std::lock_guard<std::mutex> lock(mx_);
        closed_ = true;
        not_empty_.notify_all();
        not_full_.notify_all();
    }

private:
    std::deque<T>           q_;
    std::mutex              mx_;
    std::condition_variable not_empty_, not_full_;
    std::size_t             cap_;
    bool                    closed_{false};
};

static void section3_message_passing() {
    std::cout << "=== Section 3: Message Passing ===\n";
    std::cout << "C++ has no built-in channel. Patterns:\n";
    std::cout << "  std::promise/future  — single one-shot value transfer\n";
    std::cout << "  condition_variable + queue — mpsc / spsc channel (manual)\n";
    std::cout << "  Rust contrast: mpsc::channel is in std, owns the message.\n\n";

    // std::promise / std::future — one-shot value transfer
    std::cout << "  3a. std::promise / std::future — single result:\n";
    {
        std::promise<int> p;
        std::future<int>  f = p.get_future();
        std::thread producer([](std::promise<int> p_) {
            int sum = 0;
            for (int i = 1; i <= 100; ++i) sum += i;
            PRINT("    [producer] computed sum, setting promise");
            p_.set_value(sum);
        }, std::move(p));
        std::cout << "    [consumer] future.get() = " << f.get() << "\n";
        producer.join();
    }
    std::cout << "\n";

    // std::async — higher-level future
    std::cout << "  3b. std::async — fire-and-forget with future:\n";
    {
        auto fut = std::async(std::launch::async, []() -> double {
            double acc = 0;
            for (int i = 1; i <= 1000; ++i) acc += 1.0 / i;
            return acc;
        });
        std::cout << "    harmonic series H(1000) ≈ " << fut.get() << "\n";
    }
    std::cout << "\n";

    // Custom Channel — multiple producers, single consumer
    std::cout << "  3c. Multi-producer single-consumer queue (custom Channel<T>):\n";
    {
        Channel<std::pair<int,long>> ch;
        std::vector<std::thread> producers;
        for (int i = 0; i < 3; ++i) {
            producers.emplace_back([i, &ch]() {
                static constexpr int arr[] = {1,2,3,4,5,6,7,8,9,10};
                long sum = std::accumulate(std::begin(arr), std::end(arr), 0L);
                ch.send({i, sum});
            });
        }
        for (auto& t : producers) t.join();
        ch.close();

        std::pair<int,long> msg;
        std::vector<std::pair<int,long>> results;
        while (ch.recv(msg)) results.push_back(msg);
        std::sort(results.begin(), results.end());
        for (auto& [id, val] : results)
            std::cout << "    producer " << id << " result = " << val << "\n";
    }
    std::cout << "\n";

    // Work-queue / thread pool pattern
    std::cout << "  3d. Work-queue: distribute tasks to worker threads:\n";
    {
        Channel<int> task_ch(0);   // unbounded
        Channel<long> result_ch(0);

        // 3 workers
        std::vector<std::thread> workers;
        for (int id = 0; id < 3; ++id) {
            workers.emplace_back([id, &task_ch, &result_ch]() {
                int task;
                while (task_ch.recv(task)) {
                    long r = (long)task * task;
                    PRINT("    [worker " << id << "] " << task << "^2 = " << r);
                    result_ch.send(r);
                }
            });
        }

        for (int n : {2, 3, 5, 7, 11}) task_ch.send(n);
        task_ch.close();
        for (auto& w : workers) w.join();
        result_ch.close();

        std::vector<long> results;
        long r;
        while (result_ch.recv(r)) results.push_back(r);
        std::sort(results.begin(), results.end());
        std::cout << "    sorted squares: ";
        for (long v : results) std::cout << v << " ";
        std::cout << "\n\n";
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 — Synchronisation Primitives
// ─────────────────────────────────────────────────────────────────────────────

static void section4_sync_primitives() {
    std::cout << "=== Section 4: Synchronisation Primitives ===\n\n";

    // condition_variable — wait for a condition
    std::cout << "  4a. condition_variable — producer/consumer:\n";
    {
        std::mutex mx;
        std::condition_variable cv;
        std::queue<int> q;
        bool done = false;

        std::thread producer([&]() {
            for (int n = 1; n <= 4; ++n) {
                {
                    std::lock_guard<std::mutex> lock(mx);
                    q.push(n);
                    PRINT("    [producer] pushed " << n);
                }
                cv.notify_one();
            }
            {
                std::lock_guard<std::mutex> lock(mx);
                done = true;
            }
            cv.notify_one();
        });

        std::thread consumer([&]() {
            while (true) {
                std::unique_lock<std::mutex> lock(mx);
                cv.wait(lock, [&] { return !q.empty() || done; });
                while (!q.empty()) {
                    PRINT("    [consumer] received " << q.front());
                    q.pop();
                }
                if (done) break;
            }
        });

        producer.join();
        consumer.join();
    }
    std::cout << "\n";

    // once_flag — initialise exactly once (thread-safe singleton pattern)
    std::cout << "  4b. std::once_flag — thread-safe one-time initialisation:\n";
    {
        std::once_flag flag;
        std::string shared_config;
        auto init = [&]() {
            shared_config = "config loaded";
            std::cout << "    [init] ran once\n";
        };
        std::vector<std::thread> ts;
        for (int i = 0; i < 4; ++i) {
            ts.emplace_back([&]() {
                std::call_once(flag, init);
                PRINT("    [thread] config = \"" << shared_config << "\"");
            });
        }
        for (auto& t : ts) t.join();
    }
    std::cout << "\n";

    // Atomic — lock-free counter
    std::cout << "  4c. std::atomic — lock-free counter:\n";
    {
        std::atomic<unsigned long> counter{0};
        std::vector<std::thread> ts;
        for (int i = 0; i < 4; ++i) {
            ts.emplace_back([&counter]() {
                for (int j = 0; j < 1000; ++j)
                    counter.fetch_add(1, std::memory_order_relaxed);
            });
        }
        for (auto& t : ts) t.join();
        std::cout << "    atomic counter after 4×1000 = "
                  << counter.load(std::memory_order_seq_cst) << "\n";
        std::cout << "    (no mutex — hardware atomic instruction)\n\n";
    }

    // Atomic flag as a shutdown signal
    std::cout << "  4d. std::atomic<bool> as a shutdown flag:\n";
    {
        std::atomic<bool> stop{false};
        std::thread worker([&stop]() {
            int ticks = 0;
            while (!stop.load(std::memory_order_acquire)) {
                ++ticks;
                std::this_thread::sleep_for(std::chrono::milliseconds(1));
            }
            PRINT("    [worker] exited after " << ticks << " ticks");
        });
        std::this_thread::sleep_for(std::chrono::milliseconds(5));
        stop.store(true, std::memory_order_release);
        worker.join();
    }
    std::cout << "\n";

    std::cout << "  4e. Memory order cheat-sheet:\n";
    std::cout << "    memory_order_relaxed  — no ordering; cheapest; ok for counters\n";
    std::cout << "    memory_order_acquire  — see all writes before the corresponding release\n";
    std::cout << "    memory_order_release  — make writes visible before the acquire load\n";
    std::cout << "    memory_order_acq_rel  — both acquire and release\n";
    std::cout << "    memory_order_seq_cst  — total order; matches Rust Ordering::SeqCst\n\n";
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 — Patterns and Antipatterns
// ─────────────────────────────────────────────────────────────────────────────

static void section5_patterns_antipatterns() {
    std::cout << "=== Section 5: Patterns and Antipatterns ===\n\n";

    std::cout << "  ── PATTERNS (good) ──────────────────────────────────────────\n\n";

    std::cout << "  P1. Always use RAII locks — never lock/unlock manually:\n";
    std::cout << "    std::lock_guard  — simplest, no move\n";
    std::cout << "    std::unique_lock — movable, works with condition_variable\n";
    std::cout << "    std::scoped_lock — locks multiple mutexes atomically (C++17)\n\n";

    std::cout << "  P2. Use std::scoped_lock for multiple mutexes (C++17):\n";
    {
        std::mutex ma, mb;
        std::scoped_lock guard(ma, mb); // deadlock-free, single RAII object
        std::cout << "    std::scoped_lock acquired ma and mb atomically\n\n";
    }

    std::cout << "  P3. Minimise critical section size:\n";
    std::cout << "    { std::lock_guard lock(mx); data = shared; }  // lock released\n";
    std::cout << "    process(data);  // heavy work done outside the lock\n\n";

    std::cout << "  P4. Use std::atomic for simple flags and counters:\n";
    std::cout << "    std::atomic<bool> stop{false};   // shutdown flag\n";
    std::cout << "    std::atomic<int>  count{0};      // statistics\n\n";

    std::cout << "  P5. Use std::async / std::future for task-based parallelism:\n";
    std::cout << "    Avoids manual thread management for one-shot computations.\n\n";

    std::cout << "  P6. Prefer std::jthread (C++20) — auto-joins on destruction:\n";
    std::cout << "    std::jthread t([] { /* work */ });  // RAII, no explicit join\n\n";

    std::cout << "  ── ANTIPATTERNS (avoid) ─────────────────────────────────────\n\n";

    std::cout << "  A1. Forgetting to join or detach a thread → std::terminate:\n";
    std::cout << "    std::thread t(work);  // if t goes out of scope → terminate()\n";
    std::cout << "    Always join() or detach() before thread object is destroyed.\n\n";

    std::cout << "  A2. Sharing raw pointers / references without synchronisation:\n";
    std::cout << "    int x = 0;\n";
    std::cout << "    std::thread t([&x] { x++; });  // data race — UB!\n";
    std::cout << "    Protect with mutex or use std::atomic.\n\n";

    std::cout << "  A3. Holding a lock while calling user code / callbacks:\n";
    std::cout << "    // lock held → callback acquires same lock → deadlock\n";
    std::cout << "    std::lock_guard lock(mx);\n";
    std::cout << "    observer.notify();  // observer may call back into locked code\n\n";

    std::cout << "  A4. Using condition_variable without a predicate (spurious wakeups):\n";
    std::cout << "    cv.wait(lock);                       // WRONG — spurious wakeup\n";
    std::cout << "    cv.wait(lock, [&]{ return !q.empty(); }); // correct\n\n";

    std::cout << "  A5. Locking in a specific order inconsistently (deadlock):\n";
    std::cout << "    Thread A: lock(mx1) then lock(mx2)\n";
    std::cout << "    Thread B: lock(mx2) then lock(mx1)  → deadlock\n";
    std::cout << "    Fix: use std::scoped_lock(mx1, mx2) in both threads.\n\n";

    std::cout << "  A6. Detaching threads that reference local variables:\n";
    std::cout << "    void foo() {\n";
    std::cout << "        int local = 0;\n";
    std::cout << "        std::thread t([&local] { local++; });\n";
    std::cout << "        t.detach();  // foo() returns, local destroyed → UB\n";
    std::cout << "    }\n\n";

    std::cout << "  A7. Using volatile instead of std::atomic for shared state:\n";
    std::cout << "    volatile int flag = 0;  // NOT thread-safe\n";
    std::cout << "    volatile prevents compiler optimisation but gives NO memory\n";
    std::cout << "    ordering guarantees. Use std::atomic.\n\n";

    std::cout << "  Summary:\n";
    std::cout << "    C++ gives you raw power but NO compile-time race detection.\n";
    std::cout << "    Rust comparison:\n";
    std::cout << "      Rust Arc<Mutex<T>>      ↔  C++ shared_ptr<T> + mutex\n";
    std::cout << "      Rust Arc<RwLock<T>>     ↔  C++ shared_ptr<T> + shared_mutex\n";
    std::cout << "      Rust mpsc::channel      ↔  C++ Channel<T> (manual, above)\n";
    std::cout << "      Rust Send + Sync traits ↔  C++ (no equivalent, no enforcement)\n";
    std::cout << "      Rust atomic + Ordering  ↔  C++ std::atomic + memory_order\n";
    std::cout << "    In Rust, sharing &mut T across threads is a compile error.\n";
    std::cout << "    In C++, it compiles — and silently causes undefined behaviour.\n";
}
