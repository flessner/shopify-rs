fn main() {
    cynic_codegen::register_schema("storefront")
        .from_sdl_file("schemas/storefront.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
