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
template<int Lanes, class Block>
static int encode_blocks(void (*encode)(const float *, Block *, int64_t)) {
    uint32_t word;
    while (std::scanf("%u", &word) == 1) {
      float block[Lanes];
      for (int i = 0; i < Lanes; ++i) {
        if (i && std::scanf("%u", &word) != 1) return 2;
        std::memcpy(&block[i], &word, sizeof(word));
      }
      Block encoded{};
      encode(block, &encoded, Lanes);
      const auto *bytes = reinterpret_cast<const uint8_t *>(&encoded);
      for (size_t i = 0; i < sizeof(encoded); ++i) std::printf("%u\n", unsigned(bytes[i]));
    }
    return std::ferror(stdin) ? 3 : 0;
}
// Observe the scale computed by the imported rotary kernel, preserving the real call.
extern "C" float __real_powf(float, float);
static bool observe_power = false;
static std::vector<float> *head_powers = nullptr;
static unsigned power_calls = 0;
static float observed_base, observed_exponent, observed_ratio;
extern "C" float __wrap_powf(float base, float exponent) {
  const float ratio = __real_powf(base, exponent);
  if (head_powers) head_powers->push_back(ratio);
  if (observe_power) {
    ++power_calls; observed_base=base; observed_exponent=exponent; observed_ratio=ratio;
  }
  return ratio;
}
int main(int argc, char **argv) {
  if (argc != 2) return 1;
  ggml_cpu_init();
  if (!std::strcmp(argv[1], "logarithm")) {
    unsigned word;
    while (std::scanf("%u", &word) == 1) {
      float x, y; std::memcpy(&x, &word, 4); ggml_vec_log_f32(1, &y, &x);
      std::memcpy(&word, &y, 4); std::printf("%u\n",word);
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "normalized-exp")) {
    unsigned width, present, sink_bits;
    while (std::scanf("%u %u %u", &width, &present, &sink_bits) == 3) {
      if (!width || width > 4096 || present > 1) return 3;
      auto *ctx = ggml_init({1024*1024, nullptr, false});
      auto *src = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, width);
      for (unsigned i = 0; i < width; ++i) {
        uint32_t word;
        if (std::scanf("%u", &word) != 1) return 2;
        std::memcpy(static_cast<float*>(src->data) + i, &word, sizeof(word));
      }
      auto *out = ggml_soft_max(ctx, src);
      if (present) {
        auto *extra = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 1);
        std::memcpy(extra->data, &sink_bits, sizeof(sink_bits));
        ggml_soft_max_add_sinks(out, extra);
      }
      std::vector<float> scratch(width + 256);
      ggml_compute_params params{}; params.nth = 1;
      params.wdata = scratch.data(); params.wsize = scratch.size()*sizeof(float);
      ggml_compute_forward_soft_max(&params, out);
      for (unsigned i = 0; i < width; ++i) {
        uint32_t word; std::memcpy(&word, static_cast<float*>(out->data) + i, sizeof(word));
        std::printf("%u\n", word);
      }
      ggml_free(ctx);
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "row-rank")) {
    unsigned width, count, descending;
    while (std::scanf("%u %u %u", &width, &count, &descending)==3) {
      if (!count || count>width || width>4096 || descending>1 || (count!=width && !descending)) return 3;
      auto *ctx=ggml_init({1024*1024,nullptr,false});
      auto *src=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,width);
      for (unsigned i=0;i<width;++i) {
        uint32_t bits;
        if (std::scanf("%u",&bits)!=1) return 2;
        std::memcpy(static_cast<float*>(src->data)+i,&bits,4);
      }
      std::vector<int32_t> scratch(width+256);
      ggml_compute_params params{};params.nth=1;params.wdata=scratch.data();params.wsize=scratch.size()*sizeof(int32_t);
      auto *out=count==width?ggml_argsort(ctx,src,descending?GGML_SORT_ORDER_DESC:GGML_SORT_ORDER_ASC):ggml_top_k(ctx,src,count);
      if (count==width) ggml_compute_forward_argsort(&params,out);
      else ggml_compute_forward_top_k(&params,out);
      for (unsigned i=0;i<count;++i) std::printf("%u\n",static_cast<uint32_t>(static_cast<int32_t*>(out->data)[i]));
      ggml_free(ctx);
    }
    return std::feof(stdin)?0:2;
  }
  if (!std::strcmp(argv[1], "head-slopes")) {
    unsigned bits, heads;
    while (std::scanf("%u %u", &bits, &heads)==2) {
      if (!heads || heads>2048) return 3;
      float bias; std::memcpy(&bias,&bits,4);
      if (!(bias>0.0f)) return 4;
      auto *ctx=ggml_init({1024*1024,nullptr,false});
      auto *input=ggml_new_tensor_3d(ctx,GGML_TYPE_F32,2,1,heads);
      auto *mask=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,2);
      std::memset(input->data,0,ggml_nbytes(input));
      static_cast<float*>(mask->data)[0]=0; static_cast<float*>(mask->data)[1]=1;
      auto *out=ggml_soft_max_ext(ctx,input,mask,1.0f,bias);
      std::vector<float> scratch(128),powers; ggml_compute_params params{};
      params.nth=1; params.wdata=scratch.data(); params.wsize=scratch.size()*sizeof(float);
      head_powers=&powers; ggml_compute_forward_soft_max(&params,out); head_powers=nullptr;
      // Compilers may implement powers of two as exp2f. The final head-count
      // calls always compute the slopes from a runtime base in the CPU kernel.
      if (powers.size()<heads || powers.size()>heads+2) return 5;
      for(size_t i=powers.size()-heads;i<powers.size();++i) {
        std::memcpy(&bits,&powers[i],4); std::printf("%u\n",bits);
      }
      ggml_free(ctx);
    }
    return std::feof(stdin)?0:2;
  }
  if (!std::strcmp(argv[1], "rotary-ratio")) {
    unsigned word, degree;
    while(std::scanf("%u %u", &word, &degree)==2) {
      if(!degree || degree>2048) return 3;
      float base; std::memcpy(&base,&word,4);
      const int dims=2*int(degree);
      ggml_init_params init={256*1024,nullptr,false}; auto *ctx=ggml_init(init);
      auto *source=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,dims);
      auto *pos=ggml_new_tensor_1d(ctx,GGML_TYPE_I32,1);
      std::memset(source->data,0,ggml_nbytes(source)); *static_cast<int32_t*>(pos->data)=1;
      auto *out=ggml_rope_ext(ctx,source,pos,nullptr,dims,0,512,base,1.0f,0.0f,1.0f,32.0f,1.0f);
      std::vector<float> cache(dims+64); ggml_compute_params params{};
      params.ith=0; params.nth=1; params.wdata=cache.data(); params.wsize=cache.size()*sizeof(float);
      power_calls=0; observe_power=true; ggml_compute_forward_rope(&params,out); observe_power=false;
      if(power_calls!=1 || observed_base!=base || observed_exponent!=-1.0f/degree) return 4;
      std::memcpy(&word,&observed_ratio,4); std::printf("%u\n",word); ggml_free(ctx);
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "weighted-mean")) {
    int count, width, with_sink;
    unsigned sink_word;
    while (std::scanf("%d %d %d %u", &count, &width, &with_sink, &sink_word) == 4) {
      if (count < 1 || count > 128 || width < 1 || width > 128) return 2;
      ggml_init_params init{2*1024*1024, nullptr, false};
      auto *ctx = ggml_init(init);
      auto *q = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 1);
      auto *k = ggml_new_tensor_2d(ctx, GGML_TYPE_F32, 1, count);
      auto *v = ggml_new_tensor_2d(ctx, GGML_TYPE_F32, width, count);
      auto *mask = ggml_new_tensor_1d(ctx, GGML_TYPE_F16, count);
      *static_cast<float *>(q->data) = 1.0f;
      for (int i = 0; i < count; ++i) {
        unsigned score, included;
        if (std::scanf("%u %u", &score, &included) != 2) return 2;
        std::memcpy(static_cast<float *>(k->data) + i, &score, 4);
        static_cast<ggml_fp16_t *>(mask->data)[i] = included ? 0 : 0xfc00;
        for (int j = 0; j < width; ++j) {
          unsigned word;
          if (std::scanf("%u", &word) != 1) return 2;
          std::memcpy(static_cast<float *>(v->data) + i*width + j, &word, 4);
        }
      }
      auto *dst = ggml_flash_attn_ext(ctx, q, k, v, mask, 1.0f, 0.0f, 0.0f);
      if (with_sink) {
        auto *sink = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 1);
        std::memcpy(sink->data, &sink_word, 4);
        ggml_flash_attn_ext_add_sinks(dst, sink);
      }
      auto *graph = ggml_new_graph_custom(ctx, 32, false);
      ggml_build_forward_expand(graph, dst);
      if (ggml_graph_compute_with_ctx(ctx, graph, 1) != GGML_STATUS_SUCCESS) return 3;
      for (int j = 0; j < width; ++j) {
        unsigned word;
        std::memcpy(&word, static_cast<float *>(dst->data) + j, 4);
        std::printf("%u\n", word);
      }
      ggml_free(ctx);
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "plane-rotation")) {
    int dims, position, inverse, truncate;
    unsigned scale_bits, extension_bits, magnitude_bits;
    const auto decode = [](unsigned bits) { float v; std::memcpy(&v, &bits, 4); return v; };
    const auto output = [](float v) { unsigned bits; std::memcpy(&bits, &v, 4); std::printf("%u\n", bits); };
    while (std::scanf("%d %d %d %d %u %u %u", &dims, &position, &inverse, &truncate,
                      &scale_bits, &extension_bits, &magnitude_bits) == 7) {
      if (dims < 2 || dims % 2 || dims > 128) return 2;
      ggml_init_params init{256*1024, nullptr, false};
      auto *ctx = ggml_init(init);
      auto *src = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, dims);
      auto *pos = ggml_new_tensor_1d(ctx, GGML_TYPE_I32, 1);
      auto *factors = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, dims/2);
      *static_cast<int32_t *>(pos->data) = position;
      for (int j=0; j < dims; ++j) static_cast<float *>(src->data)[j] = float((7*j)%17-8)/4;
      for (int j=0; j < dims/2; ++j) static_cast<float *>(factors->data)[j] = 1.0f + float(j%5)/4;
      auto *dst = inverse
          ? ggml_rope_ext_back(ctx, src, pos, factors, dims, 0, 512, 10000,
                               decode(scale_bits), decode(extension_bits), decode(magnitude_bits), 32, 1)
          : ggml_rope_ext(ctx, src, pos, factors, dims, 0, 512, 10000,
                          decode(scale_bits), decode(extension_bits), decode(magnitude_bits), 32, 1);
      ggml_rope_set_truncate(dst, truncate != 0);
      ggml_compute_params params{};
      std::vector<float> cache(dims+64);
      params.nth=1; params.wdata=cache.data(); params.wsize=cache.size()*sizeof(float);
      if (inverse) ggml_compute_forward_rope_back(&params, dst);
      else ggml_compute_forward_rope(&params, dst);
      for (int j=0; j<dims; j+=2) {
        output(cache[j]); output(cache[j+1]);
        output(static_cast<float *>(dst->data)[j]); output(static_cast<float *>(dst->data)[j+1]);
      }
      ggml_free(ctx);
    }
    return std::feof(stdin) ? 0 : 2;
  }
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
  if (!std::strcmp(argv[1], "gelu")) {
    unsigned bits;
    while (std::scanf("%u", &bits) == 1) {
      float x;
      std::memcpy(&x, &bits, sizeof(x));
      for (float value : {ggml_gelu_f32(x), ggml_gelu_quick_f32(x)}) {
        std::memcpy(&bits, &value, sizeof(bits));
        std::printf("%u\n", bits);
      }
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "tanh")) {
    unsigned bits;
    while (std::scanf("%u", &bits) == 1) {
      float x, result;
      std::memcpy(&x, &bits, sizeof(x));
      ggml_vec_tanh_f32(1, &result, &x);
      std::memcpy(&bits, &result, sizeof(bits));
      std::printf("%u\n", bits);
    }
    return std::feof(stdin) ? 0 : 2;
  }
  if (!std::strcmp(argv[1], "logistic")) {
    unsigned bits;
    while (std::scanf("%u", &bits) == 1) {
      float x, sigmoid;
      std::memcpy(&x, &bits, sizeof(x));
      ggml_vec_sigmoid_f32(1, &sigmoid, &x);
      float silu = ggml_silu_f32(x);
      for (float result : {sigmoid, silu}) {
        std::memcpy(&bits, &result, sizeof(bits));
        std::printf("%u\n", bits);
      }
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
  if (!std::strcmp(argv[1], "grouped-dot")) {
    unsigned count, word;
    const auto dot = ggml_get_type_traits_cpu(GGML_TYPE_Q2_K)->vec_dot;
    while (std::scanf("%u", &count) == 1) {
      std::vector<block_q2_K> weights(count+1);
      std::vector<block_q8_K> activations(count+1);
      for (unsigned i=0; i<count; ++i) {
        for (size_t j=0; j<sizeof(block_q2_K); ++j) {
          if (std::scanf("%u",&word)!=1) return 2;
          reinterpret_cast<uint8_t*>(&weights[i])[j]=uint8_t(word);
        }
        float lanes[QK_K];
        for (float &lane:lanes) {
          if (std::scanf("%u",&word)!=1) return 2;
          std::memcpy(&lane,&word,sizeof(word));
        }
        quantize_row_q8_K_ref(lanes,&activations[i],QK_K);
      }
      float result;
      dot(int(count)*QK_K,&result,0,weights.data(),0,activations.data(),0,1);
      std::memcpy(&word,&result,sizeof(word)); std::printf("%u\n",word);
    }
    return std::ferror(stdin) ? 3 : 0;
  }
  if (!std::strcmp(argv[1], "symmetric-dot")) return compare_rows<block_q8_0>(GGML_TYPE_Q8_0);
  if (!std::strcmp(argv[1], "packed-dot")) return compare_rows<block_q4_0>(GGML_TYPE_Q4_0);
  if (!std::strcmp(argv[1], "symmetric-q8")) return encode_blocks<QK8_0>(quantize_row_q8_0_ref);
  if (!std::strcmp(argv[1], "grouped-q8")) return encode_blocks<QK_K>(quantize_row_q8_K_ref);
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
                .args(["-pthread", "-lm", "-ldl", "-Wl,--wrap=powf"])
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
                    "verdant_circular_magnitudes" => include_bytes!("../../../../generated/tensor-circular/packages/rust-core/verdant_circular_magnitudes/src/lib.rs"),
                    "verdant_circular_orient" => include_bytes!("../../../../generated/tensor-circular/packages/rust-core/verdant_circular_orient/src/lib.rs"),
                    "verdant_circular_prepare" => include_bytes!("../../../../generated/tensor-circular/packages/rust-core/verdant_circular_prepare/src/lib.rs"),
                    "verdant_circular_reduce" => include_bytes!("../../../../generated/tensor-circular/packages/rust-core/verdant_circular_reduce/src/lib.rs"),
                    "verdant_circular_square" => include_bytes!("../../../../generated/tensor-circular/packages/rust-core/verdant_circular_square/src/lib.rs"),
                    "verdant_logarithmic_admit" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_admit/src/lib.rs"),
                    "verdant_logarithmic_inverse" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_inverse/src/lib.rs"),
                    "verdant_logarithmic_prepare" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_prepare/src/lib.rs"),
                    "verdant_logarithmic_product" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_product/src/lib.rs"),
                    "verdant_logarithmic_scale" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_scale/src/lib.rs"),
                    "verdant_logarithmic_series" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_series/src/lib.rs"),
                    "verdant_logarithmic_term" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_term/src/lib.rs"),
                    "verdant_logarithmic_word" => include_bytes!("../../../../generated/tensor-logscale/packages/rust-core/verdant_logarithmic_word/src/lib.rs"),
                    "verdant_weighted_blend" => include_bytes!("../../../../generated/tensor-weighted/packages/rust-core/verdant_weighted_blend/src/lib.rs"),
                    "verdant_weighted_pivot" => include_bytes!("../../../../generated/tensor-weighted/packages/rust-core/verdant_weighted_pivot/src/lib.rs"),
                    "verdant_weighted_reciprocal" => include_bytes!("../../../../generated/tensor-weighted/packages/rust-core/verdant_weighted_reciprocal/src/lib.rs"),
                    "verdant_rotation_geometry" => include_bytes!("../../../../generated/tensor-plane/packages/rust-core/verdant_rotation_geometry/src/lib.rs"),
                    "verdant_rotation_limit" => include_bytes!("../../../../generated/tensor-plane/packages/rust-core/verdant_rotation_limit/src/lib.rs"),
                    "verdant_rotation_signed" => include_bytes!("../../../../generated/tensor-plane/packages/rust-core/verdant_rotation_signed/src/lib.rs"),
                    "verdant_rotation_step" => include_bytes!("../../../../generated/tensor-plane/packages/rust-core/verdant_rotation_step/src/lib.rs"),
                    "verdant_grouped_dot" => include_bytes!("../../../../generated/tensor-groupeddot/packages/rust-core/verdant_grouped_dot/src/lib.rs"),
                    "verdant_grouped_q8" => include_bytes!("../../../../generated/tensor-grouped/packages/rust-core/verdant_grouped_q8/src/lib.rs"),
                    "verdant_tanh" => include_bytes!("../../../../generated/tensor-tanh/packages/rust-core/verdant_tanh/src/lib.rs"),
                    "verdant_row_ranking" => include_bytes!("../../../../generated/tensor-rowrank/packages/rust-core/verdant_row_ranking/src/lib.rs"),
                    "verdant_head_slope_index" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_index/src/lib.rs"),
                    "verdant_head_slope_group" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_group/src/lib.rs"),
                    "verdant_head_slope_exponent" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_exponent/src/lib.rs"),
                    "verdant_head_slope_base" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_base/src/lib.rs"),
                    "verdant_head_slope_power" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_power/src/lib.rs"),
                    "verdant_head_slope_multiply" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_multiply/src/lib.rs"),
                    "verdant_head_slope_term" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_term/src/lib.rs"),
                    "verdant_head_slope_word" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_word/src/lib.rs"),
                    "verdant_head_slope_normalize" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_normalize/src/lib.rs"),
                    "verdant_head_slope_digit" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_digit/src/lib.rs"),
                    "verdant_head_slope_limbs" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope_limbs/src/lib.rs"),
                    "verdant_head_slope" => include_bytes!("../../../../generated/tensor-slopes/packages/rust-core/verdant_head_slope/src/lib.rs"),
                    "verdant_gelu" => include_bytes!("../../../../generated/tensor-gelu/packages/rust-core/verdant_gelu/src/lib.rs"),
                    "verdant_logistic" => include_bytes!("../../../../generated/tensor-logistic/packages/rust-core/verdant_logistic/src/lib.rs"),
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
fn grouped_block_encoding_vtk_26() {
    assert_eq!(
        verify_bundle("grouped", "tensor-grouped")["VerdantEngine.GroupedBlock.plan"],
        "verdant_grouped_q8"
    );
    use std::fmt::Write;
    let mut blocks = vec![[0.0; 256], [-0.0; 256]];
    for sign in [-1.0, 1.0] {
        let mut block = std::array::from_fn(|i| sign * ((i % 127) as f32 - 63.5));
        block[0] = sign * 127.0;
        block[255] = -block[0];
        blocks.push(block);
    }
    let mut state = 42u32;
    for _ in 0..128 {
        blocks.push(std::array::from_fn(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f32::from_bits((state & 0x807f_ffff) | ((77 + (state >> 24) % 100) << 23))
        }));
    }
    let mut input = String::new();
    let mut bytes = Vec::new();
    for block in blocks {
        for lane in block {
            writeln!(input, "{}", lane.to_bits()).unwrap();
        }
        let encoded = super::encode_grouped_q8_block(&block).unwrap();
        bytes.extend(encoded.scale.to_ne_bytes());
        bytes.extend(encoded.codes.map(|x| x as u8));
        bytes.extend(encoded.sums.iter().flat_map(|x| x.to_ne_bytes()));
    }
    let reference = query_oracle("grouped-q8", input);
    assert_eq!(bytes.len(), reference.len());
    for (i, (byte, expected)) in bytes.into_iter().zip(reference).enumerate() {
        assert_eq!(
            u32::from(byte),
            expected,
            "block {} byte {}",
            i / 292,
            i % 292
        );
    }
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 1e-38] {
        let mut block = [0.0; 256];
        block[255] = invalid;
        assert!(super::encode_grouped_q8_block(&block).is_err());
    }
}

