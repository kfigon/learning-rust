use std::{collections::HashMap, iter::Iterator};

// will work also with std::env::args()
fn collect_args(v: impl Iterator<Item = String>) -> HashMap<String, String> {
    v.skip(1)
        .collect::<Vec<String>>()
        .chunks_exact(2)
        .map(|pair| (pair[0].to_owned(), pair[1].to_owned()))
        .collect::<HashMap<String, String>>()
}

#[test]
fn collect_args_empty() {
    let out = collect_args(Vec::<String>::new().into_iter());

    assert_eq!(out, HashMap::new());
}

#[test]
fn collect_args_pairs() {
    let out = collect_args(
        vec!["the path", "foo", "bar", "asd", "123"]
            .iter()
            .map(|v| v.to_string()),
    );

    assert_eq!(
        out,
        HashMap::from_iter(vec![
            ("foo".to_string(), "bar".to_string()),
            ("asd".to_string(), "123".to_string()),
        ])
    );
}

#[test]
fn collect_args_pairs_with_redundant_key() {
    let out = collect_args(
        vec!["the path", "foo", "bar", "asd", "123", "skip meh"]
            .iter()
            .map(|v| v.to_string()),
    );

    assert_eq!(
        out,
        HashMap::from_iter(vec![
            ("foo".to_string(), "bar".to_string()),
            ("asd".to_string(), "123".to_string()),
        ])
    );
}

#[test]
fn collect_args_pairs_to_struct() {
    let raw = vec!["the path", "foo", "bar", "asd", "123", "skip meh"];
    let args = raw.iter().map(|v| v.to_string());
    let res: Config = collect_args(args).try_into().expect("failed to parse data");

    assert_eq!(
        res,
        Config {
            foo: "bar".to_owned(),
            asd: 123
        }
    );
}

#[derive(Debug, Clone, PartialEq)]
struct Config {
    foo: String,
    asd: i32,
}

impl TryFrom<HashMap<String, String>> for Config {
    type Error = String;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let foo = map
            .get("foo")
            .map(|v| v)
            .ok_or("missing foo".to_string())?
            .to_owned();

        let asd = map
            .get("asd")
            .ok_or("missing asd".to_string())?
            .parse::<i32>()
            .map_err(|_| "parsing error")?;

        Ok(Self { foo, asd })
    }
}
