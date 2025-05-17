mod wordcounter;
mod wordcounterw;

fn main() {
    wordcounter::run(); // cargo run -- test.txt
    wordcounterw::run_w(); // cargo run -- -w test.txt
}
