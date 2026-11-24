#[test]
fn scalar_certificate_vtk_02() {
    super::matrix_product_tests::certified_model(
        "verdant_scalar_f32",
        "VerdantEngine.ScalarF32.evaluate",
    );
}

#[test]
fn ggml_scalar_operations_vtk_02() {
    use std::fmt::Write as _;
    let edges = [
        0u32, 0x80000000, 0x3F800000, 0xBF800000, 0x7F800000, 0xFF800000, 0x7FC00001, 1,
        0x7F7FFFFF, 0x00800000, 0x007FFFFF, 0x3F800001, 0x3F000000, 0x00800001, 0x80000001,
        0x7F800001, 0x40000000, 0x40800000, 0x3F7FFFFF, 0x3EFFFFFF, 0x7F000001, 0xFF7FFFFF,
        0xFFC00001, 0xFF800001,
    ];
    let mut requests = String::new();
    let mut inputs = Vec::new();
    let mut actual = Vec::new();
    let mut record = |operation: u64, left: u32, right: u32| {
        writeln!(requests, "{operation} {left} {right}").unwrap();
        inputs.push((operation, left, right));
        actual.push(
            super::evaluate_f32_scalar(operation, f32::from_bits(left), f32::from_bits(right))
                .unwrap()
                .to_bits(),
        );
    };
    let mut state = 0x12345678u32;
    for operation in 0..8 {
        for left in edges {
            if operation < 4 {
                for right in edges {
                    record(operation, left, right);
                }
            } else {
                record(operation, left, 0);
            }
        }
        for _ in 0..1024 {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            let left = state;
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            record(operation, left, state);
        }
    }
    assert_eq!(actual.len(), 10592, "the full scalar corpus must execute");
    let reference = super::matrix_product_tests::query_oracle("scalar", requests);
    assert_eq!(reference.len(), actual.len());
    for ((expected, actual), (operation, left, right)) in
        reference.into_iter().zip(actual).zip(inputs)
    {
        if f32::from_bits(expected).is_nan() {
            assert!(
                f32::from_bits(actual).is_nan(),
                "NaN operation={operation} left={left:08x} right={right:08x} actual={actual:08x}"
            );
        } else {
            assert_eq!(
                actual, expected,
                "operation={operation} left={left:08x} right={right:08x}"
            );
        }
    }
}

#[test]
fn storage_certificate_vtk_03() {
    super::matrix_product_tests::certified_model(
        "verdant_storage_transfer",
        "VerdantEngine.StorageTransfer.transferLength",
    );
}

#[test]
fn storage_conversion_vtk_04() {
    fn check(cases: &[(u64, u64, u32)]) -> Vec<u32> {
        use std::fmt::Write as _;
        let mut requests = String::new();
        let actual: Vec<_> = cases
            .iter()
            .map(|&(source, destination, bits)| {
                writeln!(requests, "{source} {destination} {bits}").unwrap();
                u32::try_from(
                    super::convert_float_storage(source, destination, u64::from(bits))
                        .unwrap()
                        .unwrap(),
                )
                .unwrap()
            })
            .collect();
        let reference = super::matrix_product_tests::query_oracle("conversion", requests);
        assert_eq!(
            reference.len(),
            cases.len(),
            "conversion oracle omitted records"
        );
        for ((case, actual), expected) in cases.iter().zip(&actual).zip(reference) {
            assert_eq!(
                *actual, expected,
                "conversion source/destination/bits={case:?}"
            );
        }
        actual
    }
    let mut cases = Vec::new();
    for word in 0..65536_u32 {
        for source in [1, 30] {
            for destination in [0, 1, 30] {
                cases.push((source, destination, word));
            }
        }
    }
    assert_eq!(cases.len(), 393216);
    let wide = check(&cases);
    cases.clear();
    for word in 0..65536_u32 {
        for kind in 0..2 {
            let bits = wide[word as usize * 6 + kind * 3];
            let midpoint = if kind == 0 && word % 32768 == 0 {
                (bits & 0x80000000) | 0x33000000
            } else {
                bits.wrapping_add(if kind == 0 {
                    if word % 32768 < 1024 {
                        1 << (125 - ((bits >> 23) & 255))
                    } else {
                        4096
                    }
                } else {
                    32768
                })
            };
            for input in [bits, midpoint - 1, midpoint, midpoint + 1] {
                for destination in [1, 30] {
                    cases.push((0, destination, input));
                }
            }
        }
    }
    assert_eq!(cases.len(), 1048576);
    cases.sort_unstable();
    cases.dedup();
    assert_eq!(cases.len(), 1004044);
    check(&cases);
}

