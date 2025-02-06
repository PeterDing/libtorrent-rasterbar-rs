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

    // let mut b = cxx_build::bridge("src/lib.rs");
    // b.cpp(true)
    //     .flag("-ftemplate-depth-512")
    //     .flag("-std=c++14")
    //     .flag("-fvisibility-inlines-hidden")
    //     .flag("-pthread")
    //     .opt_level(3)
    //     .flag("-finline-functions")
    //     .flag("-Wno-inline")
    //     .flag("-fvisibility=hidden")
    //     .flag("-Wno-noexcept-type")
    //     .warnings(false)
    //     .extra_warnings(false)
    //     .flag("-Wno-format-zero-length")
    //     .flag("-Wpedantic")
    //     .flag("-Wvla")
    //     .define("BOOST_ALL_NO_LIB", None)
    //     .define("BOOST_ASIO_ENABLE_CANCELIO", None)
    //     .define("BOOST_ASIO_HAS_STD_CHRONO", None)
    //     .define("BOOST_ASIO_NO_DEPRECATED", None)
    //     .define("BOOST_MULTI_INDEX_DISABLE_SERIALIZATION", None)
    //     .define("BOOST_NO_DEPRECATED", None)
    //     .define("BOOST_SYSTEM_NO_DEPRECATED", None)
    //     .define("NDEBUG", None)
    //     .define("OPENSSL_NO_SSL2", None)
    //     .define("TORRENT_BUILDING_LIBRARY", None)
    //     .define("TORRENT_SSL_PEERS", None)
    //     .define("TORRENT_USE_I2P", "1")
    //     .define("TORRENT_USE_LIBCRYPTO", None)
    //     .define("TORRENT_USE_OPENSSL", None)
    //     .define("TORRENT_USE_UNC_PATHS", None)
    //     .define("_FILE_OFFSET_BITS", "64")
    //     .include("/usr/include/boost")
    //     .include("libtorrent/include")
    //     .include("libtorrent/include/libtorrent")
    //     .include("libtorrent/deps/try_signal");

    let mut build = cxx_build::bridge("src/lib.rs");

    // Set C++ standard
    build.cpp(true).flag_if_supported("-std=c++14");

    // Optimization and visibility flags
    build
        .flag_if_supported("-O3")
        .flag_if_supported("-finline-functions")
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fvisibility-inlines-hidden");

    // Warning flags
    build.warnings(false).extra_warnings(false);
    //     .flag_if_supported("-Wall")
    //     .flag_if_supported("-Wextra")
    //     .flag_if_supported("-Wpedantic")
    //     .flag_if_supported("-Wvla")
    //     .flag_if_supported("-Wno-inline")
    //     .flag_if_supported("-Wno-noexcept-type")
    //     .flag_if_supported("-Wno-format-zero-length");

    // Template depth
    build.flag_if_supported("-ftemplate-depth-512");

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
    build
        .include("/opt/homebrew/include")
        .include("/usr/local/opt/boost/include")
        .include("/usr/include/boost")
        .include("libtorrent/include")
        .include("libtorrent/include/libtorrent")
        .include("libtorrent/deps/try_signal");

    // Enable threading
    build.flag("-pthread");

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
    for cpp in glob("wrap/*.hpp").unwrap().flatten() {
        println!("cargo:rerun-if-changed={}", cpp.display());
    }
}
