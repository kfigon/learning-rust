// we dont need to derive them
// Send marker trait - we can move value to a thread
// Sync marker trait - we can pass readonly ref to a thread

#[cfg(test)]
mod for_join_test {
    use std::{sync::Arc, collections::HashSet};

    struct BigConfig{
        foo: String
    }

    fn process(v: String, big_config: &BigConfig) -> Result<String, &'static str> {
        if v == "" {
            return Err("invalid data");
        }
        Ok(format!("{v} {}", big_config.foo))
    }

    #[test]
    fn for_join_test() {
        // we cant pass read reference to move closure, we need Arc
        let conf = Arc::new(BigConfig{foo: "world!".to_string()});

        let vals = vec![
            "hello".to_string(),
            "welcome".to_string(),
            "".to_string()
        ];

        let mut threads = vec![];
        for v in vals {
            let c = conf.clone(); // just bumps the counter, does not copy the struct. Just reference does not live long enough
            let t = std::thread::spawn(move|| {
                process(v, &c)
            });
            threads.push(t);
        }

        let mut res: HashSet<Result<String, &'static str>> = HashSet::new();
        // wait for complete
        for t in threads {
            let v = t.join().expect("one child thread panicked");
            res.insert(v);
        }

        assert_eq!(
            HashSet::from_iter(vec![
                Ok("hello world!".to_string()),
                Ok("welcome world!".to_string()),
                Err("invalid data"),
            ])
            , res);
    }
}

#[cfg(test)]
mod test {
    // thread wrappers

    use std::{sync::{Arc, Mutex}, thread};

    // it's usually better to use fork-join or channels, but sometimes
    // shared state is required

    // RwLock is nice alternative for data that changes infrequently, but is read often

    // more tools - CondVar, atomics
    #[test]
    fn mutex_test() {
        let data = "hello".to_string();
        let data = Arc::new(Mutex::new(data));

        // mutex.lock().unwrap()
        // mutex.lock().unwrap() // deadlock - locking same mutex twice

        let d1 = data.clone();
        let t1 = thread::spawn(move|| {
            let mut d = d1.lock().unwrap();
            d.push('!');
        });

        let d2 = data.clone();
        let t2 = thread::spawn(move|| {
            let mut d = d2.lock().unwrap();
            d.push('!');
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let result = data.lock().unwrap();
        assert_eq!(result.as_str(), "hello!!");
    }
}