#[test]
fn grouped_row_product_vtk_27() {
    assert_eq!(
        verify_bundle("groupeddot", "tensor-groupeddot")["VerdantEngine.GroupedDot.plan"],
        "verdant_grouped_dot"
    );
    use std::fmt::Write;
    let mut input = String::new();
    let mut actual = Vec::new();
    let mut seed = 42u32;
    let mut next = || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        seed
    };
    for row in 0..97 {
        let count = row % 5;
        writeln!(input, "{count}").unwrap();
        let mut weights = Vec::new();
        let mut activations = Vec::new();
        for i in 0..count {
            let mut block = [0u8; 84];
            for byte in &mut block[..80] {
                *byte = (next() >> 24) as u8;
            }
            let scales = [0u16, 0x3800, 0xbc00, 0x4000, 0x3400, 0x4200];
            block[80..82].copy_from_slice(&scales[(row + i) % scales.len()].to_ne_bytes());
            block[82..].copy_from_slice(&scales[(row * 3 + i + 1) % scales.len()].to_ne_bytes());
            let lanes = std::array::from_fn(|_| ((next() >> 24) as f32 - 128.0) / 4.0);
            for byte in block {
                writeln!(input, "{byte}").unwrap();
            }
            for lane in lanes {
                writeln!(input, "{}", lane.to_bits()).unwrap();
            }
            weights.extend_from_slice(&block);
            activations.push(super::encode_grouped_q8_block(&lanes).unwrap());
        }
        actual.push(super::dot_grouped_q2_row(&weights, &activations).unwrap());
    }
    let expected = query_oracle("grouped-dot", input);
    assert_eq!(expected.len(), actual.len());
    for (i, (out, bits)) in actual.into_iter().zip(expected).enumerate() {
        assert_eq!(out.to_bits(), bits, "grouped row {i}");
    }
    assert!(super::dot_grouped_q2_row(&[0; 83], &[]).is_err());
    assert!(super::dot_grouped_q2_row(&[0; 84], &[]).is_err());
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
fn attention_head_slopes_vtk_30() {
    use std::fmt::Write;
    assert_eq!(
        verify_bundle("slopes", "tensor-slopes")["VerdantEngine.HeadSlope.evaluate"],
        "verdant_head_slope"
    );
    let mut cases = Vec::new();
    let mut input = String::new();
    for heads in [
        1, 2, 3, 4, 5, 7, 8, 12, 31, 32, 33, 64, 96, 127, 128, 257, 1024, 1537,
    ] {
        for bias in [
            f32::from_bits(1),
            0.015625,
            0.3,
            1.0,
            4.0,
            7.75,
            8.0,
            16.0,
            32.0,
            127.0,
            256.0,
            4096.0,
        ] {
            writeln!(input, "{} {heads}", bias.to_bits()).unwrap();
            cases.extend((0..heads).map(|head| (bias, heads, head)));
        }
    }
    let expected = query_oracle("head-slopes", input);
    assert_eq!(expected.len(), cases.len());
    let comparisons = expected.len();
    let mut maximum_delta = 0;
    for ((bias, heads, head), bits) in cases.into_iter().zip(expected) {
        let actual = super::attention_head_slope_f32(bias, heads, head).unwrap();
        let wanted = f32::from_bits(bits);
        let delta = actual.to_bits().abs_diff(bits);
        maximum_delta = maximum_delta.max(delta);
        if wanted == 0.0 || wanted == 1.0 {
            assert_eq!(actual.to_bits(), bits, "{bias}, {heads}, {head}");
        } else {
            assert!(
                (0.0..=1.0).contains(&actual) && delta <= 4,
                "{bias}, {heads}, {head}: {actual} vs {wanted} ({:#x}/{bits:#x})",
                actual.to_bits()
            );
        }
    }
    eprintln!("{comparisons} GGML head slopes, maximum difference {maximum_delta} binary32 steps");
    for bias in [0.0, -0.0, -1.0] {
        assert_eq!(super::attention_head_slope_f32(bias, 3, 2).unwrap(), 1.0);
    }
    for (bias, heads, head) in [
        (1.0, 0, 0),
        (1.0, 4, 4),
        (f32::NAN, 1, 0),
        (f32::INFINITY, 1, 0),
    ] {
        assert!(super::attention_head_slope_f32(bias, heads, head).is_err());
    }
}

