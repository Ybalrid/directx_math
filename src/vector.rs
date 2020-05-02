use std::mem;

use crate::*;

#[cfg(_XM_NO_INTRINSICS_)]
macro_rules! XMISNAN {
    ($x:expr) => {
       $x.is_nan()
    }
}

#[cfg(_XM_NO_INTRINSICS_)]
macro_rules! XMISINF {
    ($x:expr) => {
       $x.is_infinite()
    }
}

// --

#[cfg(_XM_SSE_INTRINSICS_)]
macro_rules! XM3UNPACK3INTO4 {
    ($l1:expr, $l2:expr, $l3:expr) => {
       let V3: XMVECTOR = _mm_shuffle_ps($l2, $l3, _MM_SHUFFLE(0, 0, 3, 2));
       let mut V2: XMVECTOR = _mm_shuffle_ps($l2, $l1, _MM_SHUFFLE(3, 3, 1, 0));
       V2 = XM_PERMUTE_PS!(V2, _MM_SHUFFLE!(1, 1, 0, 2));
       // NOTE: 'l3' here is 'L3' in the C macro. It seems to work by convention and the
       //        fact that C macros don't have scope.
       let V4: XMVECTOR = _mm_castsi128_ps(_mm_srli_si128(_mm_castps_si128($l3), 32 / 8));
       // NOTE: The C macro defines these in scope, but we need to return in
       //       the rust version due to scope
       (V2, V3, V4)
    }
}

#[cfg(_XM_SSE_INTRINSICS_)]
macro_rules! XM3PACK4INTO3 {
    ($V1:expr, $V2:expr, $V3:expr, $V4:expr) => {
        let v2x: let = _mm_shuffle_ps(V2, V3, _MM_SHUFFLE(1, 0, 2, 1));
        V2 = _mm_shuffle_ps(V2, V1, _MM_SHUFFLE(2, 2, 0, 0));
        V1 = _mm_shuffle_ps(V1, V2, _MM_SHUFFLE(0, 2, 1, 0));
        V3 = _mm_shuffle_ps(V3, V4, _MM_SHUFFLE(0, 0, 2, 2));
        V3 = _mm_shuffle_ps(V3, V4, _MM_SHUFFLE(2, 1, 2, 0));
        // NOTE: The C macro takes 'v2x' as input, but we need to return in
        //       the rust version due to scope. 'V2' is never used after the
        //       macro, so we omit returning it.
        (v2x, V1, V3)
    }
}

// --

/// Replicates a floating-point value referenced by pointer into all four components of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorZero>
#[inline]
pub fn XMVectorZero() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult = XMVECTORF32 { f: [0.0, 0.0, 0.0, 0.0] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_setzero_ps();
    }
}

/// Creates the zero vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSet>
#[inline]
pub fn XMVectorSet(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult = XMVECTORF32 { f: [x, y, z, w] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_set_ps(w, z, y, x);
    }
}

/// Creates a vector with unsigned integer components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetInt>
#[inline]
pub fn XMVectorSetInt(
    x: u32,
    y: u32,
    z: u32,
    w: u32,
) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult = XMVECTORU32 { u: [x, y, z, w] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_set_epi32(w as i32, z as i32, y as i32, x as i32);
        return _mm_castsi128_ps(V);
    }
}

/// Replicates a floating-point value into all four components of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReplicate>
#[inline]
pub fn XMVectorReplicate(Value: f32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = Value;
        vResult.f[1] = Value;
        vResult.f[2] = Value;
        vResult.f[3] = Value;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_set_ps1(Value);
    }
}

/// Replicates a floating-point value referenced by pointer into all four components of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReplicatePtr>
#[inline]
pub fn XMVectorReplicatePtr(pValue: &f32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        let Value = *pValue;
        vResult.f[0] = Value;
        vResult.f[1] = Value;
        vResult.f[2] = Value;
        vResult.f[3] = Value;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_AVX_INTRINSICS_)]
    unsafe {
        return _mm_broadcast_ss(pValue);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_AVX_INTRINSICS_)))]
    unsafe {
        return _mm_load_ps1(pValue);
    }
}

/// Replicates an integer value into all four components of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReplicateInt>
#[inline]
pub fn XMVectorReplicateInt(Value: u32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        vResult.u[0] = Value;
        vResult.u[1] = Value;
        vResult.u[2] = Value;
        vResult.u[3] = Value;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_set1_epi32(Value as i32);
        return _mm_castsi128_ps(vTemp);
    }
}

// TODO: XMVectorReplicateIntPtr

/// Returns a vector, each of whose components represents true (0xFFFFFFFF).
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorTrueInt>
#[inline]
pub fn XMVectorTrueInt() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult = XMVECTORU32 { u: [0xFFFFFFFFu32, 0xFFFFFFFFu32, 0xFFFFFFFFu32, 0xFFFFFFFFu32] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_set1_epi32(-1);
        return _mm_castsi128_ps(V);
    }
}

/// Returns the zero (false) vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorFalseInt>
#[inline]
pub fn XMVectorFalseInt() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult = XMVECTORF32 { f: [0.0, 0.0, 0.0, 0.0] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_setzero_ps();
    }
}

/// Replicates the x component of a vector to all of the components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatX>
#[inline]
pub fn XMVectorSplatX(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = V.vector4_f32[0];
        vResult.f[1] = V.vector4_f32[0];
        vResult.f[2] = V.vector4_f32[0];
        vResult.f[3] = V.vector4_f32[0];
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_AVX2_INTRINSICS_, _XM_FAVOR_INTEL_))]
    unsafe {
        return _mm_broadcastss_ps(V);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(all(_XM_AVX2_INTRINSICS_, _XM_FAVOR_INTEL_))))]
    unsafe {
        return XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 0, 0, 0));
    }
}

/// Replicates the y component of a vector to all of the components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatY>
#[inline]
pub fn XMVectorSplatY(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = V.vector4_f32[1];
        vResult.f[1] = V.vector4_f32[1];
        vResult.f[2] = V.vector4_f32[1];
        vResult.f[3] = V.vector4_f32[1];
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1));
    }
}

/// Replicate the z component of the vector
#[inline]
pub fn XMVectorSplatZ(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = V.vector4_f32[2];
        vResult.f[1] = V.vector4_f32[2];
        vResult.f[2] = V.vector4_f32[2];
        vResult.f[3] = V.vector4_f32[2];
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 2, 2, 2));
    }
}

/// Replicates the w component of a vector to all of the components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatW>
#[inline]
pub fn XMVectorSplatW(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = V.vector4_f32[3];
        vResult.f[1] = V.vector4_f32[3];
        vResult.f[2] = V.vector4_f32[3];
        vResult.f[3] = V.vector4_f32[3];
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 3, 3, 3));
    }
}

/// Returns a vector, each of whose components are one.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatOne>
#[inline]
pub fn XMVectorSplatOne() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = 1.0;
        vResult.f[1] = 1.0;
        vResult.f[2] = 1.0;
        vResult.f[3] = 1.0;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return g_XMOne.v;
    }
}

/// Returns a vector, each of whose components are infinity (0x7F800000).
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatInfinity>
#[inline]
pub fn XMVectorSplatInfinity() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        vResult.u[0] = 0x7F800000;
        vResult.u[1] = 0x7F800000;
        vResult.u[2] = 0x7F800000;
        vResult.u[3] = 0x7F800000;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return g_XMInfinity.v;
    }
}

/// Returns a vector, each of whose components are QNaN (0x7CF00000).
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatQNaN>
#[inline]
pub fn XMVectorSplatQNaN() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        vResult.u[0] = 0x7FC00000;
        vResult.u[1] = 0x7FC00000;
        vResult.u[2] = 0x7FC00000;
        vResult.u[3] = 0x7FC00000;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return g_XMQNaN.v;
    }
}

/// Returns a vector, each of whose components are epsilon (`1.192092896e-7`).
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatEpsilon>
#[inline]
pub fn XMVectorSplatEpsilon() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        vResult.u[0] = 0x34000000;
        vResult.u[1] = 0x34000000;
        vResult.u[2] = 0x34000000;
        vResult.u[3] = 0x34000000;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return g_XMEpsilon.v;
    }
}

/// Returns a vector, each of whose components are the sign mask (`0x80000000`).
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSplatSignMask>
#[inline]
pub fn XMVectorSplatSignMask() -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        vResult.u[0] = 0x80000000u32;
        vResult.u[1] = 0x80000000u32;
        vResult.u[2] = 0x80000000u32;
        vResult.u[3] = 0x80000000u32;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i  = _mm_set1_epi32(0x80000000u32 as i32);
        return _mm_castsi128_ps(V);
    }
}

/// Retrieve the value of one of the four components of an XMVECTOR Data Type containing floating-point data by index.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetByIndex>
#[inline]
pub fn XMVectorGetByIndex(V: XMVECTOR, i: usize) -> f32 {
    debug_assert!(i < 4);

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_f32[i];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut U: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        U.v = V;
        return U.f[i];
    }
}

/// Retrieve the `x` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetX>
#[inline]
pub fn XMVectorGetX(V: XMVECTOR) -> f32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_f32[0];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cvtss_f32(V);
    }
}

/// Retrieve the `y` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetY>
#[inline]
pub fn XMVectorGetY(V: XMVECTOR) -> f32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_f32[1];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1));
        return _mm_cvtss_f32(vTemp);
    }
}

/// Retrieve the `z` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetZ>
#[inline]
pub fn XMVectorGetZ(V: XMVECTOR) -> f32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_f32[2];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 2, 2, 2));
        return _mm_cvtss_f32(vTemp);
    }
}

/// Retrieve the `w` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetW>
#[inline]
pub fn XMVectorGetW(V: XMVECTOR) -> f32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_f32[3];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 3, 3, 3));
        return _mm_cvtss_f32(vTemp);
    }
}

// TODO: XMVectorGetByIndexPtr

/// Retrieve the `x` component of an XMVECTOR Data Type containing floating-point data, and storing that
/// component's value in an instance of float referred to by a pointer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetXPtr>
#[inline]
pub fn XMVectorGetXPtr(
    x: &mut f32,
    V: FXMVECTOR,
)
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        *x = V.vector4_f32[0];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        _mm_store_ss(x, V);
    }
}

/// Retrieve the `y` component of an XMVECTOR Data Type containing floating-point data, and storing that
/// component's value in an instance of float referred to by a pointer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetYPtr>
#[inline]
pub fn XMVectorGetYPtr(
    y: &mut f32,
    V: FXMVECTOR,
)
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        *y = V.vector4_f32[1];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        *mem::transmute::<_, *mut i32>(y) = _mm_extract_ps(V, 1);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1));
        _mm_store_ss(y, vResult);
    }
}

/// Retrieve the `z` component of an XMVECTOR Data Type containing floating-point data, and storing that
/// component's value in an instance of float referred to by a pointer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetZPtr>
#[inline]
pub fn XMVectorGetZPtr(
    z: &mut f32,
    V: FXMVECTOR,
)
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        *z = V.vector4_f32[2];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        *mem::transmute::<_, *mut i32>(z) = _mm_extract_ps(V, 2);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 2, 2, 2));
        _mm_store_ss(z, vResult);
    }
}

/// Retrieve the `w` component of an XMVECTOR Data Type containing floating-point data, and storing that
/// component's value in an instance of float referred to by a pointer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetWPtr>
#[inline]
pub fn XMVectorGetWPtr(
    w: &mut f32,
    V: FXMVECTOR,
)
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        *w = V.vector4_f32[3];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        *mem::transmute::<_, *mut i32>(w) = _mm_extract_ps(V, 3);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 3, 3, 3));
        _mm_store_ss(w, vResult);
    }
}

/// Retrieve the value of one of the four components of an XMVECTOR Data Type containing integer data by index.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetIntByIndex>
#[inline]
pub fn XMVectorGetIntByIndex(V: XMVECTOR, i: usize) -> u32 {
    debug_assert!(i < 4);

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_u32[i];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut U: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        U.v = V;
        return U.u[i];
    }
}

/// Retrieve the `x` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetIntX>
#[inline]
pub fn XMVectorGetIntX(V: XMVECTOR) -> u32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_u32[0];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return (_mm_cvtsi128_si32(_mm_castps_si128(V))) as u32;
    }
}

/// Retrieve the `y` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetIntY>
#[inline]
pub fn XMVectorGetIntY(V: XMVECTOR) -> u32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_u32[1];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let V1: __m128i = _mm_castps_si128(V);
        return (_mm_extract_epi32(V1, 1)) as u32;
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        let vResulti: __m128i = _mm_shuffle_epi32(_mm_castps_si128(V), _MM_SHUFFLE(1, 1, 1, 1));
        return (_mm_cvtsi128_si32(vResulti)) as u32;
    }
}

/// Retrieve the `z` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetIntZ>
#[inline]
pub fn XMVectorGetIntZ(V: XMVECTOR) -> u32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_u32[2];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let V1: __m128i = _mm_castps_si128(V);
        return (_mm_extract_epi32(V1, 2)) as u32;
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        let vResulti: __m128i = _mm_shuffle_epi32(_mm_castps_si128(V), _MM_SHUFFLE(2, 2, 2, 2));
        return (_mm_cvtsi128_si32(vResulti)) as u32;
    }
}

/// Retrieve the `w` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGetIntW>
#[inline]
pub fn XMVectorGetIntW(V: XMVECTOR) -> u32 {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return V.vector4_u32[3];
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let V1: __m128i = _mm_castps_si128(V);
        return (_mm_extract_epi32(V1, 3)) as u32;
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        let vResulti: __m128i = _mm_shuffle_epi32(_mm_castps_si128(V), _MM_SHUFFLE(3, 3, 3, 3));
        return (_mm_cvtsi128_si32(vResulti)) as u32;
    }
}

// TODO: XMVectorGetIntByIndexPtr
// TODO: XMVectorGetIntXPtr
// TODO: XMVectorGetIntYPtr
// TODO: XMVectorGetIntZPtr
// TODO: XMVectorGetIntWPtr

/// Use a floating-point object to set the value of one of the four components of an XMVECTOR Data Type containing integer data referenced by an index.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetByIndex>
#[inline]
pub fn XMVectorSetByIndex(V: XMVECTOR, f: f32, i: usize) -> XMVECTOR {
    debug_assert!(i < 4);

    unsafe {
        let mut U: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        U.v = V;
        U.f[i] = f;
        return U.v;
    }
}

/// Set the value of the `x` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetX>
#[inline]
pub fn XMVectorSetX(V: XMVECTOR, x: f32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORF32 {
            f: [
                x,
                V.vector4_f32[1],
                V.vector4_f32[2],
                V.vector4_f32[3]
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_set_ss(x);
        vResult = _mm_move_ss(V, vResult);
        return vResult;
    }
}

/// Set the value of the `y` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetY>
#[inline]
pub fn XMVectorSetY(V: XMVECTOR, y: f32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORF32 {
            f: [
                V.vector4_f32[0],
                y,
                V.vector4_f32[2],
                V.vector4_f32[3]
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_set_ss(y);
        vResult = _mm_insert_ps(V, vResult, 0x10);
        return vResult;
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap y and x
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 2, 0, 1));
        // Convert input to vector
        let vTemp: XMVECTOR = _mm_set_ss(y);
        // Replace the x component
        vResult = _mm_move_ss(vResult, vTemp);
        // Swap y and x again
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(3, 2, 0, 1));
        return vResult;
    }
}

/// Set the value of the `z` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetZ>
#[inline]
pub fn XMVectorSetZ(V: XMVECTOR, z: f32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORF32 {
            f: [
                V.vector4_f32[0],
                V.vector4_f32[1],
                z,
                V.vector4_f32[3]
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_set_ss(z);
        vResult = _mm_insert_ps(V, vResult, 0x20);
        return vResult;
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap z and x
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 0, 1, 2));
        // Convert input to vector
        let vTemp: XMVECTOR = _mm_set_ss(z);
        // Replace the x component
        vResult = _mm_move_ss(vResult, vTemp);
        // Swap z and x again
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(3, 0, 1, 2));
        return vResult;
    }
}

/// Set the value of the `w` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetW>
#[inline]
pub fn XMVectorSetW(V: XMVECTOR, w: f32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORF32 {
            f: [
                V.vector4_f32[0],
                V.vector4_f32[1],
                V.vector4_f32[2],
                w,
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_set_ss(w);
        vResult = _mm_insert_ps(V, vResult, 0x30);
        return vResult;
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap w and x
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 2, 1, 3));
        // Convert input to vector
        let vTemp: XMVECTOR = _mm_set_ss(w);
        // Replace the x component
        vResult = _mm_move_ss(vResult, vTemp);
        // Swap w and x again
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(0, 2, 1, 3));
        return vResult;
    }
}

// TODO: XMVectorSetByIndexPtr
// TODO: XMVectorSetXPtr
// TODO: XMVectorSetYPtr
// TODO: XMVectorSetZPtr
// TODO: XMVectorSetWPtr

/// Use an integer instance to set the value of one of the four components of an XMVECTOR Data Type containing integer data referenced by an index.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetIntByIndex>
#[inline]
pub fn XMVectorSetIntByIndex(V: XMVECTOR, x: u32, i: usize) -> XMVECTOR {
    // debug_assert!(i < 4);

    unsafe {
        let mut U: XMVECTORU32 = mem::MaybeUninit::uninit().assume_init();
        U.v = V;
        U.u[i] = x;
        return U.v;
    }
}

/// Set the value of the `x` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetIntX>
#[inline]
pub fn XMVectorSetIntX(V: XMVECTOR, x: u32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORU32 {
            u: [
                x,
                V.vector4_u32[1],
                V.vector4_u32[2],
                V.vector4_u32[3]
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cvtsi32_si128(x as i32);
        let vResult: XMVECTOR = _mm_move_ss(V, _mm_castsi128_ps(vTemp));
        return vResult;
    }
}

/// Set the value of the `y` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetIntY>
#[inline]
pub fn XMVectorSetIntY(V: XMVECTOR, y: u32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORU32 {
            u: [
                V.vector4_u32[0],
                y,
                V.vector4_u32[2],
                V.vector4_u32[3]
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vResult: __m128i = _mm_castps_si128(V);
        vResult = _mm_insert_epi32(vResult, y as i32, 1);
        return _mm_castsi128_ps(vResult);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap y and x
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 2, 0, 1));
        // Convert input to vector
        let vTemp: __m128i = _mm_cvtsi32_si128(y as i32);
        // Replace the x component
        vResult = _mm_move_ss(vResult, _mm_castsi128_ps(vTemp));
        // Swap y and x again
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(3, 2, 0, 1));
        return vResult;
    }
}

/// Set the value of the `z` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetIntZ>
#[inline]
pub fn XMVectorSetIntZ(V: XMVECTOR, z: u32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORU32 {
            u: [
                V.vector4_u32[0],
                V.vector4_u32[1],
                z,
                V.vector4_u32[3]
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vResult: __m128i = _mm_castps_si128(V);
        vResult = _mm_insert_epi32(vResult, z as i32, 2);
        return _mm_castsi128_ps(vResult);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap z and x
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 0, 1, 2));
        // Convert input to vector
        let vTemp: __m128i = _mm_cvtsi32_si128(z as i32);
        // Replace the x component
        vResult = _mm_move_ss(vResult, _mm_castsi128_ps(vTemp));
        // Swap z and x again
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(3, 0, 1, 2));
        return vResult;
    }
}

/// Set the value of the `w` component of an XMVECTOR Data Type.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSetIntW>
#[inline]
pub fn XMVectorSetIntW(V: XMVECTOR, w: u32) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORU32 {
            u: [
                V.vector4_u32[0],
                V.vector4_u32[1],
                V.vector4_u32[2],
                w,
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vResult: __m128i = _mm_castps_si128(V);
        vResult = _mm_insert_epi32(vResult, w as i32, 3);
        return _mm_castsi128_ps(vResult);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap w and x
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 2, 1, 3));
        // Convert input to vector
        let vTemp: __m128i = _mm_cvtsi32_si128(w as i32);
        // Replace the x component
        vResult = _mm_move_ss(vResult, _mm_castsi128_ps(vTemp));
        // Swap w and x again
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(0, 2, 1, 3));
        return vResult;
    }
}

// TODO: XMVectorSetIntByIndexPtr
// TODO: XMVectorSetIntXPtr
// TODO: XMVectorSetIntYPtr
// TODO: XMVectorSetIntZPtr
// TODO: XMVectorSetIntWPtr

/// Swizzles a vector.
///
/// For the case of constant indices (E0, E1, E2, E3), it is much more efficent to use the template form of [`XMVectorSwizzle`].
///
/// [`XMVectorSwizzle`]: trait@crate::XMVectorSwizzle
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSwizzle>
#[inline]
pub fn XMVectorSwizzle(
    V: FXMVECTOR,
    E0: u32,
    E1: u32,
    E2: u32,
    E3: u32
) -> XMVECTOR
{
    debug_assert!((E0 < 4) && (E1 < 4) && (E2 < 4) && (E3 < 4));

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let U = XMVECTORF32 {
            f: [
                V.vector4_f32[E0 as usize],
                V.vector4_f32[E1 as usize],
                V.vector4_f32[E2 as usize],
                V.vector4_f32[E3 as usize],
            ]
        };
        return U.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_AVX_INTRINSICS_)]
    unsafe {
        let elem: [i32; 4] = [ E0 as i32, E1 as i32, E2 as i32, E3 as i32 ];
        let vControl: __m128i = _mm_loadu_si128(mem::transmute(&elem[0]));
        return _mm_permutevar_ps(V, vControl);
    }

    #[cfg(all(not(_XM_AVX_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        let aPtr = mem::transmute::<_, *const u32>(&V);

        let mut Result: XMVECTOR = mem::MaybeUninit::uninit().assume_init();

        let pWork = mem::transmute::<_, *mut u32>(&mut Result);

        idx!(mut pWork[0]) = idx!(aPtr[E0]);
        idx!(mut pWork[1]) = idx!(aPtr[E1]);
        idx!(mut pWork[2]) = idx!(aPtr[E2]);
        idx!(mut pWork[3]) = idx!(aPtr[E3]);

        return Result;
    }
}

#[test]
fn test_XMVectorSwizzle() {
    let a = XMVectorSet(1.0, 2.0, 3.0, 4.0);
    let b = XMVectorSwizzle(a, 3, 2, 1, 0);
    let c = XMVectorSet(4.0, 3.0, 2.0, 1.0);

    assert_eq!(XMVectorGetX(b), XMVectorGetX(c));
    assert_eq!(XMVectorGetY(b), XMVectorGetY(c));
    assert_eq!(XMVectorGetZ(b), XMVectorGetZ(c));
    assert_eq!(XMVectorGetW(b), XMVectorGetW(c));
}

/// Permutes the components of two vectors to create a new vector.
///
/// ### Remarks
///
/// If all 4 indices reference only a single vector (i.e. they are all in the range 0-3 or all in the range 4-7),
/// use `XMVectorSwizzle` instead for better performance.
///
/// For constant PermuteX/Y/Z/W parameters, it may more efficent to use the template form of [`XMVectorPermute`]
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorPermute>
///
/// [`XMVectorPermute`]: trait@crate::XMVectorPermute
#[inline]
pub fn XMVectorPermute(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    PermuteX: u32,
    PermuteY: u32,
    PermuteZ: u32,
    PermuteW: u32
) -> XMVECTOR
{
    debug_assert!(PermuteX <= 7 && PermuteY <= 7 && PermuteZ <= 7 && PermuteW <= 7);

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_AVX_INTRINSICS_)]
    unsafe {
        const three: XMVECTORU32 = XMVECTORU32 { u: [ 3, 3, 3, 3 ] };

        let elem: Align16<[u32; 4]> = Align16([PermuteX, PermuteY, PermuteZ, PermuteW]);
        let mut vControl: __m128i = _mm_load_si128(mem::transmute::<_, *const __m128i>(&elem[0]));

        let vSelect: __m128i = _mm_cmpgt_epi32(vControl, three.m128i());
        vControl = _mm_castps_si128(_mm_and_ps(_mm_castsi128_ps(vControl), three.v));

        let shuffled1: __m128 = _mm_permutevar_ps(V1, vControl);
        let shuffled2: __m128 = _mm_permutevar_ps(V2, vControl);

        let masked1: __m128 = _mm_andnot_ps(_mm_castsi128_ps(vSelect), shuffled1);
        let masked2: __m128 = _mm_and_ps(_mm_castsi128_ps(vSelect), shuffled2);

        return _mm_or_ps(masked1, masked2);
    }

    #[cfg(not(_XM_AVX_INTRINSICS_))]
    unsafe {
        let aPtr: &[*const u32; 2] = &[
            mem::transmute(&V1),
            mem::transmute(&V2),
        ];

        let mut Result: XMVECTOR = mem::MaybeUninit::uninit().assume_init();

        let pWork = mem::transmute::<_, *mut u32>(&mut Result);

        let i0: u32 = PermuteX & 3;
        let vi0: u32 = PermuteX >> 2;
        idx!(mut pWork[0]) = idx!(aPtr[vi0][i0]);

        let i1: u32 = PermuteY & 3;
        let vi1: u32 = PermuteY >> 2;
        idx!(mut pWork[1]) = idx!(aPtr[vi1][i1]);

        let i2: u32 = PermuteZ & 3;
        let vi2: u32 = PermuteZ >> 2;
        idx!(mut pWork[2]) = idx!(aPtr[vi2][i2]);

        let i3: u32 = PermuteW & 3;
        let vi3: u32 = PermuteW >> 2;
        idx!(mut pWork[3]) = idx!(aPtr[vi3][i3]);

        return Result;
    }
}

#[test]
fn test_XMVectorPermute() {
    let a = XMVectorSet(1.0, 2.0, 3.0, 4.0);
    let b = XMVectorSet(5.0, 6.0, 7.0, 8.0);

    let c = XMVectorPermute(a, b, 0, 2, 4, 6);
    let d = XMVectorSet(1.0, 3.0, 5.0, 7.0);

    assert_eq!(XMVectorGetX(c), XMVectorGetX(d));
    assert_eq!(XMVectorGetY(c), XMVectorGetY(d));
    assert_eq!(XMVectorGetZ(c), XMVectorGetZ(d));
    assert_eq!(XMVectorGetW(c), XMVectorGetW(d));

    let e = XMVectorPermute(a, b, 1, 3, 5, 7);
    let f = XMVectorSet(2.0, 4.0, 6.0, 8.0);

    assert_eq!(XMVectorGetX(e), XMVectorGetX(f));
    assert_eq!(XMVectorGetY(e), XMVectorGetY(f));
    assert_eq!(XMVectorGetZ(e), XMVectorGetZ(f));
    assert_eq!(XMVectorGetW(e), XMVectorGetW(f));
}

/// Defines a control vector for use in XMVectorSelect.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSelectControl>
#[inline]
pub fn XMVectorSelectControl(
    VectorIndex0: u32,
    VectorIndex1: u32,
    VectorIndex2: u32,
    VectorIndex3: u32,
) -> XMVECTOR
{
    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // x=Index0,y=Index1,z=Index2,w=Index3
        let mut vTemp: __m128i = _mm_set_epi32(VectorIndex3 as i32, VectorIndex2 as i32, VectorIndex1 as i32, VectorIndex0 as i32);
        // Any non-zero entries become 0xFFFFFFFF else 0
        vTemp = _mm_cmpgt_epi32(vTemp, g_XMZero.m128i());
        return _mm_castsi128_ps(vTemp);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(not(any(_XM_SSE_INTRINSICS_, _XM_ARM_NEON_INTRINSICS_)))]
    unsafe {
        let mut ControlVector: XMVECTOR = mem::MaybeUninit::uninit().assume_init();
        let ControlElement: [u32; 2] = [
            XM_SELECT_0,
            XM_SELECT_1,
        ];

        ControlVector.vector4_u32[0] = ControlElement[VectorIndex0 as usize];
        ControlVector.vector4_u32[1] = ControlElement[VectorIndex1 as usize];
        ControlVector.vector4_u32[2] = ControlElement[VectorIndex2 as usize];
        ControlVector.vector4_u32[3] = ControlElement[VectorIndex3 as usize];

        return ControlVector;
    }
}

/// Performs a per-component selection between two input vectors and returns the resulting vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSelect>
#[inline]
pub fn XMVectorSelect(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    Control: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                (V1.vector4_u32[0] & !Control.vector4_u32[0]) | (V2.vector4_u32[0] & Control.vector4_u32[0]),
                (V1.vector4_u32[1] & !Control.vector4_u32[1]) | (V2.vector4_u32[1] & Control.vector4_u32[1]),
                (V1.vector4_u32[2] & !Control.vector4_u32[2]) | (V2.vector4_u32[2] & Control.vector4_u32[2]),
                (V1.vector4_u32[3] & !Control.vector4_u32[3]) | (V2.vector4_u32[3] & Control.vector4_u32[3]),
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp1: XMVECTOR = _mm_andnot_ps(Control, V1);
        let vTemp2: XMVECTOR = _mm_and_ps(V2, Control);
        return _mm_or_ps(vTemp1, vTemp2);
    }
}

