use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SOFTFLOAT_DRIVER: &str = r#"#include "softfloat.h"
#include "platform.h"
#include "primitives.h"
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(int argc, char **argv) {
if(argc!=2)return 1;
softfloat_roundingMode=softfloat_round_near_even;
softfloat_detectTininess=softfloat_tininess_afterRounding;
uint64_t a,b,result;
while(scanf("%" SCNu64 " %" SCNu64,&a,&b)==2) {
    if(!strcmp(argv[1],"add"))result=f64_add((float64_t){a},(float64_t){b}).v;
    else if(!strcmp(argv[1],"widen"))result=f32_to_f64((float32_t){(uint32_t)a}).v;
    else if(!strcmp(argv[1],"narrow"))result=f64_to_f32((float64_t){a}).v;
    else if(!strcmp(argv[1],"count"))result=ui64_to_f32(a).v;
    else if(!strcmp(argv[1],"word-add")) result=softfloat_add128(0,a,0,b).v0;
    else if(!strcmp(argv[1],"word-subtract")) result=softfloat_sub128(0,a,0,b).v0;
    else if(!strcmp(argv[1],"word-multiply")) result=softfloat_mul64To128(a,b).v0;
    else if(!strcmp(argv[1],"multiply"))result=f64_mul((float64_t){a},(float64_t){b}).v;
    else if(!strcmp(argv[1],"divide"))result=f64_div((float64_t){a},(float64_t){b}).v;
    else if(!strcmp(argv[1],"count64"))result=ui64_to_f64(a).v;
    else if(!strcmp(argv[1],"signed64-f32") || !strcmp(argv[1],"signed64-f64")) {
        int64_t signed_word; memcpy(&signed_word,&a,sizeof(a));
        result=!strcmp(argv[1],"signed64-f32") ? i64_to_f32(signed_word).v : i64_to_f64(signed_word).v;
    }
    else if(!strcmp(argv[1],"f64-signed64") || !strcmp(argv[1],"f64-signed64-invalid")) {
        softfloat_exceptionFlags=0;
        result=(uint64_t)f64_to_i64_r_minMag((float64_t){a},false);
        if(!strcmp(argv[1],"f64-signed64-invalid")) result=(softfloat_exceptionFlags & softfloat_flag_invalid)!=0;
    }
    else if(!strcmp(argv[1],"f64-odd") || !strcmp(argv[1],"signed64-odd")) {
        softfloat_roundingMode=softfloat_round_odd;
        int64_t value; memcpy(&value,&a,sizeof(value));
        result=!strcmp(argv[1],"f64-odd") ? f64_to_f32((float64_t){a}).v : i64_to_f32(value).v;
    }
    else if(!strcmp(argv[1],"f64-signed32") || !strcmp(argv[1],"f64-signed32-invalid")) {
        softfloat_exceptionFlags=0;
        result=(uint32_t)f64_to_i32_r_minMag((float64_t){a},false);
        if(!strcmp(argv[1],"f64-signed32-invalid")) result=(softfloat_exceptionFlags & softfloat_flag_invalid)!=0;
    }
    else if(!strcmp(argv[1],"mean"))result=f64_to_f32(f64_div((float64_t){a},ui64_to_f64(b))).v;
    else if(!strncmp(argv[1],"equal-",6)) {
        extFloat80_t x,y;
        int64_t ai,bi;
        memcpy(&ai,&a,sizeof(a)); memcpy(&bi,&b,sizeof(b));
        if(argv[1][6]=='i') i64_to_extF80M(ai,&x); else f64_to_extF80M((float64_t){a},&x);
        if(argv[1][7]=='i') i64_to_extF80M(bi,&y); else f64_to_extF80M((float64_t){b},&y);
        result=extF80M_eq(&x,&y);
    }
    else if(!strcmp(argv[1],"compare")) {
        const float32_t x={(uint32_t)a},y={(uint32_t)b};
        result=!f32_eq(y,y)?3:!f32_eq(x,x)?4:f32_lt(x,y)?0:f32_eq(x,y)?1:2;
    }
    else return 2;
    printf("%" PRIu64 "\n",result);
}
return ferror(stdin)?3:0;
}
"#;

