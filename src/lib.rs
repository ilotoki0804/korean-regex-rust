//! # korean-regex
//!
//! 한글은 초성, 중성, 종성의 조합이기에 각각을 분리해 분석하거나 사용하는 것이 때때로 유용합니다.
//!
//! korean-regex는 한글을 초성, 중성, 종성의 조합으로 사용할 수 있도록 합니다.
//!
//! ## Syntax
//!
//! 기본적으로 korean-regex는 한 글자 OR 문법의 확장입니다. 정규표현식의 다른 문법은 건드리지 않습니다.
//!
//! 우선 초성, 중성, 종성은 각각 `[]`로 둘러싸인 뒤 `:`으로 분리됩니다.
//! 예를 들어 아래의 예시처럼 `[ㄱ:ㅏ:ㄱ]`일 경우 초성에 `ㄱ`, 중성에 `ㅏ`, 종성에 `ㄱ`이 각각 들어가 `각`이 됩니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[각]", compile("[ㄱ:ㅏ:ㄱ]", order).unwrap().to_string());
//! ```
//!
//! 한 파트에 두 개 이상의 문자를 적으면 각 가능한 경우의 수로 변환됩니다.
//! 예를 들어 아래처럼 `[ㄱㄴ:ㅏㅣ:ㄴ]`일 경우 모든 경우의 수인 `간긴난닌`로 대체됩니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[간긴난닌]", compile("[ㄱㄴ:ㅏㅣ:ㄴ]", order).unwrap().to_string());
//! ```
//!
//! 만약 해당 칸은 비워놓는다면 해당 자리는 어떤 것이든 받아들이겠다는 의미입니다.
//! 예를 들어 `[::ㅎ]`은 '종성이 `ㅎ`인 모든 음소'를 의미합니다.
//!
//! ```no_run
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[갛갷걓걯...흏흫힇힣]", compile("[::ㅎ]", order).unwrap().to_string());
//! ```
//!
//! `-`을 통해 연속되는 음소를 대체할 수 있습니다.
//!
//! 이때 기본적으로 `ㄱㄴㄷㄹ...`가 *아닌* `ㄱㄲㄴㄷㄸㄹ...`와 같은 사전순으로 match된다는 점을 주의해 주세요.
//! 이는 중성과 종성도 동일합니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[간깐난단딴란]", compile("[ㄱ-ㄹ:ㅏ:ㄴ]", order).unwrap().to_string());
//! assert_eq!("[간갠갼걘건겐견곈곤관괜괸굔군권궨귄균근긘긴]", compile("[ㄱ:ㅏ-ㅣ:ㄴ]", order).unwrap().to_string());
//! assert_eq!("[간-갈]", compile("[ㄱ:ㅏ:ㄴ-ㄹ]", order).unwrap().to_string());
//! ```
//!
//! `0`은 해당 자리에 음소가 없다는 것을 의미합니다. 기본적으로 종성에 사용됩니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[가각간나낙난다닥단]", compile("[ㄱㄴㄷ:ㅏ:0ㄱㄴ]", order).unwrap().to_string());
//! ```
//!
//! 하지만 특수하게 `[*:0:0]`이나 `[0:*:0]`과 같은 형태도 사용될 수 있습니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[ㄱㄲㄴㄷㄸㄹ]", compile("[ㄱ-ㄹ:0:0]", order).unwrap().to_string());
//! assert_eq!("[ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ]", compile("[0:ㅏ-ㅣ:0]", order).unwrap().to_string());
//! ```
//!
//! `^`을 이용하면 해당 음소에 match하고 싶은 문자 대신 match하기 싫은 문자를 지정할 수 있습니다.
//! 예를 들어 초성이 `ㄱ`이고 중성이 `ㅏ`이면서 받침이 `ㄹ`이 아닌 모든 음소는 `[ㄱ:ㅏ:^ㄹ]`로 표현할 수 있습니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[가-갇갉-갛]", compile("[ㄱ:ㅏ:^ㄹ]", order).unwrap().to_string());
//! ```
//!
//! 만약 종성이 없는 문자를 match하고 싶다면 `[*:*:0]` 대신 `[*:*]` 문법을 사용할 수도 있습니다.
//! 예를 들어 `[ㄱㄴㄷ:ㅏㅣ:0]`는 `[ㄱㄴㄷ:ㅏㅣ]`로 대체될 수 있습니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[가기나니다디]", compile("[ㄱㄴㄷ:ㅏㅣ:0]", order).unwrap().to_string());
//! assert_eq!("[가기나니다디]", compile("[ㄱㄴㄷ:ㅏㅣ]", order).unwrap().to_string());
//! ```
//!
//! 만약 별개로 몇 개의 글자를 match에 추가하고 싶다면 `|`를 그 뒤에 추가하면 됩니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[과구놔누돠두한abc]", compile("[ㄱㄴㄷ:ㅜㅘ|한abc]", order).unwrap().to_string());
//! ```
//!
//! 한글에는 두 개 이상의 글자가 합쳐서 생성된 문자들이 있습니다. `ㄲ`이나 `ㄼ`, `ㅢ` 등이 그 예입니다.
//! 만약 글자 입력기가 `ㄺ`같은 문자를 입력하는 것을 지원하지 않거나, 미관상의 이유로 코드에서 피하고 싶다면
//! 괄호를 사용해서 문자를 합칠 수 있습니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[곿괇궧궯뽟뽧쀇쀏]", compile("[ㄱ(ㅂㅂ):(ㅗㅏ)(ㅜㅔ):(ㄹㅂ)(ㄱㅅ)]", order).unwrap().to_string());
//! ```
//! 
//! 이 고유 문법이 적용되는 범위를 넘어서면 기본 정규 표현식과 같이 섞어 사용할 수 있습니다.
//! 
//! ```rust
//! use korean_regex::*;
//! 
//! let order = Order::Default;
// ! // ㅇ이 초성인 글자로 단어가 시작하는 세 글자 이하의 모든 단어를 찾음.
//! let pattern = compile(r"\b([아-잏][^ ]{0,2})\b", order).unwrap();
//! let input = "저기 양을 잡아먹는 이리 때가 오르막길을 타고 간다!";
//! let result: Vec<_> = pattern.find_iter(input).map(|m| m.as_str()).collect();
//! assert_eq!(vec!["양을", "이리"], result);
//! ```
//!
//! ## Example
//!
//! 다른 문법과 합치면 다음과 같이 사용할 수 있습니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! // 초성이 ㄱ이 아니고 그 뒤에 종성이 `ㅇ`인 모든 글자가 오며 그 다음 글자 바운더리 혹은 종성이 없는 문자가 있는 경우
//! let pattern = compile(r"[^ㄱ::][::ㅇ](\b|[:])", order).unwrap();
//! let result: Vec<_> = pattern
//!     .captures_iter("한글은 초성, 중성, 종성의 조합이기에 각각을 분리해 분석하거나 사용하는 것이 때때로 유용합니다.")
//!     .map(|captures| captures[0].to_string())
//!     .collect();
//! assert_eq!(vec!["초성", "중성", "종성의", "사용하"], result)
//! ```
//!
//! ## Hyphen replacing
//!
//! 정규표현식의 `[]` 문법에는 연속되는 문자를 대체하는 `-` 문법이 있습니다.
//!
//! 만약 연속되는 문자가 세 개 이상 있다면 korean-regex에서도 `-`문법이 이용됩니다.
//!
//! ```rust
//! use korean_regex::*;
//!
//! let order = Order::Default;
//! assert_eq!("[가-깋라-맇]", compile("[ㄱㄹ::]", order).unwrap().to_string());
//! ```

