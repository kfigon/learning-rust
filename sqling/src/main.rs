use rusqlite::Connection;

#[derive(Debug, PartialEq, Eq)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    println!("hello")
}

#[test]
fn dao_test() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB)", ()).unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    
    conn.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data)).unwrap();

    let mut statement = conn.prepare("SELECT id, name, data FROM person where name = (?1)").unwrap();

    let data: Vec<Person> = statement.query_map([("Steven".to_owned())], |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            data: row.get(2).unwrap(),
        })
    }).unwrap()
    .filter_map(|v| v.ok())
    .collect();

    assert_eq!(vec![Person{id: 1, name: "Steven".to_string(), data: None}], data);
}