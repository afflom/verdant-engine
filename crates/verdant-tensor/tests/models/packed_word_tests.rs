use super::matrix_product_tests::{certified_model, query_oracle};
use super::{encode_small_integer, extract_packed_field};
use std::fmt::Write as _;

#[test]
fn packed_certificates_vtk_12() {
    certified_model("verdant_packed_field", "VerdantEngine.PackedWord.extract");
    certified_model(
        "verdant_integer_f32",
        "VerdantEngine.PackedWord.coefficient",
    );
}

#[test]
fn integer_coefficients_vtk_12() {
    let mut requests = String::new();
    for word in 0..256 {
        for zero_point in 0..257 {
            writeln!(requests, "{word} {zero_point}").unwrap();
        }
    }
    let expected = query_oracle("integer-coefficient", requests);
    assert_eq!(expected.len(), 256 * 257);
    for (index, bits) in expected.into_iter().enumerate() {
        let (word, zero_point) = ((index / 257) as u64, (index % 257) as u64);
        assert_eq!(
            encode_small_integer(word, zero_point).unwrap(),
            Some(u64::from(bits)),
            "word={word} zero_point={zero_point}"
        );
    }
}

fn field(word: u64, factor: u64, divisor: u64, modulus: u64) -> u64 {
    extract_packed_field(word, factor, divisor, modulus)
        .unwrap()
        .unwrap()
}

#[test]
fn packed_ggml_fields_vtk_12() {
    let high_words = [
        0,
        u32::MAX,
        1,
        0x80000000,
        0xaaaaaaaa,
        0x55555555,
        0x81fe017f,
    ];
    let mut requests = String::new();
    let mut actual = Vec::new();
    for (storage, lanes) in [
        (41, 128),
        (42, 64),
        (2, 32),
        (3, 32),
        (6, 32),
        (7, 32),
        (8, 32),
        (34, 256),
        (35, 256),
    ] {
        for low in 0..256u64 {
            for &high in if [6, 7, 34].contains(&storage) {
                &high_words[..]
            } else {
                &high_words[..1]
            } {
                writeln!(requests, "{storage} {low} {high}").unwrap();
                for lane in 0..lanes {
                    let (value, zero_point) = match storage {
                        41 => (field(field(low, 1, 1 << (lane % 8), 2), 2, 1, 3), 1),
                        42 => (field(low, 1, 1 << (2 * (lane % 4)), 4), 1),
                        2 | 3 => (
                            field(low, 1, if lane < 16 { 1 } else { 16 }, 16),
                            if storage == 2 { 8 } else { 0 },
                        ),
                        6 | 7 => {
                            let low = field(low, 1, if lane < 16 { 1 } else { 16 }, 16);
                            let high = field(field(u64::from(high), 1, 1 << lane, 2), 16, 1, 17);
                            (
                                super::evaluate_encoded_word(2, low, high).unwrap().unwrap(),
                                if storage == 6 { 16 } else { 0 },
                            )
                        }
                        8 => (low, if field(low, 1, 128, 2) == 1 { 256 } else { 0 }),
                        34 => {
                            let (byte, power) = if lane < 160 {
                                (low, lane / 32)
                            } else if lane < 240 {
                                (low, (lane - 160) / 16)
                            } else {
                                (
                                    field(u64::from(high), 1, 1 << (8 * ((lane - 240) % 4)), 256),
                                    (lane - 240) / 4,
                                )
                            };
                            let trit = field(byte, [1, 3, 9, 27, 81][power], 1, 256);
                            (field(trit, 3, 256, 3), 1)
                        }
                        35 => (field(low, 1, 1 << (2 * ((lane % 128) / 32)), 4), 1),
                        _ => unreachable!(),
                    };
                    actual.push(encode_small_integer(value, zero_point).unwrap().unwrap());
                }
            }
        }
    }
    let expected = query_oracle("packed-field", requests);
    assert_eq!(actual.len(), expected.len());
    eprintln!("GGML packed field corpus: {} lanes", actual.len());
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert_eq!(actual, u64::from(expected), "packed lane {index}");
    }
}

#[test]
fn packed_refusals_vtk_12() {
    assert_eq!(extract_packed_field(7, 1, 0, 16).unwrap(), None);
    assert_eq!(extract_packed_field(7, 1, 1, 0).unwrap(), None);
    assert!(extract_packed_field(u64::MAX, 2, 1, 16).is_err());
    assert_eq!(
        extract_packed_field(u64::MAX, 1, u64::MAX, u64::MAX).unwrap(),
        Some(1)
    );
    assert_eq!(extract_packed_field(u64::MAX, 0, 1, 2).unwrap(), Some(0));
    assert_eq!(encode_small_integer(256, 0).unwrap(), None);
    assert_eq!(encode_small_integer(0, 257).unwrap(), None);
    assert_eq!(encode_small_integer(u64::MAX, u64::MAX).unwrap(), None);
}