mod substitute;

use regex::Regex;
pub use substitute::substitute;

type CompiledOrders<'a> = (&'a [char], &'a [char], &'a [char]);

const CHOSUNGS: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ',
    'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];
const JUNGSUNGS: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ',
    'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];
const JONGSUNGS: [char; 28] = [
    '0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ','ㄼ', 'ㄽ', 'ㄾ',
    'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'
];
const CHOSUNGS_REGFIRST: [char; 19] = [
    'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ',
    'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ', 'ㄲ', 'ㄸ', 'ㅃ', 'ㅆ', 'ㅉ',
];
const JUNGSUNGS_REGFIRST: [char; 21] = [
    'ㅏ', 'ㅑ', 'ㅓ', 'ㅕ', 'ㅗ', 'ㅛ', 'ㅜ', 'ㅠ', 'ㅡ', 'ㅣ', 'ㅐ', 'ㅒ', 'ㅔ', 'ㅖ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅢ'
];
const JONSGSUNGS_REGFIRST: [char; 28] = [
    '0', 'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ',
    'ㅎ', 'ㄲ', 'ㄳ', 'ㄵ', 'ㄶ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅄ', 'ㅆ'
];

/// korean-regex에서 나올 수 있는 모든 오류를 모아놓은 enum입니다.
#[derive(Debug)]
pub enum KoreanRegexError {
    /// 괄호로 묶인 문자를 합치는 것에 실패했을 때 나타나는 오류입니다.
    /// 예를 들어 `(ㄱㅇ)`는 적절하지 않은 괄호 문법이기에 오류를 냅니다.
    UnparenthesizingFailedError(String),
    /// 하이픈이 맨 앞이나 맨 뒤에 나오거나 문자 인덱스가 잘못되었을 경우 발생합니다.
    /// 예를 들어 `[-ㅅ::]`는 하이픈의 앞에 해당하는 문자가 없기에 오류를 냅니다.
    /// 또한 `[ㅅ-ㄱ::]`은 하이픈의 앞의 문자가 뒤의 문자보다 인덱스가 더 크기에 오류가 납니다.
    InvalidHyphenError(String),
    /// 0은 종성이 위치에서는 올 수 있지만 초성이나 중성에서는 제한적이게만 가능합니다.
    /// 예를 들어 `[0ㄱ:ㅏ0:ㄱ]`같이 초성이나 중성에는 0을 섞어서 쓸 수 없으며 오직
    /// 0이거나 0이 아닌 문자들이거나 둘 중 하나만 가능합니다.
    /// 또한 중성이 없는 한글은 없기에 `[ㄱ:0:ㄱ]`같은 패턴 또한 불가능하며
    /// 초성이나 중성에 문자가 들어갈 수 있는 패턴은 오직 `[*:0:0]`이나 `[0:*:0]`뿐입니다.
    /// 이 규칙을 어겼을 경우 이 오류가 납니다.
    InvalidZeroPatternError(String),
    /// 한글 음소가 아닌 글자가 왔을 경우 발생합니다. 예를 들어 `[d:ㅏ:ㄴ]`은 이 오류를 발생시킵니다.
    InvalidPhonemeError(String, char),
    /// compile 함수에서 regex 관련 오류가 일어났을 경우 사용됩니다.
    RegexError(regex::Error),
}

