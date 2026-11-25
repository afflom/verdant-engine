//! Bit-level replays against the original Arm math sources.

use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
use verdant_tensor::linear::natural_log_f32;

#[test]
fn scalar_logarithm_vtk_68() {
    compare(
        "logf",
        "d0dc89ba8516e7692a67f2059a37d3013e56a589d651aa99fa636245d9bf3371",
        "math/logf_data.c",
        256 * 64 + 17 * 5 + 8,
        |x| natural_log_f32(x).unwrap(),
    );
}

#[test]
fn scalar_sine_vtk_70() {
    use verdant_tensor::linear::evaluate_float_unary;
    for (from, to, bits) in [(1, 0, 0), (0, 1, 0), (0, 0, 1_u64 << 32)] {
        assert_eq!(evaluate_float_unary(14, from, to, bits).unwrap(), None);
    }
    compare(
        "sinf",
        "b14ff3e77b2eb1bc3fac6056f37f376524bb5e397056f2d6d35272f873eb4eaa",
        "math/sincosf_data.c",
        256 * 32 * 2 + 4 * 7 * 2 + 8,
        |x| {
            f32::from_bits(
                verdant_tensor::linear::evaluate_float_unary(14, 0, 0, u64::from(x.to_bits()))
                    .unwrap()
                    .unwrap() as u32,
            )
        },
    );
}

fn compare(symbol: &str, digest: &str, table: &str, count: usize, evaluate: impl Fn(f32) -> f32) {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/oracles/arm-math");
    let archive = source.join(format!("{symbol}-source.tar.gz"));
    assert_eq!(
        format!("{:x}", Sha256::digest(fs::read(&archive).unwrap())),
        digest
    );
    let tmp = Command::new("mktemp").arg("-d").output().unwrap();
    assert!(tmp.status.success());
    let tmp = Path::new(std::str::from_utf8(&tmp.stdout).unwrap().trim());
    assert!(Command::new("tar")
        .arg("-xzf")
        .arg(archive)
        .arg("-C")
        .arg(tmp)
        .status()
        .unwrap()
        .success());
    assert!(Command::new("cc")
        .current_dir(tmp)
        .args((symbol == "sinf").then_some("-DHAVE_FAST_ROUND=0"))
        .args([
            "-O2",
            "-fno-builtin",
            "-ffp-contract=off",
            &format!("math/{symbol}.c"),
            table,
            "math/math_errf.c"
        ])
        .arg(source.join(if symbol == "sinf" {
            "sine_words.c"
        } else {
            "log_words.c"
        }))
        .args(["-lm", "-o", "oracle"])
        .status()
        .unwrap()
        .success());
    let output = Command::new(tmp.join("oracle")).output().unwrap();
    fs::remove_dir_all(tmp).unwrap();
    assert!(output.status.success());
    let rows = std::str::from_utf8(&output.stdout).unwrap();
    assert_eq!(rows.lines().count(), count);
    for line in rows.lines() {
        let (input, expected) = line.split_once(' ').unwrap();
        let input = u32::from_str_radix(input, 16).unwrap();
        let reference = f32::from_bits(u32::from_str_radix(expected, 16).unwrap());
        let actual = evaluate(f32::from_bits(input));
        if reference.is_nan() {
            assert!(actual.is_nan(), "input {input:#010x}");
        } else {
            assert_eq!(actual.to_bits(), reference.to_bits(), "input {input:#010x}");
        }
    }
}
