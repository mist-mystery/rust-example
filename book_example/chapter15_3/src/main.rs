//! Drop

mod custom_smart_pointer {
    struct CustomSmartPointer<'a> {
        data: &'a str,
    }

    impl Drop for CustomSmartPointer<'_> {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // Drop を実装した CustomSmartPointer は、スコープを抜けたときに drop メソッドを呼び出す。
    // 変数は、生成されたのと逆の順序でドロップされるため、"my stuff" が最後に出力されることになる。
    fn cleanup_variable() {
        let _c = CustomSmartPointer { data: "my stuff" };
        let _d = CustomSmartPointer {
            data: "other stuff",
        };
        println!("CustomSmartPointers created.");
    }

    // 変数に束縛しなければインスタンスを生成した直後に破棄されるため、
    // "my stuff" -> "other stuff" -> "CustomSmartPointers created." の順で出力される。
    fn cleanup_temp() {
        CustomSmartPointer { data: "my stuff" };
        CustomSmartPointer {
            data: "other stuff",
        };
        println!("CustomSmartPointers created.");
    }

    fn mem_drop() {
        let c = CustomSmartPointer { data: "some data" };
        println!("CustomSmartPointer created.");

        // drop メソッドを呼び出すことは許されていない（cの破棄時に drop メソッドが呼ばれ、二重開放が起きてしまうため）
        // c.drop();
        drop(c); // std::med::drop 関数を呼び出すことで早期に強制的に drop させられる。
        // println!("{}", c.data); // drop(c) の後は c はムーブされるため使えなくなる。

        // c が先に drop されるため、これが関数内の最後の標準出力になる。
        println!("CustomSmartPointer dropped before the end of main.");
    }

    pub fn run() {
        cleanup_variable();
        println!();
        cleanup_temp();
        println!();
        mem_drop();
    }
}

mod destructor {
    use std::{thread::sleep, time::Duration};

    /// drop 時に callback を呼び出す構造体
    struct MyDestruct<T, F>
    where
        F: Fn(&T),
    {
        data: T,
        callback: F,
    }

    impl<T, F> Drop for MyDestruct<T, F>
    where
        F: Fn(&T),
    {
        /// self.data を引数にして self.callback を呼び出す。
        fn drop(&mut self) {
            (self.callback)(&self.data)
        }
    }

    // drop 時に callback を呼び出すようにすることで、Drop トレイト自体の実装は変えずに drop 時の挙動を変更する。
    pub fn run() {
        let _d1 = MyDestruct {
            data: 1,
            callback: |d| println!("destructed: {}", d * 2),
        };
        let _d2 = MyDestruct {
            data: 1u64,
            callback: |d| sleep(Duration::from_secs(*d)),
        };
        println!("run method end.");
    }
}

fn main() {
    custom_smart_pointer::run();
    destructor::run();
}
