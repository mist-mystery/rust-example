pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    // assert! でテスト
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // カスタム失敗メッセージを設定。
        assert!(
            larger.can_hold(&smaller),
            "receiver must larger than argument: receiver: {larger:?}, argument: {:?}",
            &smaller
        );
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    // should_panic 属性でテスト。expected 引数で失敗メッセージに与えられたテキストが含まれていることを確かめる。
    fn it_panic() {
        panic!("Make this test fail");
    }

    mod rectangle {
        use super::super::*;

        #[derive(PartialEq, Debug)]
        // assert_eq! や assert_ne! のテストは PartialEq と Debug トレイトを実装していなければならない。
        struct Rectangle {
            width: u32,
            height: u32,
        }

        #[test]
        // assert_eq! でテスト
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }

        #[test]
        // Result 型を使うようなテストも記述できる。
        // テスト成功時に Ok(()), 失敗時に Err に String を入れて返すようにする。
        // これにより ? 演算子をテスト中で使えるようになる。
        // ただし、#[should_panic] attribute を使えなくなるため、テスト失敗時には Err を返すようにする。
        fn it_works_result() -> Result<(), String> {
            let result = add(2, 2);
            if result == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }

        #[test]
        #[ignore]
        // assert_ne! でテスト
        // #[ignore] attribute をつけると、通常の `cargo test` では無視される。
        // 明示的に指定したときのみ実行される。
        fn rectangle_ne() {
            let rec1 = Rectangle {
                width: 5,
                height: 1,
            };
            let rec2 = Rectangle {
                width: 5,
                height: 2,
            };
            assert_ne!(rec1, rec2);
        }
    }
}