fn checked(command: &mut Command) -> Vec<u8> {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn digest(path: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(std::fs::read(path).unwrap()))
}

fn program() -> &'static Path {
    static PROGRAM: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    PROGRAM.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let imported = root.join("tests/oracles/softfloat");
        let archive = imported.join("source.tar.gz");
        let pin: serde_json::Value =
            serde_json::from_slice(&std::fs::read(imported.join("upstream.json")).unwrap())
                .unwrap();
        assert_eq!(
            pin["repository"],
            "https://github.com/ucb-bar/berkeley-softfloat-3"
        );
        assert_eq!(pin["commit"], "07ebd29db0cfb55723a79fbbffe99103ef38e305");
        assert_eq!(pin["specialization"], "8086-SSE");
        assert_eq!(pin["rounding"], "softfloat_round_near_even");
        assert_eq!(pin["format"], "git-archive-tar-gzip");
        assert_eq!(pin["prefix"], "softfloat/");
        let archive_sha = digest(&archive);
        assert_eq!(
            archive_sha,
            "a43936117a4b62b2c942b62144ab903f7d93c5fb313389caecbd6e790d416e34"
        );
        assert_eq!(pin["archive_sha256"], archive_sha);
        const {
            assert!(
                cfg!(target_endian = "little") && cfg!(target_pointer_width = "64"),
                "SoftFloat profile requires a little-endian 64-bit target"
            );
        }
        let mut identity = Sha256::new();
        identity.update(&archive_sha);
        let builder = include_str!("wide_accumulator_tests.rs")
            .split_once("\nfn checked")
            .unwrap()
            .1
            .split_once("\nfn query(")
            .unwrap()
            .0;
        identity.update(
            builder
                .parse::<proc_macro2::TokenStream>()
                .unwrap()
                .to_string(),
        );
        for tool in ["gcc", "cc", "ar", "make"] {
            identity.update(checked(Command::new(tool).arg("--version")));
        }
        identity.update(std::fs::read(root.join("template.lock")).unwrap());
        identity.update(std::fs::read(root.join("app/lean-toolchain")).unwrap());
        if let Some(inventory) = std::env::var_os("PRISMPM_SDK_INVENTORY") {
            identity.update(std::fs::read(inventory).unwrap());
        }
        identity.update(std::env::consts::ARCH);
        let base = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("target"))
            .join("softfloat-oracle");
        let common = base
            .join("cpu")
            .join(format!("{:x}", identity.clone().finalize()));
        if !common.exists() {
            std::fs::create_dir_all(&common).unwrap();
            checked(
                Command::new("tar")
                    .args(["-xzf"])
                    .arg(&archive)
                    .arg("-C")
                    .arg(&common),
            );
        }
        let listing = checked(Command::new("tar").arg("-tvzf").arg(&archive));
        let mut inputs = Vec::new();
        for line in std::str::from_utf8(&listing).unwrap().lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let name = fields[5];
            if name.ends_with('/') {
                continue;
            }
            assert!(
                line.starts_with('-') && fields.len() == 6 && name.starts_with("softfloat/"),
                "non-regular oracle input: {line}"
            );
            inputs.push((name.to_string(), fields[2].parse::<usize>().unwrap()));
        }
        assert!(inputs.len() > 400);
        let authoritative = checked(
            Command::new("tar")
                .arg("-xOzf")
                .arg(&archive)
                .args(inputs.iter().map(|(name, _)| name)),
        );
        let mut cached = Sha256::new();
        for (name, length) in inputs {
            let bytes = std::fs::read(common.join(&name)).unwrap();
            assert_eq!(bytes.len(), length, "SoftFloat source length differs");
            if name == "softfloat/COPYING.txt" {
                assert_eq!(
                    bytes,
                    std::fs::read(imported.join("LICENSE")).unwrap(),
                    "SoftFloat license differs"
                );
            }
            cached.update(bytes);
        }
        assert_eq!(
            cached.finalize(),
            Sha256::digest(authoritative),
            "SoftFloat source bytes differ"
        );
        let build = common.join("softfloat/build/Linux-x86_64-GCC");
        let library = build.join("softfloat.a");
        let library_stamp = build.join("softfloat.sha256");
        if !library.exists() {
            checked(
                Command::new("make")
                    .args(["-B", "-j2", "all"])
                    .current_dir(&build),
            );
            std::fs::write(&library_stamp, digest(&library)).unwrap();
        }
        assert_eq!(
            digest(&library),
            std::fs::read_to_string(&library_stamp).unwrap(),
            "SoftFloat library differs"
        );
        identity.update(SOFTFLOAT_DRIVER);
        let directory = base
            .join("program")
            .join(format!("{:x}", identity.finalize()));
        let executable = directory.join("oracle");
        let stamp = directory.join("oracle.sha256");
        if !executable.exists() {
            std::fs::create_dir_all(&directory).unwrap();
            let driver = directory.join("driver.c");
            std::fs::write(&driver, SOFTFLOAT_DRIVER).unwrap();
            checked(
                Command::new("cc")
                    .args([
                        "-O2",
                        "-std=c11",
                        "-DSOFTFLOAT_FAST_INT64",
                        "-DINLINE_LEVEL=5",
                    ])
                    .arg("-I")
                    .arg(common.join("softfloat/build/Linux-x86_64-GCC"))
                    .arg("-I")
                    .arg(common.join("softfloat/source/include"))
                    .arg(driver)
                    .arg(library)
                    .arg("-o")
                    .arg(executable.with_extension("new")),
            );
            std::fs::write(&stamp, digest(executable.with_extension("new"))).unwrap();
            std::fs::rename(executable.with_extension("new"), &executable).unwrap();
        }
        assert_eq!(
            digest(&executable),
            std::fs::read_to_string(stamp).unwrap(),
            "SoftFloat program differs"
        );
        executable
    })
}

