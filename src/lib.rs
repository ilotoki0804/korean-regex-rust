mod substitute;

use regex::Regex;
pub use substitute::substitute;

type CompiledOrders<'a> = (Vec<char>, Vec<char>, Vec<char>);

#[derive(Debug)]
pub enum KoreanRegexError {
    UnparenthesizingFailedError(String),
    InvalidHyphenError(String),
    InvalidZeroPatternError(String),
    InvalidPhonemeError(String, char),
    CharConversionFailedError,
    RegexError(regex::Error),
}

pub enum Order {
    Default,
    RegularFirst,
}

impl Order {
    fn compile(&self) -> CompiledOrders {
        match self {
            Order::Default => {
                let chosungs: Vec<char> =
                    "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ".chars().collect();
                let jungsungs: Vec<char> = "ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ"
                    .chars()
                    .collect();
                let jongsungs_with_zero: Vec<char> =
                    "0ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ"
                        .chars()
                        .collect();
                (chosungs, jungsungs, jongsungs_with_zero)
            }
            Order::RegularFirst => {
                let chosungs: Vec<char> =
                    "ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄸㅃㅆㅉ".chars().collect();
                let jungsungs: Vec<char> = "ㅏㅑㅓㅕㅗㅛㅜㅠㅡㅣㅐㅒㅔㅖㅘㅙㅚㅝㅞㅟㅢ"
                    .chars()
                    .collect();
                let jongsungs_with_zero: Vec<char> =
                    "0ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄㅆ"
                        .chars()
                        .collect();
                (chosungs, jungsungs, jongsungs_with_zero)
            }
        }
    }
}

pub fn compilestr(pattern: &str, orders: &Order) -> Result<String, KoreanRegexError> {
    let korean_regex_pattern_finder = match Regex::new(
        r"\[([0ㄱ-ㅎㅏ-ㅣ\^()-]*):([0ㄱ-ㅎㅏ-ㅣ\^()-]*)(:?)([0ㄱ-ㅎㅏ-ㅣ\^()-]*)(\|[^]]*)?\]",
    ) {
        Ok(result) => result,
        Err(regex_error) => return Err(KoreanRegexError::RegexError(regex_error)),
    };

    let mut final_error: Option<KoreanRegexError> = None;
    let result = korean_regex_pattern_finder
        .replace_all(pattern, |captured: &regex::Captures<'_>| {
            let chosung = &captured[1];
            let jungsung = &captured[2];
            let optional_delimiter = &captured[3];
            let jongsung = if optional_delimiter.is_empty() {
                "0"
            } else {
                &captured[4]
            };
            let other_options = captured
                .get(5)
                .map(|other_options| &other_options.as_str()[1..])
                .unwrap_or("");
            match substitute(chosung, jungsung, jongsung, orders, true) {
                Ok(result) => format!("[{}{}]", result, other_options),
                Err(error) => {
                    final_error = Some(error);
                    "(error)".to_string()
                }
            }
        })
        .into_owned();

    if let Some(error) = final_error {
        Err(error)
    } else {
        Ok(result)
    }
}

pub fn compile(pattern: &str, orders: &Order) -> Result<regex::Regex, KoreanRegexError> {
    match Regex::new(compilestr(pattern, orders)?.as_str()) {
        Ok(result) => Ok(result),
        Err(regex_error) => Err(KoreanRegexError::RegexError(regex_error)),
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_compilestr() {
        let order = Order::Default;
        assert_eq!(
            "123[강당항은]",
            compilestr("123[ㄱㄷㅎ:ㅏ:ㅇ|은]", &order).unwrap()
        );
        assert_eq!(
            "123[ㄱㄷㅎ:d:ㅇ|은]",
            compilestr("123[ㄱㄷㅎ:d:ㅇ|은]", &order).unwrap()
        );
        assert_eq!("[간긴난닌]", compilestr("[ㄱㄴ:ㅏㅣ:ㄴ]", &order).unwrap());
        assert_eq!("[가기나니다디]", compile("[ㄱㄴㄷ:ㅏㅣ]", &order).unwrap().to_string());
        match compilestr("123[ㄱㄷㅎ:(ㄱㄱㄱ):ㅇ|은]", &order).unwrap_err() {
            KoreanRegexError::UnparenthesizingFailedError(_) => (),
            _ => panic!("Should raise UnparenthesizingFailedError"),
        }
    }
}
