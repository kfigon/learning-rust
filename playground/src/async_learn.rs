#[cfg(test)]
mod test {
    // like threads, but blocking is delegated to scheduler. Something like green threads and non blocking code
    // IOs are long - we're mostly waiting for OS or network. That waiting can be used to process something else
    // IOs return Futures(aka promises) and can be polled for completion. 
    #[test]
    #[ignore = "learn async in the future"]
    fn learn_async() {
        todo!()
    }
}