/// Creates a new vector by combining the x and y-components of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMergeXY>
#[inline]
pub fn XMVectorMergeXY(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                V1.vector4_u32[0],
                V2.vector4_u32[0],
                V1.vector4_u32[1],
                V2.vector4_u32[1],
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_unpacklo_ps(V1, V2);
    }
}

/// Creates a new vector by combining the z and w-components of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMergeZW>
#[inline]
pub fn XMVectorMergeZW(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                V1.vector4_u32[2],
                V2.vector4_u32[2],
                V1.vector4_u32[3],
                V2.vector4_u32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_unpackhi_ps(V1, V2);
    }
}

/// Shifts a vector left by a given number of 32-bit elements, filling the vacated elements with elements from a second vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorShiftLeft>
#[inline]
pub fn XMVectorShiftLeft(V1: FXMVECTOR, V2: FXMVECTOR, Elements: u32) -> XMVECTOR {
    debug_assert!(Elements < 4);
    return XMVectorPermute(V1, V2, Elements, ((Elements)+1), ((Elements)+2), ((Elements)+3));
}

/// Rotates the vector left by a given number of 32-bit elements.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorRotateLeft>
#[inline]
pub fn XMVectorRotateLeft(V: FXMVECTOR, Elements: u32) -> XMVECTOR {
    debug_assert!(Elements < 4);
    return XMVectorSwizzle(V, Elements & 3, (Elements + 1) & 3, (Elements + 2) & 3, (Elements + 3) & 3);
}

/// Rotates the vector right by a given number of 32-bit elements.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorRotateRight>
#[inline]
pub fn XMVectorRotateRight(V: FXMVECTOR, Elements: u32) -> XMVECTOR {
    debug_assert!(Elements < 4);
    return XMVectorSwizzle(V, (4 - (Elements)) & 3, (5 - (Elements)) & 3, (6 - (Elements)) & 3, (7 - (Elements)) & 3);
}

/// Rotates a vector left by a given number of 32-bit components and insert selected elements of that result into another vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorInsert>
#[inline]
pub fn XMVectorInsert(
    VD: FXMVECTOR,
    VS: FXMVECTOR,
    VSLeftRotateElements: u32,
    Select0: u32,
    Select1: u32,
    Select2: u32,
    Select3: u32,
) -> XMVECTOR {
    let Control: XMVECTOR = XMVectorSelectControl(Select0 & 1, Select1 & 1, Select2 & 1, Select3 & 1);
    return XMVectorSelect(VD, XMVectorRotateLeft(VS, VSLeftRotateElements), Control);
}

/// Performs a per-component test for equality of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorEqual>
#[inline]
pub fn XMVectorEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_f32[0] == V2.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[1] == V2.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[2] == V2.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[3] == V2.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cmpeq_ps(V1, V2);
    }
}

/// Performs a per-component test for equality of two vectors and sets a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorEqualR>
#[inline]
pub fn XMVectorEqualR(pCR: &mut u32, V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let ux = if V1.vector4_f32[0] == V2.vector4_f32[0] { 0xFFFFFFFFu32 } else { 0 };
        let uy = if V1.vector4_f32[1] == V2.vector4_f32[1] { 0xFFFFFFFFu32 } else { 0 };
        let uz = if V1.vector4_f32[2] == V2.vector4_f32[2] { 0xFFFFFFFFu32 } else { 0 };
        let uw = if V1.vector4_f32[3] == V2.vector4_f32[3] { 0xFFFFFFFFu32 } else { 0 };
        let mut CR = 0;
        if ubool(ux & uy & uz & uw) {
            // All elements are greater
            CR = XM_CRMASK_CR6TRUE;
        } else if !ubool(ux | uy | uz | uw) {
            // All elements are not greater
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;

        let Control = XMVECTORU32 { u: [ ux, uy, uz, uw ]};
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        let mut CR = 0;
        let iTest: i32 = _mm_movemask_ps(vTemp);
        if (iTest == 0xf)
        {
            // All elements are greater
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTest)
        {
            // All elements are not greater
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;
        return vTemp;
    }
}

#[test]
fn test_XMVectorEqualR() {
    let a = XMVectorSet(1.0, 2.0, 3.0, 4.0);
    let b = XMVectorSet(1.0, 2.0, 3.0, 4.0);
    let mut cr = 0;
    let r = XMVectorEqualR(&mut cr, a, b);
    assert!(XMComparisonAllTrue(cr));
    assert_eq!([true, true, true, true],  [XMVectorGetX(r).is_nan(), XMVectorGetY(r).is_nan(), XMVectorGetZ(r).is_nan(), XMVectorGetW(r).is_nan()]);

    let a = XMVectorSet(0.0, 0.0, 0.0, 0.0);
    let b = XMVectorSplatOne();
    let r = XMVectorEqualR(&mut cr, a, b);
    assert!(XMComparisonAllFalse(cr));
    assert_eq!([false, false, false, false],  [XMVectorGetX(r).is_nan(), XMVectorGetY(r).is_nan(), XMVectorGetZ(r).is_nan(), XMVectorGetW(r).is_nan()]);

    let a = XMVectorSet(1.0, 0.0, 1.0, 0.0);
    let b = XMVectorSplatOne();
    let r = XMVectorEqualR(&mut cr, a, b);
    assert!(XMComparisonAnyFalse(cr));
    assert!(XMComparisonAnyTrue(cr));
    assert_eq!([true, false, true, false],  [XMVectorGetX(r).is_nan(), XMVectorGetY(r).is_nan(), XMVectorGetZ(r).is_nan(), XMVectorGetW(r).is_nan()]);
}

/// Performs a per-component test for the equality of two vectors, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorEqualInt>
#[inline]
pub fn XMVectorEqualInt(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_u32[0] == V2.vector4_u32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_u32[1] == V2.vector4_u32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_u32[2] == V2.vector4_u32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_u32[3] == V2.vector4_u32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return _mm_castsi128_ps(V);
    }
}

/// Performs a per-component test for equality of two vectors, treating each
/// component as an unsigned integer. In addition, this function sets a
/// comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorEqualIntR>
#[inline]
pub fn XMVectorEqualIntR(pCR: &mut u32, V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let Control: XMVECTOR = XMVectorEqualInt(V1, V2);

        *pCR = 0;
        if (XMVector4EqualInt(Control, XMVectorTrueInt()))
        {
            // All elements are equal
            *pCR |= XM_CRMASK_CR6TRUE;
        }
        else if (XMVector4EqualInt(Control, XMVectorFalseInt()))
        {
            // All elements are not equal
            *pCR |= XM_CRMASK_CR6FALSE;
        }
        return Control;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        let iTemp: i32 = _mm_movemask_ps(_mm_castsi128_ps(V));
        let mut CR: u32 = 0;
        if (iTemp == 0x0F)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTemp)
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;
        return _mm_castsi128_ps(V);
    }
}

/// Performs a per-component test for equality of two vectors within a given threshold.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorNearEqual>
#[inline]
pub fn XMVectorNearEqual(V1: FXMVECTOR, V2: FXMVECTOR, Epsilon: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let fDeltax: f32 = V1.vector4_f32[0] - V2.vector4_f32[0];
        let fDeltay: f32 = V1.vector4_f32[1] - V2.vector4_f32[1];
        let fDeltaz: f32 = V1.vector4_f32[2] - V2.vector4_f32[2];
        let fDeltaw: f32 = V1.vector4_f32[3] - V2.vector4_f32[3];

        let fDeltax = fabsf(fDeltax);
        let fDeltay = fabsf(fDeltay);
        let fDeltaz = fabsf(fDeltaz);
        let fDeltaw = fabsf(fDeltaw);

        let Control = XMVECTORU32 {
            u: [
                if fDeltax <= Epsilon.vector4_f32[0] { 0xFFFFFFFFu32 } else { 0 },
                if fDeltay <= Epsilon.vector4_f32[1] { 0xFFFFFFFFu32 } else { 0 },
                if fDeltaz <= Epsilon.vector4_f32[2] { 0xFFFFFFFFu32 } else { 0 },
                if fDeltaw <= Epsilon.vector4_f32[3] { 0xFFFFFFFFu32 } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Get the difference
        let vDelta: XMVECTOR = _mm_sub_ps(V1, V2);
        // Get the absolute value of the difference
        let mut vTemp: XMVECTOR = _mm_setzero_ps();
        vTemp = _mm_sub_ps(vTemp, vDelta);
        vTemp = _mm_max_ps(vTemp, vDelta);
        vTemp = _mm_cmple_ps(vTemp, Epsilon);
        return vTemp;
    }
}

/// Performs a per-component test for the inequality of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorNotEqual>
#[inline]
pub fn XMVectorNotEqual(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_f32[0] != V2.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[1] != V2.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[2] != V2.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[3] != V2.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cmpneq_ps(V1, V2);
    }
}

/// Performs a per-component test for the inequality of two vectors, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorNotEqualInt>
#[inline]
pub fn XMVectorNotEqualInt(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_u32[0] != V2.vector4_u32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_u32[1] != V2.vector4_u32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_u32[2] != V2.vector4_u32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_u32[3] != V2.vector4_u32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return _mm_xor_ps(_mm_castsi128_ps(V), g_XMNegOneMask.v);
    }
}

/// Performs a per-component test for greater-than between two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGreater>
#[inline]
pub fn XMVectorGreater(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_f32[0] > V2.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[1] > V2.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[2] > V2.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[3] > V2.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cmpgt_ps(V1, V2);
    }
}

/// Performs a per-component test for greater-than between two vectors and sets a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGreaterR>
#[inline]
pub fn XMVectorGreaterR(pCR: &mut u32, V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let ux = if V1.vector4_f32[0] > V2.vector4_f32[0] { 0xFFFFFFFFu32 } else { 0 };
        let uy = if V1.vector4_f32[1] > V2.vector4_f32[1] { 0xFFFFFFFFu32 } else { 0 };
        let uz = if V1.vector4_f32[2] > V2.vector4_f32[2] { 0xFFFFFFFFu32 } else { 0 };
        let uw = if V1.vector4_f32[3] > V2.vector4_f32[3] { 0xFFFFFFFFu32 } else { 0 };
        let mut CR = 0;
        if ubool(ux & uy & uz & uw) {
            // All elements are greater
            CR = XM_CRMASK_CR6TRUE;
        } else if !ubool(ux | uy | uz | uw) {
            // All elements are not greater
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;

        let Control = XMVECTORU32 { u: [ ux, uy, uz, uw ]};
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        let mut CR = 0;
        let iTest: i32 = _mm_movemask_ps(vTemp);
        if (iTest == 0xf)
        {
            // All elements are greater
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTest)
        {
            // All elements are not greater
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;
        return vTemp;
    }
}

/// Performs a per-component test for greater-than-or-equal between two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGreaterOrEqual>
#[inline]
pub fn XMVectorGreaterOrEqual(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_f32[0] >= V2.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[1] >= V2.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[2] >= V2.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[3] >= V2.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cmpge_ps(V1, V2);
    }
}

/// Performs a per-component test for greater-than-or-equal between two vectors and sets a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorGreaterOrEqualR>
#[inline]
pub fn XMVectorGreaterOrEqualR(pCR: &mut u32, V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let ux = if V1.vector4_f32[0] >= V2.vector4_f32[0] { 0xFFFFFFFFu32 } else { 0 };
        let uy = if V1.vector4_f32[1] >= V2.vector4_f32[1] { 0xFFFFFFFFu32 } else { 0 };
        let uz = if V1.vector4_f32[2] >= V2.vector4_f32[2] { 0xFFFFFFFFu32 } else { 0 };
        let uw = if V1.vector4_f32[3] >= V2.vector4_f32[3] { 0xFFFFFFFFu32 } else { 0 };
        let mut CR = 0;
        if ubool(ux & uy & uz & uw) {
            // All elements are greater
            CR = XM_CRMASK_CR6TRUE;
        } else if !ubool(ux | uy | uz | uw) {
            // All elements are not greater
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;

        let Control = XMVECTORU32 { u: [ ux, uy, uz, uw ]};
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        let mut CR = 0;
        let iTest: i32 = _mm_movemask_ps(vTemp);
        if (iTest == 0xf)
        {
            // All elements are greater
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTest)
        {
            // All elements are not greater
            CR = XM_CRMASK_CR6FALSE;
        }
        *pCR = CR;
        return vTemp;
    }
}

/// Performs a per-component test for less-than between two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorLess>
#[inline]
pub fn XMVectorLess(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_f32[0] < V2.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[1] < V2.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[2] < V2.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[3] < V2.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cmplt_ps(V1, V2);
    }
}

/// Performs a per-component test for less-than-or-equal between two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorLessOrEqual>
#[inline]
pub fn XMVectorLessOrEqual(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V1.vector4_f32[0] <= V2.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[1] <= V2.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[2] <= V2.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V1.vector4_f32[3] <= V2.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_cmple_ps(V1, V2);
    }
}

/// Tests whether the components of a given vector are within set bounds.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorInBounds>
#[inline]
pub fn XMVectorInBounds(V: FXMVECTOR, Bounds: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0] { 0xFFFFFFFF } else { 0 },
                if V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1] { 0xFFFFFFFF } else { 0 },
                if V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2] { 0xFFFFFFFF } else { 0 },
                if V.vector4_f32[3] <= Bounds.vector4_f32[3] && V.vector4_f32[3] >= -Bounds.vector4_f32[3] { 0xFFFFFFFF } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test if less than or equal
        let mut vTemp1: XMVECTOR = _mm_cmple_ps(V, Bounds);
        // Negate the bounds
        let mut vTemp2: XMVECTOR = _mm_mul_ps(Bounds, g_XMNegativeOne.v);
        // Test if greater or equal (Reversed)
        vTemp2 = _mm_cmple_ps(vTemp2, V);
        // Blend answers
        vTemp1 = _mm_and_ps(vTemp1, vTemp2);
        return vTemp1;
    }
}

/// Tests whether the components of a given vector are within certain bounds and sets a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorInBoundsR>
#[inline]
pub fn XMVectorInBoundsR(pCR: &mut u32, V: FXMVECTOR, Bounds: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let ux = if V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0] { 0xFFFFFFFF } else { 0 };
        let uy = if V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1] { 0xFFFFFFFF } else { 0 };
        let uz = if V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2] { 0xFFFFFFFF } else { 0 };
        let uw = if V.vector4_f32[3] <= Bounds.vector4_f32[3] && V.vector4_f32[3] >= -Bounds.vector4_f32[3] { 0xFFFFFFFF } else { 0 };

        let mut CR = 0;
        if ubool(ux & uy & uz & uw) {
            // All elements are in bounds
            CR = XM_CRMASK_CR6TRUE;
        }
        *pCR = CR;

        let Control = XMVECTORU32 { u: [ ux, uy, uz, uw ]};
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test if less than or equal
        let mut vTemp1: XMVECTOR = _mm_cmple_ps(V, Bounds);
        // Negate the bounds
        let mut vTemp2: XMVECTOR = _mm_mul_ps(Bounds, g_XMNegativeOne.v);
        // Test if greater or equal (Reversed)
        vTemp2 = _mm_cmple_ps(vTemp2, V);
        // Blend answers
        vTemp1 = _mm_and_ps(vTemp1, vTemp2);

        let mut CR: u32 = 0;
        if (_mm_movemask_ps(vTemp1) == 0xf)
        {
            // All elements are in bounds
            CR = XM_CRMASK_CR6BOUNDS;
        }
        *pCR = CR;
        return vTemp1;
    }
}

/// Performs a per-component NaN test on a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorIsNaN>
#[inline]
pub fn XMVectorIsNaN(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if XMISNAN!(V.vector4_f32[0]) { 0xFFFFFFFFu32 } else { 0 },
                if XMISNAN!(V.vector4_f32[1]) { 0xFFFFFFFFu32 } else { 0 },
                if XMISNAN!(V.vector4_f32[2]) { 0xFFFFFFFFu32 } else { 0 },
                if XMISNAN!(V.vector4_f32[3]) { 0xFFFFFFFFu32 } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test against itself. NaN is always not equal
        return _mm_cmpneq_ps(V, V);
    }
}

/// Makes a per-component comparison between two vectors, and returns a vector containing the smallest components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorIsInfinite>
#[inline]
pub fn XMVectorIsInfinite(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Control = XMVECTORU32 {
            u: [
                if XMISINF!(V.vector4_f32[0]) { 0xFFFFFFFFu32 } else { 0 },
                if XMISINF!(V.vector4_f32[1]) { 0xFFFFFFFFu32 } else { 0 },
                if XMISINF!(V.vector4_f32[2]) { 0xFFFFFFFFu32 } else { 0 },
                if XMISINF!(V.vector4_f32[3]) { 0xFFFFFFFFu32 } else { 0 },
            ]
        };
        return Control.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Mask off the sign bit
        let mut vTemp: __m128 = _mm_and_ps(V, g_XMAbsMask.v);
        // Compare to infinity
        vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity.v);
        // If any are infinity, the signs are true.
        return vTemp;
    }
}

/// Makes a per-component comparison between two vectors, and returns a vector containing the smallest components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMin>
#[inline]
pub fn XMVectorMin(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                if V1.vector4_f32[0] < V2.vector4_f32[0] { V1.vector4_f32[0] } else { V2.vector4_f32[0] },
                if V1.vector4_f32[1] < V2.vector4_f32[1] { V1.vector4_f32[1] } else { V2.vector4_f32[1] },
                if V1.vector4_f32[2] < V2.vector4_f32[2] { V1.vector4_f32[2] } else { V2.vector4_f32[2] },
                if V1.vector4_f32[3] < V2.vector4_f32[3] { V1.vector4_f32[3] } else { V2.vector4_f32[3] },
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_min_ps(V1, V2);
    }
}

/// Makes a per-component comparison between two vectors, and returns a vector containing the largest components.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMax>
#[inline]
pub fn XMVectorMax(V1: FXMVECTOR, V2: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                if V1.vector4_f32[0] > V2.vector4_f32[0] { V1.vector4_f32[0] } else { V2.vector4_f32[0] },
                if V1.vector4_f32[1] > V2.vector4_f32[1] { V1.vector4_f32[1] } else { V2.vector4_f32[1] },
                if V1.vector4_f32[2] > V2.vector4_f32[2] { V1.vector4_f32[2] } else { V2.vector4_f32[2] },
                if V1.vector4_f32[3] > V2.vector4_f32[3] { V1.vector4_f32[3] } else { V2.vector4_f32[3] },
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_max_ps(V1, V2);
    }
}

// Round to nearest (even) a.k.a. banker's rounding
#[inline]
#[cfg(_XM_NO_INTRINSICS_)]
fn round_to_nearest(mut x: f32) -> f32 {
    let i = floorf(x);
    x -= i;
    if (x < 0.5) {
        return i;
    }
    if (x > 0.5) {
        return i + 1.0;
    }

    let (_, int_part) = modff(i / 2.0);
    if ((2.0 * int_part) == i)
    {
        return i;
    }

    return i + 1.0;
}

#[test]
#[cfg(_XM_NO_INTRINSICS_)]
fn test_round_to_nearest() {
    assert_eq!(24.0, round_to_nearest(23.5));
    assert_eq!(24.0, round_to_nearest(24.5));

    assert_eq!(-24.0, round_to_nearest(-23.5));
    assert_eq!(-24.0, round_to_nearest(-24.5));
}

/// Rounds each component of a vector to the nearest even integer (known as "Bankers Rounding").
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorRound>
#[inline]
pub fn XMVectorRound(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                round_to_nearest(V.vector4_f32[0]),
                round_to_nearest(V.vector4_f32[0]),
                round_to_nearest(V.vector4_f32[0]),
                round_to_nearest(V.vector4_f32[0]),
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        return _mm_round_ps(V, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        let sign: __m128 = _mm_and_ps(V, g_XMNegativeZero.v);
        let sMagic: __m128 = _mm_or_ps(g_XMNoFraction.v, sign);
        let mut R1: __m128 = _mm_add_ps(V, sMagic);
        R1 = _mm_sub_ps(R1, sMagic);
        let mut R2:__m128 = _mm_and_ps(V, g_XMAbsMask.v);
        let mask: __m128 = _mm_cmple_ps(R2, g_XMNoFraction.v);
        R2 = _mm_andnot_ps(mask, V);
        R1 = _mm_and_ps(R1, mask);
        let vResult: XMVECTOR = _mm_xor_ps(R1, R2);
        return vResult;
    }
}

/// Rounds each component of a vector to the nearest integer value in the direction of zero.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorTruncate>
#[inline]
pub fn XMVectorTruncate(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut Result: XMVECTOR = mem::MaybeUninit::uninit().assume_init();

        for i in 0..4 {
            if (XMISNAN!(V.vector4_f32[i]))
            {
                Result.vector4_u32[i] = 0x7FC00000;
            }
            else if (fabsf(V.vector4_f32[i]) < 8388608.0)
            {
                Result.vector4_f32[i] = (V.vector4_f32[i] as i32) as f32;
            }
            else
            {
                Result.vector4_f32[i] = V.vector4_f32[i];
            }
        }

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        return _mm_round_ps(V, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // To handle NAN, INF and numbers greater than 8388608, use masking
        // Get the abs value
        let mut vTest: __m128i  = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask.m128i());
        // Test for greater than 8388608 (All floats with NO fractionals, NAN and INF
        vTest = _mm_cmplt_epi32(vTest, g_XMNoFraction.m128i());
        // Convert to int and back to float for rounding with truncation
        let vInt: __m128i = _mm_cvttps_epi32(V);
        // Convert back to floats
        let mut vResult: XMVECTOR = _mm_cvtepi32_ps(vInt);
        // All numbers less than 8388608 will use the round to int
        vResult = _mm_and_ps(vResult, _mm_castsi128_ps(vTest));
        // All others, use the ORIGINAL value
        vTest = _mm_andnot_si128(vTest, _mm_castps_si128(V));
        vResult = _mm_or_ps(vResult, _mm_castsi128_ps(vTest));
        return vResult;
    }
}

/// Computes the floor of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorFloor>
#[inline]
pub fn XMVectorFloor(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                floorf(V.vector4_f32[0]),
                floorf(V.vector4_f32[1]),
                floorf(V.vector4_f32[2]),
                floorf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        return _mm_floor_ps(V);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // To handle NAN, INF and numbers greater than 8388608, use masking
        let mut vTest: __m128i = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask.m128i());
        vTest = _mm_cmplt_epi32(vTest, g_XMNoFraction.m128i());
        // Truncate
        let vInt: __m128i = _mm_cvttps_epi32(V);
        let mut vResult: XMVECTOR = _mm_cvtepi32_ps(vInt);
        let mut vLarger: __m128 = _mm_cmpgt_ps(vResult, V);
        // 0 -> 0, 0xffffffff -> -1.0f
        vLarger = _mm_cvtepi32_ps(_mm_castps_si128(vLarger));
        vResult = _mm_add_ps(vResult, vLarger);
        // All numbers less than 8388608 will use the round to int
        vResult = _mm_and_ps(vResult, _mm_castsi128_ps(vTest));
        // All others, use the ORIGINAL value
        vTest = _mm_andnot_si128(vTest, _mm_castps_si128(V));
        vResult = _mm_or_ps(vResult, _mm_castsi128_ps(vTest));
        return vResult;
    }
}

/// Computes the ceiling of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorCeiling>
#[inline]
pub fn XMVectorCeiling(V: FXMVECTOR) -> XMVECTOR {
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                ceilf(V.vector4_f32[0]),
                ceilf(V.vector4_f32[1]),
                ceilf(V.vector4_f32[2]),
                ceilf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        return _mm_ceil_ps(V);
    }

    #[cfg(all(not(_XM_SSE4_INTRINSICS_), _XM_SSE_INTRINSICS_))]
    unsafe {
        // To handle NAN, INF and numbers greater than 8388608, use masking
        let mut vTest: __m128i = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask.m128i());
        vTest = _mm_cmplt_epi32(vTest, g_XMNoFraction.m128i());
        // Truncate
        let vInt: __m128i = _mm_cvttps_epi32(V);
        let mut vResult: XMVECTOR = _mm_cvtepi32_ps(vInt);
        let mut vSmaller: __m128 = _mm_cmplt_ps(vResult, V);
        // 0 -> 0, 0xffffffff -> -1.0f
        vSmaller = _mm_cvtepi32_ps(_mm_castps_si128(vSmaller));
        vResult = _mm_sub_ps(vResult, vSmaller);
        // All numbers less than 8388608 will use the round to int
        vResult = _mm_and_ps(vResult, _mm_castsi128_ps(vTest));
        // All others, use the ORIGINAL value
        vTest = _mm_andnot_si128(vTest, _mm_castps_si128(V));
        vResult = _mm_or_ps(vResult, _mm_castsi128_ps(vTest));
        return vResult;
    }
}

/// Clamps the components of a vector to a specified minimum and maximum range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorClamp>
#[inline]
pub fn XMVectorClamp(
    V: FXMVECTOR,
    Min: FXMVECTOR,
    Max: FXMVECTOR
) -> XMVECTOR
{
    debug_assert!(XMVector4LessOrEqual(Min, Max));

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;
        Result = XMVectorMax(Min, V);
        Result = XMVectorMin(Max, Result);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR;
        vResult = _mm_max_ps(Min, V);
        vResult = _mm_min_ps(Max, vResult);
        return vResult;
    }
}

/// Saturates each component of a vector to the range 0.0f to 1.0f.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSaturate>
#[inline]
pub fn XMVectorSaturate(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        // const XMVECTOR Zero = XMVectorZero();
        return XMVectorClamp(V, g_XMZero.v, g_XMOne.v);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Set <0 to 0
        let vResult: XMVECTOR = _mm_max_ps(V, g_XMZero.v);
        // Set>1 to 1
        return _mm_min_ps(vResult, g_XMOne.v);
    }
}

/// Computes the logical AND of two vectors, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorAndInt>
#[inline]
pub fn XMVectorAndInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                V1.vector4_u32[0] & V2.vector4_u32[0],
                V1.vector4_u32[1] & V2.vector4_u32[1],
                V1.vector4_u32[2] & V2.vector4_u32[2],
                V1.vector4_u32[3] & V2.vector4_u32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_and_ps(V1, V2);
    }
}

/// Computes the logical AND of one vector with the negation of a second vector, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorAndCInt>
#[inline]
pub fn XMVectorAndCInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                V1.vector4_u32[0] & !V2.vector4_u32[0],
                V1.vector4_u32[1] & !V2.vector4_u32[1],
                V1.vector4_u32[2] & !V2.vector4_u32[2],
                V1.vector4_u32[3] & !V2.vector4_u32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_andnot_si128(_mm_castps_si128(V2), _mm_castps_si128(V1));
        return _mm_castsi128_ps(V);
    }
}

/// Computes the logical OR of two vectors, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorOrInt>
#[inline]
pub fn XMVectorOrInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                V1.vector4_u32[0] | V2.vector4_u32[0],
                V1.vector4_u32[1] | V2.vector4_u32[1],
                V1.vector4_u32[2] | V2.vector4_u32[2],
                V1.vector4_u32[3] | V2.vector4_u32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_or_si128(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return _mm_castsi128_ps(V);
    }
}

/// Computes the logical NOR of two vectors, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorNorInt>
#[inline]
pub fn XMVectorNorInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                !(V1.vector4_u32[0] | V2.vector4_u32[0]),
                !(V1.vector4_u32[1] | V2.vector4_u32[1]),
                !(V1.vector4_u32[2] | V2.vector4_u32[2]),
                !(V1.vector4_u32[3] | V2.vector4_u32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut Result: __m128i;
        Result = _mm_or_si128(_mm_castps_si128(V1), _mm_castps_si128(V2));
        Result = _mm_andnot_si128(Result, g_XMNegOneMask.m128i());
        return _mm_castsi128_ps(Result);
    }
}

/// Computes the logical XOR of two vectors, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorXorInt>
#[inline]
pub fn XMVectorXorInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORU32 {
            u: [
                V1.vector4_u32[0] ^ V2.vector4_u32[0],
                V1.vector4_u32[1] ^ V2.vector4_u32[1],
                V1.vector4_u32[2] ^ V2.vector4_u32[2],
                V1.vector4_u32[3] ^ V2.vector4_u32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let V: __m128i = _mm_xor_si128(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return _mm_castsi128_ps(V);
    }
}

/// Computes the negation of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorNegate>
#[inline]
pub fn XMVectorNegate(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                -V.vector4_f32[0],
                -V.vector4_f32[1],
                -V.vector4_f32[2],
                -V.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let Z: XMVECTOR;

        Z = _mm_setzero_ps();

        return _mm_sub_ps(Z, V);
    }
}

