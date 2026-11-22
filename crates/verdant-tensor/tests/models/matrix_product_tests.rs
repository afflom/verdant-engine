const GGML_VECTOR_DRIVER: &str = r#"
#include "vec.h"
#include "ops.h"
#include "unary-ops.h"
#include "ggml-cpu-impl.h"
#include "ggml-cpu.h"
#include "ggml-quants.h"
#include "ops.h"
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <vector>
#include <type_traits>
#define GGML_COMMON_IMPL_CPP
#include "ggml-common.h"
template<class Block>
static bool read_dot_block(Block &block) {
  unsigned word;
  if (std::scanf("%u", &word) != 1) return false;
  block.d = ggml_fp16_t(word);
  for (auto &lane : block.qs) {
    if (std::scanf("%u", &word) != 1) return false;
    lane = static_cast<std::remove_reference_t<decltype(lane)>>(uint8_t(word));
  }
  return true;
}
template<class Weight>
static int compare_rows(ggml_type type) {
  const auto dot = ggml_get_type_traits_cpu(type)->vec_dot;
  unsigned blocks;
  while (std::scanf("%u", &blocks) == 1) {
    std::vector<Weight> left(blocks + 1);
    std::vector<block_q8_0> right(blocks + 1);
    for (unsigned i = 0; i < blocks; ++i)
      if (!read_dot_block(left[i]) || !read_dot_block(right[i])) return 2;
    float result;
    uint32_t bits;
    dot(int(blocks)*QK8_0, &result, 0, left.data(), 0, right.data(), 0, 1);
    std::memcpy(&bits, &result, sizeof(bits));
    std::printf("%u\n", bits);
  }
  return std::ferror(stdin) ? 3 : 0;
}
int main(int argc, char **argv) {
  if (argc != 2) return 1;
  ggml_cpu_init();
  if (!std::strcmp(argv[1], "sincos")) {
    unsigned input;
    while (std::scanf("%u", &input) == 1) {
      float x, sine, cosine; unsigned sw, cw; std::memcpy(&x,&input,sizeof x);
      ggml_vec_sin_f32(1,&sine,&x); ggml_vec_cos_f32(1,&cosine,&x);
      std::memcpy(&sw,&sine,sizeof sw); std::memcpy(&cw,&cosine,sizeof cw);
      std::printf("%u\n%u\n",sw,cw);
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "swiglu-scalar")) {
    uint32_t first, second;
    while (std::scanf("%u %u", &first, &second) == 2) {
      float x, gate, result;
      std::memcpy(&x, &first, sizeof(x)); std::memcpy(&gate, &second, sizeof(gate));
      ggml_vec_swiglu_f32(1, &result, &x, &gate);
      std::memcpy(&first, &result, sizeof(first)); std::printf("%u\n", first);
    }
    return std::ferror(stdin) ? 2 : 0;
  }
  if (!std::strcmp(argv[1], "natural-exp")) {
    uint32_t bits;
    while (std::scanf("%u", &bits) == 1) {
      float x, y;
      std::memcpy(&x, &bits, sizeof(x));
      ggml_vec_exp_f32(1, &y, &x);
      std::memcpy(&bits, &y, sizeof(bits));
      std::printf("%u\n", bits);
    }
    return std::ferror(stdin) ? 2 : 0;
  }
  if (!std::strcmp(argv[1], "symmetric-dot")) return compare_rows<block_q8_0>(GGML_TYPE_Q8_0);
  if (!std::strcmp(argv[1], "packed-dot")) return compare_rows<block_q4_0>(GGML_TYPE_Q4_0);
  if (!std::strcmp(argv[1], "symmetric-q8")) {
    uint32_t word;
    while (std::scanf("%u", &word) == 1) {
      float block[QK8_0];
      for (int i = 0; i < QK8_0; ++i) {
        if (i && std::scanf("%u", &word) != 1) return 2;
        std::memcpy(&block[i], &word, sizeof(word));
      }
      block_q8_0 encoded;
      quantize_row_q8_0_ref(block, &encoded, QK8_0);
      const auto *bytes = reinterpret_cast<const uint8_t *>(&encoded);
      for (size_t i = 0; i < sizeof(encoded); ++i) std::printf("%u\n", unsigned(bytes[i]));
    }
    return 0;
  }
  if (!std::strcmp(argv[1], "storage-block")) {
    std::printf("1196705586\n");
    for (unsigned kind=0; kind<GGML_TYPE_COUNT; ++kind) {
      const auto type=(ggml_type)kind;
      if (type!=GGML_TYPE_F32 && type!=GGML_TYPE_I32 && !ggml_get_type_traits(type)->to_float) continue;
      ggml_init_params init={1024*1024,nullptr,false};
      ggml_context *ctx=ggml_init(init); if (!ctx) return 8;
      const auto lanes=ggml_blck_size(type);
      auto *source=ggml_new_tensor_2d(ctx,type,lanes,1);
      auto *indices=ggml_new_tensor_1d(ctx,GGML_TYPE_I32,1);
      auto *result=ggml_get_rows(ctx,source,indices);
      *(int32_t *)indices->data=0;
      ggml_compute_params params={}; params.nth=1;
      const uint16_t exceptional[]={0x7c01,0xfc02,0x7e03,0xfe04,0x7c00,0xfc00,0,0x8000,0x3c00,0xbc00,0x0001,0x8001,0x7bff,0xfbff,0x3555,0xb555};
      for (unsigned pattern=0; pattern<25; ++pattern) {
        const size_t bytes=ggml_nbytes(source);
        auto *input=(uint8_t *)source->data;
        for (size_t j=0; j<bytes; ++j) {
          input[j]=pattern==0?0:pattern==1?255:pattern==2?165:
              (uint8_t)((j*(2*pattern+1)+kind*11+pattern*53)^((j+pattern)<<3));
          if (pattern>=9) input[j]=(uint8_t)(exceptional[(pattern-9+j/2)%16]>>(8*(j%2)));
        }
        ggml_compute_forward_get_rows(&params,result);
        std::printf("%u\n%u\n%u\n%u\n",kind+1,(unsigned)lanes,(unsigned)bytes,pattern);
        for (size_t j=0; j<bytes; ++j) std::printf("%u\n",(unsigned)input[j]);
        for (int64_t lane=0; lane<lanes; ++lane) {
          uint32_t word; std::memcpy(&word,(const uint8_t *)result->data+4*lane,4);
          std::printf("%u\n",word);
        }
      }
      ggml_free(ctx);
    }
    std::printf("0\n"); return 0;
  }
  if (!std::strcmp(argv[1], "quantized-tables")) {
std::printf("1196705585\n");
auto emit=[](const char *label,const auto &values) {
  const unsigned length=std::strlen(label);
  std::printf("%u\n",length);for(unsigned i=0;i<length;i++)std::printf("%u\n",(unsigned)(uint8_t)label[i]);
  std::printf("%u\n%u\n",(unsigned)sizeof(values[0]),(unsigned)(sizeof(values)/sizeof(values[0])));
  for(auto value:values){
    using U=std::make_unsigned_t<decltype(value)>;
    const uint64_t word=(U)value;
    for(unsigned byte=0;byte<sizeof(value);byte++)std::printf("%u\n",(unsigned)((word>>(8*byte))&255));
  }
};
#include "ggml-table-exports.inc"
std::printf("0\n");return 0;
  }
  if (!std::strcmp(argv[1], "integer-coefficient")) {
unsigned word, zero_point;
while(std::scanf("%u %u",&word,&zero_point)==2) {
  block_q4_1 block={};float result[QK4_1];
  const ggml_half d=GGML_CPU_FP32_TO_FP16((float)word);
  const ggml_half m=GGML_CPU_FP32_TO_FP16(-(float)zero_point);
  std::memcpy(&block,&d,sizeof(d));std::memcpy((char *)&block+sizeof(d),&m,sizeof(m));
  std::memset(block.qs,0x11,sizeof(block.qs));
  ggml_get_type_traits(GGML_TYPE_Q4_1)->to_float(&block,result,QK4_1);
  uint32_t bits;std::memcpy(&bits,result,4);std::printf("%u\n",bits);
}
return std::ferror(stdin)?3:0;
  }
  if (!std::strcmp(argv[1], "packed-field")) {
unsigned type, low, high;
while(std::scanf("%u %u %u",&type,&low,&high)==3) {
  const ggml_half d=GGML_CPU_FP32_TO_FP16(1.0f);
  auto emit=[&](const auto &block) {
    std::vector<float> result(ggml_blck_size((ggml_type)type));
    const auto *traits=ggml_get_type_traits((ggml_type)type);if(!traits->to_float)return false;
    traits->to_float(&block,result.data(),result.size());
    for(float value:result){uint32_t bits;std::memcpy(&bits,&value,4);std::printf("%u\n",bits);}return true;
  };
  switch(type) {
    case GGML_TYPE_Q1_0: {block_q1_0 b={};b.d=d;std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_Q2_0: {block_q2_0 b={};b.d=d;std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_Q4_0: {block_q4_0 b={};b.d=d;std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_Q4_1: {block_q4_1 b={};std::memcpy(&b,&d,2);std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_Q5_0: {block_q5_0 b={};b.d=d;std::memcpy(b.qh,&high,4);std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_Q5_1: {block_q5_1 b={};std::memcpy(&b,&d,2);std::memcpy(b.qh,&high,4);std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_Q8_0: {block_q8_0 b={};b.d=d;std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    case GGML_TYPE_TQ1_0: {block_tq1_0 b={};b.d=d;std::memset(b.qs,low,sizeof(b.qs));std::memcpy(b.qh,&high,4);if(!emit(b))return 7;break;}
    case GGML_TYPE_TQ2_0: {block_tq2_0 b={};b.d=d;std::memset(b.qs,low,sizeof(b.qs));if(!emit(b))return 7;break;}
    default:return 6;
  }
}
return std::ferror(stdin)?3:0;
  }
  if (!std::strcmp(argv[1], "small-float")) {
unsigned operation; uint32_t bits;
while(std::scanf("%u %u",&operation,&bits)==2) {
  float value, result; uint32_t word;
  std::memcpy(&value,&bits,4);
  switch(operation) {
    case 0: result=ggml_e8m0_to_fp32((uint8_t)bits); break;
    case 1: result=ggml_e8m0_to_fp32_half((uint8_t)bits); break;
    case 2: result=ggml_ue4m3_to_fp32((uint8_t)bits); break;
    case 3: result=ggml_f8_e4m3_to_fp32((uint8_t)bits); break;
    case 4: word=ggml_fp32_to_f8_e4m3(value); break;
    case 5: word=ggml_fp32_to_ue4m3(value); break;
    case 6: result=(float)kvalues_fp4[bits]; break;
    default: return 6;
  }
  if(operation<4 || operation==6)std::memcpy(&word,&result,4);
  std::printf("%u\n",word);
}
return std::ferror(stdin)?3:0;
  }
  if (!std::strcmp(argv[1], "unary")) {
void (*functions[])(const ggml_compute_params *,ggml_tensor *)={ggml_compute_forward_abs,ggml_compute_forward_sgn,ggml_compute_forward_neg,ggml_compute_forward_step,ggml_compute_forward_relu,ggml_compute_forward_sqr,ggml_compute_forward_sqrt,ggml_compute_forward_hardsigmoid,ggml_compute_forward_hardswish,ggml_compute_forward_floor,ggml_compute_forward_ceil,ggml_compute_forward_round,ggml_compute_forward_trunc};
unsigned operation, source_type, destination_type;uint32_t bits;
while(std::scanf("%u %u %u %u",&operation,&source_type,&destination_type,&bits)==4) {
  if(operation>=13)return 6;
  uint32_t result=0;uint16_t input=(uint16_t)bits,narrow_result=0;ggml_tensor source={},destination={};
  source.type=(ggml_type)source_type;source.data=source_type==0?(void *)&bits:&input;
  for(int axis=0;axis<4;axis++) {source.ne[axis]=1;source.nb[axis]=source_type==0?4:2;}
  destination=source;destination.type=(ggml_type)destination_type;destination.data=destination_type==0?(void *)&result:&narrow_result;destination.src[0]=&source;
  for(int axis=0;axis<4;axis++)destination.nb[axis]=destination_type==0?4:2;
  ggml_compute_params parameters={};parameters.nth=1;functions[operation](&parameters,&destination);
  if(destination_type!=0)result=narrow_result;
  std::printf("%u\n",result);
}
return std::ferror(stdin)?3:0;
  }
  if (!std::strcmp(argv[1], "transform")) {
unsigned operation, storage; uint32_t bits, a, b;
while(std::scanf("%u %u %u %u %u",&operation,&storage,&bits,&a,&b)==5) {
  uint32_t result=0; uint16_t input=(uint16_t)bits, narrow_result=0; ggml_tensor source={}, destination={};
  source.type=(ggml_type)storage; source.data=storage==0?(void *)&bits:&input;
  for(int axis=0;axis<4;axis++) {source.ne[axis]=1;source.nb[axis]=storage==0?4:2;}
  destination=source;destination.data=storage==0?(void *)&result:&narrow_result;destination.src[0]=&source;
  std::memcpy(destination.op_params,&a,4);std::memcpy((char *)destination.op_params+4,&b,4);
  ggml_compute_params parameters={};parameters.nth=1;
  if(operation==0)ggml_compute_forward_scale(&parameters,&destination);
  else if(operation==1)ggml_compute_forward_clamp(&parameters,&destination);
  else if(operation==2)ggml_compute_forward_leaky_relu(&parameters,&destination);
  else return 6;
  if(storage!=0) result=narrow_result;
  std::printf("%u\n",result);
}
return std::ferror(stdin)?3:0;
  }
  if (!std::strcmp(argv[1], "conversion")) {
unsigned source, destination; uint32_t bits;
while (std::scanf("%u %u %u", &source, &destination, &bits)==3) {
  uint32_t result=bits;
  if (source!=destination) {
    float value;
    if(source==0) std::memcpy(&value,&bits,4);
    else if(source==1) value=GGML_COMPUTE_FP16_TO_FP32((ggml_fp16_t)bits);
    else if(source==30) value=GGML_BF16_TO_FP32(ggml_bf16_t{(uint16_t)bits});
    else return 4;
    if(destination==0) std::memcpy(&result,&value,4);
    else if(destination==1) result=GGML_CPU_FP32_TO_FP16(value);
    else if(destination==30) result=GGML_FP32_TO_BF16(value).bits;
    else return 5;
  }
  std::printf("%u\n",result);
}
return std::ferror(stdin)?3:0;
  }
  if (!std::strcmp(argv[1], "scalar")) {
unsigned operation; uint32_t left_bits, right_bits;
while (std::scanf("%u %u %u", &operation, &left_bits, &right_bits) == 3) {
  float left, right, result;
  std::memcpy(&left, &left_bits, sizeof(left)); std::memcpy(&right, &right_bits, sizeof(right));
  switch (operation) {
    case 0: ggml_vec_add_f32(1, &result, &left, &right); break;
    case 1: ggml_vec_sub_f32(1, &result, &left, &right); break;
    case 2: ggml_vec_mul_f32(1, &result, &left, &right); break;
    case 3: ggml_vec_div_f32(1, &result, &left, &right); break;
    case 4: ggml_vec_sqrt_f32(1, &result, &left); break;
    case 5: ggml_vec_sqr_f32(1, &result, &left); break;
    case 6: ggml_vec_neg_f32(1, &result, &left); break;
    case 7: ggml_vec_abs_f32(1, &result, &left); break;
    default: return 2;
  }
  uint32_t bits; std::memcpy(&bits, &result, sizeof(bits)); std::printf("%u\n", bits);
}
return std::ferror(stdin) ? 3 : 0;
  }
  if (!std::strcmp(argv[1], "bytes")) {
unsigned size;
while (std::scanf("%u", &size)==1) {
  std::vector<float> source((size+3)/4), destination(source.size());
  for(unsigned i=0;i<size;i++) { unsigned value; if(std::scanf("%u", &value)!=1) return 2; ((unsigned char *)source.data())[i]=(unsigned char)value; }
  ggml_vec_cpy_f32((int)source.size(), destination.data(), source.data());
  for(unsigned i=0;i<size;i++) std::printf("%u\n",(unsigned)((unsigned char *)destination.data())[i]);
}
return std::ferror(stdin)?3:0;
  }
  if (std::strcmp(argv[1], "dot")) return 2;
  unsigned count;
  while (std::scanf("%u", &count) == 1) {
std::vector<float> left(count), right(count);
for (auto *operand : {&left, &right}) {
  for (float &value : *operand) {
    uint32_t bits;
    if (std::scanf("%u", &bits) != 1) return 2;
    std::memcpy(&value, &bits, sizeof(value));
  }
}
float result;
ggml_vec_dot_f32((int)count, &result, 0, left.data(), 0, right.data(), 0, 1);
uint32_t bits;
std::memcpy(&bits, &result, sizeof(bits));
std::printf("%u\n", bits);
  }
  return std::ferror(stdin) ? 3 : 0;
}
"#;

pub(crate) fn query_oracle(mode: &str, requests: String) -> Vec<u32> {
    use sha2::{Digest, Sha256};
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    static PROGRAM: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
    let program = PROGRAM.get_or_init(|| {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let archive = root.join("tests/oracles/ggml/source.tar.gz");
        let archive_sha = format!("{:x}", Sha256::digest(std::fs::read(&archive).unwrap()));
        assert_eq!(
            archive_sha,
            "4eec816a907885c31932e0ab4fa1ee2278af43000dc19d514929cd03cb4ff8a0"
        );
        let flags = [
            "-O2",
            "-std=c++17",
            "-ffunction-sections",
            "-fdata-sections",
            "-Wl,--gc-sections",
        ];
        let compiler = Command::new("c++").arg("--version").output().unwrap();
        assert!(compiler.status.success());
        let mut identity = Sha256::new();
        identity.update(archive_sha);
        let builder = include_str!("matrix_product_tests.rs")
            .split_once("\npub(crate) fn query_oracle")
            .unwrap()
            .1
            .split_once("\n#[test]")
            .unwrap()
            .0;
        identity.update(
            builder
                .parse::<proc_macro2::TokenStream>()
                .unwrap()
                .to_string(),
        );
        identity.update(format!("{flags:?}"));
        identity.update(compiler.stdout);
        let compiler_c = Command::new("cc").arg("--version").output().unwrap();
        assert!(compiler_c.status.success());
        identity.update(compiler_c.stdout);
        if let Some(inventory) = std::env::var_os("PRISMPM_SDK_INVENTORY") {
            identity.update(std::fs::read(inventory).unwrap());
        }
        identity.update(std::env::consts::ARCH);
        identity.update(std::fs::read(root.join("prismpm.lock")).unwrap());
        let common_key = format!("{:x}", identity.clone().finalize());
        identity.update(GGML_VECTOR_DRIVER);
        let base = std::env::var_os("CARGO_TARGET_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| root.join("target"))
            .join("ggml-vector-oracle");
        let common = base.join("cpu").join(common_key);
        let cache = base
            .join("program")
            .join(format!("{:x}", identity.finalize()));
        std::fs::create_dir_all(&common).unwrap();
        if !common.join("ggml").exists() {
            assert!(Command::new("tar")
                .args(["-xzf"])
                .arg(&archive)
                .arg("-C")
                .arg(&common)
                .status()
                .unwrap()
                .success());
        }
        let listing = Command::new("tar")
            .arg("-tvzf")
            .arg(&archive)
            .output()
            .unwrap();
        assert!(listing.status.success());
        let mut inputs = Vec::new();
        for line in std::str::from_utf8(&listing.stdout).unwrap().lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let name = fields[5];
            let relevant = name.starts_with("ggml/include/")
                || name.starts_with("ggml/src/ggml-cpu/")
                || name
                    .strip_prefix("ggml/src/")
                    .is_some_and(|tail| !tail.contains('/'));
            if relevant && !name.ends_with('/') {
                assert!(
                    line.starts_with('-') && fields.len() == 6,
                    "non-regular oracle source: {line}"
                );
                inputs.push((name.to_string(), fields[2].parse::<usize>().unwrap()));
            }
        }
        assert!(inputs.len() > 100, "oracle source inventory truncated");
        let authoritative = Command::new("tar")
            .arg("-xOzf")
            .arg(&archive)
            .args(inputs.iter().map(|(name, _)| name))
            .output()
            .unwrap();
        assert!(authoritative.status.success());
        let mut cached = Sha256::new();
        for (name, len) in inputs {
            let bytes = std::fs::read(common.join(name)).unwrap();
            assert_eq!(bytes.len(), len, "oracle source size differs");
            cached.update(bytes);
        }
        assert_eq!(
            cached.finalize(),
            Sha256::digest(authoritative.stdout),
            "oracle source bytes differ"
        );
        let program = cache.join("oracle");
        if !program.exists() {
            std::fs::create_dir_all(&cache).unwrap();
            let inventory_source = cache.join("table-inventory.cpp");
            std::fs::write(&inventory_source,
                "#define GGML_COMMON_DECL_CPP\n#define GGML_COMMON_IMPL_CPP\n#include \"ggml-common.h\"\n").unwrap();
            let inventory = Command::new("c++").args(["-E", "-std=c++17", "-I"])
                .arg(common.join("ggml/src")).arg(&inventory_source).output().unwrap();
            assert!(inventory.status.success(), "{}", String::from_utf8_lossy(&inventory.stderr));
            let mut in_header = false;
            let mut names = std::collections::BTreeSet::new();
            let mut exports = String::new();
            for line in std::str::from_utf8(&inventory.stdout).unwrap().lines() {
                if line.starts_with("# ") {
                    in_header = line.split('"').nth(1).is_some_and(|p| p.ends_with("/ggml-common.h"));
                } else if in_header {
                    if let Some(declaration) = line.trim().strip_prefix("static const ") {
                        let (_, array) = declaration.split_once(' ').expect("unmodeled table declaration");
                        let (name, _) = array.split_once('[').expect("unmodeled table extent");
                        assert!(name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'));
                        assert!(names.insert(name.to_string()), "duplicate reference table");
                        exports.push_str(&format!("emit({name:?}, {name});\n"));
                    }
                }
            }
            assert!(!names.is_empty(), "reference table declarations missing");
            std::fs::write(cache.join("ggml-table-exports.inc"), exports).unwrap();
            std::fs::write(cache.join("driver.cpp"), GGML_VECTOR_DRIVER).unwrap();
            let version = std::fs::read_to_string(common.join("ggml/src/ggml-version.h.in"))
                .unwrap()
                .replace("@GGML_VERSION@", "1.0.0")
                .replace(
                    "@GGML_BUILD_COMMIT@",
                    "9a080f44b503c64f66c84a6274b9d9ee5d5b09d5",
                );
            std::fs::write(common.join("ggml-version.h"), version).unwrap();
            let mut sources = vec![
                "ggml.c".to_string(),
                "ggml-quants.c".to_string(),
                "ggml-alloc.c".to_string(),
                "ggml-backend.cpp".to_string(),
                "ggml-backend-meta.cpp".to_string(),
                "ggml-backend-reg.cpp".to_string(),
                "ggml-backend-dl.cpp".to_string(),
                "ggml-threading.cpp".to_string(),
            ];
            let build_list =
                std::fs::read_to_string(common.join("ggml/src/ggml-cpu/CMakeLists.txt")).unwrap();
            sources.extend(
                build_list
                    .split_once("list (APPEND GGML_CPU_SOURCES")
                    .unwrap()
                    .1
                    .split_once(')')
                    .unwrap()
                    .0
                    .split_whitespace()
                    .filter(|name| name.ends_with(".c") || name.ends_with(".cpp"))
                    .map(str::to_string),
            );
            let architecture = match std::env::consts::ARCH {
                "x86_64" => "x86",
                "aarch64" => "arm",
                other => panic!("unmodeled oracle architecture: {other}"),
            };
            sources.extend([
                format!("ggml-cpu/arch/{architecture}/quants.c"),
                format!("ggml-cpu/arch/{architecture}/repack.cpp"),
            ]);
            let mut objects = Vec::new();
            for source in sources {
                let object = common.join(format!("{}.o", source.replace('/', "_")));
                if !object.exists() {
                    let c = source.ends_with(".c");
                    let mut command = Command::new(if c { "cc" } else { "c++" });
                    command.args(["-O2", "-ffunction-sections", "-fdata-sections"]);
                    command.args(if c {
                        &["-std=c11", "-D_GNU_SOURCE"][..]
                    } else {
                        &["-std=c++17"][..]
                    });
                    for directory in ["ggml/include", "ggml/src", "ggml/src/ggml-cpu", ""] {
                        command.arg("-I").arg(common.join(directory));
                    }
                    let compiled = command
                        .arg("-c")
                        .arg(common.join("ggml/src").join(&source))
                        .arg("-o")
                        .arg(object.with_extension("new"))
                        .output()
                        .unwrap();
                    assert!(
                        compiled.status.success(),
                        "{source}: {}",
                        String::from_utf8_lossy(&compiled.stderr)
                    );
                    std::fs::write(
                        object.with_extension("sha256"),
                        format!(
                            "{:x}",
                            Sha256::digest(std::fs::read(object.with_extension("new")).unwrap())
                        ),
                    )
                    .unwrap();
                    std::fs::rename(object.with_extension("new"), &object).unwrap();
                }
                assert_eq!(
                    format!("{:x}", Sha256::digest(std::fs::read(&object).unwrap())),
                    std::fs::read_to_string(object.with_extension("sha256")).unwrap(),
                    "oracle object differs: {}",
                    object.display()
                );
                objects.push(object);
            }
            let compiled = Command::new("c++")
                .args(flags)
                .arg("-I")
                .arg(common.join("ggml/include"))
                .arg("-I")
                .arg(common.join("ggml/src"))
                .arg("-I")
                .arg(common.join("ggml/src/ggml-cpu"))
                .arg("-I")
                .arg(&common)
                .arg(cache.join("driver.cpp"))
                .args(objects)
                .args(["-pthread", "-lm", "-ldl"])
                .args(["-o"])
                .arg(cache.join("oracle.new"))
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            std::fs::write(
                cache.join("oracle.sha256"),
                format!(
                    "{:x}",
                    Sha256::digest(std::fs::read(cache.join("oracle.new")).unwrap())
                ),
            )
            .unwrap();
            std::fs::rename(cache.join("oracle.new"), &program).unwrap();
        }
        program
    });
    assert_eq!(
        format!("{:x}", Sha256::digest(std::fs::read(program).unwrap())),
        std::fs::read_to_string(program.with_file_name("oracle.sha256")).unwrap()
    );
    let mut child = Command::new(program)
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let input = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        let mut input = input;
        input.write_all(requests.as_bytes()).unwrap();
    });
    let result = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(result.status.success());
    String::from_utf8(result.stdout)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[test]
fn ggml_matrix_contractions_vtk_01() {
    use std::fmt::Write as _;
    let mut requests = String::new();
    let mut actual = Vec::new();
    for rows in [1, 2, 5] {
        for cols in [1, 3, 7] {
            for inner in [0, 1, 2, 3, 7, 16, 33] {
                let left: Vec<_> = (0..rows * inner)
                    .map(|i| ((i * 7 % 17) as f32 - 8.0) / 4.0)
                    .collect();
                let right: Vec<_> = (0..inner * cols)
                    .map(|i| ((i * 11 % 19) as f32 - 9.0) / 4.0)
                    .collect();
                let mut output = vec![99.0; rows * cols];
                super::multiply_matrix_rows_f32(rows, inner, cols, &left, &right, &mut output)
                    .unwrap();
                for row in 0..rows {
                    for col in 0..cols {
                        writeln!(requests, "{inner}").unwrap();
                        for &value in &left[row * inner..(row + 1) * inner] {
                            writeln!(requests, "{}", value.to_bits()).unwrap();
                        }
                        for i in 0..inner {
                            writeln!(requests, "{}", right[i * cols + col].to_bits()).unwrap();
                        }
                        actual.push(output[row * cols + col].to_bits());
                    }
                }
            }
        }
    }
    for (left, right) in [
        (f32::INFINITY, 0.0),
        (f32::INFINITY, -1.0),
        (f32::NAN, 1.0),
        (-0.0, 1.0),
        (f32::from_bits(1), 1.0),
        (f32::MIN_POSITIVE, 0.5),
        (f32::MAX, 2.0),
        (f32::NEG_INFINITY, f32::NEG_INFINITY),
    ] {
        let mut output = [0.0];
        super::multiply_matrix_rows_f32(1, 1, 1, &[left], &[right], &mut output).unwrap();
        writeln!(requests, "1\n{}\n{}", left.to_bits(), right.to_bits()).unwrap();
        actual.push(output[0].to_bits());
    }
    assert_eq!(
        actual.len(),
        624,
        "the full contraction corpus must execute"
    );
    let reference = query_oracle("dot", requests);
    assert_eq!(
        reference.len(),
        actual.len(),
        "upstream oracle omitted a contraction"
    );
    for (index, (expected, actual)) in reference.into_iter().zip(actual).enumerate() {
        if f32::from_bits(expected).is_nan() {
            assert!(
                f32::from_bits(actual).is_nan(),
                "contraction {index} must be NaN"
            );
        } else {
            assert_eq!(actual, expected, "GGML contraction {index}");
        }
    }
}

#[test]
fn shape_certificate_vtk_01() {
    certified_model(
        "verdant_matrix_shape",
        "VerdantEngine.MatrixProduct.matrixProductShapeValid",
    );
}

pub(super) fn certified_model(package: &str, root_name: &str) {
    static VERIFIED: std::sync::OnceLock<std::collections::BTreeMap<String, String>> =
        std::sync::OnceLock::new();
    let bindings = VERIFIED.get_or_init(verify_tensor_bundle);
    assert_eq!(bindings.get(root_name).map(String::as_str), Some(package));
}

fn verify_tensor_bundle() -> std::collections::BTreeMap<String, String> {
    verify_bundle("tensor", "tensor-execution")
}

pub(super) fn certified_quotient_model() {
    static VERIFIED: std::sync::OnceLock<std::collections::BTreeMap<String, String>> =
        std::sync::OnceLock::new();
    let bindings = VERIFIED.get_or_init(|| verify_bundle("quotient", "tensor-quotient"));
    for (root, package) in [
        ("quotientWord", "verdant_quotient_f64"),
        ("classify", "verdant_division_class"),
        ("normalizedOperand", "verdant_division_norm"),
        ("divisionFinish", "verdant_division_finish"),
        ("countPlan", "verdant_count_f64"),
        ("route", "verdant_quotient_route"),
    ] {
        assert_eq!(
            bindings
                .get(&format!("VerdantEngine.F64Quotient.{root}"))
                .map(String::as_str),
            Some(package)
        );
    }
}

fn verify_bundle(scope: &str, directory: &str) -> std::collections::BTreeMap<String, String> {
    use sha2::{Digest, Sha256};
    use std::process::Command;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let bundle = root.join(format!("generated/{directory}"));
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(bundle.join("result.json")).unwrap()).unwrap();
    let checked = Command::new("lexlean")
        .current_dir(root)
        .args([
            "--project",
            &format!("app/{scope}.lexlean.toml"),
            "--diagnostic-format",
            "json",
            "check",
        ])
        .output()
        .unwrap();
    assert!(
        checked.status.success(),
        "{}",
        format!(
            "{}\n{}",
            String::from_utf8_lossy(&checked.stdout),
            String::from_utf8_lossy(&checked.stderr)
        )
    );
    let current: serde_json::Value = serde_json::from_slice(&checked.stdout).unwrap();
    for key in ["source_id", "semantic_id"] {
        assert_eq!(
            current[key], result["source"][key],
            "stale {scope} model: {key}"
        );
    }
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(std::fs::read(root.join(format!("app/{scope}.lexlean.lock"))).unwrap())
        ),
        result["source"]["lock_sha256"].as_str().unwrap()
    );
    let roots = result["roots"].as_array().unwrap();
    let request: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("app/{scope}-production.json"))).unwrap(),
    )
    .unwrap();
    let expected = request["roots"].as_array().unwrap();
    assert_eq!(roots.len(), expected.len());
    for requested in expected {
        let exported = roots
            .iter()
            .find(|r| r["root"] == requested["root"])
            .unwrap();
        let package = requested["package"]["name"].as_str().unwrap();
        let outputs = exported["outputs"].as_array().unwrap();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0]["certification"]["status"], "kernel-checked");
        assert_eq!(outputs[0]["package"]["name"], package);
        assert_eq!(
            outputs[0]["package"]["version"],
            requested["package"]["version"]
        );
        let mut source_found = false;
        for file in outputs[0]["package"]["files"].as_array().unwrap() {
            let path = file["path"].as_str().unwrap();
            let actual = std::fs::read(bundle.join(path)).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&actual)),
                file["sha256"].as_str().unwrap()
            );
            if path == format!("packages/rust-core/{package}/src/lib.rs") {
                let compiled: &[u8] = match package {
                    "verdant_matrix_shape" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_matrix_shape/src/lib.rs"),
                    "verdant_scalar_f32" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_scalar_f32/src/lib.rs"),
                    "verdant_storage_transfer" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_storage_transfer/src/lib.rs"),
                    "verdant_float_storage" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_float_storage/src/lib.rs"),
                    "verdant_float_transform" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_float_transform/src/lib.rs"),
                    "verdant_unary_plan" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_unary_plan/src/lib.rs"),
                    "verdant_unary_select" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_unary_select/src/lib.rs"),
                    "verdant_unary_activate" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_unary_activate/src/lib.rs"),
                    "verdant_unary_round" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_unary_round/src/lib.rs"),
                    "verdant_count_f32" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_count_f32/src/lib.rs"),
                    "verdant_narrow_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_narrow_f64/src/lib.rs"),
                    "verdant_normalize_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_normalize_f64/src/lib.rs"),
                    "verdant_add_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_add_f64/src/lib.rs"),
                    "verdant_round_mantissa_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_round_mantissa_f64/src/lib.rs"),
                    "verdant_widen_f32" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_widen_f32/src/lib.rs"),
                    "verdant_high_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_high_f64/src/lib.rs"),
                    "verdant_low_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_low_f64/src/lib.rs"),
                    "verdant_wide_mode" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_wide_mode/src/lib.rs"),
                    "verdant_packed_field" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_packed_field/src/lib.rs"),
                    "verdant_integer_f32" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_integer_f32/src/lib.rs"),
                    "verdant_small_float" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_small_float/src/lib.rs"),
                    "verdant_encoded_word" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_encoded_word/src/lib.rs"),
                    "verdant_pack_plan_f64" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_pack_plan_f64/src/lib.rs"),
                    "verdant_storage_block" => include_bytes!("../../../../generated/tensor-execution/packages/rust-core/verdant_storage_block/src/lib.rs"),
                    "verdant_count_f64" => include_bytes!("../../../../generated/tensor-quotient/packages/rust-core/verdant_count_f64/src/lib.rs"),
                    "verdant_quotient_f64" => include_bytes!("../../../../generated/tensor-quotient/packages/rust-core/verdant_quotient_f64/src/lib.rs"),
                    "verdant_quotient_route" => include_bytes!("../../../../generated/tensor-quotient/packages/rust-core/verdant_quotient_route/src/lib.rs"),
                    "verdant_division_class" => include_bytes!("../../../../generated/tensor-quotient/packages/rust-core/verdant_division_class/src/lib.rs"),
                    "verdant_division_norm" => include_bytes!("../../../../generated/tensor-quotient/packages/rust-core/verdant_division_norm/src/lib.rs"),
                    "verdant_division_finish" => include_bytes!("../../../../generated/tensor-quotient/packages/rust-core/verdant_division_finish/src/lib.rs"),
                    "verdant_symmetric_q8" => include_bytes!("../../../../generated/tensor-q8/packages/rust-core/verdant_symmetric_q8/src/lib.rs"),
                    "verdant_exponential_f32" => include_bytes!("../../../../generated/tensor-exponential/packages/rust-core/verdant_exponential_f32/src/lib.rs"),
                    "verdant_exponential_step" => include_bytes!("../../../../generated/tensor-exponential/packages/rust-core/verdant_exponential_step/src/lib.rs"),
                    "verdant_exponential_scale" => include_bytes!("../../../../generated/tensor-exponential/packages/rust-core/verdant_exponential_scale/src/lib.rs"),
                    "verdant_swiglu" => include_bytes!("../../../../generated/tensor-swiglu/packages/rust-core/verdant_swiglu/src/lib.rs"),
                    "verdant_packed_dot" => include_bytes!("../../../../generated/tensor-packed/packages/rust-core/verdant_packed_dot/src/lib.rs"),
                    "verdant_symmetric_dot" => include_bytes!("../../../../generated/tensor-dot/packages/rust-core/verdant_symmetric_dot/src/lib.rs"),
                    "verdant_symmetric_dot_layout" => include_bytes!("../../../../generated/tensor-dot/packages/rust-core/verdant_symmetric_dot_layout/src/lib.rs"),
                    _ => panic!("unknown certified dependency"),
                };
                assert_eq!(actual, compiled);
                source_found = true;
            }
        }
        assert!(source_found);
    }
    let verified = Command::new("lexlean")
        .args(["production", "verify-artifact"])
        .arg(&bundle)
        .output()
        .unwrap();
    assert!(
        verified.status.success(),
        "{}",
        String::from_utf8_lossy(&verified.stderr)
    );
    roots
        .iter()
        .map(|root| {
            (
                root["root"].as_str().unwrap().to_string(),
                root["outputs"][0]["package"]["name"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        })
        .collect()
}

#[test]
fn rectangular_product_vtk_01() {
    let mut output = [99.0_f32; 6];
    super::multiply_matrix_rows_f32(
        2,
        2,
        3,
        &[1.0, 2.0, 3.0, 4.0],
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        &mut output,
    )
    .unwrap();
    assert_eq!(output, [9.0, 12.0, 15.0, 19.0, 26.0, 33.0]);
    assert!(super::multiply_matrix_rows_f32(2, 2, 3, &[], &[], &mut output).is_err());
    assert_eq!(output, [9.0, 12.0, 15.0, 19.0, 26.0, 33.0]);
    assert!(super::multiply_matrix_rows_f32(usize::MAX, 2, 3, &[], &[], &mut output).is_err());
    let mut scalar = [99.0_f32];
    super::multiply_matrix_rows_f32(1, 1, 1, &[f32::from_bits(1)], &[1.0], &mut scalar).unwrap();
    assert_eq!(scalar[0].to_bits(), 1);
    super::multiply_matrix_rows_f32(1, 1, 1, &[f32::INFINITY], &[0.0], &mut scalar).unwrap();
    assert!(scalar[0].is_nan());
    super::multiply_matrix_rows_f32(1, 1, 1, &[f32::INFINITY], &[-1.0], &mut scalar).unwrap();
    assert_eq!(scalar[0], f32::NEG_INFINITY);
    super::multiply_matrix_rows_f32(0, 2, 3, &[], &[1.0; 6], &mut []).unwrap();
    super::multiply_matrix_rows_f32(2, 0, 3, &[], &[], &mut output).unwrap();
    assert_eq!(output, [0.0; 6]);
}

#[test]
fn symmetric_block_encoding_vtk_16() {
    let bindings = verify_bundle("q8", "tensor-q8");
    assert_eq!(
        bindings["VerdantEngine.SymmetricBlock.plan"],
        "verdant_symmetric_q8"
    );
    use std::fmt::Write;
    let mut blocks = vec![[0.0; 32], [-0.0; 32], [f32::from_bits(1); 32]];
    for maximum in [0.125, 1.0, 127.0, 254.0, 16_384.0, 16_777_216.0] {
        for sign in [-1.0, 1.0] {
            let mut block = std::array::from_fn(|i| sign * (i as f32 - 15.5) * maximum / 127.0);
            block[31] = sign * maximum;
            blocks.push(block);
        }
    }
    let mut state = 42u32;
    for _ in 0..256 {
        blocks.push(std::array::from_fn(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let exponent = 77 + (state >> 24) % 100;
            f32::from_bits((state & 0x807f_ffff) | (exponent << 23))
        }));
    }
    let mut input = String::new();
    let mut actual = Vec::new();
    for block in blocks {
        for value in block {
            writeln!(input, "{}", value.to_bits()).unwrap();
        }
        let encoded = super::encode_symmetric_q8_block(&block).unwrap();
        assert_eq!(std::mem::size_of_val(&encoded), 34);
        actual.extend(encoded.scale.to_ne_bytes().map(u32::from));
        actual.extend(encoded.codes.map(|code| u32::from(code as u8)));
    }
    let reference = query_oracle("symmetric-q8", input);
    assert_eq!(actual.len(), reference.len());
    for (index, (byte, expected)) in actual.into_iter().zip(reference).enumerate() {
        assert_eq!(byte, expected, "block {} byte {}", index / 34, index % 34);
    }
    assert!(matches!(
        super::encode_symmetric_q8_block(&[0.0; 31]),
        Err(crate::TensorComputeError::InvalidKernelSpecification { .. })
    ));
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 1e-38] {
        let mut block = [0.0; 32];
        block[31] = invalid;
        assert!(matches!(
            super::encode_symmetric_q8_block(&block),
            Err(crate::TensorComputeError::InvalidKernelSpecification { .. })
        ));
    }
}

