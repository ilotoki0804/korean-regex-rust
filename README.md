# korean-regex

한글은 초성, 중성, 종성의 조합이기에 각각을 분리해 분석하거나 사용하는 것이 때때로 유용합니다.

korean-regex는 한글을 초성, 중성, 종성의 조합으로 사용할 수 있도록 합니다.

자세한 설명은 [문서](https://docs.rs/korean_regex/latest/korean_regex/)를 참고하세요.

```rust
use korean_regex::*;

let order = Order::Default;
// 마지막 음절이 받침이 없는 모든 음절 캡쳐
let pattern = compile(r"[::]*[:]\b", &order).unwrap();
let result: Vec<_> = pattern
    .captures_iter("한글은 초성, 중성, 종성의 조합이기에 각각을 분리해 분석하거나 사용하는 것이 때때로 유용합니다.")
    .map(|captures| captures[0].to_string())
    .collect();
assert_eq!(vec!["종성의", "조합이기에", "분리해", "분석하거나", "것이", "때때로", "유용합니다"], result)
```