#[cfg(feature = "sse")]
use std::arch::x86_64::*;

const PI: f32 = f32::from_bits(0x40490fdb); // pi
const PI_2: f32 = f32::from_bits(0x3fc90fdb); // pi / 2
const PI_4: f32 = f32::from_bits(0x3f490fdb); // pi / 4
const TAU: f32 = f32::from_bits(0x40c90fdb); // 2 * pi
const INV_PI: f32 = f32::from_bits(0x3ea2f983); // 1 / pi

#[inline]
pub fn trunc(x: f32) -> f32 {
    #[cfg(feature = "sse")]
    unsafe {
        _mm_cvtss_f32(_mm_round_ss(
            _mm_set_ss(x),
            _mm_set_ss(x),
            _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC,
        ))
    }
    #[cfg(not(feature = "sse"))]
    {
        let i = x as i32;
        i as f32
    }
}

#[inline]
pub fn floor(x: f32) -> f32 {
    #[cfg(feature = "sse")]
    unsafe {
        _mm_cvtss_f32(_mm_floor_ss(_mm_set_ss(x), _mm_set_ss(x)))
    }
    #[cfg(not(feature = "sse"))]
    {
        let r = x.to_bits() >> 31;
        let x = x - r as f32;
        trunc(f32::from_bits(x.to_bits() - r))
    }
}

#[inline]
pub fn ceil(x: f32) -> f32 {
    #[cfg(feature = "sse")]
    unsafe {
        _mm_cvtss_f32(_mm_ceil_ss(_mm_set_ss(x), _mm_set_ss(x)))
    }
    #[cfg(not(feature = "sse"))]
    {
        let r = 1 - (x.to_bits() >> 31);
        let x = x + r as f32;
        trunc(f32::from_bits(x.to_bits() - r))
    }
}

#[inline]
pub fn round(x: f32) -> f32 {
    #[cfg(feature = "sse")]
    unsafe {
        _mm_cvtss_f32(_mm_round_ss(
            _mm_set_ss(x),
            _mm_set_ss(x),
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }
    #[cfg(not(feature = "sse"))]
    {
        let x = x + f32::from_bits(0x3effffff);
        let u = x.to_bits() >> 31;
        trunc(x - u as f32)
    }
}

#[inline]
pub fn modulo(x: f32, e: f32) -> f32 {
    x - e * floor(x / e)
}

#[inline]
pub fn fract(x: f32) -> f32 {
    x - floor(x)
}

#[inline]
pub fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fffffff)
}

#[inline]
pub fn sign(x: f32) -> f32 {
    f32::from_bits((x.to_bits() & 0x80000000) | 0x3f800000)
}

#[inline]
pub fn cos(x: f32) -> f32 {
    // stackoverflow.com/a/77792413

    let x = abs(modulo(x, TAU) - PI) - PI_2;
    let x = x + (f32::from_bits(0xbc96e670) * x) * (x * x);
    x + (f32::from_bits(0xbe17b083) * x) * (x * x)
}

#[inline]
pub fn sin(x: f32) -> f32 {
    cos(x - PI_2)
}

#[inline]
pub fn sqrt(x: f32) -> f32 {
    #[cfg(feature = "sse")]
    unsafe {
        _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(x)))
    }
    #[cfg(not(feature = "sse"))]
    {
        let s = f32::from_bits((x.to_bits() + 0x3f769e5c) >> 1);
        let s = 0.5 * (s + x / s);
        #[cfg(feature = "acc")]
        {
            0.5 * (s + x / s)
        }
        #[cfg(not(feature = "acc"))]
        {
            s
        }
    }
}

#[inline]
pub fn cbrt(x: f32) -> f32 {
    // www.mdpi.com/1996-1073/14/4/1058

    const A: f32 = f32::from_bits(0x3fe04c03);
    const B: f32 = f32::from_bits(0x3f0266d9);
    const C: f32 = f32::from_bits(0xbfa01f36);

    let s = sign(x);
    let x = abs(x);
    let i = 0x548c2b4b - (x.to_bits() / 3);
    let y = f32::from_bits(i);
    let c = x * y * y * y;
    let y = y * (A + c * (B * c + C));
    let d = x * y * y;
    let c = d - d * d * y;
    let c = c * f32::from_bits(0x3eaaaaab) + d;
    s * c
}

#[inline]
pub fn rsqrt(x: f32) -> f32 {
    #[cfg(feature = "sse")]
    unsafe {
        _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x)))
    }
    #[cfg(not(feature = "sse"))]
    {
        // en.wikipedia.org/wiki/Fast_inverse_square_root#Overview_of_the_code

        let y = x * 0.5;
        let mut x = f32::from_bits(0x5f3759df - (x.to_bits() >> 1));

        x *= 1.5 - (y * x * x);
        #[cfg(feature = "acc")]
        {
            x * (1.5 - (y * x * x))
        }
        #[cfg(not(feature = "acc"))]
        {
            x
        }
    }
}