#[test]
fn storage_conversion_refusals_vtk_04() {
    for &(source, destination, bits) in &[
        (2, 0, 0),
        (0, 2, 0),
        (1, 0, 65536),
        (30, 0, 65536),
        (0, 0, 4294967296),
    ] {
        assert_eq!(
            super::convert_float_storage(source, destination, bits).unwrap(),
            None
        );
    }
    assert_eq!(
        super::convert_float_storage(0, 0, 0xffffffff).unwrap(),
        Some(0xffffffff)
    );
}

#[test]
fn conversion_certificate_vtk_04() {
    super::matrix_product_tests::certified_model(
        "verdant_float_storage",
        "VerdantEngine.FloatStorage.convert",
    );
}

#[test]
fn float_transform_vtk_05() {
    use std::fmt::Write as _;
    let words = [
        0, 0x80000000, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc00001, 0x7f800001,
        0xff800001, 1, 0x80000001, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x3f000000, 0xbf000000,
        0x3f800001, 0x3f7fffff,
    ];
    let half_words = [
        0, 0x8000, 0x3c00, 0xbc00, 0x7c00, 0xfc00, 0x7e01, 0x7c01, 0x7f80, 0xff80, 0x7fc1, 0x7f81,
        1, 0x8001, 0x0400, 0x7bff, 0x0080, 0x7f7f,
    ];
    let mut cases = Vec::new();
    let mut requests = String::new();
    let mut actual = Vec::new();
    for storage in [0, 1, 30] {
        let inputs = if storage == 0 { &words } else { &half_words };
        for operation in 0..3 {
            for &a in &words {
                for &b in if operation == 2 {
                    &words[..1]
                } else {
                    &words[..]
                } {
                    for &bits in inputs {
                        cases.push((operation, storage, bits, a, b));
                        writeln!(requests, "{operation} {storage} {bits} {a} {b}").unwrap();
                        actual.push(
                            super::transform_float_storage(operation, storage, bits, a, b)
                                .unwrap()
                                .unwrap() as u32,
                        );
                    }
                }
            }
        }
    }
    let expected = super::matrix_product_tests::query_oracle("transform", requests);
    assert_eq!(expected.len(), 35964, "every transform case must execute");
    assert_eq!(actual.len(), expected.len());
    for ((case, actual), expected) in cases.into_iter().zip(actual).zip(expected) {
        let decode = |bits| {
            super::convert_float_storage(case.1, 0, u64::from(bits))
                .unwrap()
                .unwrap() as u32
        };
        if f32::from_bits(decode(expected)).is_nan() {
            assert!(
                f32::from_bits(decode(actual)).is_nan(),
                "transform {case:?}: {actual:08x} vs {expected:08x}"
            );
        } else {
            assert_eq!(actual, expected, "transform {case:?}");
        }
    }
}

#[test]
fn float_transform_refusals_vtk_05() {
    for &(op, ty, bits, a, b) in &[
        (3, 0, 0, 0, 0),
        (0, 2, 0, 0, 0),
        (0, 1, 65536, 0, 0),
        (0, 30, 65536, 0, 0),
        (0, 0, 4294967296, 0, 0),
        (0, 0, 0, 4294967296, 0),
        (2, 0, 0, 0, 4294967296),
    ] {
        assert_eq!(
            super::transform_float_storage(op, ty, bits, a, b).unwrap(),
            None
        );
    }
}

