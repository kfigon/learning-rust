CREATE TABLE IF NOT EXISTS person (
  id SERIAL PRIMARY KEY,
  name varchar(250) NOT NULL
);

CREATE TABLE IF NOT EXISTS task (
  id SERIAL PRIMARY KEY,
  owner_id SERIAL,
  description text not NULL,
  completed_at timestamp,

  CONSTRAINT fk_task_person FOREIGN KEY(owner_id) REFERENCES person(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS person_name_idx ON person (name);
CREATE UNIQUE INDEX IF NOT EXISTS task_person_idx ON task (owner_id);
