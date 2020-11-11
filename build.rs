fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=src/msg.capnp");
    capnpc::CompilerCommand::new()
        .file("src/msg.capnp")
        .run()
        .unwrap();

    println!("cargo:rerun-if-changed=src/backend/tensorflow/tfwrapper.h");
    println!("cargo:rerun-if-changed=src/backend/tensorflow/tfwrapper.cc");
    println!("cargo:rerun-if-changed=src/backend/tensorflow/tfwrapper.rs");
    println!("cargo:rustc-link-lib=static=tfwrapper");
    cxx_build::bridge("src/backend/tensorflow/tfwrapper.rs")
        .file("src/backend/tensorflow/tfwrapper.cc")
        .flag("-std=c++17")
        .extra_warnings(false)
        .include("libtensorflow_cc/include")
        .include("/usr/local/cuda-10.1/include")
        .object("libtensorflow_cc/lib/libtensorflow_cc.so")
        .object("libtensorflow_cc/lib/libtensorflow_framework.so")
        .compile("tfwrapper");
}
