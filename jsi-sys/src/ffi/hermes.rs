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
        pub fn create_runtime_config() -> UniquePtr<RuntimeConfig>;
        pub fn eval_js(rt: Pin<&mut Runtime>, js: &str) -> UniquePtr<JsiValue>;
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

pub use ffi::*;