#[test]
fn symmetric_row_dot_vtk_17() {
    check_symmetric_rows(false);
}

#[test]
fn packed_row_dot_vtk_18() {
    check_symmetric_rows(true);
}

fn check_symmetric_rows(packed: bool) {
    if packed {
        assert_eq!(
            verify_bundle("packed", "tensor-packed")["VerdantEngine.PackedDot.plan"],
            "verdant_packed_dot"
        );
    } else {
        let bindings = verify_bundle("dot", "tensor-dot");
        assert_eq!(
            bindings["VerdantEngine.SymmetricDot.accumulate"],
            "verdant_symmetric_dot"
        );
        assert_eq!(
            bindings["VerdantEngine.SymmetricDot.rowBlockBytes"],
            "verdant_symmetric_dot_layout"
        );
    }
    use std::fmt::Write;
    let mut input = String::new();
    let mut actual = Vec::new();
    let mut state = 17u32;
    for case in 0..256usize {
        let count = case % 17;
        writeln!(input, "{count}").unwrap();
        let mut weights = Vec::new();
        let mut activations = Vec::new();
        for block in 0..count {
            for side in 0..2usize {
                let scale = match case % 8 {
                    0 => [0x8000, 0x3c00][side],
                    1 => [0x7c00, 0][side],
                    2 => [1, 0x7bff][side],
                    3 => 0x7e32,
                    _ => {
                        ((case * 211 + block * 1311 + side * 10003) % 0x7c00) as u16
                            | if (block + side) % 3 == 0 { 0x8000 } else { 0 }
                    }
                };
                let codes: [i8; 32] = std::array::from_fn(|lane| {
                    state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                    match case % 7 {
                        0 => -128,
                        1 => {
                            if (lane + side) % 2 == 0 {
                                -128
                            } else {
                                127
                            }
                        }
                        _ => (state >> 24) as i8,
                    }
                });
                writeln!(input, "{scale}").unwrap();
                let stored = if packed && side == 0 {
                    &codes[..16]
                } else {
                    &codes[..]
                };
                for &code in stored {
                    writeln!(input, "{}", code as u8).unwrap();
                }
                if side == 0 {
                    weights.extend(scale.to_ne_bytes());
                    weights.extend(stored.iter().map(|&code| code as u8));
                } else {
                    activations.push(super::SymmetricQ8Block { scale, codes });
                }
            }
        }
        let value = if packed {
            super::dot_symmetric_q4_row(&weights, &activations)
        } else {
            super::dot_symmetric_q8_row(&weights, &activations)
        };
        actual.push(value.unwrap());
    }
    let expected = query_oracle(
        if packed {
            "packed-dot"
        } else {
            "symmetric-dot"
        },
        input,
    );
    assert_eq!(actual.len(), expected.len());
    for (case, (got, want)) in actual.into_iter().zip(expected).enumerate() {
        if f32::from_bits(want).is_nan() {
            assert!(got.is_nan(), "row {case}");
        } else {
            assert_eq!(got.to_bits(), want, "row {case}");
        }
    }
    let block = super::SymmetricQ8Block {
        scale: 0,
        codes: [0; 32],
    };
    let width = if packed { 18 } else { 34 };
    let dot = if packed {
        super::dot_symmetric_q4_row
    } else {
        super::dot_symmetric_q8_row
    };
    for bytes in [0, width - 1, width + 1, width * 2] {
        assert!(matches!(
            dot(&vec![0; bytes], &[block]),
            Err(crate::TensorComputeError::ZeroOrMismatchedExtents { .. })
        ));
    }
    let wrong = super::SymmetricQ8Block {
        scale: 0,
        codes: [0; 31],
    };
    let dot = if packed {
        super::dot_symmetric_q4_row
    } else {
        super::dot_symmetric_q8_row
    };
    assert!(dot(&vec![0; width], &[wrong]).is_err());
}

