// ジェネリック境界は､ジェネリック型に境界(どのようなトレイトを実装しているか)を明示する文法だった.
// ライフタイム境界は､ジェネリック境界のような感じで､ライフタイム(それ自身ジェネリック型)にも境界を与えることができる.

use std::fmt::Debug;

// `T: 'a` T内の全ての参照は'aより長生きでなくてはならない
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// `T: 'a` ､TはDebugを実装し､かつT内の全ての参照は'aより長生きでなくてはならない
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("ref:{:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
}
