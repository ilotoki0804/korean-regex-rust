# korean-regex

한글은 초성, 중성, 종성의 조합이기에 각각을 분리해 분석하거나 사용하는 것이 때때로 유용합니다.

korean-regex는 한글을 초성, 중성, 종성의 조합으로 사용할 수 있도록 합니다.

## Syntax

기본적으로 korean-regex는 한 글자 OR 문법의 확장입니다. 정규표현식의 다른 문법은 건드리지 않습니다.

우선 초성, 중성, 종성은 각각 `[]`로 둘러싸인 뒤 `:`으로 분리됩니다.
예를 들어 아래의 예시처럼 `[ㄱ:ㅏ:ㄱ]`일 경우 초성에 `ㄱ`, 중성에 `ㅏ`, 종성에 `ㄱ`이 각각 들어가 `각`이 됩니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[각]", compile("[ㄱ:ㅏ:ㄱ]", &order).unwrap().to_string());
```

한 파트에 두 개 이상의 문자를 적으면 각 가능한 경우의 수로 변환됩니다.
예를 들어 아래처럼 `[ㄱㄴ:ㅏㅣ:ㄴ]`일 경우 모든 경우의 수인 `간긴난닌`로 대체됩니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[간긴난닌]", compile("[ㄱㄴ:ㅏㅣ:ㄴ]", &order).unwrap().to_string());
```

만약 해당 칸은 비워놓는다면 해당 자리는 어떤 것이든 받아들이겠다는 의미입니다. 예를 들어 `[::ㅎ]`은 '종성이 `ㅎ`인 모든 음소'를 의미합니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[갛갷걓걯...흏흫힇힣]", compile("[::ㅎ]", &order).unwrap().to_string());
```

`-`을 통해 연속되는 음소를 대체할 수 있습니다.

이때 기본적으로 `ㄱㄴㄷㄹ...`가 *아닌* `ㄱㄲㄴㄷㄸㄹ...`와 같은 사전순으로 match된다는 점을 주의해 주세요.
이는 중성과 종성도 동일합니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[간깐난단딴란]", compile("[ㄱ-ㄹ:ㅏ:ㄴ]", &order).unwrap().to_string());
assert_eq!("[간갠갼걘건겐견곈곤관괜괸굔군권궨귄균근긘긴]", compile("[ㄱ:[ㄱ:ㅏ-ㅣ:ㄴ]:ㄴ]", &order).unwrap().to_string());
assert_eq!("[간갅갆갇갈]", compile("[ㄱ:ㅏ:ㄴ-ㄹ]", &order).unwrap().to_string());
```

`0`은 해당 자리에 음소가 없다는 것을 의미합니다. 기본적으로 종성에 사용됩니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[가각간나낙난다닥단]", compile("[ㄱㄴㄷ:ㅏ:0ㄱㄴ]", &order).unwrap().to_string());
```

하지만 특수하게 `[ㄱ:0:0]`이나 `[0:ㅏ:0]`과 같은 형태도 사용될 수 있습니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[ㄱㄲㄴㄷㄸㄹ]", compile("[ㄱ-ㄹ:0:0]", &order).unwrap().to_string());
assert_eq!("[ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ]", compile("[0:ㅏ-ㅣ:0]", &order).unwrap().to_string());
```

`^`을 이용하면 해당 음소에 match하고 싶은 문자 대신 match하기 싫은 문자를 지정할 수 있습니다. 예를 들어 초성이 `ㄱ`이고 중성이 `ㅏ`이면서 받침이 `ㄹ`이 아닌 모든 음소는 `[ㄱ:ㅏ:^ㄹ]`로 표현할 수 있습니다..

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[가각갂갃간갅갆갇갉갊갋갌갍갎갏감갑값갓갔강갖갗갘같갚갛]", compile("[ㄱ:ㅏ:^ㄹ]", &order).unwrap().to_string());
```

만약 종성이 없는 문자를 match하고 싶다면 `[*:*:0]` 대신 `[*:*]` 문법을 사용할 수도 있습니다.
예를 들어 `[ㄱㄴㄷ:ㅏㅣ:0]`는 `[ㄱㄴㄷ:ㅏㅣ]`로 대체될 수 있습니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[가기나니다디]", compile("[ㄱㄴㄷ:ㅏㅣ:0]", &order).unwrap().to_string());
assert_eq!("[가기나니다디]", compile("[ㄱㄴㄷ:ㅏㅣ]", &order).unwrap().to_string());
```