#[test]
fn natural_exponential_vtk_19() {
    assert_eq!(
        verify_bundle("exponential", "tensor-exponential")
            ["VerdantEngine.NaturalExponential.prepare"],
        "verdant_exponential_f32"
    );
    use std::fmt::Write;
    let mut words = Vec::new();
    for sign in [0, 1u32 << 31] {
        for exponent in 0..256 {
            for fraction in [0, 1, 0x3fffff, 0x400000, 0x7ffffe, 0x7fffff] {
                words.push(sign | (exponent << 23) | fraction);
            }
        }
    }
    for step in -28672..=24576 {
        let center = (step as f32 / 256.0).to_bits();
        words.extend([center.wrapping_sub(1), center, center + 1]);
    }
    for power in -151..=128 {
        let center = (f64::from(power) * std::f64::consts::LN_2) as f32;
        let word = center.to_bits();
        words.extend(
            (0..8).flat_map(|offset| [word.wrapping_add(offset), word.wrapping_sub(offset)]),
        );
    }
    words.sort_unstable();
    words.dedup();
    assert_eq!(words.len(), 166_856);
    let mut input = String::new();
    for &word in &words {
        writeln!(input, "{word}").unwrap();
    }
    let reference = query_oracle("natural-exp", input);
    assert_eq!(reference.len(), words.len());
    for (word, expected) in words.into_iter().zip(reference) {
        let actual = super::exponential_f32(f32::from_bits(word))
            .unwrap()
            .to_bits();
        let result = f32::from_bits(expected);
        if result.is_nan() {
            assert!(f32::from_bits(actual).is_nan(), "exp({word:#010x})");
        } else if result == 0.0 || result.is_infinite() {
            assert_eq!(actual, expected, "exp({word:#010x}) endpoint");
        } else {
            assert!(
                actual > 0 && actual < f32::INFINITY.to_bits() && actual.abs_diff(expected) <= 1,
                "exp({word:#010x}): {actual:#010x} versus {expected:#010x}"
            );
        }
    }
}

