# romkan

A Romaji/Kana conversion library written in pure Rust, based on [python-romkan](https://github.com/soimort/python-romkan)

## Availablity

Available on crates.io. MIT License.

## Usage

```rust
use romkan::Romkan;

fn main() {
    println!("{}", "kyouhatennkigaiidesune".to_hiragana());
    // きょうはてんきがいいですね
    println!("{}", "きょうはてんきがいいですね".to_romaji());
    // kyouhatenkigaiidesune
}
```
