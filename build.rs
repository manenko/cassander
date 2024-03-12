use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=cassandra");

    let bindings = bindgen::Builder::default()
        .header("src/driver/ffi/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .c_naming(true)
        .must_use_type("CassError")
        .must_use_type("CassErrorResult")
        .must_use_type("CassFuture")
        .enable_function_attribute_detection()
        .generate()
        .expect(
            "unable to generate bindings for DataStax C/C++ Driver for Apache \
             Cassandra",
        );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect(
        "unable to write bindings for DataStax C/C++ Driver for Apache \
         Cassandra",
    );
}
