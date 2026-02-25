fn main() {
    println!("cargo:rerun-if-changed=../../schema/linear.graphql");
    cynic_codegen::register_schema("linear")
        .from_sdl_file("../../schema/linear.graphql")
        .expect("Failed to register Linear schema")
        .as_default()
        .expect("Failed to set as default schema");
}
