use std::{collections::HashMap};

struct TermFreq<'a>(HashMap<&'a str, f64>);
pub struct TfIdfScore<'a>(pub HashMap<&'a str, HashMap<usize, f64>>);

pub fn calc<'a>(docs: &Vec<&'a str>) -> TfIdfScore<'a> {
    let tfs = docs.iter().map(|s| tf(s)).collect::<Vec<_>>();
    let mut idf: HashMap<&str, f64> = HashMap::new();


    let num_of_docs = docs.len();
    for tf in &tfs {

        for (word, _) in &tf.0 {
            let e = idf.entry(*word).or_default();
            let how_many_docs_has_it = tfs.iter().filter(|tf| tf.0.contains_key(*word)).count();

            *e = ((num_of_docs as f64)/(how_many_docs_has_it as f64)).log2();
        }
    }

    let mut out = HashMap::new();
    for (id, tf) in tfs.into_iter().enumerate() {
        for (word, term_freq) in tf.0 {
            let e: &mut HashMap<_,_> = out.entry(word).or_default();
            e.insert(id, term_freq * idf.get(word).unwrap());
        }
    }

    TfIdfScore(out)
}

fn tf<'a>(input: &'a str) -> TermFreq<'a> {
    let splitted = &input.split_whitespace().collect::<Vec<_>>();

    let mut r = splitted
        .into_iter()
        .fold(HashMap::new(), |mut acc, &word| {
            let v = acc.entry(word).or_default();
            *v += 1.0;
            acc
        });

    r.iter_mut().for_each(|(_, freq)| *freq /= splitted.len() as f64);

    TermFreq(r)
}