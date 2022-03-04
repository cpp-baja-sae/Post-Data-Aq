use std::f32::consts::PI;

use num::{complex::Complex32, Complex};

// Returns a real number as a Complex32.
fn c(real: f32) -> Complex32 {
    Complex::new(real, 0.0)
}

// Rust port of https://rdrr.io/cran/signal/src/R/cheby1.R
// Usage: (b, a) = cheby1(order, stopband_ripple, passband_edge_frequency);
// E.G. (b,a) = cheby1(6, 10.0, 0.6);
fn cheby1(n: i32, rp: f32, w: f32) -> (Vec<f32>, Vec<f32>) {
    assert!(n > 0);
    assert!(rp >= 0.0);
    assert!(w >= 0.0 && w <= 1.0);
    let digital = true;
    let (t, w) = if digital {
        (2.0, 2.0 / 2.0 * (PI * w / 2.0).tan())
    } else {
        unreachable!()
    };

    let epsilon = (10.0f32.powf(rp / 10.0) - 1.0).sqrt();
    let v0 = (1.0 / epsilon).asinh() / n as f32;
    let pole: Vec<_> = (-(n - 1)..=(n - 1))
        .step_by(2)
        .map(|x| PI * x as f32 / (2 * n) as f32)
        .map(|x| Complex::new(0.0, x).exp())
        .map(|x| Complex::new(-v0.sinh() * x.re, v0.cosh() * x.im))
        .collect();
    let zero = vec![c(0.0)];

    let gain = pole.iter().copied().map(|x| -x).product();
    let gain = if n % 2 == 0 {
        gain / 10.0f32.powf(rp / 20.0)
    } else {
        gain
    };

    // S-plane frequency transform
    let stop = false;
    let zpg = sftrans((zero, pole, gain), w, stop);

    let zpg = if digital { bilinear(zpg, t) } else { zpg };

    println!("{:?}", zpg.0);
    println!("{:?}", zpg.1);
    println!("{}", zpg.2);

    arma(zpg)
}

// Port of https://rdrr.io/cran/signal/src/R/poly.R
fn poly(roots: &[Complex32]) -> Vec<Complex32> {
    let mut out = vec![c(0.0); roots.len() + 1];
    out[0] = Complex::new(1.0, 0.0);
    for j in 0..roots.len() {
        for k in (0..=j).rev() {
            let out_k = out[k];
            out[k + 1] -= roots[j] * out_k;
        }
    }
    out
}

// Port of https://rdrr.io/cran/signal/src/R/filter.R#sym-as.Arma
fn arma(zpg: Zpg) -> (Vec<f32>, Vec<f32>) {
    let b = poly(&zpg.0).into_iter().map(|x| (zpg.2 * x).re).collect();
    let a = poly(&zpg.1).into_iter().map(|x| x.re).collect();
    (b, a)
}

// Port of https://rdrr.io/cran/signal/src/R/bilinear.R
fn bilinear((zero, pole, gain): Zpg, t: f32) -> Zpg {
    let zlen = zero.len();
    let plen = pole.len();
    assert!(zlen <= plen);
    let num: Complex32 = zero.iter().copied().map(|x| (2.0 - x * t) / t).product();
    let div: Complex32 = pole.iter().copied().map(|x| (2.0 - x * t) / t).product();
    let gain = Complex::new((gain * num / div).re, 0.0);
    let pole = pole
        .into_iter()
        .map(|x| (2.0 + x * t) / (2.0 - x * t))
        .collect();
    let mut zero: Vec<Complex32> = vec![];
    for _ in 0..plen {
        zero.push(Complex::new(-1.0, 0.0));
    }
    (zero, pole, gain)
}

type Zpg = (Vec<Complex32>, Vec<Complex32>, Complex32);

