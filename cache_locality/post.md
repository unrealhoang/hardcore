# Cache locality

In today's modern processors, the speed of one CPU cycle is much faster than the time it takes to access data on RAM, so if we are working with data structure on RAM, we would waste a lot of our CPU time waiting for RAM accessing.
Fortunately, the chip designer/maker, they have the solution for this problem: CPU Cache.

## CPU Cache

You can read the definition of CPU Cache [here](https://en.wikipedia.org/wiki/CPU_cache), or you can read my TL;DR version: 
There is some small (in computer space size) but very fast caches that are local to the CPU itself. It allows CPU to access data as fast as the speed of CPU cycle. The higher level of the cache, the slower it becomes (in exchange for bigger space). Usuallly the lower level is use for indexing data of the upper level.

For Example: the Core i7 Xeon Processor

```
Core i7 Xeon 5500 Series Data Source Latency (approximate)               [Pg. 22]

local  L1 CACHE hit,                              ~4 cycles (   2.1 -  1.2 ns )
local  L2 CACHE hit,                             ~10 cycles (   5.3 -  3.0 ns )
local  L3 CACHE hit, line unshared               ~40 cycles (  21.4 - 12.0 ns )
local  L3 CACHE hit, shared line in another core ~65 cycles (  34.8 - 19.5 ns )
local  L3 CACHE hit, modified in another core    ~75 cycles (  40.2 - 22.5 ns )

remote L3 CACHE (Ref: Fig.1 [Pg. 5])        ~100-300 cycles ( 160.7 - 30.0 ns )

local  DRAM                                                   ~60 ns
remote DRAM                                                  ~100 ns
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