fn query(mode: &str, inputs: &[(u64, u64)]) -> Vec<u64> {
    let mut requests = String::new();
    for (a, b) in inputs {
        writeln!(requests, "{a} {b}").unwrap();
    }
    let mut child = Command::new(program())
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let output = std::thread::scope(|scope| {
        let writer = scope.spawn(move || stdin.write_all(requests.as_bytes()));
        let output = child.wait_with_output().unwrap();
        writer.join().unwrap().unwrap();
        output
    });
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let values: Vec<u64> = std::str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    assert_eq!(values.len(), inputs.len(), "oracle omitted cases");
    values
}

fn compare(mode: &str, inputs: &[(u64, u64)]) {
    let expected = query(mode, inputs);
    for (index, (&(a, b), expected)) in inputs.iter().zip(expected).enumerate() {
        let actual = match mode {
            "add" => super::evaluate_wide(0, a, b).map_err(|_| ()),
            "widen" => super::evaluate_wide(1, a, 0).map_err(|_| ()),
            "narrow" => super::evaluate_wide(2, a, 0).map_err(|_| ()),
            "count" => super::evaluate_wide(3, a, 0).map_err(|_| ()),
            "word-add" => super::evaluate_wide(15, a, b).map_err(|_| ()),
            "word-subtract" => super::evaluate_wide(16, a, b).map_err(|_| ()),
            "word-multiply" => super::evaluate_wide(17, a, b).map_err(|_| ()),
            "multiply" => super::evaluate_wide(14, a, b).map_err(|_| ()),
            "divide" => super::evaluate_wide(4, a, b).map_err(|_| ()),
            "count64" => super::evaluate_wide(5, a, 0).map_err(|_| ()),
            "mean" => super::evaluate_wide(5, b, 0)
                .and_then(|count| super::evaluate_wide(4, a, count.unwrap()))
                .and_then(|mean| super::evaluate_wide(2, mean.unwrap(), 0))
                .map_err(|_| ()),
            "compare" => super::evaluate_encoded_word(0, a, b).map_err(|_| ()),
            _ => panic!("unknown mode"),
        }
        .expect("bounded arithmetic")
        .expect("admitted encoding");
        let nan = |word: u64| {
            if matches!(mode, "add" | "widen" | "divide" | "multiply" | "count64") {
                word & 0x7ff0000000000000 == 0x7ff0000000000000 && word & 0x000fffffffffffff != 0
            } else {
                word & 0x7f800000 == 0x7f800000 && word & 0x007fffff != 0
            }
        };
        assert!((nan(actual)&&nan(expected))||actual==expected,
            "{mode} case {index}: a={a:016x} b={b:016x} actual={actual:016x} expected={expected:016x}");
    }
}

