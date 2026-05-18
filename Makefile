.PHONY: all rust cpp run-rust run-cpp run-rust-mem run-cpp-mem run-rust-lt run-cpp-lt run-rust-stg run-cpp-stg run-rust-cl run-cpp-cl run-rust-it run-cpp-it clean

CXX      ?= clang++
CXXFLAGS  = -std=c++17 -Wall -Wextra -Wpedantic -O2

all: rust cpp

rust:
	cd rust && cargo build --release 2>&1

cpp: cpp/ownership_demo cpp/memory_demo cpp/lifetime_demo cpp/structs_traits_generics cpp/closures_demo cpp/iterators_demo

cpp/ownership_demo: cpp/ownership_demo.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

cpp/memory_demo: cpp/memory_demo.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

cpp/lifetime_demo: cpp/lifetime_demo.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

cpp/structs_traits_generics: cpp/structs_traits_generics.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

cpp/closures_demo: cpp/closures_demo.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

cpp/iterators_demo: cpp/iterators_demo.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

run-rust: rust
	@echo "──────────────────────────────────────"
	@echo " RUST: Ownership Demo"
	@echo "──────────────────────────────────────"
	./rust/target/release/ownership_demo

run-cpp: cpp
	@echo "──────────────────────────────────────"
	@echo " C++: Ownership Demo"
	@echo "──────────────────────────────────────"
	./cpp/ownership_demo

run-rust-mem: rust
	@echo "──────────────────────────────────────"
	@echo " RUST: Memory Demo"
	@echo "──────────────────────────────────────"
	./rust/target/release/memory_demo

run-cpp-mem: cpp
	@echo "──────────────────────────────────────"
	@echo " C++: Memory Demo"
	@echo "──────────────────────────────────────"
	./cpp/memory_demo

run-rust-lt: rust
	@echo "──────────────────────────────────────"
	@echo " RUST: Lifetime Demo"
	@echo "──────────────────────────────────────"
	./rust/target/release/lifetime_demo

run-cpp-lt: cpp
	@echo "──────────────────────────────────────"
	@echo " C++: Lifetime Demo"
	@echo "──────────────────────────────────────"
	./cpp/lifetime_demo

run-rust-stg: rust
	@echo "──────────────────────────────────────"
	@echo " RUST: Structs, Traits, Generics"
	@echo "──────────────────────────────────────"
	./rust/target/release/structs_traits_generics

run-cpp-stg: cpp
	@echo "──────────────────────────────────────"
	@echo " C++: Classes, Interfaces, Templates"
	@echo "──────────────────────────────────────"
	./cpp/structs_traits_generics

run-rust-cl: rust
	@echo "──────────────────────────────────────"
	@echo " RUST: Closures Demo"
	@echo "──────────────────────────────────────"
	./rust/target/release/closures_demo

run-cpp-cl: cpp
	@echo "──────────────────────────────────────"
	@echo " C++: Closures (Lambdas) Demo"
	@echo "──────────────────────────────────────"
	./cpp/closures_demo

run-rust-it: rust
	@echo "──────────────────────────────────────"
	@echo " RUST: Iterators Demo"
	@echo "──────────────────────────────────────"
	./rust/target/release/iterators_demo

run-cpp-it: cpp
	@echo "──────────────────────────────────────"
	@echo " C++: Iterators Demo"
	@echo "──────────────────────────────────────"
	./cpp/iterators_demo

clean:
	cd rust && cargo clean
	rm -f cpp/ownership_demo cpp/memory_demo cpp/lifetime_demo cpp/structs_traits_generics cpp/closures_demo cpp/iterators_demo