#[test]
fn transform_certificate_vtk_05() {
    super::matrix_product_tests::certified_model(
        "verdant_float_transform",
        "VerdantEngine.FloatTransform.transform",
    );
}

#[test]
fn float_unary_vtk_06() {
    use std::collections::BTreeSet;
    use std::fmt::Write as _;
    let mut words = BTreeSet::new();
    for sign in [0_u32, 0x80000000] {
        for exponent in 0..256_u32 {
            for fraction in [0, 1, 0x3fffff, 0x400000, 0x400001, 0x7ffffe, 0x7fffff] {
                words.insert(sign | (exponent << 23) | fraction);
            }
        }
        for exponent in 127..150_u32 {
            let step = 1 << (150 - exponent);
            for base in [0, step, 0x800000 - step] {
                for fraction in [0, 1, step / 2 - 1, step / 2, step / 2 + 1, step - 1] {
                    if fraction < step && base + fraction < 0x800000 {
                        words.insert(sign | (exponent << 23) | (base + fraction));
                    }
                }
            }
        }
        for center in [0x3f000000, 0x3f800000, 0x40400000, 0x40c00000, 0x4b000000] {
            for bits in [center - 1, center, center + 1] {
                words.insert(sign | bits);
            }
        }
    }
    let mut state = 0x12345678_u32;
    for _ in 0..16384 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        words.insert(state);
    }
    assert_eq!(words.len(), 20568);
    let f32_words: Vec<_> = words.into_iter().collect();
    let small_words: Vec<_> = (0..65536_u32).collect();
    let mut total = 0;
    for (source, destination) in [(0, 0), (1, 1), (1, 0), (30, 30), (30, 0)] {
        let inputs = if source == 0 {
            &f32_words
        } else {
            &small_words
        };
        let mut requests = String::new();
        let mut actual = Vec::new();
        for operation in 0..13 {
            for &bits in inputs {
                writeln!(requests, "{operation} {source} {destination} {bits}").unwrap();
                actual.push(
                    super::evaluate_float_unary(operation, source, destination, u64::from(bits))
                        .unwrap()
                        .unwrap() as u32,
                );
            }
        }
        let expected = super::matrix_product_tests::query_oracle("unary", requests);
        assert_eq!(
            expected.len(),
            13 * inputs.len(),
            "the CPU oracle omitted unary cases"
        );
        assert_eq!(actual.len(), expected.len());
        total += expected.len();
        for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
            let operation = index / inputs.len();
            let bits = inputs[index % inputs.len()];
            let decode = |word| {
                super::convert_float_storage(destination, 0, u64::from(word))
                    .unwrap()
                    .unwrap() as u32
            };
            if f32::from_bits(decode(expected)).is_nan() {
                assert!(f32::from_bits(decode(actual)).is_nan(),"unary operation={operation} source={source} destination={destination} bits={bits:08x} actual={actual:08x} expected={expected:08x}");
            } else {
                assert_eq!(actual,expected,"unary operation={operation} source={source} destination={destination} bits={bits:08x}");
            }
        }
    }
    assert_eq!(total, 3675256);
    eprintln!("unary oracle comparisons: {total}");
}

#[test]
fn float_unary_refusals_vtk_06() {
    for &(operation, source, destination, bits) in &[
        (14, 0, 0, 0),
        (0, 2, 0, 0),
        (0, 0, 1, 0),
        (0, 1, 30, 0),
        (0, 30, 1, 0),
        (0, 0, 2, 0),
        (0, 1, 1, 65536),
        (0, 30, 0, 65536),
        (0, 0, 0, 4294967296),
    ] {
        assert_eq!(
            super::evaluate_float_unary(operation, source, destination, bits).unwrap(),
            None
        );
    }
    assert_eq!(verdant_unary_select::run(3, 0).unwrap(), None);
    assert_eq!(verdant_unary_activate::run(2, 0).unwrap(), None);
    assert_eq!(verdant_unary_round::run(4, 0).unwrap(), None);
    assert_eq!(verdant_unary_select::run(0, 4294967296).unwrap(), None);
    assert_eq!(verdant_unary_activate::run(0, 4294967296).unwrap(), None);
    assert_eq!(verdant_unary_round::run(0, 4294967296).unwrap(), None);
}

