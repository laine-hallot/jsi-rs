use cxx::{type_id, ExternType, UniquePtr};
use std::pin::{pin, Pin};

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("helper.h");
        include!("jsi/jsi.h");

        #[namespace = "facebook::jsi"]
        type Runtime = crate::ffi::base::Runtime;
        #[cxx_name = "Value"]
        #[namespace = "facebook::jsi"]
        type JsiValue = crate::ffi::base::JsiValue;

        pub fn cast_hermes_runtime(ptr: UniquePtr<HermesRuntime>) -> UniquePtr<Runtime>;
        pub fn eval_js(rt: Pin<&mut Runtime>, js: &str) -> UniquePtr<JsiValue>;

        type CppHermesRuntimeConfig = crate::hermes::HermesRuntimeConfig;
        fn cpp_create_runtime_config(
            config: Pin<&mut CppHermesRuntimeConfig>,
        ) -> UniquePtr<RuntimeConfig>;
    }

    #[namespace = "hermes::vm"]
    extern "C++" {
        include!("hermes/Public/RuntimeConfig.h");

        type RuntimeConfig;
    }

    #[namespace = "facebook::hermes"]
    unsafe extern "C++" {
        include!("hermes/hermes.h");

        type HermesRuntime;

        #[cxx_name = "makeHermesRuntime"]
        pub fn create_hermes_runtime(config: &RuntimeConfig) -> UniquePtr<HermesRuntime>;
    }
}

// This is not a binding for hermes::vm::RuntimeConfig,
// passing a struct to create_runtime_config just seemed easier than individual arguments.
pub struct HermesRuntimeConfig {
    pub enable_eval: bool,
    pub verify_eval_ir: bool,
    pub optimized_eval: bool,
    pub async_break_check_in_eval: bool,
    pub es6_promise: bool,
    pub es6_proxy: bool,
    pub es6_class: bool,
    pub intl: bool,
    pub array_buffer: bool,
    pub microtask_queue: bool,
    pub enable_sampled_stats: bool,
    pub enable_sample_profiling: bool,
    pub randomize_memory_layout: bool,
    pub track_io: bool,
    pub enable_hermes_internal: bool,
    pub enable_hermes_internal_test_methods: bool,
    pub enable_generator: bool,
    pub enable_block_scoping: bool,
}

unsafe impl ExternType for HermesRuntimeConfig {
    type Id = type_id!("CppHermesRuntimeConfig");
    type Kind = cxx::kind::Opaque;
}

impl Default for HermesRuntimeConfig {
    fn default() -> Self {
        Self {
            enable_eval: true,
            verify_eval_ir: false,
            optimized_eval: false,
            async_break_check_in_eval: true,
            es6_promise: true,
            es6_proxy: true,
            es6_class: false,
            intl: true,
            array_buffer: true,
            microtask_queue: false,
            enable_sampled_stats: false,
            enable_sample_profiling: false,
            randomize_memory_layout: false,
            track_io: false,
            enable_hermes_internal: true,
            enable_hermes_internal_test_methods: false,
            enable_generator: true,
            enable_block_scoping: false,
        }
    }
}

pub fn create_runtime_config(config: HermesRuntimeConfig) -> UniquePtr<RuntimeConfig> {
    let cpp_config: Pin<&mut HermesRuntimeConfig> = pin!(config);
    return cpp_create_runtime_config(cpp_config);
}

pub use ffi::*;