/// 하이픈 구성 시 사용할 순서를 결정합니다.
///
/// 이 라이브러리와 유니코드, 한국의 글자 체계는 기본적으로 다음과 같은 글자 순서를 사용합니다.
///
/// ```raw
/// 초성: ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ
/// 중성: ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ
/// 종성: 0ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ
/// ```
///
/// 이 순서에는 장점도 있지만 단점도 있습니다 대표적으로는 `-`을 사용할 때 나타납니다.
///
/// `[ㄱ-ㄹ:0:0]`의 결과값은 `[ㄱㄲㄴㄷㄸㄹ]`입니다. 하지만 `ㄱ`부터
/// `ㄹ`의 결과값으로 `[ㄱㄴㄷㄹ]`를 얻는 것이 필요한 경우도 존재합니다.
/// 이때 `[ㄱㄴㄷㄹ]`로 적는 것 또한 좋지만 이를 `[ㄱ-ㄹ]`로 줄이는 것을
/// 더 선호할 수도 있습니다.
///
/// 이러한 경우 Order를 고쳐서 순서를 변경해 문제를 해결할 수 있습니다.
///
/// `Order::RegularFirst`를 순서로 사용하면 각각의 순서는 다음과 같이 변경됩니다.
///
/// ```raw
/// 초성: ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄸㅃㅆㅉ
/// 중성: ㅏㅑㅓㅕㅗㅛㅜㅠㅡㅣㅐㅒㅔㅖㅘㅙㅚㅝㅞㅟㅢ
/// 종성: 0ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄㅆ
/// ```
///
/// 이는 `[ㄱ-ㄹ:0:0]`의 결과값이 `[ㄱㄴㄷㄹ]`가 되도록 만듭니다.
///
/// 하지만 하이픈 대체에 영향을 줄 수 있어 이는 결과값의 순서에 영향을 주진 않습니다.
/// 예를 들어 `[ㄲㄴ:0:0]`의 결과는 `Order::Default`에서는
/// `[ㄲㄴ]`가 되고 `Order::RegularFirst`에서도 `[ㄲㄴ]`가 됩니다.
///
/// 하이픈 사용 시 두 순서 중에서 어느 것이 자신의 필요에 맞는지 확인하고 사용하시면 됩니다.
#[derive(Debug, Clone, Copy)]
pub enum Order {
    /// 기본 순서입니다.
    ///
    /// ```raw
    /// 초성: ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ
    /// 중성: ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ
    /// 종성: 0ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ
    /// ```
    Default,
    /// 정규 음운 선행 순서입니다.
    ///
    /// ```raw
    /// 초성: ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄸㅃㅆㅉ
    /// 중성: ㅏㅑㅓㅕㅗㅛㅜㅠㅡㅣㅐㅒㅔㅖㅘㅙㅚㅝㅞㅟㅢ
    /// 종성: 0ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄㅆ
    /// ```
    RegularFirst,
}

