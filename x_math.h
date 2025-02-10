//---------------------------X-MATH---------------------------//
// usage:                                                     //
//    #define X_MATH_ACC // more accurate results             //
//    #define X_MATH_SSE // uses sse (sqrtss, roundss, ...)   //
//    #include "x_math.h"                                     //
//------------------------------------------------------------//
//                 made w luv by @666rayen999                 //
//------------------------------------------------------------//

#ifndef X_MATH
#define X_MATH

#include <stdint.h>

#ifdef X_MATH_SSE
#include <xmmintrin.h>
#endif  // X_MATH_SSE

static inline const float h(const uint32_t x) { return *(float*)&x; }

inline float x_trunc(float x) {
#ifdef X_MATH_SSE
  return _mm_cvtss_f32(_mm_round_ss(_mm_set_ss(x), _mm_set_ss(x),
                                    _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC));
#else
  int i = (int)x;
  return (float)i;
#endif  // X_MATH_SSE
}

inline float x_floor(float x) {
#ifdef X_MATH_SSE
  return _mm_cvtss_f32(_mm_floor_ss(_mm_set_ss(x), _mm_set_ss(x)));
#else
  uint32_t u = *(uint32_t*)&x;
  uint32_t r = u >> 31;
  x -= (float)r;
  u = *(uint32_t*)&x;
  u -= r;
  x = *(float*)&u;
  return x_trunc(x);
#endif  // X_MATH_SSE
}

inline float x_ceil(float x) {
#ifdef X_MATH_SSE
  return _mm_cvtss_f32(_mm_ceil_ss(_mm_set_ss(x), _mm_set_ss(x)));
#else
  uint32_t u = *(uint32_t*)&x;
  uint32_t r = 1 - (u >> 31);
  x += (float)r;
  u = *(uint32_t*)&x;
  u -= r;
  x = *(float*)&u;
  return x_trunc(x);
#endif  // X_MATH_SSE
}

inline float x_round(float x) {
#ifdef X_MATH_SSE
  return _mm_cvtss_f32(
      _mm_round_ss(_mm_set_ss(x), _mm_set_ss(x),
                   _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC));
#else
  x += h(0x3effffff);
  uint32_t u = *(uint32_t*)&x;
  u >>= 31;
  x -= (float)u;
  return x_trunc(x);
#endif  // X_MATH_SSE
}

inline float x_mod(float x, float e) { return x - e * x_floor(x / e); }

inline float x_fract(float x) { return x - x_floor(x); }

inline float x_abs(float x) {
  uint32_t u = *(uint32_t*)&x;
  u &= 0x7fffffff;
  return *(float*)&u;
}

inline float x_sign(float x) {
  uint32_t u = *(uint32_t*)&x;
  u &= 0x80000000;
  u |= 0x3f800000;
  return *(float*)&u;
}

inline float x_cos(float x) {
  // stackoverflow.com/a/77792413

  x = x_abs(x_mod(x, h(0x40c90fdb)) - h(0x40490fdb)) - h(0x3fc90fdb);
  x += (h(0xbc96e670) * x) * (x * x);
  x += (h(0xbe17b083) * x) * (x * x);

  return x;
}

inline float x_sin(float x) { return x_cos(x - h(0x3fc90fdb)); }

inline float x_sqrt(float x) {
#ifdef X_MATH_SSE
  return _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(x)));
#else
  uint32_t i = *(uint32_t*)&x;
  i = (i + 0x3f769e5c) >> 1;
  float s = *(float*)&i;

  s = 0.5f * (s + x / s);
#ifdef X_MATH_ACC
  s = 0.5f * (s + x / s);
#endif  // X_MATH_ACC

  return s;
#endif  // X_MATH_SSE
}

inline float x_cbrt(float x) {
  // www.mdpi.com/1996-1073/14/4/1058

  float s = x_sign(x);
  x = x_abs(x);
  uint32_t i = *(uint32_t*)&x;
  i = 0x548c2b4b - (i / 3);
  float y = *(float*)&i;
  float c = x * y * y * y;
  y *= h(0x3fe04c03) + c * (h(0x3f0266d9) * c + h(0xbfa01f36));
  float d = x * y * y;
  c = d - d * d * y;
  c = c * h(0x3eaaaaab) + d;
  return s * c;
}

inline float x_rsqrt(float x) {
#ifdef X_MATH_SSE
  return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x)));
#else
  // en.wikipedia.org/wiki/Fast_inverse_square_root#Overview_of_the_code
  uint32_t i = *(uint32_t*)&x;
  float y = x * 0.5f;
  i = 0x5f3759df - (i >> 1);
  x = *(float*)&i;

  x *= 1.5f - (y * x * x);
