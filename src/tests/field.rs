use ark_std::{end_timer, start_timer};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

#[cfg(feature = "bits")]
pub fn random_bits_tests<F: ff::PrimeFieldBits>(type_name: String) {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    let _message = format!("to_le_bits {type_name}");
    let start = start_timer!(|| _message);
    for _ in 0..1000000 {
        let a = F::random(&mut rng);
        let bytes = a.to_repr();
        let bits = a.to_le_bits();
        for idx in 0..bits.len() {
            assert_eq!(bits[idx], ((bytes.as_ref()[idx / 8] >> (idx % 8)) & 1) == 1);
        }
    }
    end_timer!(start);
}

#[macro_export]
macro_rules! field_testing_suite {
    ($field: ident, "field") => {
        macro_rules! random_multiplication_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("multiplication {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let a = $f::random(&mut $rng);
                    let b = $f::random(&mut $rng);
                    let c = $f::random(&mut $rng);

                    let mut t0 = a; // (a * b) * c
                    t0.mul_assign(&b);
                    t0.mul_assign(&c);

                    let mut t1 = a; // (a * c) * b
                    t1.mul_assign(&c);
                    t1.mul_assign(&b);

                    let mut t2 = b; // (b * c) * a
                    t2.mul_assign(&c);
                    t2.mul_assign(&a);

                    assert_eq!(t0, t1);
                    assert_eq!(t1, t2);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_addition_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("addition {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let a = $f::random(&mut $rng);
                    let b = $f::random(&mut $rng);
                    let c = $f::random(&mut $rng);

                    let mut t0 = a; // (a + b) + c
                    t0.add_assign(&b);
                    t0.add_assign(&c);

                    let mut t1 = a; // (a + c) + b
                    t1.add_assign(&c);
                    t1.add_assign(&b);

                    let mut t2 = b; // (b + c) + a
                    t2.add_assign(&c);
                    t2.add_assign(&a);

                    assert_eq!(t0, t1);
                    assert_eq!(t1, t2);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_subtraction_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("subtraction {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let a = $f::random(&mut $rng);
                    let b = $f::random(&mut $rng);

                    let mut t0 = a; // (a - b)
                    t0.sub_assign(&b);

                    let mut t1 = b; // (b - a)
                    t1.sub_assign(&a);

                    let mut t2 = t0; // (a - b) + (b - a) = 0
                    t2.add_assign(&t1);

                    assert_eq!(t2.is_zero().unwrap_u8(), 1);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_negation_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("negation {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let a = $f::random(&mut $rng);
                    let mut b = a;
                    b = b.neg();
                    b.add_assign(&a);

                    assert_eq!(b.is_zero().unwrap_u8(), 1);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_doubling_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("doubling {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let mut a = $f::random(&mut $rng);
                    let mut b = a;
                    a.add_assign(&b);
                    b = b.double();

                    assert_eq!(a, b);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_squaring_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("squaring {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let mut a = $f::random(&mut $rng);
                    let mut b = a;
                    a.mul_assign(&b);
                    b = b.square();

                    assert_eq!(a, b);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_inversion_tests {
            ($f: ident, $rng: expr) => {
                assert!(bool::from($f::ZERO.invert().is_none()));

                let _message = format!("inversion {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let mut a = $f::random(&mut $rng);
                    let b = a.invert().unwrap(); // probablistically nonzero
                    a.mul_assign(&b);

                    assert_eq!(a, $f::ONE);
                }
                end_timer!(start);
            };
        }

        macro_rules! random_expansion_tests {
            ($f: ident, $rng: expr) => {
                let _message = format!("expansion {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    // Compare (a + b)(c + d) and (a*c + b*c + a*d + b*d)

                    let a = $f::random(&mut $rng);
                    let b = $f::random(&mut $rng);
                    let c = $f::random(&mut $rng);
                    let d = $f::random(&mut $rng);

                    let mut t0 = a;
                    t0.add_assign(&b);
                    let mut t1 = c;
                    t1.add_assign(&d);
                    t0.mul_assign(&t1);

                    let mut t2 = a;
                    t2.mul_assign(&c);
                    let mut t3 = b;
                    t3.mul_assign(&c);
                    let mut t4 = a;
                    t4.mul_assign(&d);
                    let mut t5 = b;
                    t5.mul_assign(&d);

                    t2.add_assign(&t3);
                    t2.add_assign(&t4);
                    t2.add_assign(&t5);

                    assert_eq!(t0, t2);
                }
                end_timer!(start);
            };
        }

        macro_rules! zero_tests {
            ($f: ident, $rng: expr) => {
                assert_eq!($f::ZERO.is_zero().unwrap_u8(), 1);
                {
                    let mut z = $f::ZERO;
                    z = z.neg();
                    assert_eq!(z.is_zero().unwrap_u8(), 1);
                }

                assert!(bool::from($f::ZERO.invert().is_none()));

                // Multiplication by zero
                {
                    let mut a = $f::random(&mut $rng);
                    a.mul_assign(&$f::ZERO);
                    assert_eq!(a.is_zero().unwrap_u8(), 1);
                }

                // Addition by zero
                {
                    let mut a = $f::random(&mut $rng);
                    let copy = a;
                    a.add_assign(&$f::ZERO);
                    assert_eq!(a, copy);
                }
            };
        }

        macro_rules! one_tests {
            ($f: ident, $rng: expr) => {
                assert!(bool::from($f::ONE.invert().is_some()));

                // Multiplication by one
                {
                    let mut a = $f::random(&mut $rng);
                    let copy = a;
                    a.mul_assign(&$f::ONE);
                    assert_eq!(a, copy);
                }

                // Addition by one
                {
                    let mut a = $f::random(&mut $rng);
                    let copy = a;
                    a.add_assign(&$f::ONE);
                    assert_eq!(a, copy + $f::ONE);
                }
            };
        }

        use ark_std::{end_timer, start_timer};
        use ff::Field;
        use rand::SeedableRng;
        use rand_xorshift::XorShiftRng;
        use std::ops::*;

        #[test]
        fn test_field() {
            let mut rng = XorShiftRng::from_seed([
                0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
                0xbc, 0xe5,
            ]);

            // normal cases
            random_multiplication_tests!($field, rng);
            random_addition_tests!($field, rng);
            random_subtraction_tests!($field, rng);
            random_negation_tests!($field, rng);
            random_doubling_tests!($field, rng);
            random_squaring_tests!($field, rng);
            random_inversion_tests!($field, rng);
            random_expansion_tests!($field, rng);

            // edge cases
            zero_tests!($field, rng);
            one_tests!($field, rng);
        }
    };

    ($field: ident, "conversion") => {
        macro_rules! random_conversion_tests {
            ($f: ident) => {
                let mut rng = XorShiftRng::from_seed([
                    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54,
                    0x06, 0xbc, 0xe5,
                ]);
                let _message = format!("conversion {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let a = $f::random(&mut rng);
                    let bytes = a.to_repr();
                    let b = $f::from_repr(bytes).unwrap();
                    assert_eq!(a, b);
                }
                end_timer!(start);
            };
        }

        #[test]
        fn test_conversion() {
            random_conversion_tests!($field);
        }
    };

    ($field: ident, "serialization") => {
        macro_rules! random_serialization_test {
            ($f: ident) => {
                let mut rng = XorShiftRng::from_seed([
                    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54,
                    0x06, 0xbc, 0xe5,
                ]);
                let _message = format!("serialization with SerdeObject {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    let a = $f::random(&mut rng);
                    let bytes = a.to_raw_bytes();
                    let b = $f::from_raw_bytes(&bytes).unwrap();
                    assert_eq!(a, b);
                    let mut buf = Vec::new();
                    a.write_raw(&mut buf).unwrap();
                    let b = $f::read_raw(&mut &buf[..]).unwrap();
                    assert_eq!(a, b);
                }
                end_timer!(start);
            };
        }

        #[cfg(feature = "derive_serde")]
        macro_rules! random_serde_test {
            ($f: ident) => {
                let mut rng = XorShiftRng::from_seed([
                    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54,
                    0x06, 0xbc, 0xe5,
                ]);
                let _message = format!("serialization with serde {}", stringify!($f));
                let start = start_timer!(|| _message);
                for _ in 0..1000000 {
                    // byte serialization
                    let a = $f::random(&mut rng);
                    let bytes = bincode::serialize(&a).unwrap();
                    let reader = std::io::Cursor::new(bytes);
                    let b: $f = bincode::deserialize_from(reader).unwrap();
                    assert_eq!(a, b);

                    // json serialization
                    let json = serde_json::to_string(&a).unwrap();
                    let reader = std::io::Cursor::new(json);
                    let b: $f = serde_json::from_reader(reader).unwrap();
                    assert_eq!(a, b);
                }
                end_timer!(start);
            };
        }

        #[test]
        fn test_serialization() {
            use crate::serde::SerdeObject;
            random_serialization_test!($field);
            #[cfg(feature = "derive_serde")]
            random_serde_test!($field);
        }
    };

    ($field: ident, "quadratic_residue") => {
        macro_rules! random_quadratic_residue_test {
            ($f: ident) => {
                let mut rng = XorShiftRng::from_seed([
                    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54,
                    0x06, 0xbc, 0xe5,
                ]);
                for _ in 0..100000 {
                    let elem = $f::random(&mut rng);
                    let is_quad_res_or_zero: bool = elem.sqrt().is_some().into();
                    let is_quad_non_res: bool = elem.ct_quadratic_non_residue().into();
                    assert_eq!(!is_quad_non_res, is_quad_res_or_zero)
                }
            };
        }

        #[test]
        fn test_quadratic_residue() {
            use crate::ff_ext::Legendre;
            use ff::Field;
            use rand_core::SeedableRng;
            use rand_xorshift::XorShiftRng;
            random_quadratic_residue_test!($field);
        }
    };
}
