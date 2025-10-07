use std::{env, path::PathBuf, process::Command};

fn main() {
    let base = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let pkg_base = PathBuf::from(base);

    let target_os = env::var_os("CARGO_CFG_TARGET_OS");
    let target_os = if let Some(s) = &target_os {
        s.to_str()
    } else {
        None
    };

    let hermes_build_status = Command::new("bash")
        .args([pkg_base.join("../vendor/build-hermes.sh")])
        .current_dir(pkg_base.join("../vendor"))
        .output()
        .expect("hermes build script could not be executed");

    if !hermes_build_status.status.success() {
        panic!(
            "hermes build script failed\n\nstdout: {}\n\nstderr: {}",
            String::from_utf8_lossy(&hermes_build_status.stdout),
            String::from_utf8_lossy(&hermes_build_status.stderr),
        )
    }

    let rn_base = pkg_base.join("../vendor/react-native/packages/react-native");

    let mut includes = vec![
        rn_base.join("React"),
        rn_base.join("React/Base"),
        rn_base.join("ReactCommon/jsi"),
        rn_base.join("ReactCommon/callinvoker"),
        pkg_base.join("include"),
        pkg_base.join("../vendor/hermes/API"),
        pkg_base.join("../vendor/hermes/public"),
    ];

    if let Some("android") = target_os {
        includes.push(rn_base.join("ReactAndroid/src/main/jni/react/turbomodule/"));
        includes.push(pkg_base.join("vendor/fbjni/cxx"));
    }

    let includes: Vec<_> = IntoIterator::into_iter(includes)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing include path {:?}", p)))
        .collect();

    let mut compiles = vec![rn_base.join("ReactCommon/jsi/jsi/jsi.cpp")];

    if let Some("android") = target_os {
        compiles.push(
            rn_base.join(
                "ReactAndroid/src/main/jni/react/turbomodule/ReactCommon/CallInvokerHolder.cpp",
            ),
        );
    }

    let compiles: Vec<_> = IntoIterator::into_iter(compiles)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing compile file {:?}", p)))
        .collect();

    env::set_var("CC", "/usr/bin/gcc-14");
    env::set_var("CXX", "/usr/bin/g++-14");

    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(|e| e.as_path()));

    let mut bridges = vec!["src/ffi/base.rs", "src/ffi/host.rs", "src/ffi/hermes.rs"];

    if let Some("android") = target_os {
        bridges.push("src/ffi/android.rs");
    }

    for bridge in &bridges {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    cxx_build::bridges(bridges)
        .flag_if_supported("-std=c++17")
        .files(compiles)
        .compile("jsi");

    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=include/host.h");

    println!("cargo:rustc-link-lib=hermes");
    println!(
        "cargo:rustc-link-search={}",
        pkg_base
            .join("../vendor/hermes/build/API/hermes/")
            .to_string_lossy()
    );
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}",
        pkg_base
            .join("../vendor/hermes/build/API/hermes/")
            .to_string_lossy()
    );

    /*     println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}",
        pkg_base
            .join("../vendor/hermes/build/API/hermes_abi/")
            .to_string_lossy()
    ); */

    // Add link paths and libraries
    println!("cargo:rustc-link-search=native=vendor/hermes/build/API/hermes_abi");
    println!("cargo:rustc-link-lib=dylib=hermesabi");
}
