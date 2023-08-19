use postgres::{Client, NoTls, Row};

static USER: &'static str ="postgres";
static PASSWORD: &'static str ="postgres";

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
}

impl From<&Row> for Person {
    fn from(row: &Row) -> Self {
        Self { 
            id: row.get(0),
            name: row.get(1),
         }
    }
}

#[test]
fn connect_to_db() {
    let mut client = Client::connect(&format!("host=localhost user={USER} password={PASSWORD}"), NoTls).expect("error connecting to db");

    client.execute("truncate person", &[]).expect("error truncating");
    client.execute("ALTER SEQUENCE person_id_seq RESTART WITH 1;", &[]).expect("error reseting sequence");

    client.execute("INSERT INTO person (name) VALUES ($1)", &[&"Ferris"]).expect("error inserting");
    client.execute("INSERT INTO person (name) VALUES ($1)", &[&"Steve"]).expect("error inserting");

    let data = client.query("SELECT * FROM person order by id", &[])
        .expect("error querying")
        .iter()
        .map(|row| row.into())
        .collect::<Vec<Person>>();

    assert_eq!(data, vec![
        Person{id: 1, name: "Ferris".to_string()},
        Person{id: 2, name: "Steve".to_string()},
    ]);

}