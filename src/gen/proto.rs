use super::data::*;

pub fn proto_gen(t: &Table) -> String {
    let cols = t.columns.clone();

    let result = cols
        .iter()
        .enumerate()
        .map(|(i, col)| col_to_proto(col, i))
        .collect::<Vec<String>>()
        .join("\n");

    let sing = t.singular();

    let res = format!("message {sing} {{\n{result}\n}}",);

    return res;
}

pub fn convert_table_fncs(t: &Table) -> String {
    let cols = t.columns.clone();

    let result = cols
        .iter()
        .enumerate()
        .map(|(i, col)| col_convert_proto(col))
        .collect::<Vec<String>>()
        .join("\n");

    let sing = t.singular();

    let res =
        format!("func (t {sing}) Pb() *pb.{sing} {{\n  return &pb.{sing} {{\n{result}\n}}\n}}",);

    return res;
}

pub fn create_crud_service(t: &Table) -> String {
    let sing = t.singular();
    let sing_snake = t.sing_snake();
    let name = t.name();

    let res = format!(
        r#"service {sing}CrudService {{
            rpc Create{sing}(Create{sing}Request) returns (Create{sing}Response) {{}}
            rpc Get{sing}(Get{sing}Request) returns (Get{sing}Response) {{}}
            rpc Update{sing}(Update{sing}Request) returns (Update{sing}Response) {{}}
            rpc Delete{sing}(Delete{sing}Request) returns (Delete{sing}Response) {{}}
            rpc List{sing}s(List{sing}sRequest) returns (List{sing}sResponse) {{}}
          }})
          
          message Create{sing}Request {{
            {sing} {sing_snake} = 1;
          }}

           message Create{sing}Response {{
            {sing} {sing_snake} = 1;
        }}

        message Get{sing}Request {{
            string id = 1;
        }}
        
        message Get{sing}Response {{
            {sing} {sing_snake} = 1;
        }}

        message Update{sing}Request {{
            {sing} {sing_snake} = 1;
        }}

        message Update{sing}Response {{
            {sing} {sing_snake} = 1;
        }}

        message Delete{sing}Request {{
            string id = 1;
        }}

        message Delete{sing}Response {{
            string id = 1;
        }}

        message List{sing}sRequest {{
            int32 limit = 1;
            int32 offset = 2;
        }}

        message List{sing}sResponse {{
            repeated {sing} {name}s = 1;
        }}
        "#
    );

    return res;
}

fn col_convert_proto(c: &Column) -> String {
    let name = c.singular();
    let go_name = c.ideomatic_go();

    let t = c.r#type.clone().unwrap().name;

    if t == "timestamp" {
        return format!("{name}: timestamppb.New({go_name}),",);
    }

    format!("{name}: {go_name},")
}

fn col_to_proto(c: &Column, id: usize) -> String {
    let col_type = c.r#type.clone().unwrap().name;
    let mut proto_type = db2proto(&col_type);

    if c.is_array {
        proto_type = format!("repeated {}", proto_type);
    } else if !c.not_null {
        proto_type = format!("optional {}", proto_type);
    }
    return format!("  {} {} = {};", db2proto(&proto_type), c.name(), id + 1,);
}

fn db2proto(i: &String) -> String {
    if i == "text" || i == "varchar" {
        return "string".to_string();
    }
    if i == "timestamp" {
        return "google.protobuf.Timestamp".to_string();
    }
    if i == "int" {
        return "int32".to_string();
    }

    return i.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_to_proto() {
        let table_str = include_str!("./tests/table.json");
        let table: Table = serde_json::from_str(table_str).unwrap();

        let res = proto_gen(&table);
        println!("{}", res);
        assert_eq!(res, "message Author {\n  string id = 1\n  string name = 2\n  optional text bio = 3\n  google.protobuf.Timestamp created_at = 4\n  google.protobuf.Timestamp updated_at = 5\n}");
    }

    #[test]
    fn test_convert_fn() {
        let table_str = include_str!("./tests/table.json");
        let table: Table = serde_json::from_str(table_str).unwrap();

        let res = convert_table_fncs(&table);
        println!("{}", res);
        assert_eq!(res, "func (t *Author) Pb() *pb.Author {\n  return &pb.Author{\n    Id: t.Id,\n    Name: t.Name,\n    Bio: t.Bio,\n    CreatedAt: t.CreatedAt,\n    UpdatedAt: t.UpdatedAt,\n  }\n}");
    }
}
