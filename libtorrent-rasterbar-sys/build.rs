use glob::glob;

fn pull_submodules() {
    let _ = std::process::Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .output()
        .expect("failed to update submodules");
}

fn main() {
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=boost_filesystem");

    pull_submodules();

    if cfg!(target_family = "windows") {
        if std::fs::exists(r"libtorrent\deps\try_signal\test.cpp").unwrap() {
            std::fs::remove_file(r"libtorrent\deps\try_signal\test.cpp").unwrap();
        }
        if std::fs::exists(r"libtorrent\deps\try_signal\example.cpp").unwrap() {
            std::fs::remove_file(r"libtorrent\deps\try_signal\example.cpp").unwrap();
        }
    } else {
        if std::fs::exists(r"libtorrent/deps/try_signal/test.cpp").unwrap() {
            std::fs::remove_file(r"libtorrent/deps/try_signal/test.cpp").unwrap();
        }
        if std::fs::exists(r"libtorrent/deps/try_signal/example.cpp").unwrap() {
            std::fs::remove_file(r"libtorrent/deps/try_signal/example.cpp").unwrap();
        }
    }

    let mut build = cxx_build::bridge("src/lib.rs");

    // Set C++ standard
    if cfg!(target_family = "windows") {
        build.flag("/std:c++14");
    } else {
        build.flag("-std=c++14");
    }

    // Optimization and visibility flags
    if cfg!(target_family = "windows") {
        build.flag("/O2");
        build.flag("/Ob");
    } else {
        build.flag_if_supported("-O3");
        build.flag_if_supported("-finline-functions");
        build.flag_if_supported("-fvisibility=hidden");
        build.flag_if_supported("-fvisibility-inlines-hidden");
    }

    // Warning flags
    if cfg!(target_family = "windows") {
        build.flag("/W3");
    } else {
        build.warnings(false).extra_warnings(false);
        build.flag_if_supported("-Wno-inline");
        build.flag_if_supported("-Wno-nonnull");
        build.flag_if_supported("-Wno-noexcept-type");
        build.flag_if_supported("-Wno-format-zero-length");
    }

    // Template depth
    if cfg!(target_family = "windows") {
        build.flag_if_supported("-ftemplate-depth-512");
    }

    // Definitions
    build
        .define("BOOST_ALL_NO_LIB", None)
        .define("BOOST_ASIO_ENABLE_CANCELIO", None)
        .define("BOOST_ASIO_HAS_STD_CHRONO", None)
        .define("BOOST_ASIO_NO_DEPRECATED", None)
        .define("BOOST_MULTI_INDEX_DISABLE_SERIALIZATION", None)
        .define("BOOST_NO_DEPRECATED", None)
        .define("BOOST_SYSTEM_NO_DEPRECATED", None)
        .define("NDEBUG", None)
        .define("OPENSSL_NO_SSL2", None)
        .define("TORRENT_BUILDING_LIBRARY", None)
        .define("TORRENT_SSL_PEERS", None)
        .define("TORRENT_USE_I2P", "1")
        .define("TORRENT_USE_LIBCRYPTO", None)
        .define("TORRENT_USE_OPENSSL", None)
        .define("TORRENT_USE_UNC_PATHS", None)
        .define("_FILE_OFFSET_BITS", "64");

    // Include directories
    if cfg!(target_family = "windows") {
        let boost_root = std::env::var("BOOST_ROOT").expect("BOOST_ROOT environment variable must be set");
        build.include(format!("{}/include", boost_root));
    } else if cfg!(target_vendor = "apple") {
        build.include("/opt/homebrew/include");
        build.include("/usr/local/opt/boost/include");
    } else {
        build.include("/usr/include/boost");
    }
    build.include("libtorrent/include");
    build.include("libtorrent/include/libtorrent");
    build.include("libtorrent/deps/try_signal");

    // Enable threading
    if !cfg!(target_family = "windows") {
        build.flag("-pthread");
    }

    // Add wrapper files
    for cpp in glob("wrap/*.cpp").unwrap().flatten() {
        build.file(cpp);
    }
    for cpp in glob("libtorrent/src/*.cpp").unwrap().flatten() {
        build.file(cpp);
    }
    for cpp in glob("libtorrent/src/**/*.cpp").unwrap().flatten() {
        build.file(cpp);
    }
    for cpp in glob("libtorrent/deps/try_signal/*.cpp").unwrap().flatten() {
        build.file(cpp);
    }

    // Compile
    build.compile("libtorrent-rasterbar-wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    for cpp in glob("wrap/*.cpp").unwrap().flatten() {
        println!("cargo:rerun-if-changed={}", cpp.display());
    }
    for hpp in glob("wrap/*.hpp").unwrap().flatten() {
        println!("cargo:rerun-if-changed={}", hpp.display());
    }
}

