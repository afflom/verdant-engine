//! Exact binary32 logarithm replay against the original Arm scalar implementation.

use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
use verdant_tensor::linear::natural_log_f32;

#[test]
fn scalar_logarithm_vtk_68() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/oracles/arm-math");
    let archive = source.join("logf-source.tar.gz");
    assert_eq!(
        format!("{:x}", Sha256::digest(fs::read(&archive).unwrap())),
        "d0dc89ba8516e7692a67f2059a37d3013e56a589d651aa99fa636245d9bf3371"
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
        .args([
            "-O2",
            "-fno-builtin-logf",
            "-ffp-contract=off",
            "math/logf.c",
            "math/logf_data.c",
            "math/math_errf.c"
        ])
        .arg(source.join("log_words.c"))
        .args(["-lm", "-o", "oracle"])
        .status()
        .unwrap()
        .success());
    let output = Command::new(tmp.join("oracle")).output().unwrap();
    fs::remove_dir_all(tmp).unwrap();
    assert!(output.status.success());
    let rows = std::str::from_utf8(&output.stdout).unwrap();
    assert_eq!(rows.lines().count(), 256 * 64 + 17 * 5 + 8);
    for line in rows.lines() {
        let (input, expected) = line.split_once(' ').unwrap();
        let input = u32::from_str_radix(input, 16).unwrap();
        let reference = f32::from_bits(u32::from_str_radix(expected, 16).unwrap());
        let actual = natural_log_f32(f32::from_bits(input)).unwrap();
        if reference.is_nan() {
            assert!(actual.is_nan(), "input {input:#010x}");
        } else {
            assert_eq!(actual.to_bits(), reference.to_bits(), "input {input:#010x}");
        }
    }
}