/// Computes the sum of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorAdd>
#[inline]
pub fn XMVectorAdd(
    V1: FXMVECTOR,
    V2: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V1.vector4_f32[0] + V2.vector4_f32[0],
                V1.vector4_f32[1] + V2.vector4_f32[1],
                V1.vector4_f32[2] + V2.vector4_f32[2],
                V1.vector4_f32[3] + V2.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_add_ps(V1, V2);
    }
}

/// Computes the horizontal sum of the components of an XMVECTOR.
/// The horizontal sum is the result of adding each component in
/// the vector together.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSum>
#[inline]
pub fn XMVectorSum(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let sum = V.vector4_f32[0] + V.vector4_f32[1] + V.vector4_f32[2] + V.vector4_f32[3];
        let Result = XMVECTORF32 {
            f: [ sum, sum, sum, sum ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE3_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_hadd_ps(V, V);
        return _mm_hadd_ps(vTemp, vTemp);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_)))]
    unsafe {
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 3, 0, 1));
        let vTemp2: XMVECTOR = _mm_add_ps(V, vTemp);
        vTemp = XM_PERMUTE_PS!(vTemp2, _MM_SHUFFLE(1, 0, 3, 2));
        return _mm_add_ps(vTemp, vTemp2);
    }
}

/// Adds two vectors representing angles.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorAddAngles>
#[inline]
pub fn XMVectorAddAngles(
    V1: FXMVECTOR,
    V2: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        //const XMVECTOR Zero = XMVectorZero();
        const Zero: XMVECTOR = unsafe { g_XMZero.v };

        // Add the given angles together.  If the range of V1 is such
        // that -Pi <= V1 < Pi and the range of V2 is such that
        // -2Pi <= V2 <= 2Pi, then the range of the resulting angle
        // will be -Pi <= Result < Pi.
        let mut Result: XMVECTOR = XMVectorAdd(V1, V2);

        let mut Mask: XMVECTOR = XMVectorLess(Result, g_XMNegativePi.v);
        let mut Offset: XMVECTOR = XMVectorSelect(Zero, g_XMTwoPi.v, Mask);

        Mask = XMVectorGreaterOrEqual(Result, g_XMPi.v);
        Offset = XMVectorSelect(Offset, g_XMNegativeTwoPi.v, Mask);

        Result = XMVectorAdd(Result, Offset);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Adjust the angles
        let mut vResult: XMVECTOR = _mm_add_ps(V1, V2);
        // Less than Pi?
        let mut vOffset: XMVECTOR = _mm_cmplt_ps(vResult, g_XMNegativePi.v);
        vOffset = _mm_and_ps(vOffset, g_XMTwoPi.v);
        // Add 2Pi to all entries less than -Pi
        vResult = _mm_add_ps(vResult, vOffset);
        // Greater than or equal to Pi?
        vOffset = _mm_cmpge_ps(vResult, g_XMPi.v);
        vOffset = _mm_and_ps(vOffset, g_XMTwoPi.v);
        // Sub 2Pi to all entries greater than Pi
        vResult = _mm_sub_ps(vResult, vOffset);
        return vResult;
    }
}

/// Computes the difference of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSubtract>
#[inline]
pub fn XMVectorSubtract(
    V1: FXMVECTOR,
    V2: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V1.vector4_f32[0] - V2.vector4_f32[0],
                V1.vector4_f32[1] - V2.vector4_f32[1],
                V1.vector4_f32[2] - V2.vector4_f32[2],
                V1.vector4_f32[3] - V2.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_sub_ps(V1, V2);
    }
}

/// Adds two vectors representing angles.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSubtractAngles>
#[inline]
pub fn XMVectorSubtractAngles(
    V1: FXMVECTOR,
    V2: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        //const XMVECTOR Zero = XMVectorZero();
        const Zero: XMVECTOR = unsafe { g_XMZero.v };

        // Subtract the given angles.  If the range of V1 is such
        // that -Pi <= V1 < Pi and the range of V2 is such that
        // -2Pi <= V2 <= 2Pi, then the range of the resulting angle
        // will be -Pi <= Result < Pi.
        let mut Result: XMVECTOR = XMVectorSubtract(V1, V2);

        let mut Mask: XMVECTOR = XMVectorLess(Result, g_XMNegativePi.v);
        let mut Offset: XMVECTOR = XMVectorSelect(Zero, g_XMTwoPi.v, Mask);

        Mask = XMVectorGreaterOrEqual(Result, g_XMPi.v);
        Offset = XMVectorSelect(Offset, g_XMNegativeTwoPi.v, Mask);

        Result = XMVectorAdd(Result, Offset);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Adjust the angles
        let mut vResult: XMVECTOR = _mm_sub_ps(V1, V2);
        // Less than Pi?
        let mut vOffset: XMVECTOR = _mm_cmplt_ps(vResult, g_XMNegativePi.v);
        vOffset = _mm_and_ps(vOffset, g_XMTwoPi.v);
        // Add 2Pi to all entries less than -Pi
        vResult = _mm_add_ps(vResult, vOffset);
        // Greater than or equal to Pi?
        vOffset = _mm_cmpge_ps(vResult, g_XMPi.v);
        vOffset = _mm_and_ps(vOffset, g_XMTwoPi.v);
        // Sub 2Pi to all entries greater than Pi
        vResult = _mm_sub_ps(vResult, vOffset);
        return vResult;
    }
}


/// Computes the per-component product of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMultiply>
#[inline]
pub fn XMVectorMultiply(
    V1: FXMVECTOR,
    V2: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V1.vector4_f32[0] * V2.vector4_f32[0],
                V1.vector4_f32[1] * V2.vector4_f32[1],
                V1.vector4_f32[2] * V2.vector4_f32[2],
                V1.vector4_f32[3] * V2.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_mul_ps(V1, V2);
    }
}

/// Computes the product of the first two vectors added to the third vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMultiplyAdd>
#[inline]
pub fn XMVectorMultiplyAdd(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    V3: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V1.vector4_f32[0] * V2.vector4_f32[0] + V3.vector4_f32[0],
                V1.vector4_f32[1] * V2.vector4_f32[1] + V3.vector4_f32[1],
                V1.vector4_f32[2] * V2.vector4_f32[2] + V3.vector4_f32[2],
                V1.vector4_f32[3] * V2.vector4_f32[3] + V3.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return XM_FMADD_PS!(V1, V2, V3);
    }
}

/// Divides one instance of XMVECTOR by a second instance, returning the result in a third instance.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorDivide>
#[inline]
pub fn XMVectorDivide(
    V1: FXMVECTOR,
    V2: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V1.vector4_f32[0] / V2.vector4_f32[0],
                V1.vector4_f32[1] / V2.vector4_f32[1],
                V1.vector4_f32[2] / V2.vector4_f32[2],
                V1.vector4_f32[3] / V2.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_div_ps(V1, V2);
    }
}

/// Computes the difference of a third vector and the product of the first two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorNegativeMultiplySubtract>
#[inline]
pub fn XMVectorNegativeMultiplySubtract(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    V3: FXMVECTOR
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V3.vector4_f32[0] - (V1.vector4_f32[0] * V2.vector4_f32[0]),
                V3.vector4_f32[1] - (V1.vector4_f32[1] * V2.vector4_f32[1]),
                V3.vector4_f32[2] - (V1.vector4_f32[2] * V2.vector4_f32[2]),
                V3.vector4_f32[3] - (V1.vector4_f32[3] * V2.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return XM_FNMADD_PS!(V1, V2, V3);
    }
}

/// Scalar multiplies a vector by a floating-point value.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorScale>
#[inline]
pub fn XMVectorScale(
    V: FXMVECTOR,
    ScaleFactor: f32,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                V.vector4_f32[0] * ScaleFactor,
                V.vector4_f32[1] * ScaleFactor,
                V.vector4_f32[2] * ScaleFactor,
                V.vector4_f32[3] * ScaleFactor
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vResult: XMVECTOR = _mm_set_ps1(ScaleFactor);
        return _mm_mul_ps(vResult, V);
    }
}

/// Estimates the per-component reciprocal of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReciprocalEst>
#[inline]
pub fn XMVectorReciprocalEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                1.0 / V.vector4_f32[0],
                1.0 / V.vector4_f32[1],
                1.0 / V.vector4_f32[2],
                1.0 / V.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_rcp_ps(V);
    }
}

/// Estimates the per-component reciprocal of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReciprocal>
#[inline]
pub fn XMVectorReciprocal(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                1.0 / V.vector4_f32[0],
                1.0 / V.vector4_f32[1],
                1.0 / V.vector4_f32[2],
                1.0 / V.vector4_f32[3]
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_div_ps(g_XMOne.v, V);
    }
}

/// Estimates the per-component square root of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSqrtEst>
#[inline]
pub fn XMVectorSqrtEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                sqrtf(V.vector4_f32[0]),
                sqrtf(V.vector4_f32[1]),
                sqrtf(V.vector4_f32[2]),
                sqrtf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_sqrt_ps(V);
    }
}

/// Computes the per-component square root of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSqrt>
#[inline]
pub fn XMVectorSqrt(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                sqrtf(V.vector4_f32[0]),
                sqrtf(V.vector4_f32[1]),
                sqrtf(V.vector4_f32[2]),
                sqrtf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_sqrt_ps(V);
    }
}

/// Estimates the per-component reciprocal square root of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReciprocalSqrtEst>
#[inline]
pub fn XMVectorReciprocalSqrtEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                1.0 / sqrtf(V.vector4_f32[0]),
                1.0 / sqrtf(V.vector4_f32[1]),
                1.0 / sqrtf(V.vector4_f32[2]),
                1.0 / sqrtf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        return _mm_rsqrt_ps(V);
    }
}

/// Computes the per-component reciprocal square root of a vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorReciprocalSqrt>
#[inline]
pub fn XMVectorReciprocalSqrt(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                1.0 / sqrtf(V.vector4_f32[0]),
                1.0 / sqrtf(V.vector4_f32[1]),
                1.0 / sqrtf(V.vector4_f32[2]),
                1.0 / sqrtf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_sqrt_ps(V);
        vResult = _mm_div_ps(g_XMOne.v, vResult);
        return vResult;
    }
}

/// Computes two raised to the power for each component.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorExp2>
#[inline]
pub fn XMVectorExp2(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                powf(2.0, V.vector4_f32[0]),
                powf(2.0, V.vector4_f32[1]),
                powf(2.0, V.vector4_f32[2]),
                powf(2.0, V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let itrunc: __m128i = _mm_cvttps_epi32(V);
        let ftrunc: __m128 = _mm_cvtepi32_ps(itrunc);
        let y: __m128 = _mm_sub_ps(V, ftrunc);

        let mut poly: __m128 = XM_FMADD_PS!(g_XMExpEst7.v, y, g_XMExpEst6.v);
        poly = XM_FMADD_PS!(poly, y, g_XMExpEst5.v);
        poly = XM_FMADD_PS!(poly, y, g_XMExpEst4.v);
        poly = XM_FMADD_PS!(poly, y, g_XMExpEst3.v);
        poly = XM_FMADD_PS!(poly, y, g_XMExpEst2.v);
        poly = XM_FMADD_PS!(poly, y, g_XMExpEst1.v);
        poly = XM_FMADD_PS!(poly, y, g_XMOne.v);

        let mut biased: __m128i = _mm_add_epi32(itrunc, g_XMExponentBias.m128i());
        biased = _mm_slli_epi32(biased, 23);
        let result0: __m128 = _mm_div_ps(_mm_castsi128_ps(biased), poly);

        biased = _mm_add_epi32(itrunc, g_XM253.m128i());
        biased = _mm_slli_epi32(biased, 23);
        let mut result1: __m128 = _mm_div_ps(_mm_castsi128_ps(biased), poly);
        result1 = _mm_mul_ps(g_XMMinNormal.v, result1);

        // Use selection to handle the cases
        //  if (V is NaN) -> QNaN;
        //  else if (V sign bit set)
        //      if (V > -150)
        //         if (V.exponent < -126) -> result1
        //         else -> result0
        //      else -> +0
        //  else
        //      if (V < 128) -> result0
        //      else -> +inf

        let mut comp: __m128i = _mm_cmplt_epi32(_mm_castps_si128(V), g_XMBin128.m128i());
        let mut select0: __m128i = _mm_and_si128(comp, _mm_castps_si128(result0));
        let mut select1: __m128i = _mm_andnot_si128(comp, g_XMInfinity.m128i());
        let result2: __m128i = _mm_or_si128(select0, select1);

        comp = _mm_cmplt_epi32(itrunc, g_XMSubnormalExponent.m128i());
        select1 = _mm_and_si128(comp, _mm_castps_si128(result1));
        select0 = _mm_andnot_si128(comp, _mm_castps_si128(result0));
        let result3: __m128i = _mm_or_si128(select0, select1);

        comp = _mm_cmplt_epi32(_mm_castps_si128(V), g_XMBinNeg150.m128i());
        select0 = _mm_and_si128(comp, result3);
        select1 = _mm_andnot_si128(comp, g_XMZero.m128i());
        let result4: __m128i = _mm_or_si128(select0, select1);

        let sign: __m128i = _mm_and_si128(_mm_castps_si128(V), g_XMNegativeZero.m128i());
        comp = _mm_cmpeq_epi32(sign, g_XMNegativeZero.m128i());
        select0 = _mm_and_si128(comp, result4);
        select1 = _mm_andnot_si128(comp, result2);
        let result5: __m128i = _mm_or_si128(select0, select1);

        let mut t0: __m128i = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest.m128i());
        let mut t1: __m128i = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity.m128i());
        t0 = _mm_cmpeq_epi32(t0, g_XMZero.m128i());
        t1 = _mm_cmpeq_epi32(t1, g_XMInfinity.m128i());
        let isNaN: __m128i = _mm_andnot_si128(t0, t1);

        select0 = _mm_and_si128(isNaN, g_XMQNaN.m128i());
        select1 = _mm_andnot_si128(isNaN, result5);
        let vResult: __m128i = _mm_or_si128(select0, select1);

        return _mm_castsi128_ps(vResult);
    }
}

// TODO: XMVectorExpE

/// Computes two raised to the power for each component.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorExp>
#[inline]
pub fn XMVectorExp(
    V: FXMVECTOR,
) -> XMVECTOR {
    return XMVectorExp2(V);
}

// TODO: Internal / multi_sll_epi32
// TODO: Internal / multi_srl_epi32
// TODO: Internal / GetLeadingBit __m128i
// TODO: Internal / GetLeadingBit int32x4_t
// TODO: XMVectorLog2
// TODO: XMVectorLogE
// TODO: XMVectorLog

/// Computes V1 raised to the power of V2.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorPow>
#[inline]
pub fn XMVectorPow(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                powf(V1.vector4_f32[0], V2.vector4_f32[0]),
                powf(V1.vector4_f32[1], V2.vector4_f32[1]),
                powf(V1.vector4_f32[2], V2.vector4_f32[2]),
                powf(V1.vector4_f32[3], V2.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut a: Align16<[f32; 4]> = mem::MaybeUninit::uninit().assume_init();
        let mut b: Align16<[f32; 4]> = mem::MaybeUninit::uninit().assume_init();
        _mm_store_ps(a.as_mut_ptr(), V1);
        _mm_store_ps(b.as_mut_ptr(), V2);
        let vResult: XMVECTOR = _mm_setr_ps(
            powf(a[0], b[0]),
            powf(a[1], b[1]),
            powf(a[2], b[2]),
            powf(a[3], b[3]));
        return vResult;
    }
}

/// Computes the absolute value of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorAbs>
#[inline]
pub fn XMVectorAbs(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                fabsf(V.vector4_f32[0]),
                fabsf(V.vector4_f32[1]),
                fabsf(V.vector4_f32[2]),
                fabsf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_setzero_ps();
        vResult = _mm_sub_ps(vResult, V);
        vResult = _mm_max_ps(vResult, V);
        return vResult;
    }
}

/// Computes the per-component floating-point remainder of the quotient of two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorMod>
#[inline]
pub fn XMVectorMod(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    // V1 % V2 = V1 - V2 * truncate(V1 / V2)

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Quotient: XMVECTOR = XMVectorDivide(V1, V2);
        Quotient = XMVectorTruncate(Quotient);
        let Result: XMVECTOR = XMVectorNegativeMultiplySubtract(V2, Quotient, V1);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = _mm_div_ps(V1, V2);
        vResult = XMVectorTruncate(vResult);
        return XM_FNMADD_PS!(vResult, V2, V1);
    }
}

/// Computes the per-component angle modulo 2PI.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorModAngles>
#[inline]
pub fn XMVectorModAngles(
    Angles: FXMVECTOR,
) -> XMVECTOR
{
    // V1 % V2 = V1 - V2 * truncate(V1 / V2)

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut V: XMVECTOR;
        let Result: XMVECTOR;

        // Modulo the range of the given angles such that -XM_PI <= Angles < XM_PI
        V = XMVectorMultiply(Angles, g_XMReciprocalTwoPi.v);
        V = XMVectorRound(V);
        Result = XMVectorNegativeMultiplySubtract(g_XMTwoPi.v, V, Angles);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Modulo the range of the given angles such that -XM_PI <= Angles < XM_PI
        let mut vResult: XMVECTOR = _mm_mul_ps(Angles, g_XMReciprocalTwoPi.v);
        // Use the inline function due to complexity for rounding
        vResult = XMVectorRound(vResult);
        return XM_FNMADD_PS!(vResult, g_XMTwoPi.v, Angles);
    }
}

/// Computes the sine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSin>
#[inline]
pub fn XMVectorSin(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 11-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                sinf(V.vector4_f32[0]),
                sinf(V.vector4_f32[1]),
                sinf(V.vector4_f32[2]),
                sinf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Force the value within the bounds of pi
        let mut x: XMVECTOR = XMVectorModAngles(V);

        // Map in [-pi/2,pi/2] with sin(y) = sin(x).
        let sign: __m128 = _mm_and_ps(x, g_XMNegativeZero.v);
        let c: __m128 = _mm_or_ps(g_XMPi.v, sign);  // pi when x >= 0, -pi when x < 0
        let absx: __m128 = _mm_andnot_ps(sign, x);  // |x|
        let rflx: __m128 = _mm_sub_ps(c, x);
        let comp: __m128 = _mm_cmple_ps(absx, g_XMHalfPi.v);
        let select0: __m128 = _mm_and_ps(comp, x);
        let select1: __m128 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        const SC1: XMVECTOR = unsafe { g_XMSinCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(SC1, _MM_SHUFFLE(0, 0, 0, 0));
        const SC0: XMVECTOR = unsafe { g_XMSinCoefficients0.v };
        let mut vConstants: __m128 = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(3, 3, 3, 3));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(2, 2, 2, 2));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, x);
        return Result;
    }
}

#[test]
fn test_XMVectorSin() {
    for angle in &[-XM_PI, -XM_PI/2.0, -XM_PI/3.0, -XM_PI/4.0, 0.0, XM_PI/4.0, XM_PI/3.0, XM_PI/2.0, XM_PI] {
        let scalar = angle.sin();
        let vector = XMVectorReplicate(*angle);
        let vector = XMVectorSin(vector);
        assert_approx_eq!(scalar, XMVectorGetX(vector));
        assert_approx_eq!(scalar, XMVectorGetY(vector));
        assert_approx_eq!(scalar, XMVectorGetZ(vector));
        assert_approx_eq!(scalar, XMVectorGetW(vector));
    }
}

/// Computes the cosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorCos>
#[inline]
pub fn XMVectorCos(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 10-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                cosf(V.vector4_f32[0]),
                cosf(V.vector4_f32[1]),
                cosf(V.vector4_f32[2]),
                cosf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Map V to x in [-pi,pi].
        let mut x: XMVECTOR = XMVectorModAngles(V);

        // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
        let mut sign: XMVECTOR = _mm_and_ps(x, g_XMNegativeZero.v);
        let c: __m128 = _mm_or_ps(g_XMPi.v, sign);  // pi when x >= 0, -pi when x < 0
        let absx: __m128 = _mm_andnot_ps(sign, x);  // |x|
        let rflx: __m128 = _mm_sub_ps(c, x);
        let comp: __m128  = _mm_cmple_ps(absx, g_XMHalfPi.v);
        let mut select0: __m128 = _mm_and_ps(comp, x);
        let mut select1: __m128 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);
        select0 = _mm_and_ps(comp, g_XMOne.v);
        select1 = _mm_andnot_ps(comp, g_XMNegativeOne.v);
        sign = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        const CC1: XMVECTOR = unsafe { g_XMCosCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(CC1, _MM_SHUFFLE(0, 0, 0, 0));
        const CC0: XMVECTOR = unsafe { g_XMCosCoefficients0.v };
        let mut vConstants: __m128 = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(3, 3, 3, 3));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(2, 2, 2, 2));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, sign);
        return Result;
    }
}

#[test]
fn test_XMVectorCos() {
    for angle in &[-XM_PI, -XM_PI/2.0, -XM_PI/3.0, -XM_PI/4.0, 0.0, XM_PI/4.0, XM_PI/3.0, XM_PI/2.0, XM_PI] {
        let scalar = angle.cos();
        let vector = XMVectorReplicate(*angle);
        let vector = XMVectorCos(vector);
        assert_approx_eq!(scalar, XMVectorGetX(vector));
        assert_approx_eq!(scalar, XMVectorGetY(vector));
        assert_approx_eq!(scalar, XMVectorGetZ(vector));
        assert_approx_eq!(scalar, XMVectorGetW(vector));
    }
}

/// Computes the sine and cosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSinCos>
#[inline]
pub fn XMVectorSinCos(
    pSin: &mut XMVECTOR,
    pCos: &mut XMVECTOR,
    V: FXMVECTOR,
)
{
    // 11/10-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Sin = XMVECTORF32 {
            f: [
                sinf(V.vector4_f32[0]),
                sinf(V.vector4_f32[1]),
                sinf(V.vector4_f32[2]),
                sinf(V.vector4_f32[3])
            ]
        };
        let Cos = XMVECTORF32 {
            f: [
                cosf(V.vector4_f32[0]),
                cosf(V.vector4_f32[1]),
                cosf(V.vector4_f32[2]),
                cosf(V.vector4_f32[3])
            ]
        };
        *pSin = Sin.v;
        *pCos = Cos.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Force the value within the bounds of pi
        let mut x: XMVECTOR = XMVectorModAngles(V);

        // Map in [-pi/2,pi/2] with sin(y) = sin(x), cos(y) = sign*cos(x).
        let mut sign: XMVECTOR = _mm_and_ps(x, g_XMNegativeZero.v);
        let c: __m128 = _mm_or_ps(g_XMPi.v, sign);  // pi when x >= 0, -pi when x < 0
        let absx: __m128 = _mm_andnot_ps(sign, x);  // |x|
        let rflx: __m128 = _mm_sub_ps(c, x);
        let comp: __m128 = _mm_cmple_ps(absx, g_XMHalfPi.v);
        let mut select0: __m128 = _mm_and_ps(comp, x);
        let mut select1: __m128 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);
        select0 = _mm_and_ps(comp, g_XMOne.v);
        select1 = _mm_andnot_ps(comp, g_XMNegativeOne.v);
        sign = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation of sine
        const SC1: XMVECTOR = unsafe { g_XMSinCoefficients1.v };
        let mut vConstantsB: __m128 = XM_PERMUTE_PS!(SC1, _MM_SHUFFLE(0, 0, 0, 0));
        const SC0: XMVECTOR = unsafe { g_XMSinCoefficients0.v };
        let mut vConstants: __m128 = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(3, 3, 3, 3));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(2, 2, 2, 2));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SC0, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, x);
        *pSin = Result;

        // Compute polynomial approximation of cosine
        const CC1: XMVECTOR = unsafe { g_XMCosCoefficients1.v };
        vConstantsB = XM_PERMUTE_PS!(CC1, _MM_SHUFFLE(0, 0, 0, 0));
        const CC0: XMVECTOR = unsafe { g_XMCosCoefficients0.v };
        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(3, 3, 3, 3));
        Result = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(2, 2, 2, 2));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CC0, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, sign);
        *pCos = Result;
    }
}

/// Computes the tangent of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorTan>
#[inline]
pub fn XMVectorTan(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // Cody and Waite algorithm to compute tangent.

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                tanf(V.vector4_f32[0]),
                tanf(V.vector4_f32[1]),
                tanf(V.vector4_f32[2]),
                tanf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(any(_XM_SSE_INTRINSICS_, _XM_ARM_NEON_INTRINSICS_))]
    unsafe {
        const TanCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [1.0, -4.667168334e-1, 2.566383229e-2, -3.118153191e-4] };
        const TanCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [4.981943399e-7, -1.333835001e-1, 3.424887824e-3, -1.786170734e-5] };
        const TanConstants: XMVECTORF32 = XMVECTORF32 { f: [1.570796371, 6.077100628e-11, 0.000244140625, 0.63661977228 /*2 / Pi*/ ] };
        const Mask: XMVECTORU32 = XMVECTORU32 { u: [0x1, 0x1, 0x1, 0x1] };

        let TwoDivPi: XMVECTOR = XMVectorSplatW(TanConstants.v);

        let Zero: XMVECTOR = XMVectorZero();

        let C0: XMVECTOR = XMVectorSplatX(TanConstants.v);
        let C1: XMVECTOR = XMVectorSplatY(TanConstants.v);
        let Epsilon: XMVECTOR = XMVectorSplatZ(TanConstants.v);

        let mut VA: XMVECTOR = XMVectorMultiply(V, TwoDivPi);

        VA = XMVectorRound(VA);

        let mut VC: XMVECTOR = XMVectorNegativeMultiplySubtract(VA, C0, V);

        let mut VB: XMVECTOR = XMVectorAbs(VA);

        VC = XMVectorNegativeMultiplySubtract(VA, C1, VC);

        #[cfg(_XM_ARM_NEON_INTRINSICS_)]
        {
            unimplemented!()
            // VB = vcvtq_u32_f32(VB);
        }

        #[cfg(_XM_SSE_INTRINSICS_)]
        {
            VB = mem::transmute(_mm_cvttps_epi32(VB));
        }

        // NOTE: This block does not appear to be reachable in the source
        //       and is not reachable here. It's left for reference only.
        #[cfg(all(not(_XM_SSE_INTRINSICS_), not(_XM_ARM_NEON_INTRINSICS_)))]
        {
            for i in 0..4 {
                VB.vector4_u32[i] = (VB.vector4_f32[i]) as i32 as u32;
            }
        }

        let VC2: XMVECTOR = XMVectorMultiply(VC, VC);

        let T7: XMVECTOR = XMVectorSplatW(TanCoefficients1.v);
        let T6: XMVECTOR = XMVectorSplatZ(TanCoefficients1.v);
        let T4: XMVECTOR = XMVectorSplatX(TanCoefficients1.v);
        let T3: XMVECTOR = XMVectorSplatW(TanCoefficients0.v);
        let T5: XMVECTOR = XMVectorSplatY(TanCoefficients1.v);
        let T2: XMVECTOR = XMVectorSplatZ(TanCoefficients0.v);
        let T1: XMVECTOR = XMVectorSplatY(TanCoefficients0.v);
        let T0: XMVECTOR = XMVectorSplatX(TanCoefficients0.v);

        let mut VBIsEven: XMVECTOR = XMVectorAndInt(VB, Mask.v);
        VBIsEven = XMVectorEqualInt(VBIsEven, Zero);

        let mut N: XMVECTOR = XMVectorMultiplyAdd(VC2, T7, T6);
        let mut D: XMVECTOR = XMVectorMultiplyAdd(VC2, T4, T3);
        N = XMVectorMultiplyAdd(VC2, N, T5);
        D = XMVectorMultiplyAdd(VC2, D, T2);
        N = XMVectorMultiply(VC2, N);
        D = XMVectorMultiplyAdd(VC2, D, T1);
        N = XMVectorMultiplyAdd(VC, N, VC);
        let VCNearZero: XMVECTOR = XMVectorInBounds(VC, Epsilon);
        D = XMVectorMultiplyAdd(VC2, D, T0);

        N = XMVectorSelect(N, VC, VCNearZero);
        D = XMVectorSelect(D, g_XMOne.v, VCNearZero);

        let mut R0: XMVECTOR = XMVectorNegate(N);
        let R1: XMVECTOR = XMVectorDivide(N, D);
        R0 = XMVectorDivide(D, R0);

        let VIsZero: XMVECTOR = XMVectorEqual(V, Zero);

        let mut Result: XMVECTOR = XMVectorSelect(R0, R1, VBIsEven);

        Result = XMVectorSelect(Result, Zero, VIsZero);

        return Result;
    }
}

#[test]
fn test_XMVectorTan() {
    for angle in &[-XM_PI/3.0, -XM_PI/4.0, 0.0, XM_PI/4.0, XM_PI/3.0] {
        let scalar = angle.tan();
        let vector = XMVectorReplicate(*angle);
        let vector = XMVectorTan(vector);
        assert_approx_eq!(scalar, XMVectorGetX(vector));
        assert_approx_eq!(scalar, XMVectorGetY(vector));
        assert_approx_eq!(scalar, XMVectorGetZ(vector));
        assert_approx_eq!(scalar, XMVectorGetW(vector));
    }
}