만약 별개로 몇 개의 글자를 match에 추가하고 싶다면 `|`를 그 뒤에 추가하면 됩니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[과구놔누돠두한abc]", compile("[ㄱㄴㄷ:ㅜㅘ|한abc]", &order).unwrap().to_string());
```

한글에는 두 개 이상의 글자가 합쳐서 생성된 문자들이 있습니다. `ㄲ`이나 `ㄼ`, `ㅢ` 등이 그 예입니다.
만약 글자 입력기가 `ㄺ`같은 문자를 작성하는 것을 지원하지 않거나, 미관상의 이유로 코드에서 피하고 싶다면
괄호를 사용해서 문자를 합칠 수 있습니다.

```rust
use korean_regex::*;

let order = Order::Default;
assert_eq!("[곿괇궧궯뽟뽧쀇쀏]", compile("[ㄱ(ㅂㅂ):(ㅗㅏ)(ㅜㅔ):(ㄹㅂ)(ㄱㅅ)]", &order).unwrap().to_string());
```

## Example

다른 문법과 합치면 다음과 같이 사용할 수 있습니다.

```rust
use korean_regex::*;

// 초성이 ㄱ이 아니고 그 뒤에 종성이 `ㅇ`인 모든 글자가 오며 그 다음 글자 바운더리 혹은 종성이 없는 문자가 있는 경우
let pattern = compile(r"[^ㄱ::][::ㅇ](\b|[:])", &order).unwrap();
let result: Vec<_> = pattern
    .captures_iter("한글은 초성, 중성, 종성의 조합이기에 각각을 분리해 분석하거나 사용하는 것이 때때로 유용합니다.")
    .map(|captures| captures[0].to_string())
    .collect();
assert_eq!(vec!["초성", "중성", "종성의", "사용하"], result)
```

## Order

이 라이브러리와 유니코드는 기본적으로 다음과 같은 순서를 사용합니다.

```
초성: ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ
중성: ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ
종성: 0ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ
```

이 순서에는 장점도 있지만 단점도 있습니다 대표적으로는 `-`을 사용할 때 나타납니다.

`[ㄱ-ㄹ:0:0]`의 결과값은 `[ㄱㄲㄴㄷㄸㄹ]`입니다. 하지만 일반적으로 우리는 `ㄱ`부터 `ㄹ`의 결과값으로는 `[ㄱㄴㄷㄹ]`를 기대하기 마련입니다.

이러한 경우 Order를 고쳐서 순서를 변경해 문제를 해결할 수 있습니다.

`Order::RegularFirst`를 순서로 사용하면 각각의 순서는 다음과 같이 변경됩니다.

```
초성: ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄸㅃㅆㅉ
중성: ㅏㅑㅓㅕㅗㅛㅜㅠㅡㅣㅐㅒㅔㅖㅘㅙㅚㅝㅞㅟㅢ
종성: 0ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎㄲㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄㅆ
```

이는 `[ㄱ-ㄹ:0:0]`의 결과값이 `[ㄱㄴㄷㄹ]`가 되도록 만듭니다.

또한 결과값의 순서도 변경됩니다. 예를 들어 `[ㄲㄴ:0:0]`의 결과는 `Order::Default`에서는 `[ㄲㄴ]`가 되지만 `Order::RegularFirst`에서는 `[ㄴㄲ]`가 됩니다.

하이픈 사용 시 두 순서 중에서 어느 것이 자신의 필요에 맞는지 확인하고 사용하시면 됩니다.

## compilestr

`compile`은 단순히 `compilestr`의 결과를 `Regex::new`로 감싸는 함수일 뿐입니다.
또한 Rust에서는 정규표현식 크레이트가 하나가 아닙니다. 만약 다른 정규표현식 크레이트를 이용하고 싶은 경우에 사용할 수 있습니다.

```rust
use fancy_regex::Regex;
use koran_regex::*;

let pattern = compilestr(r"(?<![ㅎ:ㅏ:])[^ㄱ::][::ㅇ]", &Order::Default);
let re = Regex::new(pattern).unwrap();
```