fn wide_words() -> &'static [u64] {
    static WORDS: std::sync::OnceLock<Vec<u64>> = std::sync::OnceLock::new();
    WORDS.get_or_init(|| {
        let mut wide = Vec::new();
        let fractions = [
            0,
            1,
            2,
            3,
            0x7ffffffffffff,
            0x8000000000000,
            0x8000000000001,
            0xffffffffffffd,
            0xffffffffffffe,
            0xfffffffffffff,
        ];
        for sign in [0, 1u64 << 63] {
            for exponent in 0..2048 {
                for fraction in fractions {
                    wide.push(sign | (exponent << 52) | fraction);
                }
            }
        }
        let mut random = 0x793c8d940faf8e7bu64;
        for _ in 0..65536 {
            random = random
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            wide.push(random);
        }
        wide.sort_unstable();
        wide.dedup();
        wide
    })
}

#[test]
fn arithmetic_oracle_vtk_07() {
    let wide = wide_words();
    let mut additions = Vec::new();
    for (index, &a) in wide.iter().enumerate() {
        for b in [
            0,
            1,
            0x8000000000000000,
            a,
            a ^ (1 << 63),
            a ^ 1,
            wide[(index * 37 + 13) % wide.len()],
            0x3ca0000000000000,
        ] {
            additions.push((a, b));
        }
    }
    assert_eq!(additions.len(), 851968);
    compare("add", &additions);
    let add_count = additions.len();
    drop(additions);
    let mut narrow = wide.to_vec();
    for exponent in 0..=254 {
        for fraction in [0, 1, 2, 0x3fffff, 0x7ffffe, 0x7fffff] {
            let word = (exponent << 23) | fraction;
            let left = f64::from(f32::from_bits(word));
            let right = f64::from(f32::from_bits(word + 1));
            let midpoint = ((left + right) * 0.5).to_bits();
            for sign in [0, 1u64 << 63] {
                for bits in [
                    left.to_bits(),
                    midpoint - 1,
                    midpoint,
                    midpoint + 1,
                    right.to_bits(),
                ] {
                    narrow.push(sign | bits);
                }
            }
        }
    }
    narrow.sort_unstable();
    narrow.dedup();
    let narrow: Vec<_> = narrow.into_iter().map(|a| (a, 0)).collect();
    assert_eq!(narrow.len(), 118712);
    compare("narrow", &narrow);
    let mut small = Vec::new();
    for sign in [0, 1u64 << 31] {
        for exponent in 0..256 {
            for fraction in [
                0, 1, 2, 3, 0x3fffff, 0x400000, 0x400001, 0x7ffffd, 0x7ffffe, 0x7fffff,
            ] {
                small.push(sign | (exponent << 23) | fraction);
            }
        }
    }
    for &word in wide {
        small.push(word & 0xffffffff);
    }
    small.sort_unstable();
    small.dedup();
    let small: Vec<_> = small.into_iter().map(|a| (a, 0)).collect();
    assert_eq!(small.len(), 70656);
    compare("widen", &small);
    let mut counts: Vec<_> = (0..65536).chain(wide.iter().copied()).collect();
    for power in 0..64 {
        let pivot = 1u64 << power;
        counts.extend([pivot - 1, pivot, pivot.saturating_add(1)]);
        if power >= 24 {
            let shift = power - 23;
            for mantissa in [
                1u64 << 23,
                (1u64 << 23) + 1,
                (1u64 << 24) - 2,
                (1u64 << 24) - 1,
            ] {
                let base = mantissa << shift;
                let midpoint = base.checked_add(1u64 << (shift - 1));
                if let Some(midpoint) = midpoint {
                    counts.extend([midpoint - 1, midpoint, midpoint.saturating_add(1)]);
                }
            }
        }
    }
    counts.extend([u64::MAX, 0]);
    counts.sort_unstable();
    counts.dedup();
    let counts: Vec<_> = counts.into_iter().map(|a| (a, 0)).collect();
    assert_eq!(counts.len(), 172604);
    compare("count", &counts);
    eprintln!(
        "VTK-07 SoftFloat comparisons: add={add_count} narrow={} widen={} count={} total={}",
        narrow.len(),
        small.len(),
        counts.len(),
        add_count + narrow.len() + small.len() + counts.len()
    );
}

