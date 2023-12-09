
#[cfg(feature = "soundtest")]
use midi_fundsp::{sounds::options, SoundTestResult};

#[cfg(feature = "soundtest")]
fn main() {
    for (name, func) in options() {
        println!("Testing {name}");
        let result = SoundTestResult::test(func);
        result.report();
    }
}

#[cfg(not(feature = "soundtest"))]
fn main() {
    println!("This example requires the soundtest feature.");
}
