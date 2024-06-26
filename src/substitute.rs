use std::char;

use crate::{CompiledOrders, KoreanRegexError, Order};

/// 초성, 중성, 종성 자리에 들어갈 raw값을 받고 실제로 컴파일된 값을 내보냅니다.
///
/// ```rust
/// use korean_regex::*;
///
/// assert_eq!("간긴난닌단딘", substitute("ㄱㄴㄷ", "ㅏㅣ", "ㄴ", Order::Default, true).unwrap());
/// ```
///
/// use_hyphen이 true라면 `ㄱㄴㄷㄹ`와 같은 연속된 문자열을 `ㄱ-ㄹ`과 같이 `-`을 이용한 식으로 변경하고,
/// false라면 변경하지 않습니다.
pub fn substitute<'a>(
    chosungs_raw: &'a str,
    jungsungs_raw: &'a str,
    jongsungs_raw: &'a str,
    order: Order,
    use_hyphen: bool,
) -> Result<String, KoreanRegexError> {
    let sanitize_raw_chars = |string, order| {
        let mut unparenthesized_chars = unparenthesize(string)?;

        let inverse: bool = if unparenthesized_chars.is_empty() {
            true
        } else if unparenthesized_chars[0] == '^' {
            unparenthesized_chars.remove(0);
            true
        } else {
            false
        };

        Ok(sanitize(unparenthesized_chars, order, inverse))
    };

    let (all_chosungs, all_jungsungs, all_jongsungs_with_zero) = order.order();
    let chosungs = if chosungs_raw == "0" {
        None
    } else {
        Some(sanitize_raw_chars(chosungs_raw, all_chosungs)??)
    };
    let jungsungs = if jungsungs_raw == "0" {
        None
    } else {
        Some(sanitize_raw_chars(jungsungs_raw, all_jungsungs)??)
    };
    let jongsungs = if jongsungs_raw == "0" {
        None
    } else {
        Some(sanitize_raw_chars(
            jongsungs_raw,
            all_jongsungs_with_zero,
        )??)
    };

    let regular_compiled_order = Order::Default.order();

    match (chosungs, jungsungs, jongsungs) {
        (None, None, None) =>
            Err(KoreanRegexError::InvalidZeroPatternError("[0:0:0] cannot be represented as Hangeul, thus invalid.".to_string())),
        (None, Some(jungsungs), Some(jongsungs)) =>
            Err(KoreanRegexError::InvalidZeroPatternError(
                format!("[0:{}:{}]([0:*:*] pattern) cannot be represented as Hangeul, thus invalid.",
                    jungsungs.into_iter().collect::<String>(),
                    jongsungs.into_iter().collect::<String>(),
                ),
            )),
        (Some(chosungs), None, Some(jongsungs)) =>
            Err(KoreanRegexError::InvalidZeroPatternError(
                    format!(
                        "[{}:0:{}]([*:0:*] pattern) cannot be represented as Hangeul, thus invalid.",
                        chosungs.into_iter().collect::<String>(),
                        jongsungs.into_iter().collect::<String>(),
                    ),
                )),
        (Some(chars), None, None)
        | (None, Some(chars), None)
        | (None, None, Some(chars)) => Ok(chars.into_iter().collect()),
        (Some(chosungs), Some(jungsungs), Some(jongsungs)) => {
            let mut result = String::new();
            for chosung in chosungs.iter() {
                for jungsung in jungsungs.iter() {
                    for jongsung in jongsungs.iter() {
                        result.push(convert_phonemes_to_syllable(
                            *chosung, *jungsung, Some(*jongsung), regular_compiled_order)?);
                    }
                }
            }
            Ok(if use_hyphen { replace_with_hyphen(result) } else { result })
        },
        (Some(first), Some(middle), None) => {
            let mut result = String::new();
            for chosung in first.iter() {
                for jungsung in middle.iter() {
                    result.push(convert_phonemes_to_syllable(
                        *chosung, *jungsung, None, regular_compiled_order)?);
                }
            }
            Ok(if use_hyphen { replace_with_hyphen(result) } else { result })
        },
    }
}

