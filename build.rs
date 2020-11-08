use protoc_rust;
use protoc_rust::Customize;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/proto/chess")
        .inputs(&["proto/chess/cell.proto", "proto/chess/move_command.proto"])
        .include("proto")
        .customize(Customize {
            expose_fields: Some(true),
            generate_accessors: Some(false),
            ..Default::default()
        })
        .run()
        .expect("protoc");
}
