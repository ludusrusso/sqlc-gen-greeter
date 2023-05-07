use gen::{crud, Named};
use gen::{CodeGenRequest, CodeGenResponse, Config};
use utils::{get_tables, run_plugin};

mod gen;
mod utils;

pub fn create_codegen_response(req: &CodeGenRequest) -> CodeGenResponse {
    let opts: Config =
        serde_json::from_slice(&req.clone().settings.unwrap().codegen.unwrap().options).unwrap();

    let tables = get_tables(req);
    let files = tables
        .iter()
        .map(|t| table_to_crud_file(t, &opts))
        .flatten()
        .collect();

    CodeGenResponse {
        files: files,
        ..Default::default()
    }
}

fn table_to_crud_file(table: &gen::Table, opts: &Config) -> Option<gen::File> {
    let cr = crud(&table, opts)?;

    let file = gen::File {
        name: format!("{}_crud.gen.sql", table.name()),
        contents: cr.as_bytes().to_vec(),
    };

    Some(file)
}

fn main() -> Result<(), prost::DecodeError> {
    run_plugin(create_codegen_response).unwrap();

    Ok(())
}
