extern crate prost_build;
fn main() {
    prost_build::compile_protos(&["src/date.proto", "src/person.proto", "src/group.proto"],
                                &["src/"]).unwrap();
}