#[derive(serde::Serialize)]
struct ImportedTable {
    name: String,
    element_bytes: usize,
    elements: usize,
    offset: usize,
    sha256: String,
}

#[derive(serde::Serialize)]
struct ImportedTables {
    spec: &'static str,
    revision: &'static str,
    archive_sha256: &'static str,
    header: &'static str,
    byte_order: &'static str,
    license_sha256: String,
    data_sha256: String,
    tables: Vec<ImportedTable>,
}

fn imported_tables() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    use sha2::{Digest, Sha256};
    let protocol = query_oracle("quantized-tables", String::new());
    let mut values = protocol.into_iter();
    assert_eq!(values.next(), Some(1196705585));
    let mut data = Vec::new();
    let mut tables = Vec::new();
    loop {
        let length = values.next().expect("table stream truncated") as usize;
        if length == 0 {
            break;
        }
        assert!(length < 128);
        let name = String::from_utf8(
            (0..length)
                .map(|_| u8::try_from(values.next().unwrap()).unwrap())
                .collect(),
        )
        .unwrap();
        assert!(!tables.iter().any(|t: &ImportedTable| t.name == name));
        let element_bytes = values.next().unwrap() as usize;
        let elements = values.next().unwrap() as usize;
        assert!([1, 2, 4, 8].contains(&element_bytes));
        assert!(elements > 0 && elements <= 4096);
        let offset = data.len();
        for _ in 0..element_bytes * elements {
            data.push(u8::try_from(values.next().unwrap()).unwrap());
        }
        tables.push(ImportedTable {
            name,
            element_bytes,
            elements,
            offset,
            sha256: format!("{:x}", Sha256::digest(&data[offset..])),
        });
    }
    assert!(values.next().is_none(), "unconsumed reference tables");
    assert!(!tables.is_empty(), "empty table import");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap();
    let license = std::process::Command::new("tar")
        .arg("-xOzf")
        .arg(root.join("tests/oracles/ggml/source.tar.gz"))
        .arg("LICENSE")
        .output()
        .unwrap();
    assert!(license.status.success());
    assert!(license.stdout.starts_with(b"MIT License\n"));
    let manifest = ImportedTables {
        spec: "verdant/ggml-quantized-tables/1",
        revision: "9a080f44b503c64f66c84a6274b9d9ee5d5b09d5",
        archive_sha256: "4eec816a907885c31932e0ab4fa1ee2278af43000dc19d514929cd03cb4ff8a0",
        header: "ggml/src/ggml-common.h",
        byte_order: "little-endian",
        license_sha256: format!("{:x}", Sha256::digest(&license.stdout)),
        data_sha256: format!("{:x}", Sha256::digest(&data)),
        tables,
    };
    (
        data,
        serde_json::to_vec_pretty(&manifest).unwrap(),
        license.stdout,
    )
}

#[test]
fn quantized_tables_vtk_13() {
    let (data, manifest, license) = imported_tables();
    // Cache the reference artifacts separately from the checked source files.
    // A first import copies these files explicitly; validation never writes src/.
    let out = std::env::var_os("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .nth(2)
                .unwrap()
                .join("target")
        })
        .join("ggml-table-import");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("ggml_tables.bin"), &data).unwrap();
    std::fs::write(out.join("ggml_tables.json"), &manifest).unwrap();
    std::fs::write(out.join("ggml_tables.LICENSE"), &license).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    same_bytes("compiled tables", super::GGML_QUANTIZED_TABLES, &data);
    same_bytes(
        "compiled table metadata",
        super::GGML_QUANTIZED_TABLE_CATALOG.as_bytes(),
        &manifest,
    );
    same_bytes(
        "table license",
        &std::fs::read(root.join("ggml_tables.LICENSE")).unwrap(),
        &license,
    );
    same_bytes(
        "table source bytes",
        &std::fs::read(root.join("ggml_tables.bin")).unwrap(),
        &data,
    );
    same_bytes(
        "table source metadata",
        &std::fs::read(root.join("ggml_tables.json")).unwrap(),
        &manifest,
    );
}

fn same_bytes(label: &str, actual: &[u8], expected: &[u8]) {
    assert!(
        actual == expected,
        "{label} differ: {} versus {} bytes, first difference {:?}",
        actual.len(),
        expected.len(),
        actual.iter().zip(expected).position(|(a, b)| a != b)
    );
}