#[test]
fn bounded_power_vtk_07() {
    for exponent in (0..=1024).chain([u32::MAX, u32::MAX - 1]) {
        assert_eq!(
            crate::pow2_u64(exponent),
            1u64.checked_shl(exponent).unwrap_or(0)
        );
    }
}

#[test]
fn quotient_smoke_vtk_15() {
    assert_eq!(
        super::evaluate_wide(4, 0x3ff0000000000000, 0x4000000000000000).unwrap(),
        Some(0x3fe0000000000000)
    );
    assert_eq!(
        super::evaluate_wide(5, u64::MAX, 0).unwrap(),
        Some(0x43f0000000000000)
    );
    assert_eq!(super::evaluate_wide(6, 0, 0).unwrap(), None);
}

#[test]
fn wide_smoke_vtk_07() {
    let words = [
        0,
        1,
        0x8000000000000000,
        0x3ff0000000000000,
        0xbff0000000000000,
        0x3ca0000000000000,
        0x7fefffffffffffff,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff0000000000001,
        0x3ff0000000000001,
    ];
    let additions: Vec<_> = words
        .iter()
        .flat_map(|&a| words.iter().map(move |&b| (a, b)))
        .collect();
    compare("add", &additions);
    let wide: Vec<_> = words.into_iter().map(|a| (a, 0)).collect();
    compare("narrow", &wide);
    compare("count", &wide);
    compare(
        "widen",
        &[
            0,
            1,
            0x80000000,
            0x3f800000,
            0xbf800000,
            0x7f800000,
            0xff800000,
            0x7f800001,
            u32::MAX,
        ]
        .map(|a| (u64::from(a), 0)),
    );
}

#[test]
fn wide_refusals_vtk_07() {
    assert!(verdant_high_f64::run(u64::MAX, 0, 0).unwrap().is_none());
    assert_eq!(
        verdant_high_f64::run(1, 2047, 4503599627370495).unwrap(),
        Some(u64::from(u32::MAX))
    );
    assert_eq!(verdant_low_f64::run(4503599627370495), u64::from(u32::MAX));
    for operation in [6, u64::MAX] {
        assert_eq!(super::evaluate_wide(operation, 0, 0).unwrap(), None);
    }
    for bits in [4294967296, u64::MAX] {
        assert_eq!(super::evaluate_wide(1, bits, 0).unwrap(), None);
    }
    for (sign, exponent, mantissa) in [
        (2, 1, 0),
        (0, 0, 0),
        (0, 2048, 0),
        (0, 1, 144115188075855872),
    ] {
        assert!(verdant_pack_plan_f64::run(sign, exponent, mantissa)
            .unwrap()
            .is_none());
    }
    for (sign, exponent, mantissa) in [
        (2, 1, 0),
        (0, 0, 0),
        (0, 2047, 0),
        (0, 1, 144115188075855872),
    ] {
        assert!(verdant_normalize_f64::run(sign, exponent, mantissa)
            .unwrap()
            .is_none());
    }
}

#[test]
fn encoding_certificate_vtk_07() {
    super::matrix_product_tests::certified_model(
        "verdant_pack_plan_f64",
        "VerdantEngine.FloatWord64Exports.packAddition",
    );
}

