use convert_case::{Case, Casing};
use plugin::{Column, Table};
use pluralizer::pluralize;

pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));
}

trait Named {
    fn name(&self) -> String;
    fn camel_case_name(&self) -> String {
        self.name().to_case(Case::UpperCamel)
    }
    fn plural(&self) -> String {
        pluralize(self.camel_case_name().as_str(), 2, false)
    }
    fn singular(&self) -> String {
        pluralize(self.camel_case_name().as_str(), 1, false)
    }
}

impl Named for Table {
    fn name(&self) -> String {
        self.rel.as_ref().unwrap().name.clone()
    }
}

pub fn crud(t: &Table) -> Option<String> {
    let create = create_query(t)?;
    let update = update_query(t)?;
    let list = list_query(t)?;
    let cnt = count_list_query(t)?;
    let delete = delete_query(t)?;

    let head = include_str!("./head.txt");

    let res = format!(
        "{}\n\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
        head, create, update, list, cnt, delete
    );
    Some(res)
}

pub fn create_query(table: &Table) -> Option<String> {
    let cols = get_create_cols(&table.columns);

    let col_list = list_columns_with_prefix(&cols, "");
    let params_list = list_columns_with_prefix(&cols, "@");

    let res = format!(
        "-- name: Create{} :one \nINSERT INTO {} ({}) VALUES ({}) RETURNING *;",
        table.singular(),
        table.name(),
        col_list,
        params_list
    );

    Some(res)
}

pub fn update_query(t: &Table) -> Option<String> {
    let cols = get_update_cols(&t.columns);
    let upds = cols
        .iter()
        .map(|c| get_update_sql(c))
        .collect::<Vec<String>>()
        .join(",\n  ");

    let res = format!(
        "-- name: Update{} :one \nUPDATE {} SET \n  {} \n  WHERE {} RETURNING *;",
        t.singular(),
        t.name(),
        upds,
        select_one(t)
    );

    Some(res)
}

pub fn delete_query(t: &Table) -> Option<String> {
    let res = format!(
        "-- name: Delete{} :one \nDELETE FROM {} WHERE {} RETURNING *;",
        t.singular(),
        t.name(),
        select_one(t)
    );

    Some(res)
}

pub fn list_query(t: &Table) -> Option<String> {
    let res = format!(
        "-- name: List{} :many \nSELECT * FROM {} LIMIT @take OFFSET @skip;",
        t.plural(),
        t.name(),
    );

    Some(res)
}

pub fn count_list_query(t: &Table) -> Option<String> {
    let res = format!(
        "-- name: CountList{} :one \nSELECT COUNT(*) FROM {};",
        t.plural(),
        t.name(),
    );

    Some(res)
}

fn select_one(_: &Table) -> &str {
    "id = @id"
}

fn get_create_cols(cols: &Vec<plugin::Column>) -> Vec<plugin::Column> {
    cols.iter()
        .filter(|col| col.name != "updated_at" && col.name != "created_at")
        .map(|c| c.clone())
        .collect::<Vec<plugin::Column>>()
}

fn get_update_cols(cols: &Vec<plugin::Column>) -> Vec<plugin::Column> {
    cols.iter()
        .filter(|col| col.name != "id" && col.name != "created_at")
        .map(|c| c.clone())
        .collect::<Vec<plugin::Column>>()
}

fn list_columns_with_prefix(cols: &Vec<Column>, prefix: &str) -> String {
    cols.iter()
        .map(|col| format!("{}{}", prefix, col.name).to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn get_update_sql(col: &Column) -> String {
    if col.name == "updated_at" {
        return format!("{} = now()", col.name);
    }

    format!(
        "{} = COALESCE(sqlc.narg({})::{}, {})",
        col.name,
        col.name,
        col.r#type.to_owned().unwrap().name,
        col.name
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use plugin;

    #[test]
    fn test_list_columns() {
        let table_json = include_str!("./tests/table.json");
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        let cols = table.columns;
        let prefix = "@";

        let res = list_columns_with_prefix(&cols, prefix);
        assert_eq!(res, "@id, @name, @bio, @created_at, @updated_at");

        let res = list_columns_with_prefix(&cols, "");
        assert_eq!(res, "id, name, bio, created_at, updated_at");
    }

    #[test]
    fn test_create_query() {
        let table_json = include_str!("./tests/table.json");
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        let res = create_query(&table);
        assert_eq!(res, Some("-- name: CreateAuthor :one \nINSERT INTO authors (id, name, bio) VALUES (@id, @name, @bio) RETURNING *;".to_string()));
    }

    #[test]
    fn test_delete_query() {
        let table_json = include_str!("./tests/table.json");
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        let res = delete_query(&table);
        assert_eq!(
            res,
            Some(
                "-- name: DeleteAuthor :one \nDELETE FROM authors WHERE id = @id RETURNING *;"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_list_query() {
        let table_json = include_str!("./tests/table.json");
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        let res = list_query(&table);
        assert_eq!(
            res,
            Some(
                "-- name: ListAuthors :many \nSELECT * FROM authors LIMIT @take OFFSET @skip;"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_count_list_query() {
        let table_json = include_str!("./tests/table.json");
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        let res = count_list_query(&table);
        assert_eq!(
            res,
            Some("-- name: CountListAuthors :one \nSELECT COUNT(*) FROM authors;".to_string())
        );
    }

    #[test]
    fn test_update_query() {
        let table_json = include_str!("./tests/table.json");
        let table: plugin::Table = serde_json::from_str(table_json).unwrap();

        let res = update_query(&table);

        assert_eq!(
            res,
            Some(
                "-- name: UpdateAuthor :one \nUPDATE authors SET \n  name = COALESCE(sqlc.narg(name)::text, name),\n  bio = COALESCE(sqlc.narg(bio)::text, bio),\n  updated_at = now() \n  WHERE id = @id RETURNING *;"
                    .to_string()
            )
        );
    }
}
