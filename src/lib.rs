#[inline(always)]
pub fn fpu_rsqrt(x: f32) -> f32 {
    x.sqrt().recip()
}

#[inline(always)]
pub fn quake3_rsqrt(x: f32) -> f32 {
    let x2: f32 = x * 0.5;
    let mut y: f32 = x;

    let mut i: i32 = y.to_bits() as i32;
    i = 0x5f3759df - (i >> 1);
    y = f32::from_bits(i as u32);

    y = y * (1.5 - (x2 * y * y));
    y
}

#[inline(always)]
pub fn see_rsqrt(x: f32) -> f32 {
    use core::arch::x86_64::*;
    unsafe {
        let y = _mm_rsqrt_ss(_mm_set1_ps(x));
        *(&y as *const _ as *const f32)
    }
}

#[inline(always)]
pub fn see_rsqrt_nr1(x: f32) -> f32 {
    let y = see_rsqrt(x);
    //taken from http://stackoverflow.com/q/14752399/556899
    (0.5 * y) * (3.0 - (x * y * y))
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[repr(align(16))]
    struct Wrapper(f32);

    #[test]
    fn aprox() {
        let mut rng = rand::thread_rng();
        let mut v = vec![];
        for _ in 0..1000 {
            v.push(Wrapper(rng.gen()));
        }

        let mut rb = 0.0;
        let mut rc = 0.0;
        let mut rd = 0.0;

        for x in v {
            let a = fpu_rsqrt(x.0);

            let b = quake3_rsqrt(x.0);
            let c = see_rsqrt(x.0);
            let d = see_rsqrt_nr1(x.0);

            rb = ((a - b) / a).abs().max(rb);
            rc = ((a - c) / a).abs().max(rc);
            rd = ((a - d) / a).abs().max(rd);
        }

        rb *= 100.0;
        rc *= 100.0;
        rd *= 100.0;

        panic!("{}%, {}%, {}%", rb, rc, rd);
    }
}
