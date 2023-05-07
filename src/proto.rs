use gen::{CodeGenRequest, CodeGenResponse, File, Named};
use utils::{get_tables, run_plugin};

mod gen;
mod utils;

pub fn get_proto_file(tables: &Vec<gen::Table>) -> gen::File {
    let protocode = tables
        .iter()
        .map(|t| gen::proto_gen(t))
        .collect::<Vec<_>>()
        .join("\n\n\n");

    let code = format!(
        "syntax = \"proto3\";\n\nimport \"google/protobuf/timestamp.proto\"; \n\n{protocode}"
    );

    gen::File {
        name: "proto.gen.proto".to_string(),
        contents: code.as_bytes().to_vec(),
    }
}

pub fn get_convs_file(tables: &Vec<gen::Table>) -> gen::File {
    let protocode = tables
        .iter()
        .map(|t| gen::convert_table_fncs(t))
        .collect::<Vec<_>>()
        .join("\n\n\n");

    gen::File {
        name: "proto.cnv.go".to_string(),
        contents: protocode.as_bytes().to_vec(),
    }
}

pub fn get_crud_svcs(tables: &Vec<gen::Table>) -> Vec<gen::File> {
    tables
        .iter()
        .map(|t| {
            let c = gen::create_crud_service(t);
            File {
                name: format!("{}_crud.proto", t.name()),
                contents: c.as_bytes().to_vec(),
            }
        })
        .collect::<Vec<gen::File>>()
}

pub fn create_codegen_response(req: &CodeGenRequest) -> CodeGenResponse {
    let tables = get_tables(req);

    let proto_file = get_proto_file(&tables);
    let convs_file = get_convs_file(&tables);

    let crud_files = get_crud_svcs(&tables);

    let mut files = vec![proto_file, convs_file];
    files.extend(crud_files);

    CodeGenResponse {
        files: files,
        ..Default::default()
    }
}

fn main() -> Result<(), prost::DecodeError> {
    run_plugin(create_codegen_response)
}