#[test]
fn gated_product_vtk_20() {
    assert_eq!(
        verify_bundle("swiglu", "tensor-swiglu")["VerdantEngine.SwiGlu.step"],
        "verdant_swiglu"
    );
    use std::fmt::Write;
    let gates = [
        0u32,
        1 << 31,
        1,
        0x00800000,
        0xbf800000,
        0x3f800000,
        0x3eaaaaab,
        0xc0b00000,
        0x7f7fffff,
        0x7f800000,
        0xff800000,
        0x7fc00042,
    ];
    let mut input = String::new();
    let mut operands = Vec::new();
    let specials = gates.into_iter().map(f32::from_bits);
    let finite = (-4608..=4608).map(|step| step as f32 / 32.0);
    let mut inputs: Vec<_> = specials.chain(finite).map(f32::to_bits).collect();
    inputs.sort_unstable();
    inputs.dedup();
    for x in inputs.into_iter().map(f32::from_bits) {
        for g in gates {
            writeln!(input, "{} {g}", x.to_bits()).unwrap();
            operands.push((x, f32::from_bits(g)));
        }
    }
    let expected = query_oracle("swiglu-scalar", input);
    assert_eq!(operands.len(), 110_700);
    assert_eq!(operands.len(), expected.len());
    for ((x, g), reference) in operands.into_iter().zip(expected) {
        let actual = super::swiglu_f32(x, g).unwrap();
        let wanted = f32::from_bits(reference);
        if wanted.is_nan() {
            assert!(actual.is_nan(), "swiglu({x}, {g})");
        } else if wanted == 0.0 || wanted.is_infinite() {
            assert_eq!(actual.to_bits(), reference, "swiglu({x}, {g})");
        } else {
            assert!(
                actual.is_finite()
                    && actual.is_sign_negative() == wanted.is_sign_negative()
                    && actual.to_bits().abs_diff(reference) <= 4,
                "swiglu({x}, {g}): {actual:?} versus {wanted:?}"
            );
        }
    }
}

