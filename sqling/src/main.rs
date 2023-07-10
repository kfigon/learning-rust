use rusqlite::{Connection, Result, Params, Error};

#[derive(Debug, PartialEq, Eq)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

struct Dao {
    conn: Connection
}

impl Dao {
    fn new(conn: Connection) -> Self {
        Self { conn: conn }
    }

    fn execute<P: Params>(&self, sql: &str, par: P) -> Result<usize> {
        self.conn.execute(sql, par)
    }

    fn query<P: Params>(&self, sql: &str, par: P) -> Result<Vec<Person>, Error> {
        let mut stmt = self.conn.prepare(sql)?;

        let it = stmt.query_map(par, |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;

        it.collect()
    }
}

fn main() {
    println!("hello")
}

#[test]
fn dao_test() {
    let dao = Dao::new(Connection::open_in_memory().unwrap());

    dao.execute("CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB)", ()).unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    
    dao.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data)).unwrap();

    let iter = dao.query("SELECT id, name, data FROM person where name = (?1)", [("Steven".to_owned())]).unwrap();
    assert_eq!(vec![Person{id: 1, name: "Steven".to_string(), data: None}], iter);
}