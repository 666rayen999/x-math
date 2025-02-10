//---------------------------X-MATH---------------------------//
// usage:                                                     //
//    MATH_ACC :: true; // more accurate results              //
//    using XMath;      // sin(x) instead of XMath.sin(x)     //
//    #load "x_math.jai";                                     //
//------------------------------------------------------------//
//                 made w luv by @666rayen999                 //
//------------------------------------------------------------//

#assert size_of(XMath) == 0;
XMath :: struct {
    PI      :: 0h40490fdb; // pi
    PI_2    :: 0h3fc90fdb; // pi / 2
    PI_4    :: 0h3f490fdb; // pi / 4
    TAU     :: 0h40c90fdb; // 2 * pi
    INV_PI  :: 0h3ea2f983; // 1 / pi

    trans :: inline (x: $T, $R: Type) -> R {
        #assert size_of(T) == size_of(R);
        r := cast(*R) *x;
        return r.*;
    }

    max   :: inline (a: float32, b: float32) -> float32 #must { return ifx a > b then a else b; }
    min   :: inline (a: float32, b: float32) -> float32 #must { return ifx a < b then a else b; }
    clamp :: inline (x: float32, a: float32, b: float32) -> float32 #must { return max(min(x, b), a); }

    floor :: inline (x: float32) -> float32 #must {
        #if CPU == .X64 {
            ret := *x;
            #asm SSE { roundss r:, [ret], 1; movss [ret], r; }
            return ret.*;
        } else {
            r := trans(x, u32) >> 31;
            x -= cast(float32) r;
            return trunc(trans(trans(x, u32) - r, float32));
        }
    }

    ceil :: inline (x: float32) -> float32 #must {
        #if CPU == .X64 {
            ret := *x;
            #asm SSE { roundss r:, [ret], 2; movss [ret], r; }
            return ret.*;
        } else {
            r := 1 - (trans(x, u32) >> 31);
            x += cast(float32) r;
            return trunc(trans(trans(x, u32) - r, float32));
        }
    }

    round :: inline (x: float32) -> float32 #must {
        #if CPU == .X64 {
            ret := *x;
            #asm SSE { roundss r:, [ret], 0; movss [ret], r; }
            return ret.*;
        } else {
            u := trans(x + 0h3effffff, u32) >> 31;
            x -= cast(float32) u;
            return trunc(x);
        }
    }

    trunc :: inline (x: float32) -> float32 #must {
        #if CPU == .X64 {
            ret := *x;
            #asm SSE { roundss r:, [ret], 3; movss [ret], r; }
            return ret.*;
        } else {
            i := cast(s32) x;
            return cast(float32) i;
        }
    }

    mod :: inline (x: float32) -> float32 #must { return x - floor(x); }
    mod :: inline (x: float32, $e: float32) -> float32 #must #modify { return e > 0; } {
        i :: 1 / e;
        return x - e * floor(x * i);
    }

    sin :: inline (x: float32) -> float32 #must { return cos(x - PI_2); }
    cos :: inline (x: float32) -> float32 #must {
        // stackoverflow.com/a/77792413

        p :: inline (a: float32, b: float32) -> float32 #must { return a + (b * a) * (a * a); }
        return p(p(abs(mod(x, TAU) - PI) - PI_2, 0hbc96e670), 0hbe17b083);
    }

    set_sign :: inline (x: float32, neg: bool) -> float32 #must {
        s := cast(u32) neg;
        return trans((trans(x, u32) & 0x7fffffff) | (s << 31), float32);
    }

    copy_sign :: inline (x: float32, s: float32) -> float32 #must {
        return trans((trans(x, u32) & 0x7fffffff) | (trans(s, u32) & 0x80000000), float32);
    }

    atan2 :: inline (y: float32, x: float32) -> float32 #must {
        // math.stackexchange.com/a/1105038

        union { r : float32 = ---; u : u32 = ---; }
        nx := trans(x, u32) >> 31;
        ny := trans(y, u32) & 0x80000000;

        x = abs(x); y = abs(y);
        c := cast(u32) (y > x);
        y = min(x, y) / max(x, y);

        r = y * y;
        r = (r * (r * 0hbd3e7316 + 0h3e232344) - 0h3ea7be2c) * r * y + y;
        r -= trans(c * 0x3fc90fdb, float32);
        u ^= (c ^ nx) << 31;
        r += trans(nx * 0x40490fdb, float32);
        u = (u & 0x7fffffff) | ny;

        return r;
    }

    acos :: inline (x: float32) -> float32 #must { return asin(-x) + PI_2; }
    asin :: inline (x: float32) -> float32 #must {
        s := sign(x);
        x = abs(x);
        z := 1 - sqrt(1 - x * x);
        a := x - .35;
        return s * (PI_4 * ( x + z + z * z * .12) + 0h3d07ae14 - a * a * 0h3e98a3d7);
    }

    sinh :: inline (x: float32) -> float32 #must {
        a := 0h3fb8aa3b * x - 1;
        b := 0hbfb8aa3b * x - 1;
        return exp2(a) - exp2(b);
    }

    cosh :: inline (x: float32) -> float32 #must {
        a := 0h3fb8aa3b * x - 1;
        b := 0hbfb8aa3b * x - 1;
        return exp2(a) + exp2(b);
    }

    // sinh_cosh :: inline (x: float32) -> v2 #must {
    //     a := exp2(0h3fb8aa3b * x - 1);
    //     b := exp2(0hbfb8aa3b * x - 1);
    //     return .{a - b, a + b};
    // }

    tanh :: inline (x: float32) -> float32 #must {
        s := sign(x);
        x = abs(x);
        if x < 1 {
            x2 := 0.07 * x * x;
            x += (x2 * x + 0hc08db6db) * x2;
        } else {
            x = (1.05 * x - 0.1) * x + 1.09;
            x = 1 - 1 / (x * x);
        }
        return s * x;
    }

    tan :: inline (x: float32) -> float32 #must {
        // observablehq.com/@jrus/fasttan

        x *= INV_PI;
        x = 2 * (x - round(x));
        y := 1 - x * x;
        return x * (0hbc994764 * y + 0h3ea1b529 + 0h3fa30738 / y);
    }

    // sin_cos :: inline (x: float32) -> v2 #must {
    //     p :: inline (a: v2, b: float32) -> v2 #must { return a + (a * b) * (a * a); }

    //     v := v2.{ mod(x - PI_2, TAU), mod(x, TAU) };
    //     v = abs(v - PI) - PI_2;
    //     v = p(p(v, 0hbc96e670), 0hbe17b083);

    //     return v;
    // }

    sqrt :: inline (x: float32) -> float32 #must {
        #if CPU == .X64 {
            ret := *x;
            #asm SSE { sqrtss r:, [ret]; movss [ret], r; }
            return ret.*;
        } else {
            i := trans(x, u32);
            i = (i + 0x3f769e5c) >> 1;
            s := trans(i, float32);

            s = .5 * (s + x / s);
            #if MATH_ACC s = .5 * (s + x / s);

            return s;
        }
    }

    cbrt :: inline (x: float32) -> float32 #must {
        // www.mdpi.com/1996-1073/14/4/1058

        s := sign(x);
        x = abs(x);
        i := trans(x, u32);
        i = 0x548c2b4b - (i / 3);
        y := trans(i, float32);
        c := x * y * y * y;
        y *= 0h3fe04c03 + c * (0h3f0266d9 * c + 0hbfa01f36);
        d := x * y * y;
        c = d - d * d * y;
        c = c * 0h3eaaaaab + d;
        return s * c;
    }

    rsqrt :: inline (x: float32) -> float32 #must {
        #if CPU == .X64 {
            ret := *x;
            #asm SSE { rsqrtss r:, [ret]; movss [ret], r; }
            return ret.*;
        } else {
            // en.wikipedia.org/wiki/Fast_inverse_square_root#Overview_of_the_code

            i := trans(x, u32);
            y := x * .5;
            i = 0x5f3759df - (i >> 1);
            x = trans(i, float32);

            x *= 1.5 - (y * x * x);
            #if MATH_ACC x *= 1.5 - (y * x * x);

            return x;
        }
    }

    log2 :: inline (x: float32) -> float32 #must {
        #if MATH_ACC {
            // docs.rs/fast-math/latest/src/fast_math/log.rs.html#66

            a := trans(x, u32);
            c := cast(s32) ((a >> 23) & 0xff);
            d := a & 0x7fffff;

            a = (a >> 22) & 1;
            c += xx,no_check a;
            c -= 127;
            d |= (a ^ 0x7f) << 23;

            x = trans(d, float32) - 1;
            f := cast(float32) c;

            return f + x * (x * 0hbf213248 + 0h3fbbc593);
        } else {
            // docs.rs/fastapprox/latest/src/fastapprox/faster/mod.rs.html#5

            x = cast(float32) trans(x, u32);
            return x * 0h34000000 - 0h42fde2a9;
        }
    }

    // pow :: inline (x: float32, n: float32) -> float32 #must {
    //     if x == 0 return 0;
    //     if n == 0 return 1;
    //     return exp2(n * log2(x));
    // }

    exp  :: inline (x: float32) -> float32 #must { return exp2(x * 0h3fb8aa3b); }
    exp2 :: inline (x: float32) -> float32 #must {
        #if MATH_ACC {
            // docs.rs/fast-math/latest/src/fast_math/exp.rs.html#32

            m := cast(s32) (x * 0h4b000000);
            l := m & 0xff800000;
            x = cast(float32) (m - l);

            x = (0h27aca418 * x + 0h33a85ada) * x + 0h3f803884;
            l += trans(x, s32);
            return trans(l, float32);
        } else {
            // docs.rs/fastapprox/latest/src/fastapprox/faster/mod.rs.html#21

            v := cast(u32) (0h4b000000 * (x + 0h42fde2a9));
            return trans(v, float32);
        }
    }


    abs :: inline (x: float32) -> float32 #must {
        return trans(trans(x, u32) & 0x7fffffff, float32);
    }

    sign :: inline (x: float32) -> float32 #must {
        return trans((trans(x, u32) & 0x80000000) | 0x3f800000, float32);
    }
}
