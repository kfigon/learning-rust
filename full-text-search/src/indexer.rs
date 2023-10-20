use std::{collections::HashMap};

type Word = String;
type FilePath<'a> = &'a str;
struct TermFreq(HashMap<Word, f64>);
pub struct TfIdfScore<'a>(pub HashMap<Word, HashMap<FilePath<'a>, f64>>);

pub fn calc<'a>(docs: &HashMap<FilePath<'a>, &'a str>) -> TfIdfScore<'a> {
    let tfs = docs.iter().map(|s| (*s.0, tf(s.1))).collect::<HashMap<FilePath, TermFreq>>();
    let how_many_docs_has_it = |word: &String| tfs.iter().filter(|(_, freq)| freq.0.contains_key(word)).count();

    let num_of_docs = docs.len();

    let idf = tfs.iter()
        .flat_map(|(_, tf)| tf.0.iter())
        .map(|(word, _)| word)
        .fold(HashMap::new(), |mut acc, word| {
            let e = acc.entry(word).or_default();
            *e = ((num_of_docs as f64)/(how_many_docs_has_it(word) as f64)).log2();
            acc
        });

    let mut out = HashMap::new();

    for (&file_name, tf) in &tfs {
        for (word, term_freq) in &tf.0 {
            let e: &mut HashMap<&str,f64> = out.entry(word.to_owned()).or_default();
            e.insert(file_name, term_freq * idf.get(word).unwrap());
        }
    }

    TfIdfScore(out)
}

fn tf(input: &str) -> TermFreq {
    let normalize = |s: &&str| {
        let mut v = s.to_lowercase();
        v = v.replace(",", "");
        v = v.replace(".", "");
        v = v.replace("-", "");
        v
    };

    let splitted = &input.split_whitespace().collect::<Vec<_>>();

    let mut r = splitted
        .into_iter()
        .map(normalize)
        .fold(HashMap::new(), |mut acc, word| {
            let v = acc.entry(word).or_default();
            *v += 1 as f64;
            acc
        });

    r.iter_mut().for_each(|(_, freq)| *freq /= splitted.len() as f64);

    TermFreq(r)
}