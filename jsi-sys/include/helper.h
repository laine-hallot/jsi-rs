#include <hermes/hermes.h>
// #include <hermes/CompileJS.h>
#include <hermes/Public/RuntimeConfig.h>
#include <jsi/jsi.h>
#include "rust/cxx.h"

std::unique_ptr<facebook::jsi::Runtime> cast_hermes_runtime(std::unique_ptr<facebook::hermes::HermesRuntime> runtime)
{
  return runtime;
}

// This is not a binding for hermes::vm::RuntimeConfig,
// passing a struct to create_runtime_config just seemed easier than individual arguments.
struct CppHermesRuntimeConfig {
  bool enable_eval;
  bool verify_eval_ir;
  bool optimized_eval;
  bool async_break_check_in_eval;
  bool es6_promise;
  bool es6_proxy;
  bool es6_class;
  bool intl;
  bool array_buffer;
  bool microtask_queue;
  bool enable_sampled_stats;
  bool enable_sample_profiling;
  bool randomize_memory_layout;
  bool track_io;
  bool enable_hermes_internal;
  bool enable_hermes_internal_test_methods;
  bool enable_generator;
  bool enable_block_scoping;
};

std::unique_ptr<hermes::vm::RuntimeConfig> cpp_create_runtime_config(CppHermesRuntimeConfig& options)
{ 
  hermes::vm::RuntimeConfig::Builder cfgBuilder = hermes::vm::RuntimeConfig::Builder();
  cfgBuilder.withEnableEval(options.enable_eval);
  cfgBuilder.withVerifyEvalIR(options.verify_eval_ir);
  cfgBuilder.withOptimizedEval(options.optimized_eval);
  cfgBuilder.withAsyncBreakCheckInEval(options.async_break_check_in_eval);
  cfgBuilder.withES6Promise(options.es6_promise);
  cfgBuilder.withES6Proxy(options.es6_proxy);
  cfgBuilder.withES6Class(options.es6_class);
  cfgBuilder.withIntl(options.intl);
  cfgBuilder.withArrayBuffer(options.array_buffer);
  cfgBuilder.withMicrotaskQueue(options.microtask_queue);
  cfgBuilder.withEnableSampledStats(options.enable_sampled_stats);
  cfgBuilder.withEnableSampleProfiling(options.enable_sample_profiling);
  cfgBuilder.withRandomizeMemoryLayout(options.randomize_memory_layout);
  cfgBuilder.withTrackIO(options.track_io);
  cfgBuilder.withEnableHermesInternal(options.enable_hermes_internal);
  cfgBuilder.withEnableHermesInternalTestMethods(options.enable_hermes_internal_test_methods);
  cfgBuilder.withEnableGenerator(options.enable_generator);
  cfgBuilder.withEnableBlockScoping(options.enable_block_scoping);
  
  auto config = cfgBuilder.build();
  return std::make_unique<hermes::vm::RuntimeConfig>(config);
}

std::unique_ptr<facebook::jsi::Value> eval_js(facebook::jsi::Runtime& rt, rust::Str js)
{
  // std::string bytecode;
  // assert(hermes::compileJS(std::string(js), bytecode));
  auto out = rt.evaluateJavaScript(
      std::make_unique<facebook::jsi::StringBuffer>(std::string(js)),
      "<evaluated javascript>");
  return std::make_unique<facebook::jsi::Value>(std::move(out));
}
