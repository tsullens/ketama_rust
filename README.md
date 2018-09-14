# ketama_rust

A thread-safe, via rwlock mutex, lib meant to implement the simple ketama
consistent hashing algorithm.
I was bored.  
* [Reference 1](https://www.metabrew.com/article/libketama-consistent-hashing-algo-memcached-clients)  
* [Reference 2](https://www.akamai.com/us/en/multimedia/documents/technical-publication/consistent-hashing-and-random-trees-distributed-caching-protocols-for-relieving-hot-spots-on-the-world-wide-web-technical-publication.pdf)   

The library is in `ketama.rs` while `main.rs` just runs some testing / examples.  

## To-Do
* Add `ketama_add_server` and `ketama_burn_server` methods
