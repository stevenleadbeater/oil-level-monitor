CREATE SEQUENCE distance_history_id_seq;
CREATE TABLE distance_history (
    id INT NOT NULL DEFAULT nextval('distance_history_id_seq') UNIQUE,
    distance_id INT NOT NULL REFERENCES distance (id),
    distance INT NOT NULL,
    time_of_reading TIMESTAMP WITH TIME ZONE
);
ALTER SEQUENCE distance_history_id_seq OWNED BY distance_history.id;