#[test]
fn circular_functions_vtk_21() {
    use std::fmt::Write;
    let mut words = Vec::new();
    for exponent in 0..256u32 {
        for index in 0..128u32 {
            let fraction = index.wrapping_mul(2654435761) & 0x007fffff;
            let word = (exponent << 23) | fraction;
            words.extend([word, word | (1 << 31), word | 0x007fffff]);
        }
    }
    for multiple in -4096..=4096 {
        let center = ((std::f64::consts::FRAC_PI_2 * f64::from(multiple)) as f32).to_bits();
        words.extend((0..=4).flat_map(|d| [center.wrapping_sub(d), center.wrapping_add(d)]));
    }
    words.sort_unstable();
    words.dedup();
    let mut input = String::new();
    for word in &words {
        writeln!(input, "{word}").unwrap();
    }
    let reference = query_oracle("sincos", input);
    assert_eq!(reference.len(), words.len() * 2);
    for (word, pair) in words.into_iter().zip(reference.chunks_exact(2)) {
        let (sine, cosine) = super::sin_cos_f32(f32::from_bits(word)).unwrap();
        for (value, &expected) in [sine, cosine].into_iter().zip(pair) {
            let target = f32::from_bits(expected);
            if target.is_nan() {
                assert!(value.is_nan(), "nonfinite input {word:#010x}");
            } else if target == 0.0 {
                assert_eq!(value.to_bits(), expected, "signed zero at {word:#010x}");
            } else {
                assert!(
                    value.is_finite()
                        && value.is_sign_negative() == target.is_sign_negative()
                        && value.to_bits().abs_diff(expected) <= 1,
                    "input {word:#010x}: {:#010x} versus {expected:#010x}",
                    value.to_bits()
                );
            }
        }
    }
}