#[test]
fn gaussian_activations_vtk_29() {
    assert_eq!(
        verify_bundle("gelu", "tensor-gelu")["VerdantEngine.GaussianActivation.evaluate"],
        "verdant_gelu"
    );
    use std::fmt::Write;
    let mut words: Vec<u32> = (0..=u16::MAX).map(|word| u32::from(word) << 16).collect();
    words.extend((-32_768..=32_768).map(|n| (n as f32 / 2048.0).to_bits()));
    words.extend([1, 0x80000001, 0x7fffff, 0x807fffff, 0x7f7fffff, 0xff7fffff]);
    words.sort_unstable();
    words.dedup();
    let mut requests = String::new();
    for word in &words {
        writeln!(requests, "{word}").unwrap();
    }
    let reference = query_oracle("gelu", requests);
    assert_eq!(reference.len(), words.len() * 2);
    for (word, expected) in words.into_iter().zip(reference.chunks_exact(2)) {
        let x = f32::from_bits(word);
        for (kind, value) in [
            super::gelu_f32(x).unwrap(),
            super::gelu_quick_f32(x).unwrap(),
        ]
        .into_iter()
        .enumerate()
        {
            let wanted = f32::from_bits(expected[kind]);
            if wanted.is_nan() {
                assert!(value.is_nan(), "kind {kind}, input {word:#x}");
            } else if x == 0.0 || !x.is_finite() || wanted.is_infinite() {
                assert_eq!(
                    value.to_bits(),
                    wanted.to_bits(),
                    "kind {kind}, input {word:#x}"
                );
            } else if kind == 1 {
                assert!(
                    value.is_finite()
                        && value.is_sign_negative() == wanted.is_sign_negative()
                        && value.to_bits().abs_diff(wanted.to_bits()) <= 5,
                    "quick input {word:#x}: {value} vs {wanted}"
                );
            } else {
                // The tangent's four-ULP envelope is multiplied by half the input.
                let allowance = f64::from(x.abs()) * f64::from(f32::EPSILON) * 3.0;
                assert!(
                    value.is_finite() && (f64::from(value) - f64::from(wanted)).abs() <= allowance,
                    "tangent input {word:#x}: {value} vs {wanted}"
                );
            }
        }
    }
}

