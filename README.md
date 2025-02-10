# X-Math

X-Math is a high-performance mathematical library originally written in Jai, then translated to C and Rust. It provides optimized implementations of common mathematical functions with significant speed improvements over standard libc functions.

### Features:

- Highly optimized mathematical functions
- Faster than standard libc implementations
- Available in both accurate and ultra-fast approximations
- Written in Jai, C and Rust

### Performance Comparison:

The following table compares the performance of X-Math against standard libc functions. Benchmarks were written in C and compiled with `-Ofast -fno-builtin` to ensure that libc does not use SSE instructions (sqrtss, roundss, etc.).

| Function | libc (ns) | x-math++ (ns) | Speedup (x) | Error++ | x-math (ns) | Speed | Error |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `sin` | 894301 | 360380 | x2.5 | 0.000071 | --- | --- | --- |
| `cos` | 905995 | 323477 | x3 | 0.000067 | --- | --- | --- |
| `tan` | 2258915 | 270688 | x8 | 0.019074 | --- | --- | --- |
| `atan2` | 1771537 | 72401 | x25 | 0.0002 | --- | --- | --- |
| `asin` | 1126285 | 344500 | x3.3 | 0.003628 | 83723 | x13 | 0.021762 |
| `acos` | 1109464 | 462021 | x2.4 | 0.003628 | 98461 | x11 | 0.021761 |
| `sinh` | 1174550 | 274020 | x4.3 | 0.002364 | 62051 | x19 | 0.035897 |
| `cosh` | 1204881 | 296126 | x4 | 0.001673 | 51518 | x23 | 0.045089 |
| `tanh` | 348166 | 128467 | x2.7 | 0.006632 | --- | --- | --- |
| `abs` | 11837 | 3478 | x3.4 | --- | --- | --- | --- |
| `trunc` | 187726 | 83275 | x32 | --- | --- | --- | --- |
| `floor` | 205602 | 25818 | x8 | --- | --- | --- | --- |
| `ceil` | 203320 | 55778 | x3.6 | --- | --- | --- | --- |
| `round` | 318375 | 23788 | x13 | --- | --- | --- | --- |
| `mod` | 1298443 | 42901 | x30 | --- | --- | --- | --- |
| `fract` | 266085 | 74657 | x3.5 | --- | --- | --- | --- |
| `sqrt` | 180706 | 34635 | x5 | 0.000015 | 9032 | x20 | 0.000626 |
| `1/sqrt` | 208274 | 108058 | x2 | 0.000046 | 51971 | x4 | 0.001748 |
| `cbrt` | 1559505 | 223349 | x7 | 0.00027 | --- | --- | --- |
| `exp2` | 362345 | 58915 | x6 | 0.001725 | 11337 | x32 | 0.038194 |
| `log2` | 418870 | 85594 | x5 | 0.008942 | 11762 | x35 | 0.028652 |
| `exp` | 411072 | 82206 | x5 | 0.001726 | 14405 | x28 | 0.038925 |

### Notes:

- `++` in the table indicates the default high-accuracy versions.
- Some functions also have even faster, but less accurate, versions.
- Functions like tan, exp, ..., are less accurate for large values, but the relative error remains small.

### Usage:

- **C**:
```c
#define X_MATH_ACC
#define X_MATH_SSE
#include "x_math.h"
```

- **Rust**:
```sh
cargo add x-math
```
```toml
[dependencies]
x-math = { version = "*", features = ["acc", "sse"] }
```

- **Jai**:
```odin
// SSE will be detected automatically (CPU == .X64)
MATH_ACC :: true;
#load "x_math.jai";
```

### License:

X-Math is released under the MIT License.

### Contributions:

Contributions are welcome! Feel free to submit a pull request or open an issue.

### Credits:

All credits are added as a comment in each function.

