use tokio_postgres::{NoTls, Row};
use crate::model::distance_history::DistanceHistory;

pub struct DistanceHistoryRepository {
    pub connection_string: String,
}

impl DistanceHistoryRepository {
    pub async fn get_by_distance_id(&self, distance_id: i32) -> Result<Vec<Row>, String> {
        match tokio_postgres::connect(self.connection_string.clone().as_str(), NoTls).await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                match client.query(r##"
                    SELECT
                        id, distance_id, distance, time_of_reading
                    FROM
                        distance_history
                    WHERE
                        distance_id = $1
                    ORDER BY time_of_reading DESC
                            "##, &[&distance_id]).await {
                    Ok(rows) => Ok(rows),
                    Err(_) => Err("Query failed".to_string())
                }
            }
            Err(_) => Err("Cannot connect to database".to_string())
        }
    }

    pub async fn get_latest_distance_history(&self, distance_id: i32) -> Result<Vec<Row>, String> {
        match tokio_postgres::connect(self.connection_string.clone().as_str(), NoTls).await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                match client.query(r##"
                    SELECT
                        id, distance_id, distance, time_of_reading
                    FROM
                        distance_history
                    WHERE
                        distance_id = $1
                    ORDER BY time_of_reading DESC
                    LIMIT 1
                            "##, &[&distance_id]).await {
                    Ok(rows) => Ok(rows),
                    Err(_) => Err("Query failed".to_string())
                }
            }
            Err(_) => Err("Cannot connect to database".to_string())
        }
    }

    pub async fn insert(&self, distance: DistanceHistory) -> Result<(), String> {
        match tokio_postgres::connect(self.connection_string.clone().as_str(), NoTls).await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                match client.execute(r##"
                    INSERT INTO distance_history
                        (distance_id, distance, time_of_reading)
                    VALUES
                        ($1, $2, $3)
                            "##, &[&distance.distance_id, &distance.distance, &distance.time_of_reading]).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Query failed".to_string())
                }
            }
            Err(_) => Err("Cannot connect to database".to_string())
        }
    }
}