/// Computes the hyperbolic sine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSinH>
#[inline]
pub fn XMVectorSinH(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 7-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                sinh(V.vector4_f32[0]),
                sinh(V.vector4_f32[1]),
                sinh(V.vector4_f32[2]),
                sinh(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        const Scale: XMVECTORF32 = XMVECTORF32 { f :[ 1.442695040888963, 1.442695040888963, 1.442695040888963, 1.442695040888963 ] }; // 1.0f / ln(2.0f)

        let V1: XMVECTOR = XM_FMADD_PS!(V, Scale.v, g_XMNegativeOne.v);
        let V2: XMVECTOR = XM_FNMADD_PS!(V, Scale.v, g_XMNegativeOne.v);
        let E1: XMVECTOR = XMVectorExp(V1);
        let E2: XMVECTOR = XMVectorExp(V2);

        return _mm_sub_ps(E1, E2);
    }
}

/// Computes the hyperbolic cosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorCosH>
#[inline]
pub fn XMVectorCosH(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                cosh(V.vector4_f32[0]),
                cosh(V.vector4_f32[1]),
                cosh(V.vector4_f32[2]),
                cosh(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        const Scale: XMVECTORF32 = XMVECTORF32 { f :[ 1.442695040888963, 1.442695040888963, 1.442695040888963, 1.442695040888963 ] }; // 1.0f / ln(2.0f)

        let V1: XMVECTOR = XM_FMADD_PS!(V, Scale.v, g_XMNegativeOne.v);
        let V2: XMVECTOR = XM_FNMADD_PS!(V, Scale.v, g_XMNegativeOne.v);
        let E1: XMVECTOR = XMVectorExp(V1);
        let E2: XMVECTOR = XMVectorExp(V2);

        return _mm_add_ps(E1, E2);
    }
}

/// Computes the hyperbolic tangent of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorTanH>
#[inline]
pub fn XMVectorTanH(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                tanh(V.vector4_f32[0]),
                tanh(V.vector4_f32[1]),
                tanh(V.vector4_f32[2]),
                tanh(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        const Scale: XMVECTORF32 = XMVECTORF32 { f :[ 2.8853900817779268, 2.8853900817779268, 2.8853900817779268, 2.8853900817779268 ] }; // 2.0f / ln(2.0f)

        let mut E: XMVECTOR = _mm_mul_ps(V, Scale.v);
        E = XMVectorExp(E);
        E = XM_FMADD_PS!(E, g_XMOneHalf.v, g_XMOneHalf.v);
        E = _mm_div_ps(g_XMOne.v, E);
        return _mm_sub_ps(g_XMOne.v, E);
    }
}

/// Computes the arcsine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorASin>
#[inline]
pub fn XMVectorASin(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 7-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                asinf(V.vector4_f32[0]),
                asinf(V.vector4_f32[1]),
                asinf(V.vector4_f32[2]),
                asinf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let nonnegative: __m128 = _mm_cmpge_ps(V, g_XMZero.v);
        let mvalue: __m128 = _mm_sub_ps(g_XMZero.v, V);
        let x: __m128 = _mm_max_ps(V, mvalue);  // |V|

        // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
        let oneMValue: __m128 = _mm_sub_ps(g_XMOne.v, x);
        let clampOneMValue: __m128 = _mm_max_ps(g_XMZero.v, oneMValue);
        let root: __m128 = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

        // Compute polynomial approximation
        const AC1: XMVECTOR = unsafe { g_XMArcCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(2, 2, 2, 2));
        let mut t0: __m128 = XM_FMADD_PS!(vConstantsB, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(1, 1, 1, 1));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(0, 0, 0, 0));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        const AC0: XMVECTOR = unsafe { g_XMArcCoefficients0.v };
        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(3, 3, 3, 3));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(2, 2, 2, 2));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(1, 1, 1, 1));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(0, 0, 0, 0));
        t0 = XM_FMADD_PS!(t0, x, vConstants);
        t0 = _mm_mul_ps(t0, root);

        let mut t1: __m128 = _mm_sub_ps(g_XMPi.v, t0);
        t0 = _mm_and_ps(nonnegative, t0);
        t1 = _mm_andnot_ps(nonnegative, t1);
        t0 = _mm_or_ps(t0, t1);
        t0 = _mm_sub_ps(g_XMHalfPi.v, t0);
        return t0;
    }
}

/// Computes the arccosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorACos>
#[inline]
pub fn XMVectorACos(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 7-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                acosf(V.vector4_f32[0]),
                acosf(V.vector4_f32[1]),
                acosf(V.vector4_f32[2]),
                acosf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let nonnegative: __m128 = _mm_cmpge_ps(V, g_XMZero.v);
        let mvalue: __m128 = _mm_sub_ps(g_XMZero.v, V);
        let x: __m128 = _mm_max_ps(V, mvalue);  // |V|

        // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
        let oneMValue: __m128 = _mm_sub_ps(g_XMOne.v, x);
        let clampOneMValue: __m128 = _mm_max_ps(g_XMZero.v, oneMValue);
        let root: __m128 = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

        // Compute polynomial approximation
        const AC1: XMVECTOR = unsafe { g_XMArcCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(2, 2, 2, 2));
        let mut t0: __m128 = XM_FMADD_PS!(vConstantsB, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(1, 1, 1, 1));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC1, _MM_SHUFFLE(0, 0, 0, 0));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        const AC0: XMVECTOR = unsafe { g_XMArcCoefficients0.v };
        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(3, 3, 3, 3));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(2, 2, 2, 2));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(1, 1, 1, 1));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AC0, _MM_SHUFFLE(0, 0, 0, 0));
        t0 = XM_FMADD_PS!(t0, x, vConstants);
        t0 = _mm_mul_ps(t0, root);

        let mut t1: __m128 = _mm_sub_ps(g_XMPi.v, t0);
        t0 = _mm_and_ps(nonnegative, t0);
        t1 = _mm_andnot_ps(nonnegative, t1);
        t0 = _mm_or_ps(t0, t1);
        return t0;
    }
}

/// Computes the arctangent of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorATan>
#[inline]
pub fn XMVectorATan(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 7-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                atanf(V.vector4_f32[0]),
                atanf(V.vector4_f32[1]),
                atanf(V.vector4_f32[2]),
                atanf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let absV: __m128 = XMVectorAbs(V);
        let invV: __m128 = _mm_div_ps(g_XMOne.v, V);
        let mut comp: __m128 = _mm_cmpgt_ps(V, g_XMOne.v);
        let mut select0: __m128 = _mm_and_ps(comp, g_XMOne.v);
        let mut select1: __m128 = _mm_andnot_ps(comp, g_XMNegativeOne.v);
        let mut sign: __m128 = _mm_or_ps(select0, select1);
        comp = _mm_cmple_ps(absV, g_XMOne.v);
        select0 = _mm_and_ps(comp, g_XMZero.v);
        select1 = _mm_andnot_ps(comp, sign);
        sign = _mm_or_ps(select0, select1);
        select0 = _mm_and_ps(comp, V);
        select1 = _mm_andnot_ps(comp, invV);
        let x: __m128 = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        const TC1: XMVECTOR = unsafe { g_XMATanCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(TC1, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(TC1, _MM_SHUFFLE(2, 2, 2, 2));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(TC1, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(TC1, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        const TC0: XMVECTOR = unsafe { g_XMATanCoefficients0.v };
        vConstants = XM_PERMUTE_PS!(TC0, _MM_SHUFFLE(3, 3, 3, 3));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(TC0, _MM_SHUFFLE(2, 2, 2, 2));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(TC0, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(TC0, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);

        Result = _mm_mul_ps(Result, x);
        let mut result1: __m128 = _mm_mul_ps(sign, g_XMHalfPi.v);
        result1 = _mm_sub_ps(result1, Result);

        comp = _mm_cmpeq_ps(sign, g_XMZero.v);
        select0 = _mm_and_ps(comp, Result);
        select1 = _mm_andnot_ps(comp, result1);
        Result = _mm_or_ps(select0, select1);
        return Result;
    }
}


/// Computes the arctangent of Y/X.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorATan2>
#[inline]
pub fn XMVectorATan2(
    Y: FXMVECTOR,
    X: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                atan2f(Y.vector4_f32[0], X.vector4_f32[0]),
                atan2f(Y.vector4_f32[1], X.vector4_f32[1]),
                atan2f(Y.vector4_f32[2], X.vector4_f32[2]),
                atan2f(Y.vector4_f32[3], X.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(not(_XM_NO_INTRINSICS_))]
    unsafe {
        // Return the inverse tangent of Y / X in the range of -Pi to Pi with the following exceptions:

        //     Y == 0 and X is Negative         -> Pi with the sign of Y
        //     y == 0 and x is positive         -> 0 with the sign of y
        //     Y != 0 and X == 0                -> Pi / 2 with the sign of Y
        //     Y != 0 and X is Negative         -> atan(y/x) + (PI with the sign of Y)
        //     X == -Infinity and Finite Y      -> Pi with the sign of Y
        //     X == +Infinity and Finite Y      -> 0 with the sign of Y
        //     Y == Infinity and X is Finite    -> Pi / 2 with the sign of Y
        //     Y == Infinity and X == -Infinity -> 3Pi / 4 with the sign of Y
        //     Y == Infinity and X == +Infinity -> Pi / 4 with the sign of Y

        const ATan2Constants: XMVECTORF32 = XMVECTORF32 { f: [ XM_PI, XM_PIDIV2, XM_PIDIV4, XM_PI * 3.0 / 4.0 ] };

        let Zero: XMVECTOR = XMVectorZero();
        let mut ATanResultValid: XMVECTOR = XMVectorTrueInt();

        let mut Pi: XMVECTOR = XMVectorSplatX(ATan2Constants.v);
        let mut PiOverTwo: XMVECTOR = XMVectorSplatY(ATan2Constants.v);
        let mut PiOverFour: XMVECTOR = XMVectorSplatZ(ATan2Constants.v);
        let mut ThreePiOverFour: XMVECTOR = XMVectorSplatW(ATan2Constants.v);

        let YEqualsZero: XMVECTOR = XMVectorEqual(Y, Zero);
        let XEqualsZero: XMVECTOR = XMVectorEqual(X, Zero);
        let mut XIsPositive: XMVECTOR = XMVectorAndInt(X, g_XMNegativeZero.v);
        XIsPositive = XMVectorEqualInt(XIsPositive, Zero);
        let YEqualsInfinity: XMVECTOR = XMVectorIsInfinite(Y);
        let XEqualsInfinity: XMVECTOR = XMVectorIsInfinite(X);

        let YSign: XMVECTOR = XMVectorAndInt(Y, g_XMNegativeZero.v);
        Pi = XMVectorOrInt(Pi, YSign);
        PiOverTwo = XMVectorOrInt(PiOverTwo, YSign);
        PiOverFour = XMVectorOrInt(PiOverFour, YSign);
        ThreePiOverFour = XMVectorOrInt(ThreePiOverFour, YSign);

        let mut R1: XMVECTOR = XMVectorSelect(Pi, YSign, XIsPositive);
        let mut R2: XMVECTOR = XMVectorSelect(ATanResultValid, PiOverTwo, XEqualsZero);
        let R3: XMVECTOR = XMVectorSelect(R2, R1, YEqualsZero);
        let R4: XMVECTOR = XMVectorSelect(ThreePiOverFour, PiOverFour, XIsPositive);
        let R5: XMVECTOR = XMVectorSelect(PiOverTwo, R4, XEqualsInfinity);
        let Result: XMVECTOR = XMVectorSelect(R3, R5, YEqualsInfinity);
        ATanResultValid = XMVectorEqualInt(Result, ATanResultValid);

        let V: XMVECTOR = XMVectorDivide(Y, X);

        let R0: XMVECTOR = XMVectorATan(V);

        R1 = XMVectorSelect(Pi, g_XMNegativeZero.v, XIsPositive);
        R2 = XMVectorAdd(R0, R1);

        return XMVectorSelect(Result, R2, ATanResultValid);
    }
}


/// Estimates the sine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSinEst>
#[inline]
pub fn XMVectorSinEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 7-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                sinf(V.vector4_f32[0]),
                sinf(V.vector4_f32[1]),
                sinf(V.vector4_f32[2]),
                sinf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(not(_XM_NO_INTRINSICS_))]
    unsafe {
        // Force the value within the bounds of pi
        let mut x: XMVECTOR = XMVectorModAngles(V);

        // Map in [-pi/2,pi/2] with sin(y) = sin(x).
        let sign: __m128 = _mm_and_ps(x, g_XMNegativeZero.v);
        let c: __m128 = _mm_or_ps(g_XMPi.v, sign);  // pi when x >= 0, -pi when x < 0
        let absx: __m128 = _mm_andnot_ps(sign, x);  // |x|
        let rflx: __m128 = _mm_sub_ps(c, x);
        let comp: __m128 = _mm_cmple_ps(absx, g_XMHalfPi.v);
        let select0: __m128 = _mm_and_ps(comp, x);
        let select1: __m128 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        const SEC: XMVECTOR = unsafe { g_XMSinCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(SEC, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(SEC, _MM_SHUFFLE(2, 2, 2, 2));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SEC, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);
        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, x);
        return Result;
    }
}


/// Estimates the cosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorCosEst>
#[inline]
pub fn XMVectorCosEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    // 6-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                cosf(V.vector4_f32[0]),
                cosf(V.vector4_f32[1]),
                cosf(V.vector4_f32[2]),
                cosf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Map V to x in [-pi,pi].
        let mut x: XMVECTOR = XMVectorModAngles(V);

        // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
        let mut sign: XMVECTOR = _mm_and_ps(x, g_XMNegativeZero.v);
        let c: __m128 = _mm_or_ps(g_XMPi.v, sign);  // pi when x >= 0, -pi when x < 0
        let absx: __m128 = _mm_andnot_ps(sign, x);  // |x|
        let rflx: __m128 = _mm_sub_ps(c, x);
        let comp: __m128  = _mm_cmple_ps(absx, g_XMHalfPi.v);
        let mut select0: __m128 = _mm_and_ps(comp, x);
        let mut select1: __m128 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);
        select0 = _mm_and_ps(comp, g_XMOne.v);
        select1 = _mm_andnot_ps(comp, g_XMNegativeOne.v);
        sign = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        const CEC: XMVECTOR = unsafe { g_XMCosCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(CEC, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(CEC, _MM_SHUFFLE(2, 2, 2, 2));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CEC, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);
        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, sign);
        return Result;
    }
}


/// Estimates the sine and cosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorSinCosEst>
#[inline]
pub fn XMVectorSinCosEst(
    pSin: &mut XMVECTOR,
    pCos: &mut XMVECTOR,
    V: FXMVECTOR,
)
{
    // 7/6-degree minimax approximation

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Sin = XMVECTORF32 {
            f: [
                sinf(V.vector4_f32[0]),
                sinf(V.vector4_f32[1]),
                sinf(V.vector4_f32[2]),
                sinf(V.vector4_f32[3])
            ]
        };
        let Cos = XMVECTORF32 {
            f: [
                cosf(V.vector4_f32[0]),
                cosf(V.vector4_f32[1]),
                cosf(V.vector4_f32[2]),
                cosf(V.vector4_f32[3])
            ]
        };
        *pSin = Sin.v;
        *pCos = Cos.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Force the value within the bounds of pi
        let mut x: XMVECTOR = XMVectorModAngles(V);

        // Map in [-pi/2,pi/2] with sin(y) = sin(x), cos(y) = sign*cos(x).
        let mut sign: XMVECTOR = _mm_and_ps(x, g_XMNegativeZero.v);
        let c: __m128 = _mm_or_ps(g_XMPi.v, sign);  // pi when x >= 0, -pi when x < 0
        let absx: __m128 = _mm_andnot_ps(sign, x);  // |x|
        let rflx: __m128 = _mm_sub_ps(c, x);
        let comp: __m128 = _mm_cmple_ps(absx, g_XMHalfPi.v);
        let mut select0: __m128 = _mm_and_ps(comp, x);
        let mut select1: __m128 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);
        select0 = _mm_and_ps(comp, g_XMOne.v);
        select1 = _mm_andnot_ps(comp, g_XMNegativeOne.v);
        sign = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation for sine
        const SEC: XMVECTOR = unsafe { g_XMSinCoefficients1.v };
        let mut vConstantsB: __m128 = XM_PERMUTE_PS!(SEC, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(SEC, _MM_SHUFFLE(2, 2, 2, 2));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(SEC, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);
        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, x);
        *pSin = Result;

        // Compute polynomial approximation for cosine
        const CEC: XMVECTOR = unsafe { g_XMCosCoefficients1.v };
        vConstantsB = XM_PERMUTE_PS!(CEC, _MM_SHUFFLE(3, 3, 3, 3));
        vConstants = XM_PERMUTE_PS!(CEC, _MM_SHUFFLE(2, 2, 2, 2));
        Result = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(CEC, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);
        Result = XM_FMADD_PS!(Result, x2, g_XMOne.v);
        Result = _mm_mul_ps(Result, sign);
        *pCos = Result;
    }
}

/// Estimates the tangent of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorTanEst>
#[inline]
pub fn XMVectorTanEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                tanf(V.vector4_f32[0]),
                tanf(V.vector4_f32[1]),
                tanf(V.vector4_f32[2]),
                tanf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let OneOverPi: XMVECTOR = XMVectorSplatW(g_XMTanEstCoefficients.v);

        let mut V1: XMVECTOR = XMVectorMultiply(V, OneOverPi);
        V1 = XMVectorRound(V1);

        V1 = XMVectorNegativeMultiplySubtract(g_XMPi.v, V1, V);

        let T0: XMVECTOR = XMVectorSplatX(g_XMTanEstCoefficients.v);
        let T1: XMVECTOR = XMVectorSplatY(g_XMTanEstCoefficients.v);
        let T2: XMVECTOR = XMVectorSplatZ(g_XMTanEstCoefficients.v);

        let V2T2: XMVECTOR = XMVectorNegativeMultiplySubtract(V1, V1, T2);
        let V2: XMVECTOR = XMVectorMultiply(V1, V1);
        let V1T0: XMVECTOR = XMVectorMultiply(V1, T0);
        let V1T1: XMVECTOR = XMVectorMultiply(V1, T1);

        let D: XMVECTOR = XMVectorReciprocalEst(V2T2);
        let N: XMVECTOR = XMVectorMultiplyAdd(V2, V1T1, V1T0);

        return XMVectorMultiply(N, D);
    }
}

/// Estimates the arcsine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorASinEst>
#[inline]
pub fn XMVectorASinEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                asinf(V.vector4_f32[0]),
                asinf(V.vector4_f32[1]),
                asinf(V.vector4_f32[2]),
                asinf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let nonnegative: __m128 = _mm_cmpge_ps(V, g_XMZero.v);
        let mvalue: __m128 = _mm_sub_ps(g_XMZero.v, V);
        let x: __m128 = _mm_max_ps(V, mvalue);  // |V|

        // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
        let oneMValue: __m128 = _mm_sub_ps(g_XMOne.v, x);
        let clampOneMValue: __m128 = _mm_max_ps(g_XMZero.v, oneMValue);
        let root: __m128 = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

        // Compute polynomial approximation
        const AEC: XMVECTOR = unsafe { g_XMArcEstCoefficients.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(2, 2, 2, 2));
        let mut t0: __m128 = XM_FMADD_PS!(vConstantsB, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(1, 1, 1, 1));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(0, 0, 0, 0));
        t0 = XM_FMADD_PS!(t0, x, vConstants);
        t0 = _mm_mul_ps(t0, root);

        let mut t1: __m128 = _mm_sub_ps(g_XMPi.v, t0);
        t0 = _mm_and_ps(nonnegative, t0);
        t1 = _mm_andnot_ps(nonnegative, t1);
        t0 = _mm_or_ps(t0, t1);
        t0 = _mm_sub_ps(g_XMHalfPi.v, t0);
        return t0;
    }
}

/// Estimates the arccosine of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorACosEst>
#[inline]
pub fn XMVectorACosEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                acosf(V.vector4_f32[0]),
                acosf(V.vector4_f32[1]),
                acosf(V.vector4_f32[2]),
                acosf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let nonnegative: __m128 = _mm_cmpge_ps(V, g_XMZero.v);
        let mvalue: __m128 = _mm_sub_ps(g_XMZero.v, V);
        let x: __m128 = _mm_max_ps(V, mvalue);  // |V|

        // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
        let oneMValue: __m128 = _mm_sub_ps(g_XMOne.v, x);
        let clampOneMValue: __m128 = _mm_max_ps(g_XMZero.v, oneMValue);
        let root: __m128 = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

        // Compute polynomial approximation
        const AEC: XMVECTOR = unsafe { g_XMArcEstCoefficients.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(2, 2, 2, 2));
        let mut t0: __m128 = XM_FMADD_PS!(vConstantsB, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(1, 1, 1, 1));
        t0 = XM_FMADD_PS!(t0, x, vConstants);

        vConstants = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(0, 0, 0, 0));
        t0 = XM_FMADD_PS!(t0, x, vConstants);
        t0 = _mm_mul_ps(t0, root);

        let mut t1: __m128 = _mm_sub_ps(g_XMPi.v, t0);
        t0 = _mm_and_ps(nonnegative, t0);
        t1 = _mm_andnot_ps(nonnegative, t1);
        t0 = _mm_or_ps(t0, t1);
        return t0;
    }
}

/// Estimates the arctangent of each component of an XMVECTOR.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorATanEst>
#[inline]
pub fn XMVectorATanEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                atanf(V.vector4_f32[0]),
                atanf(V.vector4_f32[1]),
                atanf(V.vector4_f32[2]),
                atanf(V.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let absV: __m128 = XMVectorAbs(V);
        let invV: __m128 = _mm_div_ps(g_XMOne.v, V);
        let mut comp: __m128 = _mm_cmpgt_ps(V, g_XMOne.v);
        let mut select0: __m128 = _mm_and_ps(comp, g_XMOne.v);
        let mut select1: __m128 = _mm_andnot_ps(comp, g_XMNegativeOne.v);
        let mut sign: __m128 = _mm_or_ps(select0, select1);
        comp = _mm_cmple_ps(absV, g_XMOne.v);
        select0 = _mm_and_ps(comp, g_XMZero.v);
        select1 = _mm_andnot_ps(comp, sign);
        sign = _mm_or_ps(select0, select1);
        select0 = _mm_and_ps(comp, V);
        select1 = _mm_andnot_ps(comp, invV);
        let x: __m128 = _mm_or_ps(select0, select1);

        let x2: __m128 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        const AEC: XMVECTOR = unsafe { g_XMATanEstCoefficients1.v };
        let vConstantsB: __m128 = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(3, 3, 3, 3));
        let mut vConstants: __m128 = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(2, 2, 2, 2));
        let mut Result: __m128 = XM_FMADD_PS!(vConstantsB, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(1, 1, 1, 1));
        Result = XM_FMADD_PS!(Result, x2, vConstants);

        vConstants = XM_PERMUTE_PS!(AEC, _MM_SHUFFLE(0, 0, 0, 0));
        Result = XM_FMADD_PS!(Result, x2, vConstants);
        // ATanEstCoefficients0 is already splatted
        Result = XM_FMADD_PS!(Result, x2, g_XMATanEstCoefficients0.v);
        Result = _mm_mul_ps(Result, x);
        let mut result1: __m128 = _mm_mul_ps(sign, g_XMHalfPi.v);
        result1 = _mm_sub_ps(result1, Result);

        comp = _mm_cmpeq_ps(sign, g_XMZero.v);
        select0 = _mm_and_ps(comp, Result);
        select1 = _mm_andnot_ps(comp, result1);
        Result = _mm_or_ps(select0, select1);
        return Result;
    }
}


/// Estimates the arctangent of Y/X.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorATan2Est>
#[inline]
pub fn XMVectorATan2Est(
    Y: FXMVECTOR,
    X: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 {
            f: [
                atan2f(Y.vector4_f32[0], X.vector4_f32[0]),
                atan2f(Y.vector4_f32[1], X.vector4_f32[1]),
                atan2f(Y.vector4_f32[2], X.vector4_f32[2]),
                atan2f(Y.vector4_f32[3], X.vector4_f32[3])
            ]
        };
        return Result.v;
    }

    #[cfg(not(_XM_NO_INTRINSICS_))]
    unsafe {
        const ATan2Constants: XMVECTORF32 = XMVECTORF32 { f: [ XM_PI, XM_PIDIV2, XM_PIDIV4, XM_PI * 3.0 / 4.0 ] };

        let Zero: XMVECTOR = XMVectorZero();
        let mut ATanResultValid: XMVECTOR = XMVectorTrueInt();

        let mut Pi: XMVECTOR = XMVectorSplatX(ATan2Constants.v);
        let mut PiOverTwo: XMVECTOR = XMVectorSplatY(ATan2Constants.v);
        let mut PiOverFour: XMVECTOR = XMVectorSplatZ(ATan2Constants.v);
        let mut ThreePiOverFour: XMVECTOR = XMVectorSplatW(ATan2Constants.v);

        let YEqualsZero: XMVECTOR = XMVectorEqual(Y, Zero);
        let XEqualsZero: XMVECTOR = XMVectorEqual(X, Zero);
        let mut XIsPositive: XMVECTOR = XMVectorAndInt(X, g_XMNegativeZero.v);
        XIsPositive = XMVectorEqualInt(XIsPositive, Zero);
        let YEqualsInfinity: XMVECTOR = XMVectorIsInfinite(Y);
        let XEqualsInfinity: XMVECTOR = XMVectorIsInfinite(X);

        let YSign: XMVECTOR = XMVectorAndInt(Y, g_XMNegativeZero.v);
        Pi = XMVectorOrInt(Pi, YSign);
        PiOverTwo = XMVectorOrInt(PiOverTwo, YSign);
        PiOverFour = XMVectorOrInt(PiOverFour, YSign);
        ThreePiOverFour = XMVectorOrInt(ThreePiOverFour, YSign);

        let mut R1: XMVECTOR = XMVectorSelect(Pi, YSign, XIsPositive);
        let mut R2: XMVECTOR = XMVectorSelect(ATanResultValid, PiOverTwo, XEqualsZero);
        let R3: XMVECTOR = XMVectorSelect(R2, R1, YEqualsZero);
        let R4: XMVECTOR = XMVectorSelect(ThreePiOverFour, PiOverFour, XIsPositive);
        let R5: XMVECTOR = XMVectorSelect(PiOverTwo, R4, XEqualsInfinity);
        let Result: XMVECTOR = XMVectorSelect(R3, R5, YEqualsInfinity);
        ATanResultValid = XMVectorEqualInt(Result, ATanResultValid);

        let Reciprocal: XMVECTOR = XMVectorReciprocalEst(X);
        let V: XMVECTOR = XMVectorMultiply(Y, Reciprocal);
        let R0: XMVECTOR = XMVectorATanEst(V);

        R1 = XMVectorSelect(Pi, g_XMNegativeZero.v, XIsPositive);
        R2 = XMVectorAdd(R0, R1);

        return XMVectorSelect(Result, R2, ATanResultValid);
    }
}


/// Performs a linear interpolation between two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorLerp>
#[inline]
pub fn XMVectorLerp(
    V0: FXMVECTOR,
    V1: FXMVECTOR,
    t: f32,
) -> XMVECTOR
{
    // V0 + t * (V1 - V0)

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let Scale: XMVECTOR = XMVectorReplicate(t);
        let Length: XMVECTOR = XMVectorSubtract(V1, V0);
        return XMVectorMultiplyAdd(Length, Scale, V0);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let L: XMVECTOR = _mm_sub_ps(V1, V0);
        let S: XMVECTOR = _mm_set_ps1(t);
        return XM_FMADD_PS!(L, S, V0);
    }
}

/// Performs a linear interpolation between two vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorLerpV>
#[inline]
pub fn XMVectorLerpV(
    V0: FXMVECTOR,
    V1: FXMVECTOR,
    T: FXMVECTOR,
) -> XMVECTOR
{
    // V0 + T * (V1 - V0)

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let Length: XMVECTOR = XMVectorSubtract(V1, V0);
        return XMVectorMultiplyAdd(Length, T, V0);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let Length: XMVECTOR = _mm_sub_ps(V1, V0);
        return XM_FMADD_PS!(Length, T, V0);
    }
}

