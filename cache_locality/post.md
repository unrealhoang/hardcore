# Cache locality

In today's modern processors, the speed of one CPU cycle is much faster than the time it takes to access data on RAM, so if we are working with data structure on RAM, we would waste a lot of our CPU time waiting for RAM accessing.
Fortunately, the chip designer/maker, they have the solution for this problem: CPU Cache.

## CPU Cache

You can read the definition of CPU Cache [here](https://en.wikipedia.org/wiki/CPU_cache), or you can read my TL;DR version:  
There is some small (in computer space size) but very fast caches that are local to the CPU itself. It allows CPU to access data as fast as the speed of CPU cycle. The higher level of the cache, the slower it becomes (in exchange for bigger space). Usuallly the lower level is use for indexing data of the upper level.

For Example: the Core i7 Xeon Processor

```
Latency Comparison Numbers
--------------------------
L1 cache reference                           0.5 ns
Branch mispredict                            5   ns
L2 cache reference                           7   ns                      14x L1 cache
Mutex lock/unlock                           25   ns
Main memory reference                      100   ns                      20x L2 cache, 200x L1 cache

Notes
-----
1 ns = 10^-9 seconds
1 us = 10^-6 seconds = 1,000 ns
1 ms = 10^-3 seconds = 1,000 us = 1,000,000 ns
```

We can see from the example above that, if we don't use the CPU Cache at all, one access to RAM will cost us about ~60ns (~100 CPU Cycle). Now we know about the CPU Cache, how do we utilize the power of the cache?

## Linked List vs Array

How is cache-locality affects us in our real-world problem, let's compare the two most popular datastructure, Linked List and Array

Linked List:
```rust
#[bench]
fn bench_linked_list_traversal(b: &mut Bencher) {
    let mut list = LinkedList::new();
    for e in (0..1000) {
        list.push_back(e);
    }
    let mut sum = 0;
    b.iter(move || {
        for i in list.iter() { sum = sum + i; };
        sum
    });
}
```

Array-based:
```rust
#[bench]
fn bench_array_traveral(b: &mut Bencher) {
    let mut ary = Vec::new();
    for e in (0..1000) {
        ary.push(e);
    }
    let mut sum = 0;
    b.iter(move || {
        for i in ary.iter() { sum = sum + i; };
        sum
    });
}
```

Result:
```
bench_array_traveral        ... bench:    58 ns/iter (+/- 40)
bench_linked_list_traversal ... bench: 1,456 ns/iter (+/- 197)
```

What, wait, according to complexity (Big-O) of our operation, those two are the same (O(N)), how come the result is so much different (25 times).  
It turns out, when we read an exact address in memory (RAM), the processor will also load the nearby memory block onto the CPU cache. I.e. Reading address 1000 in the memory will also load address 1001, 1002, 1003... to the CPU cache so the next read will be a cache hit, thus avoid reading the RAM entirely.  
Array is an exact match for this behaviour, LinkedList, on the other hand, having its elements scattered in the heap (i.e., not sequential), everytime it accesses the next element in the list will likely be a cache-miss, and reading from RAM is much slower as we discovered above.

## References
1. Latency every developer should know - [StackOverflow](https://gist.github.com/jboner/2841832)
2. Benchmark code can be read/copy [here](https://github.com/unrealhoang/hardcore/blob/master/cache_locality/list_v_array/src/lib.rs)