struct StorageBlockCase {
    kind: u64,
    pattern: u32,
    packed: Vec<u8>,
    expected: Vec<u32>,
}

fn storage_block_cases() -> &'static [StorageBlockCase] {
    static CASES: std::sync::OnceLock<Vec<StorageBlockCase>> = std::sync::OnceLock::new();
    CASES.get_or_init(|| {
        let mut packet = query_oracle("storage-block", String::new()).into_iter();
        assert_eq!(packet.next(), Some(1196705586));
        let mut cases = Vec::new();
        loop {
            let kind = packet.next().expect("truncated storage block reference");
            if kind == 0 {
                break;
            }
            let lanes = packet.next().unwrap() as usize;
            let bytes = packet.next().unwrap() as usize;
            let pattern = packet.next().unwrap();
            assert!((1..=256).contains(&lanes) && (1..=292).contains(&bytes));
            let packed = (0..bytes)
                .map(|_| u8::try_from(packet.next().unwrap()).unwrap())
                .collect();
            let expected = (0..lanes).map(|_| packet.next().unwrap()).collect();
            cases.push(StorageBlockCase {
                kind: u64::from(kind - 1),
                pattern,
                packed,
                expected,
            });
        }
        assert!(packet.next().is_none(), "unconsumed reference data");
        assert_eq!(cases.len(), 800);
        cases
    })
}

#[test]
fn storage_block_reference_vtk_14() {
    let cases = storage_block_cases();
    let types: std::collections::BTreeSet<_> = cases.iter().map(|case| case.kind).collect();
    assert_eq!(types.len(), 32);
    for kind in types {
        let cases: Vec<_> = cases.iter().filter(|case| case.kind == kind).collect();
        assert_eq!(
            cases.iter().map(|case| case.pattern).collect::<Vec<_>>(),
            (0..25).collect::<Vec<_>>()
        );
        assert!(cases
            .iter()
            .all(|case| !case.packed.is_empty() && !case.expected.is_empty()));
    }
}

#[test]
fn activation_storage_vtk_65() {
    let cases = storage_block_cases();
    for (kind, lanes, bytes) in [(9, 32, 36), (15, 256, 292)] {
        let rows = cases.iter().filter(|case| case.kind == kind);
        assert_eq!(rows.clone().count(), 25);
        assert!(rows
            .clone()
            .all(|c| (c.expected.len(), c.packed.len()) == (lanes, bytes)));
        compare_storage_blocks(rows);
    }
}

#[test]
fn storage_blocks_vtk_14() {
    compare_storage_blocks(
        storage_block_cases()
            .iter()
            .filter(|case| !matches!(case.kind, 9 | 15)),
    );
}

