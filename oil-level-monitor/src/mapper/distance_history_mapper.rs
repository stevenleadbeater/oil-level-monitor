use crate::model::distance_history::DistanceHistory;
use std::time::SystemTime;
use tokio_postgres::Row;

pub fn map_one(rows: Vec<Row>) -> Result<DistanceHistory, String> {
    if rows.len() != 1 {
        return Err("Expecting exactly one row for queries by id".to_string());
    }
    if rows.get(0).unwrap().len() != 4 {
        return Err("Expecting exactly two columns for distance rows".to_string());
    }
    Ok(to_distance_history(rows.get(0).unwrap()))
}

pub fn map_many(rows: Vec<Row>) -> Result<Vec<DistanceHistory>, String> {
    if rows.get(0).unwrap().len() != 4 {
        return Err("Expecting exactly two columns for distance rows".to_string());
    }
    Ok(rows.into_iter().map(|row| {
        to_distance_history(&row)
    }).collect())
}

fn to_distance_history(row: &Row) -> DistanceHistory {
    let id: i32 = row.get(0);
    let distance_id: i32 = row.get(1);
    let distance: i32 = row.get(2);
    let time_of_reading: SystemTime = row.get(3);
    DistanceHistory {
        id: Some(id),
        distance_id,
        distance,
        time_of_reading,
    }
}