#[test]
fn hyperbolic_tangent_vtk_28() {
    assert_eq!(
        verify_bundle("tanh", "tensor-tanh")["VerdantEngine.HyperbolicTangent.evaluate"],
        "verdant_tanh"
    );
    use std::fmt::Write;
    let mut words: Vec<u32> = (0u32..=u16::MAX.into()).map(|high| high << 16).collect();
    words.extend((-32768..=32768).map(|n| (n as f32 / 2048.0).to_bits()));
    for value in [1.0_f32 / 4096.0, 0.25, 10.0] {
        for magnitude in value.to_bits() - 8..=value.to_bits() + 8 {
            words.extend([magnitude, magnitude | 0x80000000]);
        }
    }
    words.extend([
        1, 0x80000001, 0x007fffff, 0x807fffff, 0x7f7fffff, 0xff7fffff,
    ]);
    words.sort_unstable();
    words.dedup();
    let mut input = String::new();
    for word in &words {
        writeln!(input, "{word}").unwrap();
    }
    let expected = query_oracle("tanh", input);
    assert_eq!(expected.len(), words.len());
    for (word, bits) in words.into_iter().zip(expected) {
        let value = super::tanh_f32(f32::from_bits(word)).unwrap();
        let reference = f32::from_bits(bits);
        if reference.is_nan() {
            assert!(value.is_nan(), "input {word:#x}");
        } else if reference == 0.0 || reference.abs() == 1.0 {
            assert_eq!(value.to_bits(), bits, "input {word:#x}");
        } else {
            assert!(
                value.is_finite()
                    && value.is_sign_negative() == reference.is_sign_negative()
                    && value.to_bits().abs_diff(bits) <= 4,
                "input {word:#x}: {value} vs {reference}"
            );
        }
    }
}