// Port of https://rdrr.io/cran/signal/src/R/sftrans.R
fn sftrans((mut zero, mut pole, mut gain): Zpg, w: f32, stop: bool) -> Zpg {
    let c = 1.0;
    let fc = w;
    let zlen = zero.len();
    let plen = pole.len();
    if stop {
        unimplemented!()
    } else {
        gain *= (c / fc).powf(zlen as f32 - plen as f32 - 1.0);
        pole = pole.into_iter().map(|x| fc * x / c).collect();
        zero = zero.into_iter().map(|x| fc * x / c).collect();
    }
    (zero, pole, gain)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cheby1() {
        let (b, a) = cheby1(6, 10.0, 0.6);
        assert_float_vec_apeq(
            &b,
            &[
                0.0095721, 0.0574327, 0.1435817, 0.1914422, 0.1435817, 0.0574327, 0.0095721,
            ],
        );

        assert_float_vec_apeq(
            &a,
            &[
                1.00000, -0.85368, 1.93219, -1.66551, 1.68772, -0.83727, 0.67380,
            ],
        )
    }

    #[test]
    fn test_poly() {
        assert_eq!(
            poly(&[c(1.0), c(2.0), c(3.0), c(4.0)]),
            vec![c(1.0), c(-10.0), c(35.0), c(-50.0), c(24.0)]
        )
    }

    fn assert_complex_apeq(a: Complex32, b: Complex32) {
        let range = a.re.abs().max(b.re.abs()).max(a.im.abs()).max(b.im.abs());
        let range = range * 1e-3;
        if a.re < b.re - range || a.re > b.re + range || a.im < b.im - range || a.im > b.im + range
        {
            panic!("{} != {}", a, b);
        }
    }

    fn assert_complex_vec_apeq(a: &[Complex32], b: &[Complex32]) {
        if a.len() != b.len() {
            panic!("length {} != length {}", a.len(), b.len());
        }
        for (&a, &b) in a.iter().zip(b.iter()) {
            assert_complex_apeq(a, b);
        }
    }

    fn assert_float_apeq(a: f32, b: f32) {
        let range = a.abs().max(b.abs());
        let range = range * 1e-3;
        if a < b - range || a > b + range {
            panic!("{} != {}", a, b);
        }
    }

    fn assert_float_vec_apeq(a: &[f32], b: &[f32]) {
        if a.len() != b.len() {
            panic!("length {} != length {}", a.len(), b.len());
        }
        for (&a, &b) in a.iter().zip(b.iter()) {
            assert_float_apeq(a, b);
        }
    }

    // #[test]
    // fn test_bilinear() {
    //     let (z, p, g) = (vec![c(0.0)], vec![c(1.0), c(2.0), c(3.0)], c(4.0));
    //     let (z, p, g) = bilinear((z, p, g), 5.0);
    //     assert_complex_vec_apeq(&z, &[c(1.0), c(-1.0), c(-1.0)]);
    //     assert_complex_vec_apeq(&p, &[c(-2.333333), c(-1.500000), c(-1.307692)]);
    //     assert_complex_apeq(g, c(-0.6410256));
    // }

    #[test]
    fn test_arma() {
        let (z, p, g) = (
            vec![c(-1.0), c(-2.0), c(-3.0)],
            vec![c(1.0), c(2.0), c(3.0)],
            c(4.0),
        );
        let (b, a) = arma((z, p, g));
        assert_float_vec_apeq(&b, &[4.0, 24.0, 44.0, 24.0]);
        assert_float_vec_apeq(&a, &[1.0, -6.0, 11.0, -6.0]);
    }

    // #[test]
    // fn test_sftrans() {
    //     let (z, p, g) = (vec![c(0.0)], vec![c(1.0), c(2.0), c(3.0)], c(4.0));
    //     let (z, p, g) = sftrans((z, p, g), 3.14, false);
    //     assert_complex_vec_apeq(&z, &[c(0.0)]);
    //     assert_complex_vec_apeq(&p, &[c(3.14), c(6.28), c(9.42)]);
    //     assert_complex_apeq(g, c(39.4384));
    // }
}
