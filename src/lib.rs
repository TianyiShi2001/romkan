mod data;
pub use data::*;
use lazy_static::lazy_static;
use regex::Regex;

pub trait Romkan {
    /// Convert a Romaji (ローマ字) to a Katakana (片仮名).
    fn to_katakana(self) -> String;
    /// Convert a Romaji (ローマ字) to a Hiragana (平仮名).
    fn to_hiragana(self) -> String;
    /// Convert a Kana (仮名) to a Hepburn Romaji (ヘボン式ローマ字).
    fn to_romaji(self) -> String;
    /// Convert a Kana (仮名) or a Kunrei-shiki Romaji (訓令式ローマ字) to a Hepburn Romaji (ヘボン式ローマ字).
    fn to_hepburn(self) -> String;
    /// Convert a Kana (仮名) or a Hepburn Romaji (ヘボン式ローマ字) to a Kunrei-shiki Romaji (訓令式ローマ字).
    fn to_kunrei(self) -> String;
}

impl<S> Romkan for S
where
    S: AsRef<str>,
{
    fn to_katakana(self) -> String {
        _convert(self, &ROMPAT, &ROMKAN)
    }

    fn to_hiragana(self) -> String {
        _convert(self, &ROMPAT_H, &ROMKAN_H)
    }
    fn to_romaji(self) -> String {
        to_hepburn_assume_kana(self.as_ref())
    }
    fn to_hepburn(self) -> String {
        let inp = self.as_ref();
        let tmp = to_hepburn_assume_kana(inp);
        if &tmp != inp {
            tmp
        } else {
            // is kunrei
            let tmp = tmp.to_lowercase();
            let tmp = normalize_double_n(&tmp);
            _replace_all(&tmp, &KUNPAT, &TO_HEPBURN)
        }
    }
    fn to_kunrei(self) -> String {
        let hepburn = self.to_hepburn();
        _replace_all(&hepburn, &HEPPAT, &TO_KUNREI)
    }
}

fn to_hepburn_assume_kana(inp: &str) -> String {
    let tmp = _replace_all(inp, &KANPAT_H, &KANROM_H);
    let tmp = _replace_all(&tmp, &KANPAT, &KANROM);
    remove_redundant_apostrophes(&tmp)
}

lazy_static! {
    static ref REDUNDANT_APOSTROPHE: Regex = Regex::new("n'([^aiueoyn]|$)").unwrap();
}

fn remove_redundant_apostrophes(input: &str) -> String {
    REDUNDANT_APOSTROPHE.replace_all(input, "n$1").into_owned()
}

fn normalize_double_n(input: &str) -> String {
    // Replace double n with n'
    let res = input.replace("nn", "n'");
    remove_redundant_apostrophes(&res)
}

fn _replace_all<S: AsRef<str>>(
    input: S,
    pattern: &Regex,
    map: &phf::Map<&'static str, &'static str>,
) -> String {
    pattern
        .replace_all(input.as_ref(), |caps: &regex::Captures<'_>| map[&caps[0]])
        .into_owned()
}

fn _convert<S: AsRef<str>>(
    input: S,
    pattern: &Regex,
    map: &phf::Map<&'static str, &'static str>,
) -> String {
    let input = input.as_ref().to_lowercase();
    let input = normalize_double_n(&input);
    _replace_all(&input, pattern, map)
}