/// Performs a Hermite spline interpolation, using the specified vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorHermite>
#[inline]
pub fn XMVectorHermite(
    Position0: FXMVECTOR,
    Tangent0: FXMVECTOR,
    Position1: FXMVECTOR,
    Tangent1: GXMVECTOR,
    t: f32,
) -> XMVECTOR
{
    // Result = (2 * t^3 - 3 * t^2 + 1) * Position0 +
    //          (t^3 - 2 * t^2 + t) * Tangent0 +
    //          (-2 * t^3 + 3 * t^2) * Position1 +
    //          (t^3 - t^2) * Tangent1

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let t2: f32 = t * t;
        let t3: f32 = t * t2;

        let P0: XMVECTOR = XMVectorReplicate(2.0 * t3 - 3.0 * t2 + 1.0);
        let T0: XMVECTOR = XMVectorReplicate(t3 - 2.0 * t2 + t);
        let P1: XMVECTOR = XMVectorReplicate(-2.0 * t3 + 3.0 * t2);
        let T1: XMVECTOR = XMVectorReplicate(t3 - t2);

        let mut Result: XMVECTOR = XMVectorMultiply(P0, Position0);
        Result = XMVectorMultiplyAdd(T0, Tangent0, Result);
        Result = XMVectorMultiplyAdd(P1, Position1, Result);
        Result = XMVectorMultiplyAdd(T1, Tangent1, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let t2: f32 = t * t;
        let t3: f32 = t * t2;

        let P0: XMVECTOR = _mm_set_ps1(2.0 * t3 - 3.0 * t2 + 1.0);
        let T0: XMVECTOR = _mm_set_ps1(t3 - 2.0 * t2 + t);
        let P1: XMVECTOR = _mm_set_ps1(-2.0 * t3 + 3.0 * t2);
        let T1: XMVECTOR = _mm_set_ps1(t3 - t2);

        let mut vResult: XMVECTOR = _mm_mul_ps(P0, Position0);
        vResult = XM_FMADD_PS!(Tangent0, T0, vResult);
        vResult = XM_FMADD_PS!(Position1, P1, vResult);
        vResult = XM_FMADD_PS!(Tangent1, T1, vResult);
        return vResult;
    }
}

/// Performs a Hermite spline interpolation, using the specified vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorHermiteV>
#[inline]
pub fn XMVectorHermiteV(
    Position0: FXMVECTOR,
    Tangent0: FXMVECTOR,
    Position1: FXMVECTOR,
    Tangent1: GXMVECTOR,
    T: HXMVECTOR,
) -> XMVECTOR
{
    // Result = (2 * t^3 - 3 * t^2 + 1) * Position0 +
    //          (t^3 - 2 * t^2 + t) * Tangent0 +
    //          (-2 * t^3 + 3 * t^2) * Position1 +
    //          (t^3 - t^2) * Tangent1

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let T2: XMVECTOR = XMVectorMultiply(T, T);
        let T3: XMVECTOR = XMVectorMultiply(T, T2);

        let P0: XMVECTOR = XMVectorReplicate(2.0 * T3.vector4_f32[0] - 3.0 * T2.vector4_f32[0] + 1.0);
        let T0: XMVECTOR = XMVectorReplicate(T3.vector4_f32[1] - 2.0 * T2.vector4_f32[1] + T.vector4_f32[1]);
        let P1: XMVECTOR = XMVectorReplicate(-2.0 * T3.vector4_f32[2] + 3.0 * T2.vector4_f32[2]);
        let T1: XMVECTOR = XMVectorReplicate(T3.vector4_f32[3] - T2.vector4_f32[3]);

        let mut Result: XMVECTOR = XMVectorMultiply(P0, Position0);
        Result = XMVectorMultiplyAdd(T0, Tangent0, Result);
        Result = XMVectorMultiplyAdd(P1, Position1, Result);
        Result = XMVectorMultiplyAdd(T1, Tangent1, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        const CatMulT2: XMVECTORF32 = XMVECTORF32 { f: [ -3.0, -2.0, 3.0, -1.0 ] };
        const CatMulT3: XMVECTORF32 = XMVECTORF32 { f: [ 2.0, 1.0, -2.0, 1.0 ] };

        let mut T2: XMVECTOR = _mm_mul_ps(T, T);
        let mut T3: XMVECTOR = _mm_mul_ps(T, T2);
        // Mul by the constants against t^2
        T2 = _mm_mul_ps(T2, CatMulT2.v);
        // Mul by the constants against t^3
        T3 = XM_FMADD_PS!(T3, CatMulT3.v, T2);
        // T3 now has the pre-result.
        // I need to add t.y only
        T2 = _mm_and_ps(T, g_XMMaskY.v);
        T3 = _mm_add_ps(T3, T2);
        // Add 1.0f to x
        T3 = _mm_add_ps(T3, g_XMIdentityR0.v);
        // Now, I have the constants created
        // Mul the x constant to Position0
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(T3, _MM_SHUFFLE(0, 0, 0, 0));
        vResult = _mm_mul_ps(vResult, Position0);
        // Mul the y constant to Tangent0
        T2 = XM_PERMUTE_PS!(T3, _MM_SHUFFLE(1, 1, 1, 1));
        vResult = XM_FMADD_PS!(T2, Tangent0, vResult);
        // Mul the z constant to Position1
        T2 = XM_PERMUTE_PS!(T3, _MM_SHUFFLE(2, 2, 2, 2));
        vResult = XM_FMADD_PS!(T2, Position1, vResult);
        // Mul the w constant to Tangent1
        T3 = XM_PERMUTE_PS!(T3, _MM_SHUFFLE(3, 3, 3, 3));
        vResult = XM_FMADD_PS!(T3, Tangent1, vResult);
        return vResult;
    }
}

/// Performs a Catmull-Rom interpolation, using the specified position vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorCatmullRom>
#[inline]
pub fn XMVectorCatmullRom(
    Position0: FXMVECTOR,
    Position1: FXMVECTOR,
    Position2: FXMVECTOR,
    Position3: GXMVECTOR,
    t: f32,
) -> XMVECTOR
{
    // Result = ((-t^3 + 2 * t^2 - t) * Position0 +
    //           (3 * t^3 - 5 * t^2 + 2) * Position1 +
    //           (-3 * t^3 + 4 * t^2 + t) * Position2 +
    //           (t^3 - t^2) * Position3) * 0.5

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let t2: f32 = t * t;
        let t3: f32 = t * t2;

        let P0: XMVECTOR = XMVectorReplicate((-t3 + 2.0 * t2 - t) * 0.5);
        let P1: XMVECTOR = XMVectorReplicate((3.0 * t3 - 5.0 * t2 + 2.0) * 0.5);
        let P2: XMVECTOR = XMVectorReplicate((-3.0 * t3 + 4.0 * t2 + t) * 0.5);
        let P3: XMVECTOR = XMVectorReplicate((t3 - t2) * 0.5);

        let mut Result: XMVECTOR = XMVectorMultiply(P0, Position0);
        Result = XMVectorMultiplyAdd(P1, Position1, Result);
        Result = XMVectorMultiplyAdd(P2, Position2, Result);
        Result = XMVectorMultiplyAdd(P3, Position3, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let t2: f32 = t * t;
        let t3: f32 = t * t2;

        let mut P0: XMVECTOR = _mm_set_ps1((-t3 + 2.0 * t2 - t) * 0.5);
        let mut P1: XMVECTOR = _mm_set_ps1((3.0 * t3 - 5.0 * t2 + 2.0) * 0.5);
        let mut P2: XMVECTOR = _mm_set_ps1((-3.0 * t3 + 4.0 * t2 + t) * 0.5);
        let mut P3: XMVECTOR = _mm_set_ps1((t3 - t2) * 0.5);

        P1 = _mm_mul_ps(Position1, P1);
        P0 = XM_FMADD_PS!(Position0, P0, P1);
        P3 = _mm_mul_ps(Position3, P3);
        P2 = XM_FMADD_PS!(Position2, P2, P3);
        P0 = _mm_add_ps(P0, P2);
        return P0;
    }
}

/// Performs a Catmull-Rom interpolation, using the specified position vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorCatmullRomV>
#[inline]
pub fn XMVectorCatmullRomV(
    Position0: FXMVECTOR,
    Position1: FXMVECTOR,
    Position2: FXMVECTOR,
    Position3: GXMVECTOR,
    T: HXMVECTOR,
) -> XMVECTOR
{
    // Result = ((-t^3 + 2 * t^2 - t) * Position0 +
    //           (3 * t^3 - 5 * t^2 + 2) * Position1 +
    //           (-3 * t^3 + 4 * t^2 + t) * Position2 +
    //           (t^3 - t^2) * Position3) * 0.5

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let fx: f32 = T.vector4_f32[0];
        let fy: f32 = T.vector4_f32[1];
        let fz: f32 = T.vector4_f32[2];
        let fw: f32 = T.vector4_f32[3];
        let vResult = XMVECTORF32 { f: [
            0.5 * ((-fx * fx * fx + 2.0 * fx * fx - fx) * Position0.vector4_f32[0]
            + (3.0 * fx * fx * fx - 5.0 * fx * fx + 2.0) * Position1.vector4_f32[0]
            + (-3.0 * fx * fx * fx + 4.0 * fx * fx + fx) * Position2.vector4_f32[0]
            + (fx * fx * fx - fx * fx) * Position3.vector4_f32[0]),

            0.5 * ((-fy * fy * fy + 2.0 * fy * fy - fy) * Position0.vector4_f32[1]
            + (3.0 * fy * fy * fy - 5.0 * fy * fy + 2.0) * Position1.vector4_f32[1]
            + (-3.0 * fy * fy * fy + 4.0 * fy * fy + fy) * Position2.vector4_f32[1]
            + (fy * fy * fy - fy * fy) * Position3.vector4_f32[1]),

            0.5 * ((-fz * fz * fz + 2.0 * fz * fz - fz) * Position0.vector4_f32[2]
            + (3.0 * fz * fz * fz - 5.0 * fz * fz + 2.0) * Position1.vector4_f32[2]
            + (-3.0 * fz * fz * fz + 4.0 * fz * fz + fz) * Position2.vector4_f32[2]
            + (fz * fz * fz - fz * fz) * Position3.vector4_f32[2]),

            0.5 * ((-fw * fw * fw + 2.0 * fw * fw - fw) * Position0.vector4_f32[3]
            + (3.0 * fw * fw * fw - 5.0 * fw * fw + 2.0) * Position1.vector4_f32[3]
            + (-3.0 * fw * fw * fw + 4.0 * fw * fw + fw) * Position2.vector4_f32[3]
            + (fw * fw * fw - fw * fw) * Position3.vector4_f32[3])
         ] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        const Catmul2: XMVECTORF32 = XMVECTORF32 { f: [ 2.0, 2.0, 2.0, 2.0 ] };
        const Catmul3: XMVECTORF32 = XMVECTORF32 { f: [ 3.0, 3.0, 3.0, 3.0 ] };
        const Catmul4: XMVECTORF32 = XMVECTORF32 { f :[ 4.0, 4.0, 4.0, 4.0 ] };
        const Catmul5: XMVECTORF32 = XMVECTORF32 { f: [5.0, 5.0, 5.0, 5.0 ] };
        // Cache T^2 and T^3
        let T2: XMVECTOR = _mm_mul_ps(T, T);
        let mut T3: XMVECTOR = _mm_mul_ps(T, T2);
        // Perform the Position0 term
        let mut vResult: XMVECTOR = _mm_add_ps(T2, T2);
        vResult = _mm_sub_ps(vResult, T);
        vResult = _mm_sub_ps(vResult, T3);
        vResult = _mm_mul_ps(vResult, Position0);
        // Perform the Position1 term and add
        let mut vTemp: XMVECTOR = _mm_mul_ps(T3, Catmul3.v);
        vTemp = XM_FNMADD_PS!(T2, Catmul5.v, vTemp);
        vTemp = _mm_add_ps(vTemp, Catmul2.v);
        vResult = XM_FMADD_PS!(vTemp, Position1, vResult);
        // Perform the Position2 term and add
        vTemp = _mm_mul_ps(T2, Catmul4.v);
        vTemp = XM_FNMADD_PS!(T3, Catmul3.v, vTemp);
        vTemp = _mm_add_ps(vTemp, T);
        vResult = XM_FMADD_PS!(vTemp, Position2, vResult);
        // Position3 is the last term
        T3 = _mm_sub_ps(T3, T2);
        vResult = XM_FMADD_PS!(T3, Position3, vResult);
        // Multiply by 0.5f and exit
        vResult = _mm_mul_ps(vResult, g_XMOneHalf.v);
        return vResult;
    }
}

/// Returns a point in Barycentric coordinates, using the specified position vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorBaryCentric>
#[inline]
pub fn XMVectorBaryCentric(
    Position0: FXMVECTOR,
    Position1: FXMVECTOR,
    Position2: FXMVECTOR,
    f: f32,
    g: f32,
) -> XMVECTOR
{
    // Result = Position0 + f * (Position1 - Position0) + g * (Position2 - Position0)

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let P10: XMVECTOR = XMVectorSubtract(Position1, Position0);
        let ScaleF: XMVECTOR = XMVectorReplicate(f);

        let P20: XMVECTOR = XMVectorSubtract(Position2, Position0);
        let ScaleG: XMVECTOR = XMVectorReplicate(g);

        let mut Result: XMVECTOR = XMVectorMultiplyAdd(P10, ScaleF, Position0);
        Result = XMVectorMultiplyAdd(P20, ScaleG, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut R1: XMVECTOR = _mm_sub_ps(Position1, Position0);
        let R2: XMVECTOR = _mm_sub_ps(Position2, Position0);
        let SF: XMVECTOR = _mm_set_ps1(f);
        R1 = XM_FMADD_PS!(R1, SF, Position0);
        let SG: XMVECTOR = _mm_set_ps1(g);
        return XM_FMADD_PS!(R2, SG, R1);
    }
}

/// Returns a point in Barycentric coordinates, using the specified position vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVectorBaryCentricV>
#[inline]
pub fn XMVectorBaryCentricV(
    Position0: FXMVECTOR,
    Position1: FXMVECTOR,
    Position2: FXMVECTOR,
    F: GXMVECTOR,
    G: HXMVECTOR,
) -> XMVECTOR
{
    // Result = Position0 + f * (Position1 - Position0) + g * (Position2 - Position0)

    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let P10: XMVECTOR = XMVectorSubtract(Position1, Position0);
        let P20: XMVECTOR = XMVectorSubtract(Position2, Position0);

        let mut Result: XMVECTOR = XMVectorMultiplyAdd(P10, F, Position0);
        Result = XMVectorMultiplyAdd(P20, G, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut R1: XMVECTOR = _mm_sub_ps(Position1, Position0);
        let R2: XMVECTOR = _mm_sub_ps(Position2, Position0);
        R1 = XM_FMADD_PS!(R1, F, Position0);
        return XM_FMADD_PS!(R2, G, R1);
    }
}

// 2D Vector

/// Tests whether two 2D vectors are equal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Equal>
#[inline]
pub fn XMVector2Equal(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] == V2.vector4_f32[0]) && (V1.vector4_f32[1] == V2.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        // z and w are don't care
        return (((_mm_movemask_ps(vTemp) & 3) == 3) != false);
    }
}

/// Tests whether two 2D vectors are equal. In addition, this function returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2EqualR>
#[inline]
pub fn XMVector2EqualR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_f32[0] == V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] == V2.vector4_f32[1]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] != V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] != V2.vector4_f32[1]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        // z and w are don't care
        let iTest: i32 = _mm_movemask_ps(vTemp) & 3;
        let mut CR = 0;
        if (iTest == 3)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (!ibool(iTest))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether two 2D vectors are equal, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2EqualInt>
#[inline]
pub fn XMVector2EqualInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_u32[0] == V2.vector4_u32[0]) && (V1.vector4_u32[1] == V2.vector4_u32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 3) == 3) != false);
    }
}

/// Tests whether two 2D vectors are equal, treating each component as an unsigned integer. In addition, this function returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2EqualIntR>
#[inline]
pub fn XMVector2EqualIntR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_u32[0] == V2.vector4_u32[0]) &&
            (V1.vector4_u32[1] == V2.vector4_u32[1]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_u32[0] != V2.vector4_u32[0]) &&
            (V1.vector4_u32[1] != V2.vector4_u32[1]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        let iTest: i32 = _mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 3;
        let mut CR: u32 = 0;
        if (iTest == 3)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (!ibool(iTest))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 2D vector is near another 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2NearEqual>
#[inline]
pub fn XMVector2NearEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    Epsilon: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let dx: f32 = fabsf(V1.vector4_f32[0] - V2.vector4_f32[0]);
        let dy: f32 = fabsf(V1.vector4_f32[1] - V2.vector4_f32[1]);
        return ((dx <= Epsilon.vector4_f32[0]) &&
            (dy <= Epsilon.vector4_f32[1]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        // Get the difference
        let vDelta: XMVECTOR = _mm_sub_ps(V1, V2);
        // Get the absolute value of the difference
        let mut vTemp: XMVECTOR = _mm_setzero_ps();
        vTemp = _mm_sub_ps(vTemp, vDelta);
        vTemp = _mm_max_ps(vTemp, vDelta);
        vTemp = _mm_cmple_ps(vTemp, Epsilon);
        // z and w are don't care
        return (((_mm_movemask_ps(vTemp) & 3) == 0x3) != false);
    }
}


/// Tests whether two 2D vectors are not equal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2NotEqual>
#[inline]
pub fn XMVector2NotEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] != V2.vector4_f32[0]) || (V1.vector4_f32[1] != V2.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        // z and w are don't care
        return (((_mm_movemask_ps(vTemp) & 3) != 3) != false);
    }
}

/// Test whether two vectors are not equal, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2NotEqualInt>
#[inline]
pub fn XMVector2NotEqualInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_u32[0] != V2.vector4_u32[0]) || (V1.vector4_u32[1] != V2.vector4_u32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 3) != 3) != false);
    }
}

/// Tests whether one 2D vector is greater than another 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Greater>
#[inline]
pub fn XMVector2Greater(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] > V2.vector4_f32[0]) && (V1.vector4_f32[1] > V2.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        // z and w are don't care
        return (((_mm_movemask_ps(vTemp) & 3) == 3) != false);
    }
}

/// Tests whether one 2D vector is greater than another 2D vector and returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2GreaterR>
#[inline]
pub fn XMVector2GreaterR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR = 0;
        if ((V1.vector4_f32[0] > V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] > V2.vector4_f32[1]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] <= V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] <= V2.vector4_f32[1]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        let iTest: i32 = _mm_movemask_ps(vTemp) & 3;
        let mut CR = 0;
        if (iTest == 3)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (!ibool(iTest))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 2D vector is greater-than-or-equal-to another 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2GreaterOrEqual>
#[inline]
pub fn XMVector2GreaterOrEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] >= V2.vector4_f32[0]) && (V1.vector4_f32[1] >= V2.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 3) == 3) != false);
    }
}

/// Tests whether one 2D vector is greater-than-or-equal-to another 2D vector and returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2GreaterOrEqualR>
#[inline]
pub fn XMVector2GreaterOrEqualR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR = 0;
        if ((V1.vector4_f32[0] >= V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] >= V2.vector4_f32[1]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] < V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] < V2.vector4_f32[1]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        let iTest: i32 = _mm_movemask_ps(vTemp) & 3;
        let mut CR: u32 = 0;
        if (iTest == 3)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (!ibool(iTest))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 2D vector is less than another 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Less>
#[inline]
pub fn XMVector2Less(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] < V2.vector4_f32[0]) && (V1.vector4_f32[1] < V2.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmplt_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 3) == 3) != false);
    }
}

/// Tests whether one 2D vector is less-than-or-equal-to another 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2LessOrEqual>
#[inline]
pub fn XMVector2LessOrEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] <= V2.vector4_f32[0]) && (V1.vector4_f32[1] <= V2.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmple_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 3) == 3) != false);
    }
}

/// Tests whether the components of a 2D vector are within set bounds.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2InBounds>
#[inline]
pub fn XMVector2InBounds(
    V: FXMVECTOR,
    Bounds: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) &&
            (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        // Test if less than or equal
        let mut vTemp1: XMVECTOR = _mm_cmple_ps(V, Bounds);
        // Negate the bounds
        let mut vTemp2: XMVECTOR = _mm_mul_ps(Bounds, g_XMNegativeOne.v);
        // Test if greater or equal (Reversed)
        vTemp2 = _mm_cmple_ps(vTemp2, V);
        // Blend answers
        vTemp1 = _mm_and_ps(vTemp1, vTemp2);
        // x and y in bounds? (z and w are don't care)
        return (((_mm_movemask_ps(vTemp1) & 0x3) == 0x3) != false);
    }
}

/// Tests whether any component of a 2D vector is a NaN.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2IsNaN>
#[inline]
pub fn XMVector2IsNaN(
    V: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (XMISNAN!(V.vector4_f32[0]) ||
            XMISNAN!(V.vector4_f32[1]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        // Test against itself. NaN is always not equal
        let vTempNan: XMVECTOR = _mm_cmpneq_ps(V, V);
        // If x or y are NaN, the mask is non-zero
        return ((_mm_movemask_ps(vTempNan) & 3) != 0);
    }
}

/// Tests whether any component of a 2D vector is positive or negative infinity.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2IsInfinite>
#[inline]
pub fn XMVector2IsInfinite(
    V: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (XMISINF!(V.vector4_f32[0]) ||
            XMISINF!(V.vector4_f32[1]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        // Mask off the sign bit
        let mut vTemp: __m128 = _mm_and_ps(V, g_XMAbsMask.v);
        // Compare to infinity
        vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity.v);
        // If x or z are infinity, the signs are true.
        return ((_mm_movemask_ps(vTemp) & 3) != 0);
    }
}

/// Computes the dot product between 2D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Dot>
#[inline]
pub fn XMVector2Dot(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let fDot: f32 = V1.vector4_f32[0] * V2.vector4_f32[0] + V1.vector4_f32[1] * V2.vector4_f32[1];
        let mut Result: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        Result.f[0] = fDot;
        Result.f[1] = fDot;
        Result.f[2] = fDot;
        Result.f[3] = fDot;
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        // Perform the dot product on x and y
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V1, V2);
        // vTemp has y splatted
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        // x+y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }
}

/// Computes the 2D cross product.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Cross>
#[inline]
pub fn XMVector2Cross(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let fCross: f32 = (V1.vector4_f32[0] * V2.vector4_f32[1]) - (V1.vector4_f32[1] * V2.vector4_f32[0]);
        let mut Result: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        Result.f[0] = fCross;
        Result.f[1] = fCross;
        Result.f[2] = fCross;
        Result.f[3] = fCross;
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        // Swap x and y
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V2, _MM_SHUFFLE(0, 1, 0, 1));
        // Perform the muls
        vResult = _mm_mul_ps(vResult, V1);
        // Splat y
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(1, 1, 1, 1));
        // Sub the values
        vResult = _mm_sub_ss(vResult, vTemp);
        // Splat the cross product
        vResult = XM_PERMUTE_PS!(vResult, _MM_SHUFFLE(0, 0, 0, 0));
        return vResult;
    }
}

/// Computes the square of the length of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2LengthSq>
#[inline]
pub fn XMVector2LengthSq(
    V: FXMVECTOR,
) -> XMVECTOR
{
    return XMVector2Dot(V, V);
}

/// Estimates the reciprocal of the length of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2ReciprocalLengthEst>
#[inline]
pub fn XMVector2ReciprocalLengthEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;
        Result = XMVector2LengthSq(V);
        Result = XMVectorReciprocalSqrtEst(Result);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE4_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x3f);
        return _mm_rsqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let vTemp: XMVECTOR = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_rsqrt_ss(vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has y splatted
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        // x+y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = _mm_rsqrt_ss(vLengthSq);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }
}

/// Computes the reciprocal of the length of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2ReciprocalLength>
#[inline]
pub fn XMVector2ReciprocalLength(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;
        Result = XMVector2LengthSq(V);
        Result = XMVectorReciprocalSqrt(Result);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE4_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x3f);
        let vLengthSq: XMVECTOR = _mm_sqrt_ps(vTemp);
        return _mm_div_ps(g_XMOne.v, vLengthSq);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let vTemp: XMVECTOR = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ss(vTemp);
        vLengthSq = _mm_div_ss(g_XMOne.v, vLengthSq);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has y splatted
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        // x+y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = _mm_sqrt_ss(vLengthSq);
        vLengthSq = _mm_div_ss(g_XMOne.v, vLengthSq);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }
}

/// Estimates the length of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2LengthEst>
#[inline]
pub fn XMVector2LengthEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;
        Result = XMVector2LengthSq(V);
        Result = XMVectorSqrtEst(Result);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE4_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x3f);
        return _mm_sqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let vTemp: XMVECTOR = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ss(vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has y splatted
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        // x+y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = _mm_sqrt_ss(vLengthSq);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }
}

/// Computes the length of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Length>
#[inline]
pub fn XMVector2Length(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;
        Result = XMVector2LengthSq(V);
        Result = XMVectorSqrt(Result);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE4_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x3f);
        return _mm_sqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let vTemp: XMVECTOR = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ss(vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has y splatted
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        // x+y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }
}

/// Estimates the normalized version of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2NormalizeEst>
#[inline]
pub fn XMVector2NormalizeEst(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;
        Result = XMVector2ReciprocalLength(V);
        Result = XMVectorMultiply(V, Result);
        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE4_INTRINSICS_))]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x3f);
        let vResult: XMVECTOR = _mm_rsqrt_ps(vTemp);
        return _mm_mul_ps(vResult, V);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has y splatted
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        // x+y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = _mm_rsqrt_ss(vLengthSq);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        vLengthSq = _mm_mul_ps(vLengthSq, V);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y only
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Reciprocal mul to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }
}

/// Returns the normalized version of a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Normalize>
#[inline]
pub fn XMVector2Normalize(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = XMVector2Length(V);
        let mut fLength: f32 = vResult.vector4_f32[0];

        // Prevent divide by zero
        if (fLength > 0.0)
        {
            fLength = 1.0 / fLength;
        }

        vResult.vector4_f32[0] = V.vector4_f32[0] * fLength;
        vResult.vector4_f32[1] = V.vector4_f32[1] * fLength;
        vResult.vector4_f32[2] = V.vector4_f32[2] * fLength;
        vResult.vector4_f32[3] = V.vector4_f32[3] * fLength;
        return vResult;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE4_INTRINSICS_))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_dp_ps(V, V, 0x3f);
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Reciprocal mul to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y only
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_moveldup_ps(vLengthSq);
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Reciprocal mul to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x and y only
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Reciprocal mul to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }
}

/// Clamps the length of a 2D vector to a given range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2ClampLength>
#[inline]
pub fn XMVector2ClampLength(
    V: FXMVECTOR,
    LengthMin: f32,
    LengthMax: f32,
) -> XMVECTOR
{
    let ClampMax: XMVECTOR = XMVectorReplicate(LengthMax);
    let ClampMin: XMVECTOR = XMVectorReplicate(LengthMin);
    return XMVector2ClampLengthV(V, ClampMin, ClampMax);
}

/// Clamps the length of a 2D vector to a given range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2ClampLengthV>
#[inline]
pub fn XMVector2ClampLengthV(
    V: FXMVECTOR,
    LengthMin: FXMVECTOR,
    LengthMax: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        debug_assert!((XMVectorGetY(LengthMin) == XMVectorGetX(LengthMin)));
        debug_assert!((XMVectorGetY(LengthMax) == XMVectorGetX(LengthMax)));
        debug_assert!(XMVector2GreaterOrEqual(LengthMin, g_XMZero.v));
        debug_assert!(XMVector2GreaterOrEqual(LengthMax, g_XMZero.v));
        debug_assert!(XMVector2GreaterOrEqual(LengthMax, LengthMin));

        let LengthSq: XMVECTOR = XMVector2LengthSq(V);

        // const
        let Zero: XMVECTOR = XMVectorZero();

        let RcpLength: XMVECTOR = XMVectorReciprocalSqrt(LengthSq);

        let InfiniteLength: XMVECTOR = XMVectorEqualInt(LengthSq, g_XMInfinity.v);
        let ZeroLength: XMVECTOR = XMVectorEqual(LengthSq, Zero);

        let mut Length: XMVECTOR = XMVectorMultiply(LengthSq, RcpLength);

        let mut Normal: XMVECTOR = XMVectorMultiply(V, RcpLength);

        let Select: XMVECTOR = XMVectorEqualInt(InfiniteLength, ZeroLength);
        Length = XMVectorSelect(LengthSq, Length, Select);
        Normal = XMVectorSelect(LengthSq, Normal, Select);

        let ControlMax: XMVECTOR = XMVectorGreater(Length, LengthMax);
        let ControlMin: XMVECTOR = XMVectorLess(Length, LengthMin);

        let mut ClampLength: XMVECTOR = XMVectorSelect(Length, LengthMax, ControlMax);
        ClampLength = XMVectorSelect(ClampLength, LengthMin, ControlMin);

        let mut Result: XMVECTOR = XMVectorMultiply(Normal, ClampLength);

        // Preserve the original vector (with no precision loss) if the length falls within the given range
        let Control: XMVECTOR = XMVectorEqualInt(ControlMax, ControlMin);
        Result = XMVectorSelect(Result, V, Control);

        return Result;
    }
}

