use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

fn main() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', x.value);
    assert_eq!('a', *x); // derefによって直接値にアクセスできるようになった（SwiftでもこんなProtocolがあったような

    fn foo(s: &str) {
        // 一瞬だけ文字列を借用します
    }
    // String は Deref<Target=str> を実装しています
    let owned = "Hello".to_string();
    // なので、以下のコードはきちんと動作します:
    foo(&owned);
}