#[test]
fn wide_certificate_vtk_07() {
    super::matrix_product_tests::certified_model(
        "verdant_add_f64",
        "VerdantEngine.ScalarF64.planAdd",
    );
}
#[test]
fn word_refusals_vtk_08() {
    for op in [0, 1] {
        for (a, b) in [(4294967296, 0), (0, 4294967296), (u64::MAX, u64::MAX)] {
            assert_eq!(super::evaluate_encoded_word(op, a, b).unwrap(), None);
        }
    }
    for op in [4, u64::MAX] {
        assert_eq!(super::evaluate_encoded_word(op, 0, 0).unwrap(), None);
    }
    for (op, a, b, expected) in [
        (1, 1, 1, 1),
        (1, 0x80000000, 0, 0),
        (2, u64::MAX - 1, 1, u64::MAX),
        (3, u64::MAX, 0, u64::MAX),
    ] {
        assert_eq!(
            super::evaluate_encoded_word(op, a, b).unwrap(),
            Some(expected)
        );
    }
    assert!(super::evaluate_encoded_word(2, u64::MAX, 1).is_err());
}

#[test]
fn comparison_oracle_vtk_08() {
    let mut words = Vec::new();
    for sign in [0, 0x80000000_u64] {
        for exponent in 0..256 {
            for fraction in [
                0, 1, 2, 3, 0x3ffffe, 0x3fffff, 0x400000, 0x400001, 0x7ffffc, 0x7ffffd, 0x7ffffe,
                0x7fffff,
            ] {
                words.push(sign | (exponent << 23) | fraction);
            }
        }
    }
    let mut random = 0x97389a42u32;
    for _ in 0..65536 {
        random = random.wrapping_mul(1664525).wrapping_add(1013904223);
        words.push(u64::from(random));
    }
    words.sort_unstable();
    words.dedup();
    let mut inputs = Vec::new();
    for (index, &a) in words.iter().enumerate() {
        for b in [
            0,
            0x80000000,
            a,
            a ^ 0x80000000,
            a ^ 1,
            words[(index * 37 + 13) % words.len()],
            0x7f800001,
            0xff800000,
        ] {
            inputs.push((a, b));
        }
    }
    assert_eq!(inputs.len(), 573440);
    compare("compare", &inputs);
    eprintln!("VTK-08: 573440 imported SoftFloat comparison cases");
}

#[test]
fn comparison_certificate_vtk_08() {
    super::matrix_product_tests::certified_model(
        "verdant_encoded_word",
        "VerdantEngine.WordComparison.evaluate",
    );
}

#[test]
fn quotient_oracle_vtk_15() {
    super::matrix_product_tests::certified_quotient_model();
    let wide = wide_words();
    let mut divisions = Vec::new();
    for (index, &a) in wide.iter().enumerate() {
        for b in [
            0,
            1,
            1u64 << 63,
            a,
            a ^ (1u64 << 63),
            a ^ 1,
            wide[(index * 37 + 13) % wide.len()],
            0x3fe0000000000000,
            0x3ff0000000000000,
            0x4008000000000000,
            0x4014000000000000,
            0x7fefffffffffffff,
        ] {
            divisions.push((a, b));
        }
    }
    divisions.sort_unstable();
    divisions.dedup();
    assert_eq!(divisions.len(), 1277924);
    compare("divide", &divisions);
    let division_count = divisions.len();
    drop(divisions);
    let mut counts: Vec<_> = (0..65536).chain(wide.iter().copied()).collect();
    counts.push(u64::MAX);
    for power in 0..64 {
        let pivot = 1u64 << power;
        counts.extend([pivot - 1, pivot, pivot.saturating_add(1)]);
        if power >= 53 {
            let shift = power - 52;
            for mantissa in [
                1u64 << 52,
                (1u64 << 52) + 1,
                (1u64 << 53) - 2,
                (1u64 << 53) - 1,
            ] {
                let midpoint = (mantissa << shift) + (1u64 << (shift - 1));
                counts.extend([midpoint - 1, midpoint, midpoint + 1]);
            }
        }
    }
    counts.sort_unstable();
    counts.dedup();
    let counts: Vec<_> = counts.into_iter().map(|a| (a, 0)).collect();
    assert_eq!(counts.len(), 172246);
    compare("count64", &counts);
    let means: Vec<_> = wide.iter().flat_map(|&a| [(a, 3), (a, 5)]).collect();
    assert_eq!(means.len(), 212992);
    compare("mean", &means);
    eprintln!(
        "VTK-15 SoftFloat comparisons: divide={division_count} count64={} mean={} total={}",
        counts.len(),
        means.len(),
        division_count + counts.len() + means.len()
    );
}