/// 이 크레이트에는 일부 조합형 글자를 괄호를 통해 표시하는 것이 가능합니다.
///
/// 예를 들어 `ㅢ`의 경우 `(ㅡㅣ)`로 표시할 수 있고, `ㄼ`의 경우 `ㄹㅂ`으로 표시할 수 있습니다.
/// 이는 글자 입력기가 조합을 지원하지 않는 경우 유용하게 사용할 수 있습니다.
///
/// ```rust
/// use korean_regex::*;
/// assert_eq!("[깕깗끩끫낅낇딹딻띍띏띩띫]",
///            compilestr("[(ㄱㄱ)ㄸ:ㅏㅣ(ㅡㅣ):(ㄹㅂ)ㄺ]", Order::Default).unwrap())
/// ```
fn unparenthesize(parenthesized_string: &str) -> Result<Vec<char>, KoreanRegexError> {
    let mut does_inside_parenthisis = false;
    let mut chars_inside_parenthesis = String::with_capacity(2);
    let mut unparenthesized_chars = Vec::with_capacity(parenthesized_string.len());
    for char in parenthesized_string.chars() {
        match char {
            '(' => {
                if does_inside_parenthisis {
                    return Err(KoreanRegexError::UnparenthesizingFailedError(
                        "Invalid Syntax: Open parenthesis after another open parenthesis."
                            .to_string(),
                    ));
                } else {
                    does_inside_parenthisis = true
                }
            }
            ')' => {
                if does_inside_parenthisis {
                    does_inside_parenthisis = false;
                    let converted_char = match chars_inside_parenthesis.as_str() {
                        "ㅗㅏ" => 'ㅘ',
                        "ㅗㅐ" => 'ㅙ',
                        "ㅗㅣ" => 'ㅚ',
                        "ㅜㅓ" => 'ㅝ',
                        "ㅜㅔ" => 'ㅞ',
                        "ㅜㅣ" => 'ㅟ',
                        "ㅡㅣ" => 'ㅢ',
                        "ㄱㅅ" => 'ㄳ',
                        "ㄴㅈ" => 'ㄵ',
                        "ㄴㅎ" => 'ㄶ',
                        "ㄹㄱ" => 'ㄺ',
                        "ㄹㅁ" => 'ㄻ',
                        "ㄹㅂ" => 'ㄼ',
                        "ㄹㅅ" => 'ㄽ',
                        "ㄹㅌ" => 'ㄾ',
                        "ㄹㅎ" => 'ㅀ',
                        "ㅂㅅ" => 'ㅄ',
                        "ㄱㄱ" => 'ㄲ',
                        "ㄷㄷ" => 'ㄸ',
                        "ㅈㅈ" => 'ㅉ',
                        "ㅂㅂ" => 'ㅃ',
                        other => {
                            return Err(KoreanRegexError::UnparenthesizingFailedError(format!(
                                "Invalid Syntax: Unknown item inside parenthesis({}).",
                                other
                            )));
                        }
                    };
                    chars_inside_parenthesis.clear();
                    unparenthesized_chars.push(converted_char);
                } else {
                    return Err(KoreanRegexError::UnparenthesizingFailedError(
                        "Invalid Syntax: Close parenthesis after another close parenthesis."
                            .to_string(),
                    ));
                }
            }
            others => {
                if does_inside_parenthisis {
                    chars_inside_parenthesis.push(others)
                } else {
                    unparenthesized_chars.push(others)
                }
            }
        }
    }
    Ok(unparenthesized_chars)
}

