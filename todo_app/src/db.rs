use postgres::{Client, NoTls, Row};

static USER: &'static str ="postgres";
static PASSWORD: &'static str ="postgres";

#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
}

impl TryFrom<&Row> for Person {
    type Error = &'static str;

    fn try_from(value: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get(0).map_err(|_| "cant get id")?,
            name: value.try_get(1).map_err(|_| "cant get name")?,
        })
    }
}

#[test]
fn connect_to_db() {
    let mut client = Client::connect(&format!("host=localhost user={USER} password={PASSWORD}"), NoTls).expect("error connecting to db");

    client.execute("delete from task", &[]).expect("error truncating task");
    client.execute("delete from person", &[]).expect("error truncating person");

    client.execute("INSERT INTO person (name) VALUES ($1)", &[&"Ferris"]).expect("error inserting");
    client.execute("INSERT INTO person (name) VALUES ($1)", &[&"Steve"]).expect("error inserting");

    let data = client.query("SELECT id,name FROM person order by id", &[])
        .expect("error querying")
        .iter()
        .map(|row| row.try_into())
        .collect::<Result<Vec<Person>,_>>()
        .expect("error mapping rows");

    assert_eq!(data.len(), 2);
    assert_eq!(data[0].name, "Ferris".to_string());
    assert_eq!(data[1].name, "Steve".to_string());
}