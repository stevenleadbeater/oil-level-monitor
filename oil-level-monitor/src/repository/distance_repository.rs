use crate::model::distance::Distance;
use tokio_postgres::{NoTls, Row};
use crate::repository::connection_string::connection_string;

pub async fn get_by_id(id: i32) -> Result<Vec<Row>, String> {
    match tokio_postgres::connect(connection_string().clone().as_str(), NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });
            match client.query(r##"
            SELECT
                id, distance
            FROM
                distance
            WHERE
                id = $1
                            "##, &[&id]).await {
                Ok(rows) => Ok(rows),
                Err(_) => Err("Query failed".to_string())
            }
        }
        Err(_) => Err("Cannot connect to database".to_string())
    }
}

pub async fn upsert(distance: Distance) -> Result<(), String> {
    match tokio_postgres::connect(connection_string().clone().as_str(), NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });
            match client.execute(r##"
                    INSERT INTO distance (id, distance) VALUES ($1, $2)
                    ON CONFLICT (id)
                    DO
                        UPDATE
                        SET distance = $2
                        WHERE distance.id = $1;
                            "##, &[&distance.id, &distance.distance]).await {
                Ok(_) => Ok(()),
                Err(_) => Err("Query failed".to_string())
            }
        }
        Err(_) => Err("Cannot connect to database".to_string())
    }
}