#![no_main]
use libfuzzer_sys::fuzz_target;
use thingbuf::ThingBuf;

fuzz_target!(|input: Vec<String>| {
    if input.len() > 0 {
        let buf: ThingBuf<String> = ThingBuf::new(input.len());
        input.iter().for_each(|x| {
            let _ = buf.push(x.to_string());
        });

        for _ in 0..input.len() {
            buf.pop();
        }
    }
});