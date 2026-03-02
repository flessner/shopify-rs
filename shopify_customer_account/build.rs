fn main() {
    cynic_codegen::register_schema("ca")
        .from_sdl_file("schemas/customer_account.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