#[inline]
pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

#[inline]
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

#[inline]
pub fn clamp(x: f32, a: f32, b: f32) -> f32 {
    min(max(x, a), b)
}

#[inline]
pub fn atan2(y: f32, x: f32) -> f32 {
    // math.stackexchange.com/a/1105038

    let nx = x.to_bits() >> 31;
    let ny = y.to_bits() & 0x80000000;

    let x = abs(x);
    let y = abs(y);

    let p = (y > x) as u32;
    let y = min(x, y) / max(x, y);

    let r = y * y;
    let r = ((f32::from_bits(0xbd3e7316) * r + f32::from_bits(0x3e232344)) * r
        - f32::from_bits(0x3ea7be2c))
        * r
        * y
        + y;

    let r = r - f32::from_bits(p * 0x3fc90fdb);
    let u = r.to_bits() ^ ((p ^ nx) << 31);
    let r = f32::from_bits(u) + f32::from_bits(nx * 0x40490fdb);

    f32::from_bits((r.to_bits() & 0x7fffffff) | ny)
}

#[inline]
pub fn asin(x: f32) -> f32 {
    let s = sign(x);
    let x = abs(x);
    let z = 1.0 - sqrt(1.0 - x * x);
    let a = x - 0.35;
    s * (PI_4 * (x + z + 0.12 * z * z) + f32::from_bits(0x3d07ae14)
        - f32::from_bits(0x3e98a3d7) * a * a)
}

#[inline]
pub fn acos(x: f32) -> f32 {
    asin(-x) + PI_2
}

#[inline]
pub fn exp2(x: f32) -> f32 {
    #[cfg(feature = "acc")]
    {
        // docs.rs/fast-math/latest/src/fast_math/exp.rs.html#32

        let n = (x * f32::from_bits(0x4b000000)) as u32;
        let l = n & 0xff800000;
        let x = (n - l) as f32;

        let x = (f32::from_bits(0x27aca418) * x + f32::from_bits(0x33a85ada)) * x
            + f32::from_bits(0x3f803884);
        f32::from_bits(l + x.to_bits())
    }
    #[cfg(not(feature = "acc"))]
    {
        // docs.rs/fastapprox/latest/src/fastapprox/faster/mod.rs.html#21

        f32::from_bits(((x + f32::from_bits(0x42fde2a9)) * f32::from_bits(0x4b000000)) as u32)
    }
}

#[inline]
pub fn exp(x: f32) -> f32 {
    exp2(x * f32::from_bits(0x3fb8aa3b))
}

#[inline]
pub fn sinh(x: f32) -> f32 {
    let a = f32::from_bits(0x3fb8aa3b) * x - 1.0;
    let b = f32::from_bits(0xbfb8aa3b) * x - 1.0;
    exp2(a) - exp2(b)
}

#[inline]
pub fn cosh(x: f32) -> f32 {
    let a = f32::from_bits(0x3fb8aa3b) * x - 1.0;
    let b = f32::from_bits(0xbfb8aa3b) * x - 1.0;
    exp2(a) + exp2(b)
}

#[inline]
pub fn tanh(x: f32) -> f32 {
    let s = sign(x);
    let x = abs(x);
    // couldnt figure out a branchless way
    if x < 1.0 {
        let z = 0.07 * x * x;
        s * (x + (z * x + f32::from_bits(0xc08db6db)) * z)
    } else {
        let x = (1.05 * x - 0.1) * x + 1.09;
        s - s / (x * x)
    }
}

#[inline]
pub fn tan(x: f32) -> f32 {
    // observablehq.com/@jrus/fasttan

    let x = x * INV_PI;
    let x = 2.0 * (x - round(x));
    let y = 1.0 - x * x;
    x * (f32::from_bits(0xbc994764) * y
        + f32::from_bits(0x3ea1b529)
        + f32::from_bits(0x3fa30738) / y)
}

#[inline]
pub fn log2(x: f32) -> f32 {
    #[cfg(feature = "acc")]
    {
        // docs.rs/fast-math/latest/src/fast_math/log.rs.html#66

        let a = x.to_bits();
        let c = ((a >> 23) & 0xff) as i32;
        let d = a & 0x7fffff;

        let a = (a >> 22) & 1;

        let x = f32::from_bits(d | ((a ^ 0x7f) << 23)) - 1.0;
        let f = (a as i32 + c - 127) as f32;

        f + x * (x * f32::from_bits(0xbf213248) + f32::from_bits(0x3fbbc593))
    }
    #[cfg(not(feature = "acc"))]
    {
        // docs.rs/fastapprox/latest/src/fastapprox/faster/mod.rs.html#5

        let z = x.to_bits() as f32;
        z * f32::from_bits(0x34000000) - f32::from_bits(0x42fde2a9)
    }
}
