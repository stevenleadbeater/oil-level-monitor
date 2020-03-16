use crate::model::distance::Distance;
use tokio_postgres::Row;

pub fn map(rows: Vec<Row>) -> Result<Distance, String> {
    if rows.len() != 1 {
        return Err("Expecting exactly one row for queries by id".to_string());
    }
    if rows.get(0).unwrap().len() != 2 {
        return Err("Expecting exactly two columns for distance rows".to_string());
    }
    let id: i32 = rows.get(0).unwrap().get(0);
    let distance: i32 = rows.get(0).unwrap().get(1);
    Ok(Distance {
        id,
        distance,
    })
}