fn signed_conversion_inputs() -> Vec<(u64, u64)> {
    let mut integers = vec![
        (0_u64, 0),
        (1, 0),
        (i64::MIN as u64, 0),
        (i64::MAX as u64, 0),
    ];
    for shift in 1..63 {
        let pivot = 1_u64 << shift;
        let half_ulp = if shift >= 24 {
            1_u64 << (shift - 24)
        } else {
            1
        };
        let half_wide = if shift >= 53 {
            1_u64 << (shift - 53)
        } else {
            1
        };
        for offset in [1, half_ulp, half_wide] {
            for word in [
                pivot - offset,
                pivot + offset - 1,
                pivot + offset,
                pivot + offset + 1,
            ] {
                integers.extend([(word, 0), (word.wrapping_neg(), 0)]);
            }
        }
    }
    integers.sort_unstable();
    integers.dedup();
    integers
}

fn binary64_conversion_inputs() -> Vec<(u64, u64)> {
    let mut floats = Vec::new();
    for exponent in 0..2048 {
        for fraction in [0, 1, (1 << 51) - 1, 1 << 51, (1 << 52) - 1] {
            let word = (exponent << 52) | fraction;
            floats.extend([(word, 0), (word | (1 << 63), 0)]);
        }
    }
    floats
}

#[test]
fn signed_word_conversion_vtk_63() {
    super::matrix_product_tests::certified_quotient_model();
    let integers = signed_conversion_inputs();
    for (mode, code) in [("signed64-f32", 9), ("signed64-f64", 7)] {
        for (&(word, _), expected) in integers.iter().zip(query(mode, &integers)) {
            assert_eq!(
                super::evaluate_wide(code, word, 0).unwrap(),
                Some(expected),
                "{mode} {word:016x}"
            );
        }
    }
    let floats = binary64_conversion_inputs();
    let invalid = query("f64-signed64-invalid", &floats);
    for ((&(word, _), expected), flag) in floats
        .iter()
        .zip(query("f64-signed64", &floats))
        .zip(invalid)
    {
        assert_eq!(
            super::evaluate_wide(8, word, 0).unwrap(),
            (flag == 0).then_some(expected),
            "f64 {word:016x}"
        );
    }
    assert_eq!(super::evaluate_wide(10, 0, 0).unwrap(), None);
}

#[test]
fn wide_narrowing_vtk_64() {
    super::matrix_product_tests::certified_quotient_model();
    let signed = signed_conversion_inputs();
    for (&(word, _), expected) in signed.iter().zip(query("signed64-odd", &signed)) {
        assert_eq!(
            super::evaluate_wide(12, word, 0).unwrap(),
            Some(expected),
            "signed64 odd {word:016x}"
        );
    }
    let mut floats = binary64_conversion_inputs();
    for exponent in [
        872, 873, 874, 895, 896, 897, 898, 1023, 1053, 1054, 1150, 1151,
    ] {
        for fraction in [
            0x0000_000f_ffff,
            0x0000_1000_0000,
            0x0000_1000_0001,
            0x1fff_ffff,
            0x2000_0000,
            0x2000_0001,
        ] {
            let word = (exponent << 52) | fraction;
            floats.extend([(word, 0), (word | (1 << 63), 0)]);
        }
    }
    for (&(word, _), expected) in floats.iter().zip(query("f64-odd", &floats)) {
        let actual = super::evaluate_wide(11, word, 0).unwrap().unwrap();
        assert!(
            expected == actual
                || (f32::from_bits(expected as u32).is_nan()
                    && f32::from_bits(actual as u32).is_nan()),
            "f64 odd {word:016x}: {actual:08x} != {expected:08x}"
        );
    }
    let invalid = query("f64-signed32-invalid", &floats);
    for ((&(word, _), expected), flag) in floats
        .iter()
        .zip(query("f64-signed32", &floats))
        .zip(invalid)
    {
        assert_eq!(
            super::evaluate_wide(13, word, 0).unwrap(),
            (flag == 0).then_some(expected),
            "f64 signed32 {word:016x}"
        );
    }
}