#[test]
fn logistic_functions_vtk_25() {
    assert_eq!(
        verify_bundle("logistic", "tensor-logistic")["VerdantEngine.Logistic.step"],
        "verdant_logistic"
    );
    use std::fmt::Write;
    let mut words: Vec<u32> = (-4608..=4608)
        .map(|step| (step as f32 / 32.0).to_bits())
        .collect();
    for magnitude in [
        0, 1, 0x007fffff, 0x00800000, 0x7f7fffff, 0x7f800000, 0x7fc00042,
    ] {
        words.extend([magnitude, magnitude | 0x80000000]);
    }
    words.sort_unstable();
    words.dedup();
    let mut requests = String::new();
    for &word in &words {
        writeln!(requests, "{word}").unwrap();
    }
    let expected = query_oracle("logistic", requests);
    assert_eq!(expected.len(), words.len() * 2);
    for (word, pair) in words.into_iter().zip(expected.chunks_exact(2)) {
        let x = f32::from_bits(word);
        let actual = [super::sigmoid_f32(x).unwrap(), super::silu_f32(x).unwrap()];
        for (observed, &bits) in actual.into_iter().zip(pair) {
            let wanted = f32::from_bits(bits);
            if wanted.is_nan() {
                assert!(observed.is_nan(), "input {word:#x}");
            } else if wanted == 0.0 || wanted.is_infinite() {
                assert_eq!(observed.to_bits(), bits, "input {word:#x}");
            } else {
                assert!(
                    observed.is_finite()
                        && observed.is_sign_negative() == wanted.is_sign_negative()
                        && observed.to_bits().abs_diff(bits) <= 4,
                    "input {word:#x}: {observed} versus {wanted}"
                );
            }
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
    assert_eq!(verify_bundle("sincos", "tensor-circular").len(), 5);
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
    // These binary32 angles approach multiples of pi/2 particularly closely.
    // Both neighbors exercise the lost-significance boundary in the range reduction.
    for center in [
        0x6f79be45, 0x50a3e87f, 0x6ff9be45, 0x5123e87f, 0x437ce5f1, 0x7079be45, 0x6a1976f1,
        0x53b146a6, 0x65898498, 0x51a3e87f, 0x43fce5f1, 0x77584625, 0x4c2332e9, 0x4096cbe4,
        0x6c55da58, 0x70f9be45,
    ] {
        for word in [center - 1, center, center + 1] {
            words.extend([word, word | (1 << 31)]);
        }
    }
    for half in 0..=u16::MAX {
        words.push(u32::from(half) << 16);
        words.push(
            super::convert_float_storage(1, 0, u64::from(half))
                .unwrap()
                .unwrap() as u32,
        );
    }
    words.sort_unstable();
    words.dedup();
    let mut input = String::new();
    for word in &words {
        writeln!(input, "{word}").unwrap();
    }
    assert_eq!(words.len(), 260_284);
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

#[test]
fn logarithmic_frequency_vtk_22() {
    assert_eq!(verify_bundle("logscale", "tensor-logscale").len(), 8);
    use std::fmt::Write;
    let mut words = vec![0x80000000, 0xbf800000, 0xff800000, 0x7fc00042];
    for exp in 0..=255u32 {
        for sample in 0..64u32 {
            words.push((exp << 23) | (sample.wrapping_mul(2654435761) & 0x007fffff));
        }
    }
    for delta in 0..4096 {
        words.extend([0x3f800000 - delta, 0x3f800000 + delta]);
    }
    words.sort_unstable();
    words.dedup();
    let mut input = String::new();
    for w in &words {
        writeln!(input, "{w}").unwrap();
    }
    let expected = query_oracle("logarithm", input);
    assert_eq!(expected.len(), words.len());
    let compare = |actual: f32, bits: u32| {
        let value = f32::from_bits(bits);
        if value.is_nan() {
            assert!(actual.is_nan());
        } else if value.is_infinite() || value == 0.0 {
            assert_eq!(actual.to_bits(), bits);
        } else {
            assert!(
                actual.is_finite()
                    && actual.is_sign_negative() == value.is_sign_negative()
                    && actual.to_bits().abs_diff(bits) <= 1,
                "{actual:?} versus {value:?}"
            );
        }
    };
    for (word, bits) in words.into_iter().zip(expected) {
        compare(super::natural_log_f32(f32::from_bits(word)).unwrap(), bits);
    }
    let mut inputs = String::new();
    let mut pairs = Vec::new();
    for degree in [
        1, 2, 3, 4, 8, 10, 12, 16, 18, 24, 30, 32, 40, 64, 128, 255, 256, 513, 1024, 2048,
    ] {
        for base in [
            f32::from_bits(1),
            f32::MIN_POSITIVE,
            0.01,
            0.5,
            0.99999994,
            1.0,
            1.0000001,
            2.0,
            10.0,
            100.0,
            10000.0,
            1000000.0,
            f32::MAX,
        ] {
            writeln!(inputs, "{} {degree}", base.to_bits()).unwrap();
            pairs.push((base, degree));
        }
    }
    let expected = query_oracle("rotary-ratio", inputs);
    assert_eq!(pairs.len(), expected.len());
    for ((base, degree), bits) in pairs.into_iter().zip(expected) {
        let actual = super::inverse_root_f32(base, degree).unwrap();
        compare(actual, bits);
        if base == 10000.0 || base == 1000000.0 {
            assert_eq!(actual.to_bits(), bits, "base {base} degree {degree}");
        }
    }
    assert!(super::inverse_root_f32(2.0, 0).is_err());
    for base in [0.0, -1.0, f32::INFINITY, f32::NAN] {
        assert!(super::inverse_root_f32(base, 3).is_err());
    }
}

#[test]
fn plane_rotation_vtk_23() {
    assert_eq!(verify_bundle("plane", "tensor-plane").len(), 4);
    use std::fmt::Write;
    let mut request = String::new();
    let mut cases = Vec::new();
    for dims in [16, 48, 128] {
        for position in [-7, 0, 1, 59, 512] {
            for inverse in [false, true] {
                for truncate in [false, true] {
                    for (scale, extension, magnitude) in [
                        (1_f32, 0_f32, 1_f32),
                        (0.5, 0.0, 0.8),
                        (0.25, 1.0, 1.0),
                        (0.7, 0.6, 1.2),
                    ] {
                        writeln!(
                            request,
                            "{dims} {position} {} {} {} {} {}",
                            u8::from(inverse),
                            u8::from(truncate),
                            scale.to_bits(),
                            extension.to_bits(),
                            magnitude.to_bits()
                        )
                        .unwrap();
                        cases.push((
                            dims, position, inverse, truncate, scale, extension, magnitude,
                        ));
                    }
                }
            }
        }
    }
    let words = query_oracle("plane-rotation", request);
    let mut results = words.chunks_exact(4);
    assert!(results.remainder().is_empty());
    for (dims, position, inverse, truncate, scale, extension, magnitude) in cases {
        let spectrum = super::RotarySpectrum::new(super::RotaryParameters {
            dimensions: dims,
            context: 512,
            base: 10000.0,
            frequency_scale: scale,
            extension,
            magnitude,
            rotations: [32.0, 1.0],
            truncate,
        })
        .unwrap();
        let mut phase = super::signed_word_f32(position).unwrap();
        for lane in 0..dims / 2 {
            let row = results
                .next()
                .expect("upstream coefficient and rotated pair");
            let weights = spectrum
                .weights(phase, 1.0 + (lane % 5) as f32 / 4.0, lane, inverse)
                .unwrap();
            for (actual, expected) in [weights.0, weights.1].into_iter().zip(row) {
                assert!(
                    actual.to_bits().abs_diff(*expected) <= 3,
                    "dims {dims} position {position} lane {lane}: {actual:?} versus {:?}",
                    f32::from_bits(*expected)
                );
            }
            let coordinate = |i: u32| ((7 * i % 17) as i32 - 8) as f32 / 4.0;
            let actual = super::rotate_pair_f32(
                (coordinate(2 * lane), coordinate(2 * lane + 1)),
                (f32::from_bits(row[0]), f32::from_bits(row[1])),
            )
            .unwrap();
            assert_eq!([actual.0.to_bits(), actual.1.to_bits()], row[2..]);
            phase = spectrum.advance(phase).unwrap();
        }
    }
    assert!(results.next().is_none());
}

#[test]
fn online_weighted_mean_vtk_24() {
    assert_eq!(verify_bundle("weighted", "tensor-weighted").len(), 3);
    use std::fmt::Write;
    let mut input = String::new();
    let mut actual = Vec::new();
    for count in [1, 3, 8, 31, 64] {
        for width in [1, 4, 13, 48] {
            for pattern in 0..6 {
                for sink in [None, Some(-8.0_f32), Some(3.5_f32)] {
                    let mut mean = super::WeightedMeanF32::new(width);
                    writeln!(
                        input,
                        "{count} {width} {} {}",
                        u8::from(sink.is_some()),
                        sink.unwrap_or_default().to_bits()
                    )
                    .unwrap();
                    for i in 0..count {
                        let enabled = pattern != 5 && (pattern != 4 || i % 3 != 0);
                        let score = match pattern {
                            0 => i as f32 / 8.0,
                            1 => -(i as f32) / 8.0,
                            2 => 2.5,
                            _ => ((i * 7 % 13) as f32 - 6.0) * 4.0,
                        };
                        write!(input, "{} {}", score.to_bits(), u8::from(enabled)).unwrap();
                        let row: Vec<_> = (0..width)
                            .map(|j| ((i * 11 + j * 7) % 31) as f32 / 16.0 - 0.75)
                            .collect();
                        for x in &row {
                            write!(input, " {}", x.to_bits()).unwrap();
                        }
                        input.push('\n');
                        if enabled {
                            mean.push(score, Some(&row)).unwrap();
                        }
                    }
                    if let Some(score) = sink {
                        mean.push(score, None).unwrap();
                    }
                    actual.extend(mean.finish().unwrap());
                }
            }
        }
    }
    let expected = query_oracle("weighted-mean", input);
    assert_eq!(actual.len(), expected.len());
    for (value, word) in actual.into_iter().zip(expected) {
        let reference = f32::from_bits(word);
        assert!(
            (value - reference).abs() <= 0.000001,
            "{value} vs {reference}"
        );
    }
    let mut mean = super::WeightedMeanF32::new(2);
    assert!(mean.push(0.0, Some(&[1.0])).is_err());
    assert_eq!(mean.finish().unwrap(), [0.0, 0.0]);
}

#[test]
fn binary32_row_ranking_vtk_31() {
    assert_eq!(
        verify_bundle("rowrank", "tensor-rowrank")["VerdantEngine.RowRanking.evaluate"],
        "verdant_row_ranking"
    );
    use std::fmt::Write;
    let patterns = [f32::NEG_INFINITY, -2.0, -0.0, 0.0, 0.5, 9.0, f32::INFINITY];
    let mut queries = String::new();
    let mut cases = Vec::new();
    for width in [1_usize, 3, 9, 32, 127, 257] {
        let row: Vec<f32> = (0..width)
            .map(|i| patterns[(i * 13 + i / 11) % patterns.len()])
            .collect();
        for (count, descending) in std::collections::BTreeSet::from([
            (width, false),
            (width, true),
            (width.div_ceil(2), true),
            (1, true),
        ]) {
            writeln!(queries, "{width} {count} {}", u8::from(descending)).unwrap();
            for word in &row {
                writeln!(queries, "{}", word.to_bits()).unwrap();
            }
            cases.push((row.clone(), count, descending));
        }
    }
    let expected = query_oracle("row-rank", queries);
    let mut oracle = expected.as_slice();
    for (row, count, descending) in cases {
        let (reference, tail) = oracle.split_at(count);
        oracle = tail;
        let mut rank = super::RowRankingF32::new(row.len(), count, descending).unwrap();
        let result = rank.select(&row).unwrap();
        assert_eq!(result.len(), count);
        let mut values: Vec<_> = reference.iter().map(|&i| row[i as usize]).collect();
        values.sort_by(|a, b| {
            if descending {
                b.partial_cmp(a).unwrap()
            } else {
                a.partial_cmp(b).unwrap()
            }
        });
        let mut previous = None;
        for (&i, value) in result.iter().zip(values) {
            assert_eq!(row[i as usize], value);
            if let Some((last, j)) = previous {
                if value == last {
                    assert!(i > j);
                }
            }
            previous = Some((value, i));
        }
    }
    assert!(oracle.is_empty());
    let mut rank = super::RowRankingF32::new(3, 3, true).unwrap();
    assert_eq!(rank.select(&[f32::NAN, 1.0, f32::NAN]).unwrap(), [1, 0, 2]);
    assert_eq!(rank.select(&[9.0, -2.0, 11.0]).unwrap(), [2, 0, 1]);
    assert!(rank.select(&[1.0]).is_err());
    assert!(super::RowRankingF32::new(0, 1, false).is_err());
    assert!(super::RowRankingF32::new(1, 0, true)
        .unwrap()
        .select(&[1.0])
        .unwrap()
        .is_empty());
}

#[test]
fn normalized_exponential_vtk_32() {
    use std::fmt::Write;
    let mut requests = String::new();
    let mut cases = Vec::new();
    for width in [1, 3, 8, 31, 64, 257, 1024] {
        for sink in [None, Some(-11.0_f32), Some(9.0_f32)] {
            let row: Vec<_> = (0..width).map(|i| match i % 7 {
                1 => f32::NEG_INFINITY,
                2 => -65.0,
                3 => -0.0,
                _ => (((i * 53 + 5) % 97) as f32 - 48.0) / 4.0,
            }).collect();
            writeln!(requests,"{width} {} {}",u8::from(sink.is_some()),sink.unwrap_or_default().to_bits()).unwrap();
            for &score in &row { writeln!(requests,"{}",score.to_bits()).unwrap(); }
            cases.push((row,sink));
        }
    }
    let words = query_oracle("normalized-exp", requests);
    let mut oracle = words.as_slice();
    for (mut row, sink) in cases {
        let (expected, tail) = oracle.split_at(row.len()); oracle = tail;
        super::normalize_exponential_f32(&mut row, sink).unwrap();
        for (actual, &word) in row.into_iter().zip(expected) {
            let expected = f32::from_bits(word);
            assert!((actual-expected).abs() <= 1.0e-6 * expected.abs() + 1.0e-8, "{actual} != {expected}");
        }
    }
    assert!(oracle.is_empty());
    for mut invalid in [vec![f32::NAN], vec![f32::INFINITY], vec![f32::NEG_INFINITY]] {
        let before = invalid[0].to_bits();
        assert!(super::normalize_exponential_f32(&mut invalid, None).is_err());
        assert_eq!(before,invalid[0].to_bits());
    }
    assert!(super::normalize_exponential_f32(&mut [], None).is_ok());
}