impl Default for Order {
    fn default() -> Self {
        Self::Default
    }
}

impl Order {
    /// (초성, 중성, 종성(+0))으로 이루어진 튜플을 반환합니다.
    pub fn order(self) -> (&'static [char], &'static [char], &'static [char]) {
        match self {
            Order::Default => {
                (&CHOSUNGS, &JUNGSUNGS, &JONGSUNGS)
            }
            Order::RegularFirst => {
                (&CHOSUNGS_REGFIRST, &JUNGSUNGS_REGFIRST, &JONSGSUNGS_REGFIRST)
            }
        }
    }
}

/// 컴파일 결과를 Regex로 컴파일하는 대신 String 값으로 받습니다.
///
/// `compile`은 단순히 `compilestr`의 결과를 `Regex::new`로 감싸는 함수일 뿐입니다.
///
/// ```rust
/// use korean_regex::*;
/// use regex::Regex;
///
/// let order = Order::Default;
/// assert_eq!(
///     compilestr("[ㄱ::]", order).unwrap(),
///     compile("[ㄱ::]", order).unwrap().to_string()
/// );
/// ```
///
/// 만약 다른 정규표현식 크레이트를 이용하고 싶은 경우에
/// compilestr을 이용해 다른 정규표현식 크레이트를 사용할 수 있습니다.
///
/// ```pass
/// use fancy_regex::Regex;
/// use korean_regex::*;
///
/// let pattern = compilestr(r"(?<![ㅎ:ㅏ:])[^ㄱ::][::ㅇ]", Order::Default);
/// let re = Regex::new(&pattern.unwrap()).unwrap();
/// ```
pub fn compilestr(pattern: &str, order: Order) -> Result<String, KoreanRegexError> {
    let korean_regex_pattern_finder = Regex::new(
        r"\[([0ㄱ-ㅎㅏ-ㅣ\^()-]*):([0ㄱ-ㅎㅏ-ㅣ\^()-]*)(:?)([0ㄱ-ㅎㅏ-ㅣ\^()-]*)(\|[^]]*)?\]",
    )
    .map_err(KoreanRegexError::RegexError)?;

    let mut final_error: Option<KoreanRegexError> = None;
    let result = korean_regex_pattern_finder
        .replace_all(pattern, |captured: &regex::Captures<'_>| {
            let chosungs = &captured[1];
            let jungsungs = &captured[2];
            let optional_delimiter = &captured[3];
            let jongsungs = if optional_delimiter.is_empty() {
                "0"
            } else {
                &captured[4]
            };
            let other_one_letter_options = captured
                .get(5)
                .map(|other_options| &other_options.as_str()[1..])
                .unwrap_or("");
            match substitute(chosungs, jungsungs, jongsungs, order, true) {
                Ok(result) => format!("[{}{}]", result, other_one_letter_options),
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

/// 한국어 regex가 담긴 패턴을 받아 Regex로 컴파일합니다.
pub fn compile(pattern: &str, order: Order) -> Result<regex::Regex, KoreanRegexError> {
    Regex::new(compilestr(pattern, order)?.as_str()).map_err(KoreanRegexError::RegexError)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_compilestr() {
        let order = Order::Default;
        assert_eq!(
            "123[강당항은]",
            compilestr("123[ㄱㄷㅎ:ㅏ:ㅇ|은]", order).unwrap()
        );
        assert_eq!(
            "123[ㄱㄷㅎ:d:ㅇ|은]",
            compilestr("123[ㄱㄷㅎ:d:ㅇ|은]", order).unwrap()
        );
        assert_eq!("[간긴난닌]", compilestr("[ㄱㄴ:ㅏㅣ:ㄴ]", order).unwrap());
        assert_eq!(
            "[가기나니다디]",
            compile("[ㄱㄴㄷ:ㅏㅣ]", order).unwrap().to_string()
        );
        match compilestr("123[ㄱㄷㅎ:(ㄱㄱㄱ):ㅇ|은]", order).unwrap_err() {
            KoreanRegexError::UnparenthesizingFailedError(_) => (),
            _ => panic!("Should raise UnparenthesizingFailedError"),
        }
    }
}