/// 이 함수는 다음과 같은 일을 합니다.
///
/// 1. hyphen이 이용된 경우 풀어 씁니다.
/// 1. 함수를 orders의 순서대로 정렬합니다.
/// 1. 같은 글자가 있을 경우 중복을 제거합니다.
/// 1. 만약 inverse=True일 경우 결과값을 뒤집습니다.
fn sanitize(
    unsanitized_chars: Vec<char>,
    order: &[char],
    inverse: bool,
) -> Result<Vec<char>, KoreanRegexError> {
    fn add_chars_in_range(
        char_present_table: &mut [bool],
        char_before_hyphen: char,
        char_after_hyphen: char,
        order: &[char],
    ) -> Result<(), KoreanRegexError> {
        let before_letter_index = order.iter().position(|&r| r == char_before_hyphen).ok_or(
            KoreanRegexError::InvalidPhonemeError(
                format!("Charactor `{}` is not valid phoneme.", char_before_hyphen),
                char_before_hyphen,
            ),
        )?;
        let after_letter_index = order.iter().position(|&r| r == char_after_hyphen).ok_or(
            KoreanRegexError::InvalidPhonemeError(
                format!("Charactor `{}` is not valid phoneme.", char_after_hyphen),
                char_before_hyphen,
            ),
        )?;

        if before_letter_index > after_letter_index {
            return Err(KoreanRegexError::InvalidHyphenError(format!(
                "The charactor before hyphen({char_before_hyphen}) is bigger than\
                         the charactor after it({char_after_hyphen})."
            )));
        }

        #[allow(clippy::needless_range_loop)]
        for j in before_letter_index + 1..after_letter_index {
            char_present_table[j] = true;
        }
        Ok(())
    }

    let mut char_present_table = vec![false; order.len()];
    if !unsanitized_chars.is_empty() {
        for (char_index, unsanitized_char) in unsanitized_chars.iter().enumerate() {
            for (order_index, char_in_order) in order.iter().enumerate() {
                if unsanitized_char == &'-' {
                    if char_index >= unsanitized_chars.len() - 1 || char_index == 0 {
                        return Err(KoreanRegexError::InvalidHyphenError(
                            "Position of hyphen is invalid.".to_string(),
                        ));
                    }

                    let char_before_hyphen = unsanitized_chars[char_index - 1];
                    let char_after_hyphen = unsanitized_chars[char_index + 1];

                    add_chars_in_range(
                        &mut char_present_table,
                        char_before_hyphen,
                        char_after_hyphen,
                        order,
                    )?;
                } else if unsanitized_char == char_in_order {
                    char_present_table[order_index] = true;
                    break;
                }
            }
        }
    }

    if inverse {
        char_present_table = char_present_table.into_iter().map(|value| !value).collect();
    }

    let mut result = Vec::with_capacity(order.len());
    for (chr, does_present) in order.iter().zip(char_present_table) {
        if does_present {
            result.push(*chr);
        }
    }
    Ok(result)
}

/// 한국어 음소(ㄱ,ㅏ,ㅢ, 등)를 모아 하나의 음절(가,각, 등)로 만듭니다.
///
/// 이때 마지막 음소는 None이 될 수 있습니다.
///
/// 만약 한글 음소가 아니거나 잘못된 위치라면 InvalidPhonemeError를 냅니다.
///
/// orders는 한글 음소의 순서인데, Order::Default.compile()의 결과만 받습니다.
fn convert_phonemes_to_syllable(
    chosung: char,
    jungsung: char,
    jongsung: Option<char>,
    orders: CompiledOrders,
) -> Result<char, KoreanRegexError> {
    let (all_chosungs, all_jungsungs, all_jongsungs_with_zero) = orders;

    let Some(chsung_position) = all_chosungs.iter().position(|chr| chr == &chosung) else {
        return Err(KoreanRegexError::InvalidPhonemeError(
            format!("{chosung} is not valid phoneme."),
            chosung,
        ));
    };
    let Some(jungsung_position) = all_jungsungs.iter().position(|chr| chr == &jungsung) else {
        return Err(KoreanRegexError::InvalidPhonemeError(
            format!("{jungsung} is not valid phoneme."),
            jungsung,
        ));
    };
    let jongsung_position = if let Some(last) = jongsung {
        let Some(jongsung_position) = all_jongsungs_with_zero.iter().position(|chr| chr == &last) else {
            return Err(KoreanRegexError::InvalidPhonemeError(
                format!("{last} is not valid phoneme."),
                last,
            ));
        };
        jongsung_position
    } else {
        0
    };

    Ok(char::from_u32(
        (0xAC00 + 588 * chsung_position + 28 * jungsung_position + jongsung_position) as u32,
    )
    .expect("This charactor conversion should succeed. Please create issue if this panic present."))
}