#[test]
fn wide_product_vtk_66() {
    super::matrix_product_tests::certified_quotient_model();
    let edges = [
        0,
        1,
        3,
        (1 << 52) - 1,
        1 << 52,
        0x3e40_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x3fe0_0000_0000_0001,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0001,
        0x3ff8_0000_0000_0000,
        0x4000_0000_0000_0000,
        0x7fef_ffff_ffff_ffff,
        0x7ff0_0000_0000_0000,
        0x7ff0_0000_0000_0001,
        0x7ff8_0000_0000_0000,
    ];
    let mut pairs = Vec::new();
    for a in edges {
        for b in edges {
            for sign_a in [0, 1 << 63] {
                for sign_b in [0, 1 << 63] {
                    pairs.push((a | sign_a, b | sign_b));
                }
            }
        }
    }
    for (index, &word) in wide_words().iter().enumerate() {
        pairs.push((word, edges[index % edges.len()]));
        pairs.push((
            word,
            wide_words()[(index * 65537 + 17) % wide_words().len()],
        ));
    }
    compare("multiply", &pairs);
}

#[test]
fn modular_word_arithmetic_vtk_67() {
    super::matrix_product_tests::certified_quotient_model();
    let limbs = [
        0u64,
        1,
        2,
        (1 << 32) - 1,
        1 << 32,
        (1 << 32) + 1,
        i64::MAX as u64,
        1 << 63,
        u64::MAX - 1,
        u64::MAX,
    ];
    let mut cases: Vec<_> = limbs
        .iter()
        .flat_map(|&a| limbs.iter().map(move |&b| (a, b)))
        .collect();
    for words in wide_words().chunks_exact(2).step_by(13) {
        cases.push((words[0], words[1].rotate_left(31)));
    }
    for op in ["word-add", "word-subtract", "word-multiply"] {
        compare(op, &cases);
    }
}

#[test]
fn numeric_equality_vtk_69() {
    super::matrix_product_tests::certified_quotient_model();
    let mut words = vec![
        0,
        1,
        u64::MAX,
        i64::MAX as u64,
        i64::MIN as u64,
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
        f64::NAN.to_bits(),
        0x7ff0000000000001,
    ];
    for exponent in 0..64 {
        let unit = 1_u64 << exponent;
        for value in [unit.wrapping_neg(), unit] {
            words.extend([value.wrapping_sub(1), value, value.wrapping_add(1)]);
            let bits = ((value as i64) as f64).to_bits();
            words.extend([bits - 1, bits, bits + 1]);
        }
    }
    words.sort_unstable();
    words.dedup();
    let mut cases = Vec::new();
    for (index, &a) in words.iter().enumerate() {
        let b = ((a as i64) as f64).to_bits();
        for candidate in [
            a,
            a.wrapping_add(1),
            b,
            b.wrapping_sub(1),
            b.wrapping_add(1),
            words[(index * 31) % words.len()],
        ] {
            cases.extend([(a, candidate), (candidate, a)]);
        }
    }
    cases.sort_unstable();
    cases.dedup();
    for (mode, left_signed, right_signed) in [
        ("equal-ii", true, true),
        ("equal-if", true, false),
        ("equal-fi", false, true),
        ("equal-ff", false, false),
    ] {
        let expected = query(mode, &cases);
        for (&(a, b), answer) in cases.iter().zip(expected) {
            assert_eq!(
                crate::linear::equal_numeric_words(left_signed, right_signed, a, b).unwrap(),
                answer != 0,
                "{mode} {a:#018x} {b:#018x}"
            );
        }
    }
}
