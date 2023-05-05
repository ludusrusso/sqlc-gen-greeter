use crud::crud;
use crud::plugin;
use plugin::CodeGenRequest;
use prost::Message;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;

mod crud;

pub fn deserialize_codegen_request(
    buf: &[u8],
) -> Result<plugin::CodeGenRequest, prost::DecodeError> {
    plugin::CodeGenRequest::decode(&mut Cursor::new(buf))
}

pub fn serialize_codegen_response(resp: &plugin::CodeGenResponse) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(resp.encoded_len());

    resp.encode(&mut buf).unwrap();
    buf
}

pub fn create_codegen_response(req: &CodeGenRequest) -> plugin::CodeGenResponse {
    let mut file = plugin::File::default();
    file.name = "hello.txt".to_string();
    file.contents = "Hello World Ludovico".as_bytes().to_vec();

    let mut resp = plugin::CodeGenResponse::default();
    resp.files.push(file);

    let mut file = plugin::File::default();
    file.name = "req.json".to_string();
    file.contents = serde_json::to_string(&req).unwrap().as_bytes().to_vec();

    req.catalog
        .iter()
        .for_each(|catalog| handle_catalog(&catalog, &mut resp));

    resp.files.push(file);
    resp
}

fn handle_catalog(catalog: &plugin::Catalog, res: &mut plugin::CodeGenResponse) {
    catalog
        .schemas
        .iter()
        .for_each(|schema| handle_schema(&schema, res));
}

fn handle_schema(schema: &plugin::Schema, res: &mut plugin::CodeGenResponse) {
    schema
        .tables
        .iter()
        .for_each(|table| handle_table(&table, res));
}

fn handle_table(table: &plugin::Table, res: &mut plugin::CodeGenResponse) {
    let mut file = plugin::File::default();
    let name = table.rel.clone().unwrap().name.clone();
    file.name = format!("{}_crud.gen.sql", name);
    file.contents = crud(&table).unwrap().as_bytes().to_vec();
    res.files.push(file);
}

fn main() -> Result<(), prost::DecodeError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    let req = match deserialize_codegen_request(&buffer) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
    };

    let resp = create_codegen_response(&req);
    let out = serialize_codegen_response(&resp);

    let _ = match io::stdout().write_all(&out) {
        Ok(result) => result,
        Err(_e) => std::process::exit(1),
    };

    Ok(())
}

// write test for handle_table
#[cfg(test)]
mod tests {
    use super::*;
    use plugin;

    #[test]
    fn test_handle_table() {
        let table_json = include_str!("./tests/table.json");

        let mut res = plugin::CodeGenResponse::default();
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        handle_table(&table, &mut res);
        assert_eq!(res.files.len(), 1);
        assert_eq!(res.files[0].name, "authors_crud.gen.sql");
        println!(
            "{}",
            String::from_utf8(res.files[0].contents.clone()).unwrap()
        );
    }
}