/// Reflects an incident 2D vector across a 2D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Reflect>
#[inline]
pub fn XMVector2Reflect(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
) -> XMVECTOR
{
    // Result = Incident - (2 * dot(Incident, Normal)) * Normal

    let mut Result: XMVECTOR;
    Result = XMVector2Dot(Incident, Normal);
    Result = XMVectorAdd(Result, Result);
    Result = XMVectorNegativeMultiplySubtract(Result, Normal, Incident);
    return Result;
}

/// Refracts an incident 2D vector across a 2D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Refract>
#[inline]
pub fn XMVector2Refract(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
    RefractionIndex: f32,
) -> XMVECTOR
{
    let Index: XMVECTOR = XMVectorReplicate(RefractionIndex);
    return XMVector2RefractV(Incident, Normal, Index);
}

/// Refracts an incident 2D vector across a 2D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2RefractV>
#[inline]
pub fn XMVector2RefractV(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
    RefractionIndex: FXMVECTOR,
) -> XMVECTOR
{
    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let IDotN: f32 = (Incident.vector4_f32[0] * Normal.vector4_f32[0]) + (Incident.vector4_f32[1] * Normal.vector4_f32[1]);
        // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
        let mut RY: f32 = 1.0 - (IDotN * IDotN);
        let mut RX: f32 = 1.0 - (RY * RefractionIndex.vector4_f32[0] * RefractionIndex.vector4_f32[0]);
        RY = 1.0 - (RY * RefractionIndex.vector4_f32[1] * RefractionIndex.vector4_f32[1]);
        if (RX >= 0.0)
        {
            RX = (RefractionIndex.vector4_f32[0] * Incident.vector4_f32[0]) - (Normal.vector4_f32[0] * ((RefractionIndex.vector4_f32[0] * IDotN) + sqrtf(RX)));
        }
        else
        {
            RX = 0.0;
        }
        if (RY >= 0.0)
        {
            RY = (RefractionIndex.vector4_f32[1] * Incident.vector4_f32[1]) - (Normal.vector4_f32[1] * ((RefractionIndex.vector4_f32[1] * IDotN) + sqrtf(RY)));
        }
        else
        {
            RY = 0.0;
        }

        let mut vResult: XMVECTOR = uninitialized();
        vResult.vector4_f32[0] = RX;
        vResult.vector4_f32[1] = RY;
        vResult.vector4_f32[2] = 0.0;
        vResult.vector4_f32[3] = 0.0;
        return vResult;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
        // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))
        // Get the 2D Dot product of Incident-Normal
        let IDotN: XMVECTOR = XMVector2Dot(Incident, Normal);
        // vTemp = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
        let mut vTemp: XMVECTOR = XM_FNMADD_PS!(IDotN, IDotN, g_XMOne.v);
        vTemp = _mm_mul_ps(vTemp, RefractionIndex);
        vTemp = XM_FNMADD_PS!(vTemp, RefractionIndex, g_XMOne.v);
        // If any terms are <=0, sqrt() will fail, punt to zero
        let vMask: XMVECTOR = _mm_cmpgt_ps(vTemp, g_XMZero.v);
        // R = RefractionIndex * IDotN + sqrt(R)
        vTemp = _mm_sqrt_ps(vTemp);
        vTemp = XM_FMADD_PS!(RefractionIndex, IDotN, vTemp);
        // Result = RefractionIndex * Incident - Normal * R
        let mut vResult: XMVECTOR = _mm_mul_ps(RefractionIndex, Incident);
        vResult = XM_FNMADD_PS!(vTemp, Normal, vResult);
        vResult = _mm_and_ps(vResult, vMask);
        return vResult;
    }
}

/// Computes a vector perpendicular to a 2D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Orthogonal>
#[inline]
pub fn XMVector2Orthogonal(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result: XMVECTORF32 = XMVECTORF32 { f: [
            -V.vector4_f32[1],
            V.vector4_f32[0],
            0.0,
            0.0
        ]};
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 2, 0, 1));
        vResult = _mm_mul_ps(vResult, g_XMNegateX.v);
        return vResult;
    }
}

/// Estimates the radian angle between two normalized 2D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2AngleBetweenNormalsEst>
#[inline]
pub fn XMVector2AngleBetweenNormalsEst(
    N1: FXMVECTOR,
    N2: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let mut Result: XMVECTOR = XMVector2Dot(N1, N2);
        Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
        Result = XMVectorACosEst(Result);
        return Result;
    }
}

/// Computes the radian angle between two normalized 2D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2AngleBetweenNormals>
#[inline]
pub fn XMVector2AngleBetweenNormals(
    N1: FXMVECTOR,
    N2: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let mut Result: XMVECTOR = XMVector2Dot(N1, N2);
        Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
        Result = XMVectorACos(Result);
        return Result;
    }
}

/// Computes the radian angle between two 2D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2AngleBetweenVectors>
#[inline]
pub fn XMVector2AngleBetweenVectors(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let mut L1: XMVECTOR = XMVector2ReciprocalLength(V1);
        let L2: XMVECTOR = XMVector2ReciprocalLength(V2);

        let Dot: XMVECTOR = XMVector2Dot(V1, V2);

        L1 = XMVectorMultiply(L1, L2);

        let mut CosAngle: XMVECTOR = XMVectorMultiply(Dot, L1);
        CosAngle = XMVectorClamp(CosAngle, g_XMNegativeOne.v, g_XMOne.v);

        return XMVectorACos(CosAngle);
    }
}

/// Computes the minimum distance between a line and a point.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2LinePointDistance>
#[inline]
pub fn XMVector2LinePointDistance(
    LinePoint1: FXMVECTOR,
    LinePoint2: FXMVECTOR,
    Point: FXMVECTOR,
) -> XMVECTOR
{
    // Given a vector PointVector from LinePoint1 to Point and a vector
    // LineVector from LinePoint1 to LinePoint2, the scaled distance
    // PointProjectionScale from LinePoint1 to the perpendicular projection
    // of PointVector onto the line is defined as:
    //
    //     PointProjectionScale = dot(PointVector, LineVector) / LengthSq(LineVector)

    let PointVector: XMVECTOR = XMVectorSubtract(Point, LinePoint1);
    let LineVector: XMVECTOR = XMVectorSubtract(LinePoint2, LinePoint1);

    let LengthSq: XMVECTOR = XMVector2LengthSq(LineVector);

    let mut PointProjectionScale: XMVECTOR = XMVector2Dot(PointVector, LineVector);
    PointProjectionScale = XMVectorDivide(PointProjectionScale, LengthSq);

    let mut DistanceVector: XMVECTOR = XMVectorMultiply(LineVector, PointProjectionScale);
    DistanceVector = XMVectorSubtract(PointVector, DistanceVector);

    return XMVector2Length(DistanceVector);
}

/// Finds the intersection of two lines.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2IntersectLine>
#[inline]
pub fn XMVector2IntersectLine(
    Line1Point1: FXMVECTOR,
    Line1Point2: FXMVECTOR,
    Line2Point1: FXMVECTOR,
    Line2Point2: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let V1: XMVECTOR = XMVectorSubtract(Line1Point2, Line1Point1);
        let V2: XMVECTOR = XMVectorSubtract(Line2Point2, Line2Point1);
        let V3: XMVECTOR = XMVectorSubtract(Line1Point1, Line2Point1);

        let C1: XMVECTOR = XMVector2Cross(V1, V2);
        let C2: XMVECTOR = XMVector2Cross(V2, V3);

        let Result: XMVECTOR;
        // const let Zero: XMVECTOR = XMVectorZero();
        const Zero: XMVECTOR = unsafe { g_XMZero.v };
        if (XMVector2NearEqual(C1, Zero, g_XMEpsilon.v))
        {
            if (XMVector2NearEqual(C2, Zero, g_XMEpsilon.v))
            {
                // Coincident
                Result = g_XMInfinity.v;
            }
            else
            {
                // Parallel
                Result = g_XMQNaN.v;
            }
        }
        else
        {
            // let point: Intersection = Line1Point1 + V1 * (C2 / C1)
            let mut Scale: XMVECTOR = XMVectorReciprocal(C1);
            Scale = XMVectorMultiply(C2, Scale);
            Result = XMVectorMultiplyAdd(V1, Scale, Line1Point1);
        }

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(all(_XM_SSE_INTRINSICS_))]
    unsafe {
        let V1: XMVECTOR = _mm_sub_ps(Line1Point2, Line1Point1);
        let V2: XMVECTOR = _mm_sub_ps(Line2Point2, Line2Point1);
        let V3: XMVECTOR = _mm_sub_ps(Line1Point1, Line2Point1);
        // Generate the cross products
        let C1: XMVECTOR = XMVector2Cross(V1, V2);
        let C2: XMVECTOR = XMVector2Cross(V2, V3);
        // If C1 is not close to epsilon, use the calculated value
        let mut vResultMask: XMVECTOR = _mm_setzero_ps();
        vResultMask = _mm_sub_ps(vResultMask, C1);
        vResultMask = _mm_max_ps(vResultMask, C1);
        // 0xFFFFFFFF if the calculated value is to be used
        vResultMask = _mm_cmpgt_ps(vResultMask, g_XMEpsilon.v);
        // If C1 is close to epsilon, which fail type is it? INFINITY or NAN?
        let mut vFailMask: XMVECTOR = _mm_setzero_ps();
        vFailMask = _mm_sub_ps(vFailMask, C2);
        vFailMask = _mm_max_ps(vFailMask, C2);
        vFailMask = _mm_cmple_ps(vFailMask, g_XMEpsilon.v);
        let mut vFail: XMVECTOR = _mm_and_ps(vFailMask, g_XMInfinity.v);
        vFailMask = _mm_andnot_ps(vFailMask, g_XMQNaN.v);
        // vFail is NAN or INF
        vFail = _mm_or_ps(vFail, vFailMask);
        // let point: Intersection = Line1Point1 + V1 * (C2 / C1)
        let mut vResult: XMVECTOR = _mm_div_ps(C2, C1);
        vResult = XM_FMADD_PS!(vResult, V1, Line1Point1);
        // Use result, or failure value
        vResult = _mm_and_ps(vResult, vResultMask);
        vResultMask = _mm_andnot_ps(vResultMask, vFail);
        vResult = _mm_or_ps(vResult, vResultMask);
        return vResult;
    }
}

/// Transforms a 2D vector by a matrix.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2Transform>
#[inline]
pub fn XMVector2Transform(
    V: FXMVECTOR,
    M: FXMMATRIX,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Y: XMVECTOR = XMVectorSplatY(V);
        let X: XMVECTOR = XMVectorSplatX(V);

        let mut Result: XMVECTOR = XMVectorMultiplyAdd(Y, M.r[1], M.r[3]);
        Result = XMVectorMultiplyAdd(X, M.r[0], Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
        vResult = XM_FMADD_PS!(vResult, M.r[1], M.r[3]);
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
        vResult = XM_FMADD_PS!(vTemp, M.r[0], vResult);
        return vResult;
    }
}

// TODO: XMVector2TransformStream

/// Transforms a 2D vector by a given matrix, projecting the result back into w = 1.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2TransformCoord>
#[inline]
pub fn XMVector2TransformCoord(
    V: FXMVECTOR,
    M: FXMMATRIX,
) -> FXMVECTOR {
    unsafe {
        let Y: XMVECTOR = XMVectorSplatY(V);
        let X: XMVECTOR = XMVectorSplatX(V);

        let mut Result: XMVECTOR = XMVectorMultiplyAdd(Y, M.r[1], M.r[3]);
        Result = XMVectorMultiplyAdd(X, M.r[0], Result);

        let W: XMVECTOR = XMVectorSplatW(Result);
        return XMVectorDivide(Result, W);
    }
}

// TODO: XMVector2TransformCoordStream

/// Transforms a 2D vector by a matrix.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector2TransformNormal>
#[inline]
pub fn XMVector2TransformNormal(
    V: FXMVECTOR,
    M: FXMMATRIX,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Y: XMVECTOR = XMVectorSplatY(V);
        let X: XMVECTOR = XMVectorSplatX(V);

        let mut Result: XMVECTOR = XMVectorMultiply(Y, M.r[1]);
        Result = XMVectorMultiplyAdd(X, M.r[0], Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
        vResult = _mm_mul_ps(vResult, M.r[1]);
        let vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
        vResult = XM_FMADD_PS!(vTemp, M.r[0], vResult);
        return vResult;
    }
}

// TODO: XMVector2TransformNormalStream

// 3D Vector

/// Tests whether two 3D vectors are equal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Equal>
#[inline]
pub fn XMVector3Equal(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] == V2.vector4_f32[0]) && (V1.vector4_f32[1] == V2.vector4_f32[1]) && (V1.vector4_f32[2] == V2.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 7) == 7) != false);
    }
}

#[test]
fn test_XMVector3Equal() {
    let a = XMVectorReplicate(1.0);
    let b = XMVectorReplicate(1.0);

    assert!(XMVector3Equal(a, b));
    assert!(XMVector3Equal(a, XMVectorSetW(b, 2.0)));

    assert!(!XMVector3Equal(a, XMVectorSetX(b, 2.0)));
    assert!(!XMVector3Equal(a, XMVectorSetY(b, 2.0)));
    assert!(!XMVector3Equal(a, XMVectorSetZ(b, 2.0)));
}

/// Tests whether two 3D vectors are equal. In addition, this function returns
/// a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3EqualR>
#[inline]
pub fn XMVector3EqualR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_f32[0] == V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] == V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] == V2.vector4_f32[2]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] != V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] != V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] != V2.vector4_f32[2]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        let iTest: i32 = _mm_movemask_ps(vTemp) & 7;
        let mut CR: u32 = 0;
        if (iTest == 7)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTest)
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

#[test]
fn test_XMVector3EqualR() {
    let a = XMVectorReplicate(1.0);
    let b = XMVectorReplicate(1.0);

    let r = XMVector3EqualR(a, b);
    assert!(XMComparisonAnyTrue(r));
    assert!(!XMComparisonAnyFalse(r));
    assert!(XMComparisonAllTrue(r));
    assert!(!XMComparisonAllFalse(r));

    let r = XMVector3EqualR(a, XMVectorReplicate(2.0));
    assert!(!XMComparisonAnyTrue(r));
    assert!(XMComparisonAnyFalse(r));
    assert!(!XMComparisonAllTrue(r));
    assert!(XMComparisonAllFalse(r));

    let r = XMVector3EqualR(a, XMVectorSetX(b, 2.0));
    assert!(XMComparisonAnyTrue(r));
    assert!(XMComparisonAnyFalse(r));
    assert!(!XMComparisonAllTrue(r));
    assert!(!XMComparisonAllFalse(r));
}

/// Tests whether two 3D vectors are equal, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3EqualInt>
#[inline]
pub fn XMVector3EqualInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_u32[0] == V2.vector4_u32[0]) && (V1.vector4_u32[1] == V2.vector4_u32[1]) && (V1.vector4_u32[2] == V2.vector4_u32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 7) == 7) != false);
    }
}

/// Tests whether two 3D vectors are equal, treating each component as an
/// unsigned integer. In addition, this function returns a comparison value
/// that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3EqualIntR>
#[inline]
pub fn XMVector3EqualIntR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_u32[0] == V2.vector4_u32[0]) &&
            (V1.vector4_u32[1] == V2.vector4_u32[1]) &&
            (V1.vector4_u32[2] == V2.vector4_u32[2]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_u32[0] != V2.vector4_u32[0]) &&
            (V1.vector4_u32[1] != V2.vector4_u32[1]) &&
            (V1.vector4_u32[2] != V2.vector4_u32[2]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        let iTemp: i32 = _mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 7;
        let mut CR: u32 = 0;
        if (iTemp == 7)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTemp)
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 3D vector is near another 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3NearEqual>
#[inline]
pub fn XMVector3NearEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    Epsilon: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let (dx, dy, dz): (f32, f32, f32);

        dx = fabsf(V1.vector4_f32[0] - V2.vector4_f32[0]);
        dy = fabsf(V1.vector4_f32[1] - V2.vector4_f32[1]);
        dz = fabsf(V1.vector4_f32[2] - V2.vector4_f32[2]);
        return (((dx <= Epsilon.vector4_f32[0]) &&
            (dy <= Epsilon.vector4_f32[1]) &&
            (dz <= Epsilon.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Get the difference
        let vDelta: XMVECTOR = _mm_sub_ps(V1, V2);
        // Get the absolute value of the difference
        let mut vTemp: XMVECTOR = _mm_setzero_ps();
        vTemp = _mm_sub_ps(vTemp, vDelta);
        vTemp = _mm_max_ps(vTemp, vDelta);
        vTemp = _mm_cmple_ps(vTemp, Epsilon);
        // w is don't care
        return (((_mm_movemask_ps(vTemp) & 7) == 0x7) != false);
    }
}

/// Tests whether two 3D vectors are not equal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3NotEqual>
#[inline]
pub fn XMVector3NotEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] != V2.vector4_f32[0]) || (V1.vector4_f32[1] != V2.vector4_f32[1]) || (V1.vector4_f32[2] != V2.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 7) != 7) != false);
    }
}

/// Test whether two 3D vectors are not equal, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3NotEqualInt>
#[inline]
pub fn XMVector3NotEqualInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_u32[0] != V2.vector4_u32[0]) || (V1.vector4_u32[1] != V2.vector4_u32[1]) || (V1.vector4_u32[2] != V2.vector4_u32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 7) != 7) != false);
    }
}

/// Tests whether one 3D vector is greater than another 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Greater>
#[inline]
pub fn XMVector3Greater(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] > V2.vector4_f32[0]) && (V1.vector4_f32[1] > V2.vector4_f32[1]) && (V1.vector4_f32[2] > V2.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 7) == 7) != false);
    }
}

/// Tests whether one 3D vector is greater than another 3D vector and returns a
/// comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3GreaterR>
#[inline]
pub fn XMVector3GreaterR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_f32[0] > V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] > V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] > V2.vector4_f32[2]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] <= V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] <= V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] <= V2.vector4_f32[2]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        let mut CR: u32 = 0;
        let iTest: i32 = _mm_movemask_ps(vTemp) & 7;
        if (iTest == 7)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTest)
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 3D vector is greater-than-or-equal-to another 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3GreaterOrEqual>
#[inline]
pub fn XMVector3GreaterOrEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] >= V2.vector4_f32[0]) && (V1.vector4_f32[1] >= V2.vector4_f32[1]) && (V1.vector4_f32[2] >= V2.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 7) == 7) != false);
    }
}

/// Tests whether one 3D vector is greater-than-or-equal-to another 3D vector and returns a
/// comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3GreaterOrEqualR>
#[inline]
pub fn XMVector3GreaterOrEqualR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_f32[0] >= V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] >= V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] >= V2.vector4_f32[2]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] < V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] < V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] < V2.vector4_f32[2]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        let mut CR: u32 = 0;
        let iTest: i32 = _mm_movemask_ps(vTemp) & 7;
        if (iTest == 7)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if !ibool(iTest)
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 3D vector is less than another 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Less>
#[inline]
pub fn XMVector3Less(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] < V2.vector4_f32[0]) && (V1.vector4_f32[1] < V2.vector4_f32[1]) && (V1.vector4_f32[2] < V2.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmplt_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 7) == 7) != false);
    }
}


/// Tests whether one 3D vector is less than or equal to another 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3LessOrEqual>
#[inline]
pub fn XMVector3LessOrEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] <= V2.vector4_f32[0]) && (V1.vector4_f32[1] <= V2.vector4_f32[1]) && (V1.vector4_f32[2] <= V2.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmple_ps(V1, V2);
        return (((_mm_movemask_ps(vTemp) & 7) == 7) != false);
    }
}

/// Tests whether the components of a 3D vector are within set bounds.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3InBounds>
#[inline]
pub fn XMVector3InBounds(
    V: FXMVECTOR,
    Bounds: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) &&
            (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1]) &&
            (V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test if less than or equal
        let mut vTemp1: XMVECTOR = _mm_cmple_ps(V, Bounds);
        // Negate the bounds
        let mut vTemp2: XMVECTOR = _mm_mul_ps(Bounds, g_XMNegativeOne.v);
        // Test if greater or equal (Reversed)
        vTemp2 = _mm_cmple_ps(vTemp2, V);
        // Blend answers
        vTemp1 = _mm_and_ps(vTemp1, vTemp2);
        // x,y and z in bounds? (w is don't care)
        return (((_mm_movemask_ps(vTemp1) & 0x7) == 0x7) != false);
    }

    // NOTE: The source contains a fallback that does not seem to be reachable.
    // return XMComparisonAllInBounds(XMVector3InBoundsR(V, Bounds));
}

/// Tests whether any component of a 3D vector is a NaN.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3IsNaN>
#[inline]
pub fn XMVector3IsNaN(
    V: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (XMISNAN!(V.vector4_f32[0]) ||
            XMISNAN!(V.vector4_f32[1]) ||
            XMISNAN!(V.vector4_f32[2]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test against itself. NaN is always not equal
        let vTempNan: XMVECTOR = _mm_cmpneq_ps(V, V);
        // If x or y or z are NaN, the mask is non-zero
        return ((_mm_movemask_ps(vTempNan) & 7) != 0);
    }
}

/// Tests whether any component of a 3D vector is positive or negative infinity.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3IsInfinite>
#[inline]
pub fn XMVector3IsInfinite(
    V: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (XMISINF!(V.vector4_f32[0]) ||
            XMISINF!(V.vector4_f32[1]) ||
            XMISINF!(V.vector4_f32[2]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Mask off the sign bit
        let mut vTemp: __m128 = _mm_and_ps(V, g_XMAbsMask.v);
        // Compare to infinity
        vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity.v);
        // If x,y or z are infinity, the signs are true.
        return ((_mm_movemask_ps(vTemp) & 7) != 0);
    }
}

/// Computes the dot product between 3D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Dot>
#[inline]
pub fn XMVector3Dot(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let fValue: f32 = V1.vector4_f32[0] * V2.vector4_f32[0] + V1.vector4_f32[1] * V2.vector4_f32[1] + V1.vector4_f32[2] * V2.vector4_f32[2];
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = fValue;
        vResult.f[1] = fValue;
        vResult.f[2] = fValue;
        vResult.f[3] = fValue;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        return _mm_dp_ps(V1, V2, 0x7f);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product
        let mut vDot: XMVECTOR = _mm_mul_ps(V1, V2);
        // x=Dot.vector4_f32[1], y=Dot.vector4_f32[2]
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vDot, _MM_SHUFFLE(2, 1, 2, 1));
        // Result.vector4_f32[0] = x+y
        vDot = _mm_add_ss(vDot, vTemp);
        // x=Dot.vector4_f32[2]
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        // Result.vector4_f32[0] = (x+y)+z
        vDot = _mm_add_ss(vDot, vTemp);
        // Splat x
        return XM_PERMUTE_PS!(vDot, _MM_SHUFFLE(0, 0, 0, 0));
    }
}

/// Computes the cross product between 3D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Cross>
#[inline]
pub fn XMVector3Cross(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult = XMVECTORF32 {
            f: [
                (V1.vector4_f32[1] * V2.vector4_f32[2]) - (V1.vector4_f32[2] * V2.vector4_f32[1]),
                (V1.vector4_f32[2] * V2.vector4_f32[0]) - (V1.vector4_f32[0] * V2.vector4_f32[2]),
                (V1.vector4_f32[0] * V2.vector4_f32[1]) - (V1.vector4_f32[1] * V2.vector4_f32[0]),
                0.0
            ]
        };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // y1,z1,x1,w1
        let mut vTemp1: XMVECTOR = XM_PERMUTE_PS!(V1, _MM_SHUFFLE(3, 0, 2, 1));
        // z2,x2,y2,w2
        let mut vTemp2: XMVECTOR = XM_PERMUTE_PS!(V2, _MM_SHUFFLE(3, 1, 0, 2));
        // Perform the left operation
        let mut vResult: XMVECTOR = _mm_mul_ps(vTemp1, vTemp2);
        // z1,x1,y1,w1
        vTemp1 = XM_PERMUTE_PS!(vTemp1, _MM_SHUFFLE(3, 0, 2, 1));
        // y2,z2,x2,w2
        vTemp2 = XM_PERMUTE_PS!(vTemp2, _MM_SHUFFLE(3, 1, 0, 2));
        // Perform the right operation
        vResult = XM_FNMADD_PS!(vTemp1, vTemp2, vResult);
        // Set w to zero
        return _mm_and_ps(vResult, g_XMMask3.v);
    }
}

/// Computes the square of the length of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3LengthSq>
#[inline]
pub fn XMVector3LengthSq(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    return XMVector3Dot(V, V);
}

/// Estimates the reciprocal of the length of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3ReciprocalLengthEst>
#[inline]
pub fn XMVector3ReciprocalLengthEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector3LengthSq(V);
        Result = XMVectorReciprocalSqrtEst(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x7f);
        return _mm_rsqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y and z
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and y
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 2, 1, 2));
        // x+z, y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        // y,y,y,y
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        // x+z+y,??,??,??
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        // Splat the length squared
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        // Get the reciprocal
        vLengthSq = _mm_rsqrt_ps(vLengthSq);
        return vLengthSq;
    }
}

/// Computes the reciprocal of the length of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3ReciprocalLength>
#[inline]
pub fn XMVector3ReciprocalLength(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector3LengthSq(V);
        Result = XMVectorReciprocalSqrt(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x7f);
        let vLengthSq: XMVECTOR = _mm_sqrt_ps(vTemp);
        return _mm_div_ps(g_XMOne.v, vLengthSq);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vDot: XMVECTOR = _mm_mul_ps(V, V);
        vDot = _mm_and_ps(vDot, g_XMMask3.v);
        vDot = _mm_hadd_ps(vDot, vDot);
        vDot = _mm_hadd_ps(vDot, vDot);
        vDot = _mm_sqrt_ps(vDot);
        vDot = _mm_div_ps(g_XMOne.v, vDot);
        return vDot
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product
        let mut vDot: XMVECTOR = _mm_mul_ps(V, V);
        // x=Dot.y, y=Dot.z
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vDot, _MM_SHUFFLE(2, 1, 2, 1));
        // Result.x = x+y
        vDot = _mm_add_ss(vDot, vTemp);
        // x=Dot.z
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        // Result.x = (x+y)+z
        vDot = _mm_add_ss(vDot, vTemp);
        // Splat x
        vDot = XM_PERMUTE_PS!(vDot, _MM_SHUFFLE(0, 0, 0, 0));
        // Get the reciprocal
        vDot = _mm_sqrt_ps(vDot);
        // Get the reciprocal
        vDot = _mm_div_ps(g_XMOne.v, vDot);
        return vDot;
    }
}

/// Estimates the length of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3LengthEst>
#[inline]
pub fn XMVector3LengthEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector3LengthSq(V);
        Result = XMVectorSqrtEst(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x7f);
        return _mm_sqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3.v);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y and z
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and y
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 2, 1, 2));
        // x+z, y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        // y,y,y,y
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        // x+z+y,??,??,??
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        // Splat the length squared
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        // Get the length
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }
}

/// Computes the length of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Length>
#[inline]
pub fn XMVector3Length(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector3LengthSq(V);
        Result = XMVectorSqrt(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x7f);
        return _mm_sqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3.v);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y and z
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and y
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 2, 1, 2));
        // x+z, y
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        // y,y,y,y
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        // x+z+y,??,??,??
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        // Splat the length squared
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        // Get the length
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }
}

/// Estimates the normalized version of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3NormalizeEst>
#[inline]
pub fn XMVector3NormalizeEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector3ReciprocalLength(V);
        Result = XMVectorMultiply(V, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0x7f);
        let vResult: XMVECTOR = _mm_rsqrt_ps(vTemp);
        return _mm_mul_ps(vResult, V);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vDot: XMVECTOR = _mm_mul_ps(V, V);
        vDot = _mm_and_ps(vDot, g_XMMask3.v);
        vDot = _mm_hadd_ps(vDot, vDot);
        vDot = _mm_hadd_ps(vDot, vDot);
        vDot = _mm_rsqrt_ps(vDot);
        vDot = _mm_mul_ps(vDot, V);
        return vDot;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product
        let mut vDot: XMVECTOR = _mm_mul_ps(V, V);
        // x=Dot.y, y=Dot.z
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vDot, _MM_SHUFFLE(2, 1, 2, 1));
        // Result.x = x+y
        vDot = _mm_add_ss(vDot, vTemp);
        // x=Dot.z
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        // Result.x = (x+y)+z
        vDot = _mm_add_ss(vDot, vTemp);
        // Splat x
        vDot = XM_PERMUTE_PS!(vDot, _MM_SHUFFLE(0, 0, 0, 0));
        // Get the reciprocal
        vDot = _mm_rsqrt_ps(vDot);
        // Perform the normalization
        vDot = _mm_mul_ps(vDot, V);
        return vDot;
    }
}

