# ketama_rust

A thread-safe, via rwlock mutex, lib meant to implement the simple ketama 
consistent hashing algorithm.
[Reference](https://www.metabrew.com/article/libketama-consistent-hashing-algo-memcached-clients)

The library is in `ketama.rs` while `main.rs` just runs some testing / examples.

## To-Do
* Add `ketama_add_server` and `ketama_burn_server` methods