#[test]
fn unary_composition_smoke_vtk_06() {
    use std::fmt::Write as _;
    let wide: [u32; 32] = [
        0, 0x80000000, 0x3f000000, 0xbf000000, 0x3fc00000, 0xbfc00000, 0x40200000, 0xc0200000,
        0x40400000, 0xc0400000, 0x403fffff, 0x40400001, 0xc03fffff, 0xc0400001, 0x3f800000,
        0xbf800000, 0x7f800000, 0xff800000, 0x7fc00001, 0x7f800001, 0xff800001, 1, 0x80000001,
        0x00800000, 0x7f7fffff, 0xff7fffff, 0x4affffff, 0xcaffffff, 0x4b000000, 0xcb000000,
        0x3f7fffff, 0xbf7fffff,
    ];
    let small: [u32; 32] = [
        0, 0x8000, 0x3c00, 0xbc00, 0x3f80, 0xbf80, 0x3800, 0xb800, 0x3e00, 0xbe00, 0x3f00, 0xbf00,
        0x3fc0, 0xbfc0, 0x7c00, 0xfc00, 0x7f80, 0xff80, 0x7e01, 0x7c01, 0x7fc1, 0x7f81, 1, 0x8001,
        0x0400, 0x7bff, 0x0080, 0x7f7f, 0x7fff, 0xffff, 0x3fff, 0xbfff,
    ];
    let mut requests = String::new();
    let mut cases = Vec::new();
    let mut actual = Vec::new();
    for (source, destination) in [(0, 0), (1, 1), (1, 0), (30, 30), (30, 0)] {
        for operation in 0..13 {
            for &bits in if source == 0 { &wide } else { &small } {
                writeln!(requests, "{operation} {source} {destination} {bits}").unwrap();
                cases.push((operation, source, destination, bits));
                actual.push(
                    super::evaluate_float_unary(operation, source, destination, u64::from(bits))
                        .unwrap()
                        .unwrap() as u32,
                );
            }
        }
    }
    let expected = super::matrix_product_tests::query_oracle("unary", requests);
    assert_eq!(cases.len(), 2080);
    assert_eq!(actual.len(), 2080);
    assert_eq!(expected.len(), 2080);
    for ((case, actual), expected) in cases.into_iter().zip(actual).zip(expected) {
        let decode = |word| {
            super::convert_float_storage(case.2, 0, u64::from(word))
                .unwrap()
                .unwrap() as u32
        };
        if f32::from_bits(decode(expected)).is_nan() {
            assert!(
                f32::from_bits(decode(actual)).is_nan(),
                "unary smoke {case:?}"
            );
        } else {
            assert_eq!(actual, expected, "unary smoke {case:?}");
        }
    }
}

#[test]
fn unary_certificate_vtk_06() {
    super::matrix_product_tests::certified_model(
        "verdant_unary_plan",
        "VerdantEngine.FloatUnary.plan",
    );
}

#[test]
fn scalar_operations_vtk_02() {
    assert_eq!(super::evaluate_f32_scalar(0, 1.0, 1.0).unwrap(), 2.0);
    assert!(super::evaluate_f32_scalar(8, 1.0, 1.0).is_err());
}
#[test]
fn wide_accumulator_vtk_07() {
    assert_eq!(
        super::evaluate_wide(0, 0x3ff0000000000000, 0x3ff0000000000000).unwrap(),
        Some(0x4000000000000000)
    );
    assert_eq!(
        super::evaluate_wide(1, 0x3f800000, 0).unwrap(),
        Some(0x3ff0000000000000)
    );
    assert_eq!(
        super::evaluate_wide(2, 0x3ff0000000000000, 0).unwrap(),
        Some(0x3f800000)
    );
    assert_eq!(
        super::evaluate_wide(3, u64::MAX, 0).unwrap(),
        Some(0x5f800000)
    );
}
