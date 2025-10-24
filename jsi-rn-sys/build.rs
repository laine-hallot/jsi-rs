use std::{env, path::PathBuf};

fn main() {
    let base = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let pkg_base = PathBuf::from(base);

    let rn_base = pkg_base.join("../vendor/react-native/packages/react-native");
    let hermes_base = pkg_base.join("../vendor/hermes");

    let includes = vec![
        /*         rn_base.join("React"),
        rn_base.join("React/Base"),
        rn_base.join("ReactCommon"),
        rn_base.join("ReactCommon/jsi"),
        rn_base.join("ReactCommon/callinvoker"),
        rn_base.join("ReactCommon/runtimeexecutor"),
        rn_base.join("ReactCommon/react/runtime/"),
        rn_base.join("ReactCommon/react/featureflags/"),
        rn_base.join("ReactCxxPlatform/react/runtime/platform/cxx/react/runtime/"),
        rn_base.join("ReactCxxPlatform"),
        pkg_base.join("include"),
        hermes_base.join("../API"),
        hermes_base.join("../public"), */
        rn_base.join("ReactCommon"),
        rn_base.join("ReactCommon/jsi"),
        rn_base.join("ReactCommon/callinvoker"),
        rn_base.join("ReactCommon/runtimeexecutor"),
        rn_base.join("ReactCommon/react/runtime/hermes"),
        rn_base.join("ReactCommon/react/bridging"),
        rn_base.join("ReactCommon/react/nativemodule/core"),
        rn_base.join("ReactCommon/react/runtime"),
        rn_base.join("ReactCxxPlatform/react/runtime/platform/cxx/react/runtime/"),
        rn_base.join("ReactCxxPlatform"),
        hermes_base.join("API"),
        hermes_base.join("public"),
        pkg_base.join("include"),
    ];
    let includes: Vec<_> = IntoIterator::into_iter(includes)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing include path {:?}", p)))
        .collect();

    env::set_var("CC", "/usr/bin/gcc-14");
    env::set_var("CXX", "/usr/bin/g++-14");

    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(|e| e.as_path()));

    let bridges = vec!["src/ffi/console_host.rs"];

    for bridge in &bridges {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    cxx_build::bridges(bridges)
        .flag_if_supported("-std=c++23")
        .flag("-fPIC")
        //.flag("-DHERMES_ENABLE_DEBUGGER=1")
        .file("src/ffi/console_host.cpp")
        .compile("jsi_rn_sys");

    // Build React Native implementation files
    let mut rn_runtime = cc::Build::new();
    rn_runtime
        .cpp(true)
        .flag("-std=c++23")
        .flag("-fPIC")
        .includes(&includes)
        .file(rn_base.join("ReactCommon/react/runtime/TimerManager.cpp"))
        .file(rn_base.join("ReactCxxPlatform/react/threading/TaskDispatchThread.cpp"))
        .file(
            rn_base
                .join("ReactCxxPlatform/react/runtime/platform/cxx/react/runtime/PlatformTimerRegistryImpl.cpp"),
        )
        .compile("react_runtime");

    // Build React Native JSI
    let mut jsi_build = cc::Build::new();
    jsi_build
        .includes(&includes)
        .cpp(true)
        .flag("-std=c++17")
        .flag("-fPIC")
        //.flag("-DHERMES_ENABLE_DEBUGGER=1")
        .file(rn_base.join("ReactCommon/jsi/jsi/jsi.cpp"))
        //.file(rn_base.join("ReactCommon/jsi/jsi/instrumentation.cpp"))
        .compile("jsi-rn-sys");

    /*
    // Build CallInvoker
    let mut callinvoker_build = cc::Build::new();
    callinvoker_build
        .cpp(true)
        .flag("-std=c++17")
        .flag("-fPIC")
        .file(rn_base.join("ReactCommon/callinvoker/ReactCommon-callinvoker.cpp"))
        .includes(&includes)
        .compile("callinvoker"); */

    /*
    // Build RuntimeExecutor
    let mut runtime_build = cc::Build::new();
    runtime_build
        .cpp(true)
        .flag("-std=c++17")
        .flag("-fPIC")
        .includes(&includes)
        .file(rn_base.join("ReactCommon/runtimeexecutor/ReactCommon-runtimeexecutor.cpp"))
        .compile("runtimeexecutor");
    */
    // Link against Hermes
    println!("cargo:rustc-link-lib=hermesvm");
    println!(
        "cargo:rustc-link-search={}",
        pkg_base
            .join("../vendor/hermes/build/lib/")
            .to_string_lossy()
    );
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}",
        pkg_base
            .join("../vendor/hermes/build/lib/")
            .to_string_lossy()
    );

    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=glog");
    println!("cargo:rustc-link-lib=dylib=folly");

    //println!("cargo:rustc-link-lib=static=react_platform");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi/console_host.cpp");
    println!("cargo:rerun-if-changed=include/console_host.h");
}
