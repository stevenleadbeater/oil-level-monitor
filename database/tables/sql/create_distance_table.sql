CREATE SEQUENCE distance_id_seq;
CREATE TABLE distance (
    id INT NOT NULL DEFAULT nextval('distance_id_seq'),
    distance INT NOT NULL
);
ALTER SEQUENCE distance_id_seq OWNED BY distance.id;