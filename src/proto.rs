use gen::{CodeGenRequest, CodeGenResponse};
use utils::{get_tables, run_plugin};

mod gen;
mod utils;

pub fn create_codegen_response(req: &CodeGenRequest) -> CodeGenResponse {
    let mut resp = CodeGenResponse::default();

    let mut file = gen::File::default();
    file.name = "req.json".to_string();
    file.contents = serde_json::to_string(&req).unwrap().as_bytes().to_vec();

    resp.files.push(file);

    let tables = get_tables(req);

    let protocode = tables
        .iter()
        .map(|t| gen::proto_gen(t))
        .collect::<Vec<_>>()
        .join("\n\n\n");

    let code = format!(
        "syntax = \"proto3\";\n\nimport \"google/protobuf/timestamp.proto\"; \n\n{protocode}"
    );

    resp.files.push(gen::File {
        name: "proto.gen.proto".to_string(),
        contents: code.as_bytes().to_vec(),
    });

    resp
}

fn main() -> Result<(), prost::DecodeError> {
    run_plugin(create_codegen_response)
}