#ifdef X_MATH_ACC
  x *= 1.5f - (y * x * x);
#endif  // X_MATH_ACC

  return x;
#endif  // X_MATH_SSE
}

inline float x_min(float a, float b) { return a < b ? a : b; }
inline float x_max(float a, float b) { return a > b ? a : b; }
inline float x_clamp(float x, float min, float max) {
  return x_min(x_max(x, min), max);
}

inline float x_atan2(float y, float x) {
  // math.stackexchange.com/a/1105038

  uint32_t nx = *(uint32_t*)&x;
  nx >>= 31;
  uint32_t ny = *(uint32_t*)&y;
  ny &= 0x80000000;

  x = x_abs(x);
  y = x_abs(y);
  const uint32_t p = y > x;
  y = x_min(x, y) / x_max(x, y);

  union {
    float r;
    uint32_t u;
  } z = {.r = y * y};
  z.r =
      ((h(0xbd3e7316) * z.r + h(0x3e232344)) * z.r - h(0x3ea7be2c)) * z.r * y +
      y;

  const uint32_t d = p * 0x3fc90fdb;
  z.r -= *(float*)&d;
  z.u ^= (p ^ nx) << 31;
  nx *= 0x40490fdb;
  z.r += *(float*)&nx;
  z.u = (z.u & 0x7fffffff) | ny;
  return z.r;
}

inline float x_asin(float x) {
  float s = x_sign(x);
  x = x_abs(x);
  float z = 1.0f - x_sqrt(1.0f - x * x);
  float a = x - 0.35f;
  x = h(0x3f490fdb) * (x + z + 0.12f * z * z) + h(0x3d07ae14) -
      h(0x3e98a3d7) * a * a;

  return s * x;
}

inline float x_acos(float x) { return x_asin(-x) + h(0x3fc90fdb); }

inline float x_exp2(float x) {
#ifdef X_MATH_ACC
  // docs.rs/fast-math/latest/src/fast_math/exp.rs.html#32

  const int32_t n = (int32_t)(x * h(0x4b000000));
  int32_t l = n & 0xff800000;
  x = (float)(n - l);

  x = (h(0x27aca418) * x + h(0x33a85ada)) * x + h(0x3f803884);
  l += *(int32_t*)&x;
  return *(float*)&l;
#else
  // docs.rs/fastapprox/latest/src/fastapprox/faster/mod.rs.html#21

  uint32_t v = (uint32_t)(h(0x4b000000) * (x + h(0x42fde2a9)));
  return *(float*)&v;
#endif  // X_MATH_ACC
}

inline float x_exp(float x) { return x_exp2(x * h(0x3fb8aa3b)); }

inline float x_sinh(float x) {
  float a = h(0x3fb8aa3b) * x - 1.0f;
  float b = h(0xbfb8aa3b) * x - 1.0f;
  return x_exp2(a) - x_exp2(b);
}

inline float x_cosh(float x) {
  float a = h(0x3fb8aa3b) * x - 1.0f;
  float b = h(0xbfb8aa3b) * x - 1.0f;
  return x_exp2(a) + x_exp2(b);
}

inline float x_tanh(float x) {
  float s = x_sign(x);
  x = x_abs(x);
  // couldnt figure out a branchless way
  if (x < 1.0f) {
    float z = 0.07f * x * x;
    x += (z * x + h(0xc08db6db)) * z;
  } else {
    x = (1.05f * x - 0.1f) * x + 1.09f;
    x = 1.0f - 1.0f / (x * x);
  }
  return s * x;
}

inline float x_tan(float x) {
  // observablehq.com/@jrus/fasttan

  x *= h(0x3ea2f983);
  x = 2.0f * (x - x_round(x));
  float y = 1.0f - x * x;
  return x * (h(0xbc994764) * y + h(0x3ea1b529) + h(0x3fa30738) / y);
}

inline float x_log2(float x) {
#ifdef X_MATH_ACC
  // docs.rs/fast-math/latest/src/fast_math/log.rs.html#66

  uint32_t a = *(uint32_t*)&x;
  int32_t c = (a >> 23) & 0xff;
  uint32_t d = a & 0x7fffff;

  a = (a >> 22) & 1;
  c += a - 127;
  d |= (a ^ 0x7f) << 23;

  x = *(float*)&d;
  x -= 1.0f;
  float f = (float)c;

  return f + x * (x * h(0xbf213248) + h(0x3fbbc593));
#else
  // docs.rs/fastapprox/latest/src/fastapprox/faster/mod.rs.html#5

  const uint32_t y = *(uint32_t*)&x;
  float z = (float)y;
  z *= h(0x34000000);
  return z - h(0x42fde2a9);
#endif  // X_MATH_ACC
}

#endif  // X_MATH