/// Returns the normalized version of a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Normalize>
#[inline]
pub fn XMVector3Normalize(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut fLength: f32;
        let mut vResult: XMVECTOR;

        vResult = XMVector3Length(V);
        fLength = vResult.vector4_f32[0];

        // Prevent divide by zero
        if (fLength > 0.0)
        {
            fLength = 1.0 / fLength;
        }

        vResult.vector4_f32[0] = V.vector4_f32[0] * fLength;
        vResult.vector4_f32[1] = V.vector4_f32[1] * fLength;
        vResult.vector4_f32[2] = V.vector4_f32[2] * fLength;
        vResult.vector4_f32[3] = V.vector4_f32[3] * fLength;

        return vResult;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_dp_ps(V, V, 0x7f);
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Divide to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y and z only
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3.v);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Divide to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y and z only
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 1, 2, 1));
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vTemp = XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
        vLengthSq = _mm_add_ss(vLengthSq, vTemp);
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Divide to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }
}

/// Clamps the length of a 3D vector to a given range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3ClampLength>
#[inline]
pub fn XMVector3ClampLength(
    V: FXMVECTOR,
    LengthMin: f32,
    LengthMax: f32,
) -> FXMVECTOR
{
    let ClampMax: XMVECTOR = XMVectorReplicate(LengthMax);
    let ClampMin: XMVECTOR = XMVectorReplicate(LengthMin);

    return XMVector3ClampLengthV(V, ClampMin, ClampMax);
}

/// Clamps the length of a 3D vector to a given range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3ClampLengthV>
#[inline]
pub fn XMVector3ClampLengthV(
    V: FXMVECTOR,
    LengthMin: FXMVECTOR,
    LengthMax: FXMVECTOR,
) -> FXMVECTOR
{
    let LengthSq: XMVECTOR = XMVector3LengthSq(V);

    // const Zero: XMVECTOR = XMVectorZero();

    let RcpLength: XMVECTOR = XMVectorReciprocalSqrt(LengthSq);

    let InfiniteLength: XMVECTOR = XMVectorEqualInt(LengthSq, unsafe { g_XMInfinity.v });
    let ZeroLength: XMVECTOR = XMVectorEqual(LengthSq, unsafe { g_XMZero.v });

    let mut Normal: XMVECTOR = XMVectorMultiply(V, RcpLength);

    let mut Length: XMVECTOR = XMVectorMultiply(LengthSq, RcpLength);

    let Select: XMVECTOR = XMVectorEqualInt(InfiniteLength, ZeroLength);
    Length = XMVectorSelect(LengthSq, Length, Select);
    Normal = XMVectorSelect(LengthSq, Normal, Select);

    let ControlMax: XMVECTOR = XMVectorGreater(Length, LengthMax);
    let ControlMin: XMVECTOR = XMVectorLess(Length, LengthMin);

    let mut ClampLength: XMVECTOR = XMVectorSelect(Length, LengthMax, ControlMax);
    ClampLength = XMVectorSelect(ClampLength, LengthMin, ControlMin);

    let mut Result: XMVECTOR = XMVectorMultiply(Normal, ClampLength);

    // Preserve the original vector (with no precision loss) if the length falls within the given range
    let Control: XMVECTOR = XMVectorEqualInt(ControlMax, ControlMin);
    Result = XMVectorSelect(Result, V, Control);

    return Result;
}

/// Reflects an incident 3D vector across a 3D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Reflect>
#[inline]
pub fn XMVector3Reflect(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR
) -> FXMVECTOR
{
    // Result = Incident - (2 * dot(Incident, Normal)) * Normal

    let mut Result: XMVECTOR = XMVector3Dot(Incident, Normal);
    Result = XMVectorAdd(Result, Result);
    Result = XMVectorNegativeMultiplySubtract(Result, Normal, Incident);

    return Result;
}

/// Refracts an incident 3D vector across a 3D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Refract>
#[inline]
pub fn XMVector3Refract(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
    RefractionIndex: f32,
) -> FXMVECTOR
{
    let Index: XMVECTOR = XMVectorReplicate(RefractionIndex);
    return XMVector3RefractV(Incident, Normal, Index);
}

/// Refracts an incident 3D vector across a 3D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3RefractV>
#[inline]
pub fn XMVector3RefractV(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
    RefractionIndex: FXMVECTOR,
) -> FXMVECTOR
{
    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        const Zero: XMVECTOR = unsafe { g_XMZero.v };

        let IDotN: XMVECTOR = XMVector3Dot(Incident, Normal);

        // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
        let mut R: XMVECTOR = XMVectorNegativeMultiplySubtract(IDotN, IDotN, g_XMOne.v);
        R = XMVectorMultiply(R, RefractionIndex);
        R = XMVectorNegativeMultiplySubtract(R, RefractionIndex, g_XMOne.v);

        if (XMVector4LessOrEqual(R, Zero))
        {
            // Total internal reflection
            return Zero;
        }
        else
        {
            // R = RefractionIndex * IDotN + sqrt(R)
            R = XMVectorSqrt(R);
            R = XMVectorMultiplyAdd(RefractionIndex, IDotN, R);

            // Result = RefractionIndex * Incident - Normal * R
            let mut Result: XMVECTOR = XMVectorMultiply(RefractionIndex, Incident);
            Result = XMVectorNegativeMultiplySubtract(Normal, R, Result);

            return Result;
        }
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
        // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))
        let IDotN: XMVECTOR = XMVector3Dot(Incident, Normal);
        // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
        let mut R: XMVECTOR = XM_FNMADD_PS!(IDotN, IDotN, g_XMOne.v);
        let R2: XMVECTOR = _mm_mul_ps(RefractionIndex, RefractionIndex);
        R = XM_FNMADD_PS!(R, R2, g_XMOne.v);

        let mut vResult: XMVECTOR = _mm_cmple_ps(R, g_XMZero.v);
        if (_mm_movemask_ps(vResult) == 0x0f)
        {
            // Total internal reflection
            vResult = g_XMZero.v;
        }
        else
        {
            // R = RefractionIndex * IDotN + sqrt(R)
            R = _mm_sqrt_ps(R);
            R = XM_FMADD_PS!(RefractionIndex, IDotN, R);
            // Result = RefractionIndex * Incident - Normal * R
            vResult = _mm_mul_ps(RefractionIndex, Incident);
            vResult = XM_FNMADD_PS!(R, Normal, vResult);
        }
        return vResult;
    }
}

/// Computes a vector perpendicular to a 3D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Orthogonal>
#[inline]
pub fn XMVector3Orthogonal(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    let Zero: XMVECTOR = XMVectorZero();
    let Z: XMVECTOR = XMVectorSplatZ(V);
    // NOTE: (PERFORMANCE) The fast-path XMVectorSwizzle template functions are not yet implemented.
    // let YZYY: XMVECTOR = XMVectorSwizzle(V, XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y);

    // TODO: Delete note above after benchmarking
    let YZYY: XMVECTOR = <(XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y)>::XMVectorSwizzle(V);

    let NegativeV: XMVECTOR = XMVectorSubtract(Zero, V);

    let ZIsNegative: XMVECTOR = XMVectorLess(Z, Zero);
    let YZYYIsNegative: XMVECTOR = XMVectorLess(YZYY, Zero);

    let S: XMVECTOR = XMVectorAdd(YZYY, Z);
    let D: XMVECTOR = XMVectorSubtract(YZYY, Z);

    let Select: XMVECTOR = XMVectorEqualInt(ZIsNegative, YZYYIsNegative);

    // NOTE: (PERFORMANCE) The fast-path XMVectorPermute template functions are not yet implemented.
    // let R0: XMVECTOR = XMVectorPermute(NegativeV, S, XM_PERMUTE_1X, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_0X);
    // let R1: XMVECTOR = XMVectorPermute(V, D, XM_PERMUTE_1X, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_0X);

    // TODO: Delete note above after benchmarking
    let R0: XMVECTOR = <(XM_PERMUTE_1X, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_0X)>::XMVectorPermute(NegativeV, S);
    let R1: XMVECTOR = <(XM_PERMUTE_1X, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_0X)>::XMVectorPermute(V, D);

    return XMVectorSelect(R1, R0, Select);
}

/// Estimates the radian angle between two normalized 3D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3AngleBetweenNormalsEst>
#[inline]
pub fn XMVector3AngleBetweenNormalsEst(
    N1: FXMVECTOR,
    N2: FXMVECTOR,
) -> FXMVECTOR
{
    unsafe {
        let mut Result: XMVECTOR = XMVector3Dot(N1, N2);
        Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
        Result = XMVectorACosEst(Result);
        return Result;
    }
}

/// Computes the radian angle between two normalized 3D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3AngleBetweenNormals>
#[inline]
pub fn XMVector3AngleBetweenNormals(
    N1: FXMVECTOR,
    N2: FXMVECTOR,
) -> FXMVECTOR
{
    unsafe {
        let mut Result: XMVECTOR = XMVector3Dot(N1, N2);
        Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
        Result = XMVectorACos(Result);
        return Result;
    }
}

/// Computes the radian angle between two 3D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3AngleBetweenVectors>
#[inline]
pub fn XMVector3AngleBetweenVectors(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> FXMVECTOR
{
    unsafe {
        let mut L1: XMVECTOR = XMVector3ReciprocalLength(V1);
        let L2: XMVECTOR = XMVector3ReciprocalLength(V2);

        let Dot: XMVECTOR = XMVector3Dot(V1, V2);

        L1 = XMVectorMultiply(L1, L2);

        let mut CosAngle: XMVECTOR = XMVectorMultiply(Dot, L1);
        CosAngle = XMVectorClamp(CosAngle, g_XMNegativeOne.v, g_XMOne.v);

        return XMVectorACos(CosAngle);
    }
}

/// Computes the minimum distance between a line and a point.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3LinePointDistance>
#[inline]
pub fn XMVector3LinePointDistance(
    LinePoint1: FXMVECTOR,
    LinePoint2: FXMVECTOR,
    Point: FXMVECTOR
) -> FXMVECTOR
{
    // Given a vector PointVector from LinePoint1 to Point and a vector
    // LineVector from LinePoint1 to LinePoint2, the scaled distance
    // PointProjectionScale from LinePoint1 to the perpendicular projection
    // of PointVector onto the line is defined as:
    //
    //     PointProjectionScale = dot(PointVector, LineVector) / LengthSq(LineVector)

    let PointVector: XMVECTOR = XMVectorSubtract(Point, LinePoint1);
    let LineVector: XMVECTOR = XMVectorSubtract(LinePoint2, LinePoint1);

    let LengthSq: XMVECTOR = XMVector3LengthSq(LineVector);

    let mut PointProjectionScale: XMVECTOR = XMVector3Dot(PointVector, LineVector);
    PointProjectionScale = XMVectorDivide(PointProjectionScale, LengthSq);

    let mut DistanceVector: XMVECTOR = XMVectorMultiply(LineVector, PointProjectionScale);
    DistanceVector = XMVectorSubtract(PointVector, DistanceVector);

    return XMVector3Length(DistanceVector);
}

/// Using a reference normal vector, splits a 3D vector into components that are parallel and perpendicular to the normal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3ComponentsFromNormal>
#[inline]
pub fn XMVector3ComponentsFromNormal(
    pParallel: &mut XMVECTOR,
    pPerpendicular: &mut XMVECTOR,
    V: FXMVECTOR,
    Normal: FXMVECTOR
)
{
    let Scale: XMVECTOR = XMVector3Dot(V, Normal);

    let Parallel: XMVECTOR = XMVectorMultiply(Normal, Scale);

    *pParallel = Parallel;
    *pPerpendicular = XMVectorSubtract(V, Parallel);
}


/// Rotates a 3D vector using a quaternion.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Rotate>
#[inline]
pub fn XMVector3Rotate(
    V: FXMVECTOR,
    RotationQuaternion: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let A: XMVECTOR = XMVectorSelect(g_XMSelect1110.v, V, g_XMSelect1110.v);
        let Q: XMVECTOR = XMQuaternionConjugate(RotationQuaternion);
        let Result: XMVECTOR = XMQuaternionMultiply(Q, A);
        return XMQuaternionMultiply(Result, RotationQuaternion);
    }
}


/// Rotates a 3D vector using the inverse of a quaternion.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3InverseRotate>
#[inline]
pub fn XMVector3InverseRotate(
    V: FXMVECTOR,
    RotationQuaternion: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let A: XMVECTOR = XMVectorSelect(g_XMSelect1110.v, V, g_XMSelect1110.v);
        let Result: XMVECTOR = XMQuaternionMultiply(RotationQuaternion, A);
        let Q: XMVECTOR = XMQuaternionConjugate(RotationQuaternion);
        return XMQuaternionMultiply(Result, Q);
    }
}

/// Transforms a 3D vector by a matrix.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Transform>
#[inline]
pub fn XMVector3Transform(
    V: FXMVECTOR,
    M: FXMMATRIX,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Z: XMVECTOR = XMVectorSplatZ(V);
        let Y: XMVECTOR = XMVectorSplatY(V);
        let X: XMVECTOR = XMVectorSplatX(V);

        let mut Result: XMVECTOR = XMVectorMultiplyAdd(Z, M.r[2], M.r[3]);
        Result = XMVectorMultiplyAdd(Y, M.r[1], Result);
        Result = XMVectorMultiplyAdd(X, M.r[0], Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 2, 2, 2)); // Z
        vResult = XM_FMADD_PS!(vResult, M.r[2], M.r[3]);
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
        vResult = XM_FMADD_PS!(vTemp, M.r[1], vResult);
        vTemp = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
        vResult = XM_FMADD_PS!(vTemp, M.r[0], vResult);
        return vResult;
    }
}

/// Transforms a 3D vector by a given matrix, projecting the result back into w = 1.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3TransformCoord>
#[inline]
pub fn XMVector3TransformCoord(
    V: FXMVECTOR,
    M: FXMMATRIX,
) -> FXMVECTOR
{
    unsafe {
        let Z: XMVECTOR = XMVectorSplatZ(V);
        let Y: XMVECTOR = XMVectorSplatY(V);
        let X: XMVECTOR = XMVectorSplatX(V);

        let mut Result: XMVECTOR = XMVectorMultiplyAdd(Z, M.r[2], M.r[3]);
        Result = XMVectorMultiplyAdd(Y, M.r[1], Result);
        Result = XMVectorMultiplyAdd(X, M.r[0], Result);

        let W: XMVECTOR = XMVectorSplatW(Result);
        return XMVectorDivide(Result, W);
    }
}

// TODO: XMVector3TransformCoordStream


/// Transforms the 3D vector normal by the given matrix.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3TransformNormal>
#[inline]
pub fn XMVector3TransformNormal(
    V: FXMVECTOR,
    M: FXMMATRIX,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Z: XMVECTOR = XMVectorSplatZ(V);
        let Y: XMVECTOR = XMVectorSplatY(V);
        let X: XMVECTOR = XMVectorSplatX(V);

        let mut Result: XMVECTOR = XMVectorMultiply(Z, M.r[2]);
        Result = XMVectorMultiplyAdd(Y, M.r[1], Result);
        Result = XMVectorMultiplyAdd(X, M.r[0], Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 2, 2, 2)); // Z
        vResult = _mm_mul_ps(vResult, M.r[2]);
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
        vResult = XM_FMADD_PS!(vTemp, M.r[1], vResult);
        vTemp = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
        vResult = XM_FMADD_PS!(vTemp, M.r[0], vResult);
        return vResult;
    }
}

// TODO: XMVector3TransformNormalStream


/// Project a 3D vector from object space into screen space.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Project>
#[inline]
pub fn XMVector3Project(
    V: FXMVECTOR,
    ViewportX: f32,
    ViewportY: f32,
    ViewportWidth: f32,
    ViewportHeight: f32,
    ViewportMinZ: f32,
    ViewportMaxZ: f32,
    Projection: FXMMATRIX,
    View: CXMMATRIX,
    World: CXMMATRIX,
) -> FXMVECTOR
{
    let HalfViewportWidth: f32 = ViewportWidth * 0.5;
    let HalfViewportHeight: f32 = ViewportHeight * 0.5;

    let Scale: XMVECTOR = XMVectorSet(HalfViewportWidth, -HalfViewportHeight, ViewportMaxZ - ViewportMinZ, 0.0);
    let Offset: XMVECTOR = XMVectorSet(ViewportX + HalfViewportWidth, ViewportY + HalfViewportHeight, ViewportMinZ, 0.0);

    let mut Transform: XMMATRIX = XMMatrixMultiply(*World, View);
    Transform = XMMatrixMultiply(Transform, &Projection);

    let mut Result: XMVECTOR = XMVector3TransformCoord(V, Transform);

    Result = XMVectorMultiplyAdd(Result, Scale, Offset);

    return Result;
}

// TODO: XMVector3ProjectStream

/// Projects a 3D vector from screen space into object space.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector3Unproject>
#[inline]
pub fn XMVector3Unproject(
    V: FXMVECTOR,
    ViewportX: f32,
    ViewportY: f32,
    ViewportWidth: f32,
    ViewportHeight: f32,
    ViewportMinZ: f32,
    ViewportMaxZ: f32,
    Projection: FXMMATRIX,
    View: CXMMATRIX,
    World: CXMMATRIX,
) -> FXMVECTOR
{
    const D: XMVECTORF32 = XMVECTORF32 { f: [ -1.0, 1.0, 0.0, 0.0] };

    let mut Scale: XMVECTOR = XMVectorSet(ViewportWidth * 0.5, -ViewportHeight * 0.5, ViewportMaxZ - ViewportMinZ, 1.0);
    Scale = XMVectorReciprocal(Scale);

    let mut Offset: XMVECTOR = XMVectorSet(-ViewportX, -ViewportY, -ViewportMinZ, 0.0);
    Offset = XMVectorMultiplyAdd(Scale, Offset, unsafe { D.v });

    let mut Transform: XMMATRIX = XMMatrixMultiply(*World, View);
    Transform = XMMatrixMultiply(Transform, &Projection);
    let mut det = unsafe { mem::MaybeUninit::uninit().assume_init() };
    Transform = XMMatrixInverse(&mut det, Transform);

    let Result: XMVECTOR = XMVectorMultiplyAdd(V, Scale, Offset);

    return XMVector3TransformCoord(Result, Transform);
}

// TODO: XMVector3UnprojectStream


// 4D Vector

/// Tests whether two 4D vectors are equal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Equal>
#[inline]
pub fn XMVector4Equal(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] == V2.vector4_f32[0]) && (V1.vector4_f32[1] == V2.vector4_f32[1]) && (V1.vector4_f32[2] == V2.vector4_f32[2]) && (V1.vector4_f32[3] == V2.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        return ((_mm_movemask_ps(vTemp) == 0x0f) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllTrue(XMVector4EqualR(V1, V2));
}

/// Tests whether two 4D vectors are equal. In addition, this function returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4EqualR>
#[inline]
pub fn XMVector4EqualR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;

        if ((V1.vector4_f32[0] == V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] == V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] == V2.vector4_f32[2]) &&
            (V1.vector4_f32[3] == V2.vector4_f32[3]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] != V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] != V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] != V2.vector4_f32[2]) &&
            (V1.vector4_f32[3] != V2.vector4_f32[3]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpeq_ps(V1, V2);
        let iTest: i32 = _mm_movemask_ps(vTemp);
        let mut CR: u32 = 0;
        if (iTest == 0xf)     // All equal?
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (iTest == 0)  // All not equal?
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether two 4D vectors are equal, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4EqualInt>
#[inline]
pub fn XMVector4EqualInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_u32[0] == V2.vector4_u32[0]) && (V1.vector4_u32[1] == V2.vector4_u32[1]) && (V1.vector4_u32[2] == V2.vector4_u32[2]) && (V1.vector4_u32[3] == V2.vector4_u32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return ((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) == 0xf) != false);
    }

    // NOTE: The source has a fallback that does not seem reachable
    // return return XMComparisonAllTrue(XMVector4EqualIntR(V1, V2));
}

/// Tests whether two 4D vectors are equal, treating each component as an
/// unsigned integer. In addition, this function returns a comparison value
/// that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4EqualIntR>
#[inline]
pub fn XMVector4EqualIntR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if (V1.vector4_u32[0] == V2.vector4_u32[0] &&
            V1.vector4_u32[1] == V2.vector4_u32[1] &&
            V1.vector4_u32[2] == V2.vector4_u32[2] &&
            V1.vector4_u32[3] == V2.vector4_u32[3])
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (V1.vector4_u32[0] != V2.vector4_u32[0] &&
            V1.vector4_u32[1] != V2.vector4_u32[1] &&
            V1.vector4_u32[2] != V2.vector4_u32[2] &&
            V1.vector4_u32[3] != V2.vector4_u32[3])
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        let iTest: i32 = _mm_movemask_ps(_mm_castsi128_ps(vTemp));
        let mut CR: u32 = 0;
        if (iTest == 0xf)     // All equal?
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (iTest == 0)  // All not equal?
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 4D vector is near another 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4NearEqual>
#[inline]
pub fn XMVector4NearEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    Epsilon: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let dx: f32 = fabsf(V1.vector4_f32[0] - V2.vector4_f32[0]);
        let dy: f32 = fabsf(V1.vector4_f32[1] - V2.vector4_f32[1]);
        let dz: f32 = fabsf(V1.vector4_f32[2] - V2.vector4_f32[2]);
        let dw: f32 = fabsf(V1.vector4_f32[3] - V2.vector4_f32[3]);

        return (((dx <= Epsilon.vector4_f32[0]) &&
            (dy <= Epsilon.vector4_f32[1]) &&
            (dz <= Epsilon.vector4_f32[2]) &&
            (dw <= Epsilon.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Get the difference
        let vDelta: XMVECTOR = _mm_sub_ps(V1, V2);
        // Get the absolute value of the difference
        let mut vTemp: XMVECTOR = _mm_setzero_ps();
        vTemp = _mm_sub_ps(vTemp, vDelta);
        vTemp = _mm_max_ps(vTemp, vDelta);
        vTemp = _mm_cmple_ps(vTemp, Epsilon);
        return ((_mm_movemask_ps(vTemp) == 0xf) != false);
    }
}

/// Tests whether two 4D vectors are not equal.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4NotEqual>
#[inline]
pub fn XMVector4NotEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] != V2.vector4_f32[0]) || (V1.vector4_f32[1] != V2.vector4_f32[1]) || (V1.vector4_f32[2] != V2.vector4_f32[2]) || (V1.vector4_f32[3] != V2.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpneq_ps(V1, V2);
        return ((_mm_movemask_ps(vTemp)) != 0);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAnyFalse(XMVector4EqualR(V1, V2));
}

/// Test whether two 4D vectors are not equal, treating each component as an unsigned integer.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4NotEqualInt>
#[inline]
pub fn XMVector4NotEqualInt(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_u32[0] != V2.vector4_u32[0]) || (V1.vector4_u32[1] != V2.vector4_u32[1]) || (V1.vector4_u32[2] != V2.vector4_u32[2]) || (V1.vector4_u32[3] != V2.vector4_u32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: __m128i = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
        return ((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) != 0xF) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAnyFalse(XMVector4EqualIntR(V1, V2));
}

/// Tests whether one 4D vector is greater than another 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Greater>
#[inline]
pub fn XMVector4Greater(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] > V2.vector4_f32[0]) && (V1.vector4_f32[1] > V2.vector4_f32[1]) && (V1.vector4_f32[2] > V2.vector4_f32[2]) && (V1.vector4_f32[3] > V2.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        return ((_mm_movemask_ps(vTemp) == 0x0f) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllTrue(XMVector4GreaterR(V1, V2));
}

/// Tests whether one 4D vector is greater than another 4D vector and returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4GreaterR>
#[inline]
pub fn XMVector4GreaterR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if (V1.vector4_f32[0] > V2.vector4_f32[0] &&
            V1.vector4_f32[1] > V2.vector4_f32[1] &&
            V1.vector4_f32[2] > V2.vector4_f32[2] &&
            V1.vector4_f32[3] > V2.vector4_f32[3])
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (V1.vector4_f32[0] <= V2.vector4_f32[0] &&
            V1.vector4_f32[1] <= V2.vector4_f32[1] &&
            V1.vector4_f32[2] <= V2.vector4_f32[2] &&
            V1.vector4_f32[3] <= V2.vector4_f32[3])
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        let vTemp: XMVECTOR = _mm_cmpgt_ps(V1, V2);
        let iTest: i32 = _mm_movemask_ps(vTemp);
        if (iTest == 0xf)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (!ibool(iTest))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 4D vector is greater-than-or-equal-to another 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4GreaterOrEqual>
#[inline]
pub fn XMVector4GreaterOrEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] >= V2.vector4_f32[0]) && (V1.vector4_f32[1] >= V2.vector4_f32[1]) && (V1.vector4_f32[2] >= V2.vector4_f32[2]) && (V1.vector4_f32[3] >= V2.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        return ((_mm_movemask_ps(vTemp) == 0x0f) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllTrue(XMVector4GreaterOrEqualR(V1, V2));
}


/// Tests whether one 4D vector is greater-than-or-equal-to another 4D vector and returns a comparison value that can be examined using functions such as XMComparisonAllTrue.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4GreaterOrEqualR>
#[inline]
pub fn XMVector4GreaterOrEqualR(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> u32
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        if ((V1.vector4_f32[0] >= V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] >= V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] >= V2.vector4_f32[2]) &&
            (V1.vector4_f32[3] >= V2.vector4_f32[3]))
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if ((V1.vector4_f32[0] < V2.vector4_f32[0]) &&
            (V1.vector4_f32[1] < V2.vector4_f32[1]) &&
            (V1.vector4_f32[2] < V2.vector4_f32[2]) &&
            (V1.vector4_f32[3] < V2.vector4_f32[3]))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut CR: u32 = 0;
        let vTemp: XMVECTOR = _mm_cmpge_ps(V1, V2);
        let iTest: i32 = _mm_movemask_ps(vTemp);
        if (iTest == 0x0f)
        {
            CR = XM_CRMASK_CR6TRUE;
        }
        else if (!ibool(iTest))
        {
            CR = XM_CRMASK_CR6FALSE;
        }
        return CR;
    }
}

/// Tests whether one 4D vector is less than another 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Less>
#[inline]
pub fn XMVector4Less(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] < V2.vector4_f32[0]) && (V1.vector4_f32[1] < V2.vector4_f32[1]) && (V1.vector4_f32[2] < V2.vector4_f32[2]) && (V1.vector4_f32[3] < V2.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmplt_ps(V1, V2);
        return ((_mm_movemask_ps(vTemp) == 0x0f) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllTrue(XMVector4GreaterR(V2, V1));
}

/// Tests whether one 4D vector is less than or equal to another 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4LessOrEqual>
#[inline]
pub fn XMVector4LessOrEqual(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V1.vector4_f32[0] <= V2.vector4_f32[0]) && (V1.vector4_f32[1] <= V2.vector4_f32[1]) && (V1.vector4_f32[2] <= V2.vector4_f32[2]) && (V1.vector4_f32[3] <= V2.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_cmple_ps(V1, V2);
        return ((_mm_movemask_ps(vTemp) == 0x0f) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllTrue(XMVector4GreaterOrEqualR(V2, V1));
}

/// Tests whether the components of a 4D vector are within set bounds.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4InBounds>
#[inline]
pub fn XMVector4InBounds(
    V: FXMVECTOR,
    Bounds: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (((V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) &&
            (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1]) &&
            (V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2]) &&
            (V.vector4_f32[3] <= Bounds.vector4_f32[3] && V.vector4_f32[3] >= -Bounds.vector4_f32[3])) != false);
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test if less than or equal
        let mut vTemp1: XMVECTOR = _mm_cmple_ps(V, Bounds);
        // Negate the bounds
        let mut vTemp2: XMVECTOR = _mm_mul_ps(Bounds, g_XMNegativeOne.v);
        // Test if greater or equal (Reversed)
        vTemp2 = _mm_cmple_ps(vTemp2, V);
        // Blend answers
        vTemp1 = _mm_and_ps(vTemp1, vTemp2);
        // All in bounds?
        return ((_mm_movemask_ps(vTemp1) == 0x0f) != false);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllInBounds(XMVector4InBoundsR(V, Bounds));
}

