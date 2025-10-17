# dicers
rust implementation of dice

testing here right now, subscribers and logic around them will me moved into own project later

## run examples:
### malloc_exmaple:
```
LD_PRELOAD=dice/build/src/dice/libdice.so:dice/build/src/mod/dice-malloc.so:dice/build/src/mod/dice-self.so:target/debug/libdicers.so ./tests/malloc_example
```

### atomic_test
```
clang++ -std=c++17 -Wall -Wextra -pthread -fsanitize=thread -shared-libsan tests/atomic_test.cc -o tests/atomic_test

TSANO_LIBDIR=dice/build/deps/tsano/ ./dice/deps/tsano/tsano LD_PRELOAD=dice/build/src/dice/libdice.so:dice/build/src/mod/dice-malloc.so:dice/build/src/mod/dice-self.so:dice/build/src/mod/dice-tsan.so:target/debug/libdicers.so ./tests/atomic_test
```