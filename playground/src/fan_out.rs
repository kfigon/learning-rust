use std::{thread, vec};
use std::sync::mpsc::channel;

fn get_results() -> Vec<String> {

    let (tx, rx) = channel::<String>();
    let requests = vec![1,2,3,4,5,6];
    let req_len = requests.len();

    for i in requests {
        let t = tx.clone();
        thread::spawn(move || { // we need to move sender channel. Incidentally also requests are moved
            t.send(i.to_string()).unwrap();
        });
    }

    // typically we should be using thread join and channel as iterator
    let mut out: Vec<String> = Vec::new();
    for _ in 0..req_len {
        out.push(rx.recv().unwrap());
    }

    out
}

#[test]
fn aggregator() {
    let mut r = get_results();
    r.sort();

    assert_eq!(r, vec!["1","2","3","4","5","6"]);
}