/// Tests whether any component of a 4D vector is a NaN.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4IsNaN>
#[inline]
pub fn XMVector4IsNaN(
    V: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (XMISNAN!(V.vector4_f32[0]) ||
            XMISNAN!(V.vector4_f32[1]) ||
            XMISNAN!(V.vector4_f32[2]) ||
            XMISNAN!(V.vector4_f32[3]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Test against itself. NaN is always not equal
        let vTempNan: XMVECTOR = _mm_cmpneq_ps(V, V);
        // If any are NaN, the mask is non-zero
        return (_mm_movemask_ps(vTempNan) != 0);
    }

    // NOTE: The source contains a fallback that does not seem reachable
    // return XMComparisonAllInBounds(XMVector4InBoundsR(V, Bounds));
}

/// Tests whether any component of a 4D vector is a NaN.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4IsInfinite>
#[inline]
pub fn XMVector4IsInfinite(
    V: FXMVECTOR,
) -> bool
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        return (XMISINF!(V.vector4_f32[0]) ||
            XMISINF!(V.vector4_f32[1]) ||
            XMISINF!(V.vector4_f32[2]) ||
            XMISINF!(V.vector4_f32[3]));
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // Mask off the sign bit
        let mut vTemp: XMVECTOR = _mm_and_ps(V, g_XMAbsMask.v);
        // Compare to infinity
        vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity.v);
        // If any are infinity, the signs are true.
        return (_mm_movemask_ps(vTemp) != 0);
    }
}


/// Computes the dot product between 4D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Dot>
#[inline]
pub fn XMVector4Dot(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Value = V1.vector4_f32[0] * V2.vector4_f32[0] + V1.vector4_f32[1] * V2.vector4_f32[1] + V1.vector4_f32[2] * V2.vector4_f32[2] + V1.vector4_f32[3] * V2.vector4_f32[3];
        let mut vResult: XMVECTORF32 = mem::MaybeUninit::uninit().assume_init();
        vResult.f[0] = Value;
        vResult.f[1] = Value;
        vResult.f[2] = Value;
        vResult.f[3] = Value;
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        return _mm_dp_ps(V1, V2, 0xff);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vTemp: XMVECTOR = _mm_mul_ps(V1, V2);
        vTemp = _mm_hadd_ps(vTemp, vTemp);
        return _mm_hadd_ps(vTemp, vTemp);
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vTemp2: XMVECTOR = V2;
        let mut vTemp: XMVECTOR = _mm_mul_ps(V1, vTemp2);
        vTemp2 = _mm_shuffle_ps(vTemp2, vTemp, _MM_SHUFFLE(1, 0, 0, 0)); // Copy X to the Z position and Y to the W position
        vTemp2 = _mm_add_ps(vTemp2, vTemp);          // Add Z = X+Z; W = Y+W;
        vTemp = _mm_shuffle_ps(vTemp, vTemp2, _MM_SHUFFLE(0, 3, 0, 0));  // Copy W to the Z position
        vTemp = _mm_add_ps(vTemp, vTemp2);           // Add Z and W together
        return XM_PERMUTE_PS!(vTemp, _MM_SHUFFLE(2, 2, 2, 2));    // Splat Z and return
    }
}


/// Computes the cross product between 4D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Cross>
#[inline]
pub fn XMVector4Cross(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
    V3: FXMVECTOR,
) -> FXMVECTOR
{
    // [ ((v2.z*v3.w-v2.w*v3.z)*v1.y)-((v2.y*v3.w-v2.w*v3.y)*v1.z)+((v2.y*v3.z-v2.z*v3.y)*v1.w),
    //   ((v2.w*v3.z-v2.z*v3.w)*v1.x)-((v2.w*v3.x-v2.x*v3.w)*v1.z)+((v2.z*v3.x-v2.x*v3.z)*v1.w),
    //   ((v2.y*v3.w-v2.w*v3.y)*v1.x)-((v2.x*v3.w-v2.w*v3.x)*v1.y)+((v2.x*v3.y-v2.y*v3.x)*v1.w),
    //   ((v2.z*v3.y-v2.y*v3.z)*v1.x)-((v2.z*v3.x-v2.x*v3.z)*v1.y)+((v2.y*v3.x-v2.x*v3.y)*v1.z) ]

    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let vResult: XMVECTORF32 = XMVECTORF32 {
            f: [
                (((V2.vector4_f32[2] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[2])) * V1.vector4_f32[1]) - (((V2.vector4_f32[1] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[1])) * V1.vector4_f32[2]) + (((V2.vector4_f32[1] * V3.vector4_f32[2]) - (V2.vector4_f32[2] * V3.vector4_f32[1])) * V1.vector4_f32[3]),
                (((V2.vector4_f32[3] * V3.vector4_f32[2]) - (V2.vector4_f32[2] * V3.vector4_f32[3])) * V1.vector4_f32[0]) - (((V2.vector4_f32[3] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[3])) * V1.vector4_f32[2]) + (((V2.vector4_f32[2] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[2])) * V1.vector4_f32[3]),
                (((V2.vector4_f32[1] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[1])) * V1.vector4_f32[0]) - (((V2.vector4_f32[0] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[0])) * V1.vector4_f32[1]) + (((V2.vector4_f32[0] * V3.vector4_f32[1]) - (V2.vector4_f32[1] * V3.vector4_f32[0])) * V1.vector4_f32[3]),
                (((V2.vector4_f32[2] * V3.vector4_f32[1]) - (V2.vector4_f32[1] * V3.vector4_f32[2])) * V1.vector4_f32[0]) - (((V2.vector4_f32[2] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[2])) * V1.vector4_f32[1]) + (((V2.vector4_f32[1] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[1])) * V1.vector4_f32[2]),
            ]
        };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        // V2zwyz * V3wzwy
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V2, _MM_SHUFFLE(2, 1, 3, 2));
        let mut vTemp3: XMVECTOR = XM_PERMUTE_PS!(V3, _MM_SHUFFLE(1, 3, 2, 3));
        vResult = _mm_mul_ps(vResult, vTemp3);
        // - V2wzwy * V3zwyz
        let mut vTemp2: XMVECTOR = XM_PERMUTE_PS!(V2, _MM_SHUFFLE(1, 3, 2, 3));
        vTemp3 = XM_PERMUTE_PS!(vTemp3, _MM_SHUFFLE(1, 3, 0, 1));
        vResult = XM_FNMADD_PS!(vTemp2, vTemp3, vResult);
        // term1 * V1yxxx
        let mut vTemp1: XMVECTOR = XM_PERMUTE_PS!(V1, _MM_SHUFFLE(0, 0, 0, 1));
        vResult = _mm_mul_ps(vResult, vTemp1);

        // V2ywxz * V3wxwx
        vTemp2 = XM_PERMUTE_PS!(V2, _MM_SHUFFLE(2, 0, 3, 1));
        vTemp3 = XM_PERMUTE_PS!(V3, _MM_SHUFFLE(0, 3, 0, 3));
        vTemp3 = _mm_mul_ps(vTemp3, vTemp2);
        // - V2wxwx * V3ywxz
        vTemp2 = XM_PERMUTE_PS!(vTemp2, _MM_SHUFFLE(2, 1, 2, 1));
        vTemp1 = XM_PERMUTE_PS!(V3, _MM_SHUFFLE(2, 0, 3, 1));
        vTemp3 = XM_FNMADD_PS!(vTemp2, vTemp1, vTemp3);
        // vResult - temp * V1zzyy
        vTemp1 = XM_PERMUTE_PS!(V1, _MM_SHUFFLE(1, 1, 2, 2));
        vResult = XM_FNMADD_PS!(vTemp1, vTemp3, vResult);

        // V2yzxy * V3zxyx
        vTemp2 = XM_PERMUTE_PS!(V2, _MM_SHUFFLE(1, 0, 2, 1));
        vTemp3 = XM_PERMUTE_PS!(V3, _MM_SHUFFLE(0, 1, 0, 2));
        vTemp3 = _mm_mul_ps(vTemp3, vTemp2);
        // - V2zxyx * V3yzxy
        vTemp2 = XM_PERMUTE_PS!(vTemp2, _MM_SHUFFLE(2, 0, 2, 1));
        vTemp1 = XM_PERMUTE_PS!(V3, _MM_SHUFFLE(1, 0, 2, 1));
        vTemp3 = XM_FNMADD_PS!(vTemp1, vTemp2, vTemp3);
        // vResult + term * V1wwwz
        vTemp1 = XM_PERMUTE_PS!(V1, _MM_SHUFFLE(2, 3, 3, 3));
        vResult = XM_FMADD_PS!(vTemp3, vTemp1, vResult);
        return vResult;
    }
}

/// Computes the square of the length of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4LengthSq>
#[inline]
pub fn XMVector4LengthSq(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    return XMVector4Dot(V, V);
}

/// Estimates the reciprocal of the length of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4ReciprocalLengthEst>
#[inline]
pub fn XMVector4ReciprocalLengthEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector4LengthSq(V);
        Result = XMVectorReciprocalSqrtEst(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0xff);
        return _mm_rsqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_rsqrt_ps(vLengthSq);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and w
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
        // x+z, y+w
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // x+z,x+z,x+z,y+w
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
        // ??,??,y+w,y+w
        vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
        // ??,??,x+z+y+w,??
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // Splat the length
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
        // Get the reciprocal
        vLengthSq = _mm_rsqrt_ps(vLengthSq);
        return vLengthSq;
    }
}

/// Computes the reciprocal of the length of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4ReciprocalLength>
#[inline]
pub fn XMVector4ReciprocalLength(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector4LengthSq(V);
        Result = XMVectorReciprocalSqrt(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0xff);
        let vLengthSq: XMVECTOR = _mm_sqrt_ps(vTemp);
        return _mm_div_ps(g_XMOne.v, vLengthSq);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        vLengthSq = _mm_div_ps(g_XMOne.v, vLengthSq);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and w
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
        // x+z, y+w
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // x+z,x+z,x+z,y+w
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
        // ??,??,y+w,y+w
        vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
        // ??,??,x+z+y+w,??
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // Splat the length
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
        // Get the reciprocal
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        // Accurate!
        vLengthSq = _mm_div_ps(g_XMOne.v, vLengthSq);
        return vLengthSq;
    }
}

/// Estimates the length of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4LengthEst>
#[inline]
pub fn XMVector4LengthEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector4LengthSq(V);
        Result = XMVectorSqrtEst(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0xff);
        return _mm_sqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and w
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
        // x+z, y+w
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // x+z,x+z,x+z,y+w
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
        // ??,??,y+w,y+w
        vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
        // ??,??,x+z+y+w,??
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // Splat the length
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
        // Get the length
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }
}


/// Computes the length of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Length>
#[inline]
pub fn XMVector4Length(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector4LengthSq(V);
        Result = XMVectorSqrt(Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0xff);
        return _mm_sqrt_ps(vTemp);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and w
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
        // x+z, y+w
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // x+z,x+z,x+z,y+w
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
        // ??,??,y+w,y+w
        vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
        // ??,??,x+z+y+w,??
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // Splat the length
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
        // Get the length
        vLengthSq = _mm_sqrt_ps(vLengthSq);
        return vLengthSq;
    }
}

/// Estimates the normalized version of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4NormalizeEst>
#[inline]
pub fn XMVector4NormalizeEst(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    {
        let mut Result: XMVECTOR;

        Result = XMVector4ReciprocalLength(V);
        Result = XMVectorMultiply(V, Result);

        return Result;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let vTemp: XMVECTOR = _mm_dp_ps(V, V, 0xff);
        let vResult: XMVECTOR = _mm_rsqrt_ps(vTemp);
        return _mm_mul_ps(vResult, V);
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        let mut vDot: XMVECTOR = _mm_mul_ps(V, V);
        vDot = _mm_hadd_ps(vDot, vDot);
        vDot = _mm_hadd_ps(vDot, vDot);
        vDot = _mm_rsqrt_ps(vDot);
        vDot = _mm_mul_ps(vDot, V);
        return vDot;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and w
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
        // x+z, y+w
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // x+z,x+z,x+z,y+w
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
        // ??,??,y+w,y+w
        vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
        // ??,??,x+z+y+w,??
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // Splat the length
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
        // Get the reciprocal
        let mut vResult: XMVECTOR = _mm_rsqrt_ps(vLengthSq);
        // Reciprocal mul to perform the normalization
        vResult = _mm_mul_ps(vResult, V);
        return vResult;
    }
}

/// Computes the normalized version of a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Normalize>
#[inline]
pub fn XMVector4Normalize(
    V: FXMVECTOR,
) -> FXMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let mut fLength: f32;
        let mut vResult: XMVECTOR;

        vResult = XMVector4Length(V);
        fLength = vResult.vector4_f32[0];

        // Prevent divide by zero
        if (fLength > 0.0)
        {
            fLength = 1.0 / fLength;
        }

        vResult.vector4_f32[0] = V.vector4_f32[0] * fLength;
        vResult.vector4_f32[1] = V.vector4_f32[1] * fLength;
        vResult.vector4_f32[2] = V.vector4_f32[2] * fLength;
        vResult.vector4_f32[3] = V.vector4_f32[3] * fLength;
        return vResult;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE4_INTRINSICS_)]
    unsafe {
        let mut vLengthSq: XMVECTOR = _mm_dp_ps(V, V, 0xff);
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Divide to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }

    #[cfg(all(_XM_SSE3_INTRINSICS_, not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Divide to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }

    #[cfg(all(_XM_SSE_INTRINSICS_, not(_XM_SSE3_INTRINSICS_), not(_XM_SSE4_INTRINSICS_)))]
    unsafe {
        // Perform the dot product on x,y,z and w
        let mut vLengthSq: XMVECTOR = _mm_mul_ps(V, V);
        // vTemp has z and w
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
        // x+z, y+w
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // x+z,x+z,x+z,y+w
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
        // ??,??,y+w,y+w
        vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
        // ??,??,x+z+y+w,??
        vLengthSq = _mm_add_ps(vLengthSq, vTemp);
        // Splat the length
        vLengthSq = XM_PERMUTE_PS!(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
        // Prepare for the division
        let mut vResult: XMVECTOR = _mm_sqrt_ps(vLengthSq);
        // Create zero with a single instruction
        let mut vZeroMask: XMVECTOR = _mm_setzero_ps();
        // Test for a divide by zero (Must be FP to detect -0.0)
        vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
        // Failsafe on zero (Or epsilon) length planes
        // If the length is infinity, set the elements to zero
        vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity.v);
        // Divide to perform the normalization
        vResult = _mm_div_ps(V, vResult);
        // Any that are infinity, set to zero
        vResult = _mm_and_ps(vResult, vZeroMask);
        // Select qnan or result based on infinite length
        let vTemp1: XMVECTOR = _mm_andnot_ps(vLengthSq, g_XMQNaN.v);
        let vTemp2: XMVECTOR = _mm_and_ps(vResult, vLengthSq);
        vResult = _mm_or_ps(vTemp1, vTemp2);
        return vResult;
    }
}

/// Clamps the length of a 4D vector to a given range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4ClampLength>
#[inline]
pub fn XMVector4ClampLength(
    V: FXMVECTOR,
    LengthMin: f32,
    LengthMax: f32,
) -> XMVECTOR
{
    let ClampMax: XMVECTOR = XMVectorReplicate(LengthMax);
    let ClampMin: XMVECTOR = XMVectorReplicate(LengthMin);

    return XMVector4ClampLengthV(V, ClampMin, ClampMax);
}


/// Clamps the length of a 4D vector to a given range.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4ClampLengthV>
#[inline]
pub fn XMVector4ClampLengthV(
    V: FXMVECTOR,
    LengthMin: FXMVECTOR,
    LengthMax: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        debug_assert!((XMVectorGetY(LengthMin) == XMVectorGetX(LengthMin)) && (XMVectorGetZ(LengthMin) == XMVectorGetX(LengthMin)) && (XMVectorGetW(LengthMin) == XMVectorGetX(LengthMin)));
        debug_assert!((XMVectorGetY(LengthMax) == XMVectorGetX(LengthMax)) && (XMVectorGetZ(LengthMax) == XMVectorGetX(LengthMax)) && (XMVectorGetW(LengthMax) == XMVectorGetX(LengthMax)));
        debug_assert!(XMVector4GreaterOrEqual(LengthMin, XMVectorZero()));
        debug_assert!(XMVector4GreaterOrEqual(LengthMax, XMVectorZero()));
        debug_assert!(XMVector4GreaterOrEqual(LengthMax, LengthMin));

        let LengthSq: XMVECTOR = XMVector4LengthSq(V);

        const Zero: XMVECTOR = unsafe { g_XMZero.v };

        let RcpLength: XMVECTOR = XMVectorReciprocalSqrt(LengthSq);

        let InfiniteLength: XMVECTOR = XMVectorEqualInt(LengthSq, g_XMInfinity.v);
        let ZeroLength: XMVECTOR = XMVectorEqual(LengthSq, Zero);

        let mut Normal: XMVECTOR = XMVectorMultiply(V, RcpLength);

        let mut Length: XMVECTOR = XMVectorMultiply(LengthSq, RcpLength);

        let Select: XMVECTOR = XMVectorEqualInt(InfiniteLength, ZeroLength);
        Length = XMVectorSelect(LengthSq, Length, Select);
        Normal = XMVectorSelect(LengthSq, Normal, Select);

        let ControlMax: XMVECTOR = XMVectorGreater(Length, LengthMax);
        let ControlMin: XMVECTOR = XMVectorLess(Length, LengthMin);

        let mut ClampLength: XMVECTOR = XMVectorSelect(Length, LengthMax, ControlMax);
        ClampLength = XMVectorSelect(ClampLength, LengthMin, ControlMin);

        let mut Result: XMVECTOR = XMVectorMultiply(Normal, ClampLength);

        // Preserve the original vector (with no precision loss) if the length falls within the given range
        let Control: XMVECTOR = XMVectorEqualInt(ControlMax, ControlMin);
        Result = XMVectorSelect(Result, V, Control);

        return Result;
    }
}

/// Reflects an incident 4D vector across a 4D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Reflect>
#[inline]
pub fn XMVector4Reflect(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
) -> XMVECTOR
{
    // Result = Incident - (2 * dot(Incident, Normal)) * Normal

    let mut Result: XMVECTOR = XMVector4Dot(Incident, Normal);
    Result = XMVectorAdd(Result, Result);
    Result = XMVectorNegativeMultiplySubtract(Result, Normal, Incident);

    return Result;
}

/// Reflects an incident 4D vector across a 4D normal vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Refract>
#[inline]
pub fn XMVector4Refract(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
    RefractionIndex: f32,
) -> XMVECTOR
{
    let Index: XMVECTOR = XMVectorReplicate(RefractionIndex);
    return XMVector4RefractV(Incident, Normal, Index);
}

/// Computes a vector perpendicular to a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4RefractV>
#[inline]
pub fn XMVector4RefractV(
    Incident: FXMVECTOR,
    Normal: FXMVECTOR,
    RefractionIndex: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let IDotN: XMVECTOR;
        let mut R: XMVECTOR;
        const Zero: XMVECTOR = unsafe { g_XMZero.v };

        // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
        // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))

        IDotN = XMVector4Dot(Incident, Normal);

        // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
        R = XMVectorNegativeMultiplySubtract(IDotN, IDotN, g_XMOne.v);
        R = XMVectorMultiply(R, RefractionIndex);
        R = XMVectorNegativeMultiplySubtract(R, RefractionIndex, g_XMOne.v);

        if (XMVector4LessOrEqual(R, Zero))
        {
            // Total internal reflection
            return Zero;
        }
        else
        {
            let mut Result: XMVECTOR;

            // R = RefractionIndex * IDotN + sqrt(R)
            R = XMVectorSqrt(R);
            R = XMVectorMultiplyAdd(RefractionIndex, IDotN, R);

            // Result = RefractionIndex * Incident - Normal * R
            Result = XMVectorMultiply(RefractionIndex, Incident);
            Result = XMVectorNegativeMultiplySubtract(Normal, R, Result);

            return Result;
        }
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let IDotN: XMVECTOR = XMVector4Dot(Incident, Normal);

        // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
        let mut R: XMVECTOR = XM_FNMADD_PS!(IDotN, IDotN, g_XMOne.v);
        let R2: XMVECTOR = _mm_mul_ps(RefractionIndex, RefractionIndex);
        R = XM_FNMADD_PS!(R, R2, g_XMOne.v);

        let mut vResult: XMVECTOR = _mm_cmple_ps(R, g_XMZero.v);
        if (_mm_movemask_ps(vResult) == 0x0f)
        {
            // Total internal reflection
            vResult = g_XMZero.v;
        }
        else
        {
            // R = RefractionIndex * IDotN + sqrt(R)
            R = _mm_sqrt_ps(R);
            R = XM_FMADD_PS!(RefractionIndex, IDotN, R);
            // Result = RefractionIndex * Incident - Normal * R
            vResult = _mm_mul_ps(RefractionIndex, Incident);
            vResult = XM_FNMADD_PS!(R, Normal, vResult);
        }
        return vResult;
    }
}


/// Computes a vector perpendicular to a 4D vector.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Orthogonal>
#[inline]
pub fn XMVector4Orthogonal(
    V: FXMVECTOR,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let Result = XMVECTORF32 { f: [
            V.vector4_f32[2],
            V.vector4_f32[3],
            -V.vector4_f32[0],
            -V.vector4_f32[1]
        ]};
        return Result.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        const FlipZW: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0, -1.0, -1.0 ] };
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 0, 3, 2));
        vResult = _mm_mul_ps(vResult, FlipZW.v);
        return vResult;
    }
}

/// Estimates the radian angle between two normalized 4D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4AngleBetweenNormalsEst>
#[inline]
pub fn XMVector4AngleBetweenNormalsEst(
    N1: FXMVECTOR,
    N2: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let mut Result: XMVECTOR = XMVector4Dot(N1, N2);
        Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
        Result = XMVectorACosEst(Result);
        return Result;
    }
}

/// Computes the radian angle between two normalized 4D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4AngleBetweenNormals>
#[inline]
pub fn XMVector4AngleBetweenNormals(
    N1: FXMVECTOR,
    N2: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let mut Result: XMVECTOR = XMVector4Dot(N1, N2);
        Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
        Result = XMVectorACos(Result);
        return Result;
    }
}

/// Compute the radian angle between two 4D vectors.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4AngleBetweenVectors>
#[inline]
pub fn XMVector4AngleBetweenVectors(
    V1: FXMVECTOR,
    V2: FXMVECTOR,
) -> XMVECTOR
{
    unsafe {
        let mut L1: XMVECTOR = XMVector4ReciprocalLength(V1);
        let L2: XMVECTOR = XMVector4ReciprocalLength(V2);

        let Dot: XMVECTOR = XMVector4Dot(V1, V2);

        L1 = XMVectorMultiply(L1, L2);

        let mut CosAngle: XMVECTOR = XMVectorMultiply(Dot, L1);
        CosAngle = XMVectorClamp(CosAngle, g_XMNegativeOne.v, g_XMOne.v);

        return XMVectorACos(CosAngle);
    }
}

/// Transforms a 4D vector by a matrix.
///
/// <https://docs.microsoft.com/en-us/windows/win32/api/directxmath/nf-directxmath-XMVector4Transform>
#[inline]
pub fn XMVector4Transform(
    V: FXMVECTOR,
    M: XMMATRIX,
) -> XMVECTOR
{
    #[cfg(_XM_NO_INTRINSICS_)]
    unsafe {
        let fX: f32 = (M.m[0][0] * V.vector4_f32[0]) + (M.m[1][0] * V.vector4_f32[1]) + (M.m[2][0] * V.vector4_f32[2]) + (M.m[3][0] * V.vector4_f32[3]);
        let fY: f32 = (M.m[0][1] * V.vector4_f32[0]) + (M.m[1][1] * V.vector4_f32[1]) + (M.m[2][1] * V.vector4_f32[2]) + (M.m[3][1] * V.vector4_f32[3]);
        let fZ: f32 = (M.m[0][2] * V.vector4_f32[0]) + (M.m[1][2] * V.vector4_f32[1]) + (M.m[2][2] * V.vector4_f32[2]) + (M.m[3][2] * V.vector4_f32[3]);
        let fW: f32 = (M.m[0][3] * V.vector4_f32[0]) + (M.m[1][3] * V.vector4_f32[1]) + (M.m[2][3] * V.vector4_f32[2]) + (M.m[3][3] * V.vector4_f32[3]);
        let vResult = XMVECTORF32 { f: [ fX, fY, fZ, fW ] };
        return vResult.v;
    }

    #[cfg(_XM_ARM_NEON_INTRINSICS_)]
    {
        unimplemented!()
    }

    #[cfg(_XM_SSE_INTRINSICS_)]
    unsafe {
        let mut vResult: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(3, 3, 3, 3)); // W
        vResult = _mm_mul_ps(vResult, M.r[3]);
        let mut vTemp: XMVECTOR = XM_PERMUTE_PS!(V, _MM_SHUFFLE(2, 2, 2, 2)); // Z
        vResult = XM_FMADD_PS!(vTemp, M.r[2], vResult);
        vTemp = XM_PERMUTE_PS!(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
        vResult = XM_FMADD_PS!(vTemp, M.r[1], vResult);
        vTemp = XM_PERMUTE_PS!(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
        vResult = XM_FMADD_PS!(vTemp, M.r[0], vResult);
        return vResult;
    }
}

// TODO: XMVector4TransformStream

impl From<&[f32; 4]> for XMVector {
    #[inline]
    fn from(v: &[f32; 4]) -> XMVector {
        XMVector(XMLoadFloat4(v.into()))
    }
}

impl std::ops::Deref for XMVector {
    type Target = XMVECTOR;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for XMVector {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl XMVector {
    #[inline(always)]
    pub fn set(x: f32, y: f32, z: f32, w: f32) -> XMVector {
        XMVector(XMVectorSet(x, y, z, w))
    }
}

impl std::ops::Add for XMVector {
    type Output = XMVector;
    #[inline]
    fn add(self, V2: XMVector) -> Self::Output {
        XMVector(XMVectorAdd(self.0, V2.0))
    }
}

impl std::ops::AddAssign for XMVector {
    #[inline]
    fn add_assign(&mut self, V2: XMVector) {
        self.0 = XMVectorAdd(self.0, V2.0);
    }
}

impl std::ops::Sub for XMVector {
    type Output = XMVector;
    #[inline]
    fn sub(self, V2: XMVector) -> Self::Output {
        XMVector(XMVectorSubtract(self.0, V2.0))
    }
}

impl std::ops::SubAssign for XMVector {
    #[inline]
    fn sub_assign(&mut self, V2: XMVector) {
        self.0 = XMVectorSubtract(self.0, V2.0);
    }
}

impl std::ops::Mul for XMVector {
    type Output = XMVector;
    #[inline]
    fn mul(self, V2: XMVector) -> Self::Output {
        XMVector(XMVectorMultiply(self.0, V2.0))
    }
}

impl std::ops::MulAssign for XMVector {
    #[inline]
    fn mul_assign(&mut self, V2: XMVector) {
        self.0 = XMVectorMultiply(self.0, V2.0);
    }
}

impl std::ops::Div for XMVector {
    type Output = XMVector;
    #[inline]
    fn div(self, V2: XMVector) -> Self::Output {
        XMVector(XMVectorDivide(self.0, V2.0))
    }
}

impl std::ops::DivAssign for XMVector {
    #[inline]
    fn div_assign(&mut self, V2: XMVector) {
        self.0 = XMVectorDivide(self.0, V2.0);
    }
}

impl std::ops::Mul<XMVector> for f32 {
    type Output = XMVector;
    #[inline]
    fn mul(self, V: XMVector) -> Self::Output {
        let S = self;
        XMVector(XMVectorScale(V.0, S))
    }
}

impl std::ops::Mul<f32> for XMVector {
    type Output = XMVector;
    #[inline]
    fn mul(self, S: f32) -> Self::Output {
        XMVector(XMVectorScale(self.0, S))
    }
}

impl std::ops::MulAssign<f32> for XMVector {
    #[inline]
    fn mul_assign(&mut self, S: f32) {
        self.0 = XMVectorScale(self.0, S);
    }
}

impl std::ops::Div<f32> for XMVector {
    type Output = XMVector;
    #[inline]
    fn div(self, S: f32) -> Self::Output {
        let vS = XMVectorReplicate(S);
        XMVector(XMVectorDivide(self.0, vS))
    }
}

impl std::ops::DivAssign<f32> for XMVector {
    #[inline]
    fn div_assign(&mut self, S: f32) {
        let vS = XMVectorReplicate(S);
        self.0 = XMVectorDivide(self.0, vS);
    }
}

impl std::ops::Neg for XMVector {
    type Output = XMVector;
    #[inline]
    fn neg(self) -> Self::Output {
        XMVector(XMVectorNegate(*self))
    }
}

impl std::cmp::PartialEq for XMVector {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        XMVector4NearEqual(self.0, rhs.0, unsafe { g_XMEpsilon.v })
    }
}

impl std::fmt::Debug for XMVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list()
            .entry(&XMVectorGetX(self.0))
            .entry(&XMVectorGetY(self.0))
            .entry(&XMVectorGetZ(self.0))
            .entry(&XMVectorGetW(self.0))
            .finish()
    }
}


#[test]
fn test_debug() {
    #[rustfmt::skip]
    let m = XMVector::from(&[1.0, 2.0, 3.0, 4.0]);
    let s = format!("{:?}", m);
    assert_eq!("[1.0, 2.0, 3.0, 4.0]", s);
}