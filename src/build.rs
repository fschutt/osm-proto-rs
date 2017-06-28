extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/osm.proto"],
                                &["src/"],
                                None).unwrap();
    prost_build::compile_protos(&["src/fileformat.proto"],
                                &["src/"],
                                None).unwrap();
}