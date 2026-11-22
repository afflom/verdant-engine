use super::evaluate_small_float;
use super::matrix_product_tests::{certified_model, query_oracle};
use std::fmt::Write as _;

#[test]
fn small_float_certificate_vtk_11() {
    certified_model("verdant_small_float", "VerdantEngine.SmallFloat.evaluate");
}

fn compare(inputs: &[(u64, u32)]) {
    let mut requests = String::new();
    for (operation, word) in inputs {
        writeln!(requests, "{operation} {word}").unwrap();
    }
    let expected = query_oracle("small-float", requests);
    assert_eq!(inputs.len(), expected.len());
    for ((operation, word), reference) in inputs.iter().zip(expected) {
        assert_eq!(
            evaluate_small_float(*operation, u64::from(*word)).unwrap(),
            Some(u64::from(reference)),
            "op={operation} word={word:#010x}"
        );
    }
}

#[test]
fn ggml_small_float_decode_vtk_11() {
    let mut inputs = Vec::new();
    for operation in 0..4 {
        for word in 0..256 {
            inputs.push((operation, word));
        }
    }
    for word in 0..16 {
        inputs.push((6, word));
    }
    compare(&inputs);
}

#[test]
fn ggml_small_float_encode_vtk_11() {
    // Generate the dense source corpus with the imported conversion oracle.
    let mut requests = String::new();
    for storage in [1, 30] {
        for word in 0..65536 {
            writeln!(requests, "{storage} 0 {word}").unwrap();
        }
    }
    let mut words = query_oracle("conversion", requests);
    // Midpoints and adjacent F32 words cover every F8 rounding transition.
    let mut requests = String::new();
    for byte in 0..127 {
        writeln!(requests, "3 {byte}").unwrap();
    }
    let grid = query_oracle("small-float", requests);
    for pair in grid.windows(2) {
        let midpoint = ((f64::from(f32::from_bits(pair[0])) + f64::from(f32::from_bits(pair[1])))
            * 0.5) as f32;
        for word in [
            pair[0],
            midpoint.to_bits() - 1,
            midpoint.to_bits(),
            midpoint.to_bits() + 1,
        ] {
            words.extend([word, word | 0x80000000]);
        }
    }
    words.extend([
        0x3c700000, 0x43800000, 0x43e00000, 0x7f800001, 0xff800001, 0x7fffffff, 0xffffffff,
        0x00000001, 0x80000001,
    ]);
    words.sort_unstable();
    words.dedup();
    let inputs: Vec<_> = words
        .into_iter()
        .flat_map(|word| [(4, word), (5, word)])
        .collect();
    eprintln!("GGML small-float encoder corpus: {} cases", inputs.len());
    compare(&inputs);
}

#[test]
fn small_float_refusals_vtk_11() {
    for operation in 0..4 {
        assert_eq!(evaluate_small_float(operation, 256).unwrap(), None);
    }
    for operation in 4..6 {
        assert_eq!(evaluate_small_float(operation, 1u64 << 32).unwrap(), None);
    }
    assert_eq!(evaluate_small_float(6, 16).unwrap(), None);
    assert_eq!(evaluate_small_float(7, 0).unwrap(), None);
    assert_eq!(evaluate_small_float(u64::MAX, u64::MAX).unwrap(), None);
}