/// `ㄱㄴㄷㄹ`와 같이 연속된 문자들을 `ㄱ-ㄹ`와 같이 `-`를 이용해 압축합니다.
fn replace_with_hyphen(string: String) -> String {
    fn collect_hyphen(hyphen_replaced_chars: &mut Vec<char>, continuous_chars: &mut Vec<char>) {
        if continuous_chars.len() <= 2 {
            hyphen_replaced_chars.append(continuous_chars);
        } else {
            hyphen_replaced_chars.push(continuous_chars[0]);
            hyphen_replaced_chars.push('-');
            hyphen_replaced_chars.push(continuous_chars[continuous_chars.len() - 1]);
            continuous_chars.clear();
        }
    }

    let mut hyphen_replaced_chars: Vec<char> = Vec::new();
    let mut continuous_chars: Vec<char> = Vec::new();
    for chr in string.chars() {
        if continuous_chars.is_empty() || *continuous_chars.last().unwrap() as u32 + 1 == chr as u32
        {
            continuous_chars.push(chr);
            continue;
        }
        collect_hyphen(&mut hyphen_replaced_chars, &mut continuous_chars);
        continuous_chars.push(chr);
    }
    collect_hyphen(&mut hyphen_replaced_chars, &mut continuous_chars);

    hyphen_replaced_chars.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unparenthesize() {
        assert_eq!(
            vec![
                'ㄼ', 'ㄱ', 'ㄷ', 'ㅊ', 'ㅁ', 'ㅌ', 'ㅈ', 'ㅁ', 'ㄷ', 'ㅘ', 'ㅢ', 'ㅓ', 'ㅑ', 'ㅢ',
                'ㅓ', 'ㅕ', 'ㅢ'
            ],
            unparenthesize("(ㄹㅂ)ㄱㄷㅊㅁㅌㅈㅁㄷ(ㅗㅏ)(ㅡㅣ)ㅓㅑㅢㅓㅕ(ㅡㅣ)")
                .unwrap()
        );
        assert_eq!(
            Vec::<char>::new(),
            unparenthesize("").unwrap()
        );

        match unparenthesize("(ㄹㅂ)ㄱㄷ(ㅊㅁㅌㅈㅁㄷ(ㅗㅏ)(ㅡㅣ)ㅓㅑㅢㅓㅕ(ㅡㅣ)")
            .unwrap_err()
        {
            KoreanRegexError::UnparenthesizingFailedError(_) => (),
            _ => panic!("Shoud raise UnparenthesizingFailedError"),
        };
        match unparenthesize("(ㄹㅂ)ㄱㄷ(ㅊㅁㅌㅈㅁㄷ(ㅗㅏ)(ㅡㅣ)ㅓㅑㅢㅓㅕ(ㅡ)ㅣ)")
            .unwrap_err()
        {
            KoreanRegexError::UnparenthesizingFailedError(_) => (),
            _ => panic!("Shoud raise UnparenthesizingFailedError"),
        };
        match unparenthesize("(ㄹㅂ)ㄱㄷㅊㅁㅌㅈㅁㄷ(ㅗㅏ)(ㅡㅣ)ㅓㅑ)ㅢㅓㅕ(ㅡㅣ)")
            .unwrap_err()
        {
            KoreanRegexError::UnparenthesizingFailedError(_) => (),
            _ => panic!("Shoud raise UnparenthesizingFailedError"),
        };
        match unparenthesize("(ㄹㅂ)ㄱㄷㅊㅁㅌㅈㅁㄷ(ㅗㅏ)(ㅡㅣ)ㅓㅑㅢㅓㅕ(ㅡㅣ))")
            .unwrap_err()
        {
            KoreanRegexError::UnparenthesizingFailedError(_) => (),
            _ => panic!("Shoud raise UnparenthesizingFailedError"),
        };
    }

    #[test]
    fn test_convert() {
        let order = Vec::from_iter("ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ".chars());

        assert_eq!(
            "ㄴㅃㅎ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㅃ', 'ㄴ', 'ㅎ'], &order, false).unwrap()
        );
        assert_eq!(
            "ㄴㅃㅎ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㅃ', 'ㄴ', 'ㅎ', 'ㅎ', 'ㅃ'], &order, false).unwrap()
        );
        assert_eq!(
            "ㄱㄲㄷㄸㄹㅁㅂㅅㅆㅇㅈㅉㅊㅋㅌㅍ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㅃ', 'ㄴ', 'ㅎ'], &order, true).unwrap()
        );
        assert_eq!(
            "ㄴㄷㄸㄹㅃㅎ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㅃ', 'ㄴ', '-', 'ㄹ', 'ㅎ'], &order, false).unwrap()
        );
        assert_eq!(
            "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㅃ', 'ㄱ', '-', 'ㅎ'], &order, false).unwrap()
        );
        assert_eq!(
            "ㄱㄲㅁㅂㅅㅆㅇㅈㅉㅊㅋㅌㅍ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㅃ', 'ㄴ', '-', 'ㄹ', 'ㅎ'], &order, true).unwrap()
        );

        match sanitize(vec!['ㅃ', 'ㄹ', '-', 'ㄴ', 'ㅎ'], &order, true).unwrap_err() {
            KoreanRegexError::InvalidHyphenError(_) => (),
            _ => panic!("Shoud raise InvalidHyphenError"),
        };

        // 확인할 수 없는 문자열 있을 때 검사
        assert_eq!(
            "ㄴㄷㄸㄹ".chars().collect::<Vec<char>>(),
            sanitize(vec!['ㄴ', '-', 'ㄹ', 'h', ' '], &order, false).unwrap()
        );
        match sanitize(vec!['ㄴ', '-', 'h'], &order, false).unwrap_err() {
            KoreanRegexError::InvalidPhonemeError(..) => (),
            _ => panic!("Shoud raise InvalidPhonemeError"),
        };

        // 빈 문자열 검사
        assert_eq!(
            "".chars().collect::<Vec<char>>(),
            sanitize(Vec::<char>::new(), &order, false).unwrap()
        );
        assert_eq!(
            "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ".chars().collect::<Vec<char>>(),
            sanitize(Vec::<char>::new(), &order, true).unwrap()
        );

        match sanitize(vec!['-', 'ㅃ', 'ㄴ', 'ㄹ', 'ㅎ'], &order, true).unwrap_err() {
            KoreanRegexError::InvalidHyphenError(_) => (),
            _ => panic!("Shoud raise InvalidHyphenError"),
        };
        match sanitize(vec!['ㅃ', 'ㄴ', 'ㄹ', 'ㅎ', '-'], &order, true).unwrap_err() {
            KoreanRegexError::InvalidHyphenError(_) => (),
            _ => panic!("Shoud raise InvalidHyphenError"),
        };
    }

    #[test]
    fn test_convert_phoneme_to_syllable() {
        assert_eq!(
            '둳',
            convert_phonemes_to_syllable('ㄷ', 'ㅝ', Some('ㄷ'), Order::Default.order())
                .unwrap()
        );
        assert_eq!(
            '둬',
            convert_phonemes_to_syllable('ㄷ', 'ㅝ', None, Order::Default.order()).unwrap()
        );
        match convert_phonemes_to_syllable('ㄷ', 'ㅝ', Some('d'), Order::Default.order())
            .unwrap_err()
        {
            KoreanRegexError::InvalidPhonemeError(_, syllable) => assert_eq!('d', syllable),
            _ => panic!("Shoud raise InvalidPhonemeError"),
        };
        match convert_phonemes_to_syllable('ㄷ', 'f', Some('ㅇ'), Order::Default.order())
            .unwrap_err()
        {
            KoreanRegexError::InvalidPhonemeError(_, syllable) => assert_eq!('f', syllable),
            _ => panic!("Shoud raise InvalidPhonemeError"),
        };
        match convert_phonemes_to_syllable('ㄷ', 'ㄷ', Some('ㅇ'), Order::Default.order())
            .unwrap_err()
        {
            KoreanRegexError::InvalidPhonemeError(_, syllable) => assert_eq!('ㄷ', syllable),
            _ => panic!("Shoud raise InvalidPhonemeError"),
        };
        match convert_phonemes_to_syllable('ㅏ', 'ㅏ', Some('ㅇ'), Order::Default.order())
            .unwrap_err()
        {
            KoreanRegexError::InvalidPhonemeError(_, syllable) => assert_eq!('ㅏ', syllable),
            _ => panic!("Shoud raise InvalidPhonemeError"),
        };
        match convert_phonemes_to_syllable('e', 'ㅏ', Some('ㅇ'), Order::Default.order())
            .unwrap_err()
        {
            KoreanRegexError::InvalidPhonemeError(_, syllable) => assert_eq!('e', syllable),
            _ => panic!("Shoud raise InvalidPhonemeError"),
        };
    }

    #[test]
    fn test_subtitude() {
        let order = Order::Default;

        assert_eq!("가각갋갖긔긕긟긪기긱긻깆다닥닯닺듸듹딃딎디딕딟딪아악앏앚의읙읣읮이익읿잊차착찳찾츼츽칇칒치칙칣칮",
                   substitute("ㄱㄷㅊㅇ", "ㅏㅣ(ㅡㅣ)", "ㄱ(ㄹㅂ)ㅈ0", order, false).unwrap());
        assert_eq!(
            "가긔기다듸디아의이차츼치",
            substitute("ㄱㄷㅊㅇ", "ㅏㅣ(ㅡㅣ)", "0", order, false).unwrap()
        );

        assert_eq!(
            "다닥닦닧단닩닪닫달닭닮닯닰닱닲닳담답닶닷닸당닺닻닼닽닾닿",
            substitute("ㄷ", "ㅏ", "", order, false).unwrap()
        );
        assert_eq!(
            "닿댛댷덓덯뎋뎧돃돟돻됗됳둏둫뒇뒣뒿듛듷딓딯",
            substitute("ㄷ", "", "ㅎ", order, false).unwrap()
        );
        assert_eq!(
            "갛깧낳닿땋랗맣밯빻샇쌓앟잫짷챃캏탛팧핳",
            substitute("", "ㅏ", "ㅎ", order, false).unwrap()
        );

        assert_eq!(
            "ㄱㄷㅇㅊ",
            substitute("ㄱㄷㅊㅇ", "0", "0", order, false).unwrap()
        );
        assert_eq!(
            "ㅏㅗㅢ",
            substitute("0", "ㅏ(ㅡㅣ)ㅗ", "0", order, false).unwrap()
        );
        assert_eq!(
            "ㄼㅅㅆㅇ",
            substitute("0", "0", "ㅇ(ㄹㅂ)ㅅㅆ", order, false).unwrap()
        );

        // hyphen 대체 테스트
        assert_eq!(
            "가-깋라-맇바-빟",
            substitute("ㄱㄹㅂ", "", "", order, true).unwrap()
        );
        assert_eq!(
            "강당항",
            substitute("ㄱㄷㅎ", "ㅏ", "ㅇ", order, true).unwrap()
        );
        assert_eq!("가각갋갖긔긕긟긪기긱긻깆다닥닯닺듸듹딃딎디딕딟딪아악앏앚의읙읣읮이익읿잊차착찳찾츼츽칇칒치칙칣칮",
                   substitute("ㄱㄷㅊㅇ", "ㅏㅣ(ㅡㅣ)", "ㄱ(ㄹㅂ)ㅈ0", order, false).unwrap());

        match substitute("0", "0", "0", order, false).unwrap_err() {
            KoreanRegexError::InvalidZeroPatternError(_) => (),
            _ => panic!("Shoud raise InvalidZeroPatternError"),
        }
        match substitute("0", "ㅏ", "ㅁ", order, false).unwrap_err() {
            KoreanRegexError::InvalidZeroPatternError(_) => (),
            _ => panic!("Shoud   raise InvalidZeroPatternError"),
        }
        match substitute("ㅎ", "0", "ㅁ", order, false).unwrap_err() {
            KoreanRegexError::InvalidZeroPatternError(_) => (),
            _ => panic!("Shoud raise InvalidZeroPatternError"),
        }
    }

    #[test]
    fn test_subtitude_with_regular_first_order() {
        let order = Order::RegularFirst;

        assert_eq!("가각갖갋기긱깆긻긔긕긪긟다닥닺닯디딕딪딟듸듹딎딃아악앚앏이익잊읿의읙읮읣차착찾찳치칙칮칣츼츽칒칇",
                   substitute("ㄱㄷㅊㅇ", "ㅏㅣ(ㅡㅣ)", "ㄱ(ㄹㅂ)ㅈ0", order, false).unwrap());
        assert_eq!(
            "가기긔다디듸아이의차치츼",
            substitute("ㄱㄷㅊㅇ", "ㅏㅣ(ㅡㅣ)", "0", order, false).unwrap()
        );

        assert_eq!(
            "다닥단닫달담답닷당닺닻닼닽닾닿닦닧닩닪닭닮닯닰닱닲닳닶닸",
            substitute("ㄷ", "ㅏ", "", order, false).unwrap()
        );
        assert_eq!(
            "닿댷덯뎧돟둏둫듛듷딯댛덓뎋돃돻됗됳뒇뒣뒿딓",
            substitute("ㄷ", "", "ㅎ", order, false).unwrap()
        );
        assert_eq!(
            "갛낳닿랗맣밯샇앟잫챃캏탛팧핳깧땋빻쌓짷",
            substitute("", "ㅏ", "ㅎ", order, false).unwrap()
        );

        assert_eq!(
            "ㄱㄷㅇㅊ",
            substitute("ㄱㄷㅊㅇ", "0", "0", order, false).unwrap()
        );
        assert_eq!(
            "ㅏㅗㅢ",
            substitute("0", "ㅏ(ㅡㅣ)ㅗ", "0", order, false).unwrap()
        );
        assert_eq!(
            "ㅅㅇㄼㅆ",
            substitute("0", "0", "ㅇ(ㄹㅂ)ㅅㅆ", order, false).unwrap()
        );

        assert_eq!(
            "가각간갇갈감갑갓강-갛갂갃갅갆갉-갏값갔",
            &substitute("ㄱ", "ㅏ", "", order, true).unwrap()
        );

        match substitute("0", "0", "0", order, false).unwrap_err() {
            KoreanRegexError::InvalidZeroPatternError(_) => (),
            _ => panic!("Shoud raise InvalidZeroPatternError"),
        }
        match substitute("0", "ㅏ", "ㅁ", order, false).unwrap_err() {
            KoreanRegexError::InvalidZeroPatternError(_) => (),
            _ => panic!("Shoud raise InvalidZeroPatternError"),
        }
        match substitute("ㅎ", "0", "ㅁ", order, false).unwrap_err() {
            KoreanRegexError::InvalidZeroPatternError(_) => (),
            _ => panic!("Shoud raise InvalidZeroPatternError"),
        }
    }

    #[test]
    fn test_replace_with_hyphen() {
        dbg!(replace_with_hyphen("강당항".to_string()));
    }
}
