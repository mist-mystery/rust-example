fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic = "crash and burn"]
    // プログラマによる明示的な panic. 回復手段はない。
    fn macro_panic() {
        panic!("crash and burn");
    }

    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 99"]
    // Vec に対する無効な添え字アクセスで buffer overread は起こらず panic する。
    fn vec_panic() {
        let vec = vec![1, 2, 3];
        vec[99];
    }

    #[test]
    #[should_panic = "index out of bounds: the len is 3 but the index is 99"]
    // slice に対する無効な添え字アクセスも全く同様。なお、固定長配列の場合コンパイルエラー。
    fn slice_panic() {
        let a = [1, 2, 3];
        let slice = &a[..];
        slice[99];
    }
}