fn compare_storage_blocks<'a>(cases: impl Iterator<Item = &'a StorageBlockCase>) {
    use super::decode_storage_block;
    let mut failures = Vec::new();
    for case in cases {
        for stride in [4, 8, 12] {
            let mut actual = vec![0xa5; (case.expected.len() - 1) * stride + 15];
            let mut expected = actual.clone();
            for (lane, word) in case.expected.iter().enumerate() {
                expected[lane * stride..lane * stride + 4].copy_from_slice(&word.to_le_bytes());
            }
            let result = decode_storage_block(case.kind, &case.packed, &mut actual, stride);
            if result.is_err() || actual != expected {
                let difference = actual.iter().zip(&expected).position(|(a, b)| a != b);
                let word = difference.map(|offset| {
                    let offset = offset / stride * stride;
                    (&actual[offset..offset + 4], &expected[offset..offset + 4])
                });
                let mut nan_pairs = 0;
                let mut other_words = 0;
                for lane in 0..case.expected.len() {
                    let actual = u32::from_le_bytes(
                        actual[lane * stride..lane * stride + 4].try_into().unwrap(),
                    );
                    let expected = case.expected[lane];
                    if actual != expected {
                        if f32::from_bits(actual).is_nan()
                            && (actual & 0x0040_0000) != 0
                            && f32::from_bits(expected).is_nan()
                        {
                            nan_pairs += 1;
                        } else {
                            other_words += 1;
                        }
                    }
                }
                let span = (case.expected.len() - 1) * stride + 4;
                let padding = actual
                    .iter()
                    .zip(&expected)
                    .enumerate()
                    .filter(|(offset, (a, b))| a != b && (*offset >= span || *offset % stride >= 4))
                    .count();
                if matches!(case.kind, 9 | 15) || result.is_err() || other_words > 0 || padding > 0
                {
                    failures.push(format!(
                        "kind={} pattern={} stride={stride} result={result:?} first_byte={difference:?} words={word:?} nan_pairs={nan_pairs} other_words={other_words} padding={padding}",
                        case.kind, case.pattern
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "storage block mismatches:\n{}",
        failures.join("\n")
    );
    certified_model(
        "verdant_storage_block",
        "VerdantEngine.StorageBlock.evaluate",
    );
}

#[test]
fn storage_block_refusals_vtk_14() {
    use super::decode_storage_block;
    use crate::TensorComputeError;
    for case in storage_block_cases()
        .iter()
        .filter(|case| case.pattern == 0)
    {
        let bytes = case.expected.len() * 4;
        for (source_len, stride, output_len) in [
            (case.packed.len() - 1, 4, bytes),
            (case.packed.len(), 0, bytes),
            (case.packed.len(), 1, bytes),
            (case.packed.len(), 2, bytes),
            (case.packed.len(), 3, bytes),
            (case.packed.len(), 4, bytes - 1),
        ] {
            let mut output = vec![0xa5; output_len];
            assert!(matches!(
                decode_storage_block(case.kind, &case.packed[..source_len], &mut output, stride),
                Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
            ));
            assert!(output.iter().all(|byte| *byte == 0xa5));
        }
        let mut oversized = case.packed.clone();
        oversized.push(0);
        let mut output = vec![0xa5; bytes];
        assert!(matches!(
            decode_storage_block(case.kind, &oversized, &mut output, 4),
            Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
        ));
        assert!(output.iter().all(|byte| *byte == 0xa5));
    }
    for kind in [4, 5, 24, 25, 27, 28, 31, 32, 33, 36, 37, 38, 44, u64::MAX] {
        let mut output = [0xa5; 16];
        assert!(matches!(
            decode_storage_block(kind, &[0; 32], &mut output, 4),
            Err(TensorComputeError::InvalidKernelSpecification { .. })
        ));
        assert_eq!(output, [0xa5; 16]);
    }
    let mut output = [0xa5; 16];
    assert!(matches!(
        decode_storage_block(2, &[0; 18], &mut output, usize::MAX),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));
    assert_eq!(output, [0xa5; 16]);
}

#[test]
fn storage_arithmetic_words_vtk_14() {
    use std::fmt::Write;
    use verdant_storage_block::{run, Adt0 as Request, Adt1 as Instruction};
    certified_model(
        "verdant_storage_block",
        "VerdantEngine.StorageBlock.evaluate",
    );
    let edges = [
        0, 0x80000000, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc00001, 0x7fc00002,
        0xffc00001, 0x7f800001, 0x7f800002, 0xff800002,
    ];
    let mut cases = Vec::new();
    for operation in 0..4 {
        for left in edges {
            for right in edges {
                cases.push((operation, left, right));
            }
        }
        for sign in [0, 0x80000000] {
            for payload in 1..1024 {
                let word = sign | 0x7f800000 | (payload << 13);
                cases.push((operation, word, 0x3f800000));
                cases.push((operation, 0x3f800000, word));
            }
        }
    }
    assert_eq!(cases.len(), 16_944);
    let mut requests = String::new();
    for (operation, left, right) in &cases {
        writeln!(requests, "{operation} {left} {right}").unwrap();
    }
    let expected = query_oracle("scalar", requests);
    assert_eq!(expected.len(), cases.len());
    for ((operation, left, right), expected) in cases.into_iter().zip(expected) {
        let Instruction::C3(code, a, b, register) =
            run(Request::C3(operation, left, right, 2)).unwrap()
        else {
            panic!("arithmetic instruction refused {operation} {left:08x} {right:08x}")
        };
        assert_eq!(register, 2);
        let actual = match code {
            0..=3 => verdant_scalar_f32::run(code, a, b).unwrap().unwrap(),
            11 => super::evaluate_encoded_word(3, a, 0).unwrap().unwrap(),
            _ => panic!("unexpected arithmetic effect {code}"),
        };
        if f32::from_bits(expected).is_nan() {
            let bits = u32::try_from(actual).expect("binary32 word");
            assert!(
                f32::from_bits(bits).is_nan() && (bits & 0x0040_0000) != 0,
                "operation={operation} left={left:08x} right={right:08x} actual={actual:08x}"
            );
        } else {
            assert_eq!(
                actual,
                u64::from(expected),
                "operation={operation} left={left:08x} right={right:08x}"
            );
        }
    }
    for (operation, left, right, register) in [
        (4, 0, 0, 0),
        (0, 1 << 32, 0, 0),
        (0, 0, 1 << 32, 0),
        (0, 0, 0, 8),
        (u64::MAX, u64::MAX, u64::MAX, u64::MAX),
    ] {
        assert!(matches!(
            run(Request::C3(operation, left, right, register)).unwrap(),
            Instruction::C0(0)
        ));
    }
}
