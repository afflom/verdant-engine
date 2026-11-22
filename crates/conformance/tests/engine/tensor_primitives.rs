use super::repo_root;
use verdant_tensor::TensorComputeError;

/// VTK-01: Arbitrary F32 matrix storage is admitted before the imported contraction.
#[test]
fn engine_vtk_01() {
    let mut output = [0.0_f32; 6];
    verdant_tensor::linear::multiply_matrix_rows_f32(
        3,
        1,
        2,
        &[1.0, 2.0, 3.0],
        &[4.0, -2.0],
        &mut output,
    )
    .unwrap();
    assert_eq!(output, [4.0, -2.0, 8.0, -4.0, 12.0, -6.0]);
    assert!(matches!(
        verdant_tensor::linear::multiply_matrix_rows_f32(3, 1, 2, &[], &[4.0, -2.0], &mut output,),
        Err(TensorComputeError::ZeroOrMismatchedExtents { .. })
    ));
    assert_eq!(output, [4.0, -2.0, 8.0, -4.0, 12.0, -6.0]);
}

/// VTK-02: The certified scalar model preserves signed zeros and rejects unknown operations.
#[test]
fn engine_vtk_02() {
    assert_eq!(
        verdant_tensor::linear::evaluate_f32_scalar(0, 1.0, 1.0).unwrap(),
        2.0
    );
    assert_eq!(
        verdant_tensor::linear::evaluate_f32_scalar(4, 4.0, 0.0).unwrap(),
        2.0
    );
    assert_eq!(
        verdant_tensor::linear::evaluate_f32_scalar(2, -0.0, 1.0)
            .unwrap()
            .to_bits(),
        (-0.0_f32).to_bits()
    );
    assert!(matches!(
        verdant_tensor::linear::evaluate_f32_scalar(u64::MAX, 1.0, 1.0),
        Err(TensorComputeError::InvalidKernelSpecification { .. })
    ));
}

/// VTK-11: packed scales and F8 use a certified production root.
#[test]
fn engine_vtk_11() {
    let request: serde_json::Value = serde_json::from_slice(
        &std::fs::read(repo_root().join("app/tensor-production.json")).unwrap(),
    )
    .unwrap();
    assert!(
        request["roots"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["root"] == "VerdantEngine.SmallFloat.evaluate"
                && entry["package"]["name"] == "verdant_small_float"),
        "packed float production root missing"
    );
}

/// VTK-12: packed fields and biased coefficients have certified production roots.
#[test]
fn engine_vtk_12() {
    let request: serde_json::Value = serde_json::from_slice(
        &std::fs::read(repo_root().join("app/tensor-production.json")).unwrap(),
    )
    .unwrap();
    for (root, package) in [
        ("VerdantEngine.PackedWord.extract", "verdant_packed_field"),
        (
            "VerdantEngine.PackedWord.coefficient",
            "verdant_integer_f32",
        ),
    ] {
        assert!(
            request["roots"]
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| entry["root"] == root && entry["package"]["name"] == package),
            "missing certified packed root {root}"
        );
    }
}

/// VTK-13: codebook data and its source metadata must be materialized.
#[test]
fn engine_vtk_13() {
    for name in ["ggml_tables.bin", "ggml_tables.json", "ggml_tables.LICENSE"] {
        assert!(
            repo_root()
                .join("crates/verdant-tensor/src")
                .join(name)
                .is_file(),
            "imported GGML table artifact missing: {name}"
        );
    }
}

/// VTK-14: complete storage-block decoding has a certified production root.
#[test]
fn engine_vtk_14() {
    let request: serde_json::Value = serde_json::from_slice(
        &std::fs::read(repo_root().join("app/tensor-production.json")).unwrap(),
    )
    .unwrap();
    assert!(
        request["roots"]
            .as_array()
            .unwrap()
            .iter()
            .any(
                |entry| entry["root"] == "VerdantEngine.StorageBlock.evaluate"
                    && entry["package"]["name"] == "verdant_storage_block"
            ),
        "complete storage-block production root missing"
    );
}
