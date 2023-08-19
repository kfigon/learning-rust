CREATE TABLE IF NOT EXISTS person (
  id SERIAL PRIMARY KEY, -- default btree index
  name varchar(250) NOT NULL
);

CREATE TABLE IF NOT EXISTS task (
  id SERIAL PRIMARY KEY, -- default btree index
  owner_id int not NULL,
  description text not NULL,
  completed_at timestamp,

  CONSTRAINT fk_task_person FOREIGN KEY(owner_id) REFERENCES person(id)
);

CREATE UNIQUE INDEX person_name_idx ON person (name);
CREATE UNIQUE INDEX task_person_idx ON task (owner_id);
