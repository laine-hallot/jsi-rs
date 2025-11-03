#[cxx::bridge]
pub(crate) mod console_host {

    unsafe extern "C++" {
        include!("console_host.h");

        #[namespace = "facebook::jsi"]
        type Runtime = jsi_sys::base::Runtime;

        pub fn install_timer_globals(runtime: Pin<&mut Runtime>) -> ();
    }
}

pub use console_host::*;
