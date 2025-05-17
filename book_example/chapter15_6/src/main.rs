mod mut_ref {
    #[derive(Debug, PartialEq)]
    struct Node<'a, T>(T, Vec<&'a Node<'a, T>>);

    // 普通に循環参照を作ろうとしても、借用規則により不可能
    pub fn run() {
        #[allow(unused_mut)]
        let mut a = Node(5, vec![]);
        let b = Node(10, vec![&a]);

        // b は a の不変借用を使っているため、b のライフタイム中に a の可変メソッドは使用不可
        // a.1.push(&b);

        assert_eq!(b, Node(10, vec![&Node(5, vec![])]));
    }
}

// Box で循環参照を作ろうと試みる例
mod mut_box {
    #[derive(Debug, PartialEq)]
    struct Stack<T>(Option<Box<(T, Stack<T>)>>);

    impl<T> Stack<T> {
        fn tail(&self) -> Option<&Stack<T>> {
            self.0.as_ref().map(|rc| &rc.1)
        }

        fn tail_mut(&mut self) -> Option<&mut Stack<T>> {
            self.0.as_mut().map(|rc| &mut rc.1)
        }
    }

    // 普通に循環参照を作ろうとしても、借用規則により不可能
    pub fn run() {
        let a = Stack(Some(Box::new((5, Stack(None)))));
        assert_eq!(a.tail(), Some(&Stack(None)));

        let mut b = Stack(Some(Box::new((10, a))));
        assert_eq!(b.tail(), Some(&Stack(Some(Box::new((5, Stack(None)))))));

        // Some(Box((5, Stack(None)))) の Stack への可変参照を取得したい
        let link = b.tail_mut().unwrap().tail_mut().unwrap();
        assert_eq!(link, &Stack(None));
        // *link = b; // link が生きている間は b は 借用中であるため move 不可
    }
}

// RefCell で循環参照が発生してスタックオーバーフローになる例
mod refcell {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq)]
    /// RefCell で List ごと包むことで、リストの要素 T でなく RefCelledList を可変にする。
    enum RefCelledList<T> {
        Cons(T, RefCell<Rc<RefCelledList<T>>>),
        Nil,
    }

    impl<T> RefCelledList<T> {
        fn tail(&self) -> Option<&RefCell<Rc<RefCelledList<T>>>> {
            match self {
                RefCelledList::Cons(_, item) => Some(item),
                RefCelledList::Nil => None,
            }
        }
    }

    // a が b の Cons を参照、b が a の Cons を参照するようにすると、循環参照となる。表示しようとするとスタックオーバーフローする。
    pub fn run() {
        let a_tail = RefCell::new(Rc::new(RefCelledList::Nil::<i32>));
        let a = Rc::new(RefCelledList::Cons(5, a_tail.clone()));
        assert_eq!(Rc::strong_count(&a), 1);
        assert_eq!(a.tail(), Some(&RefCell::new(Rc::new(RefCelledList::Nil))));

        let b = Rc::new(RefCelledList::Cons(10, RefCell::new(Rc::clone(&a))));
        assert_eq!(Rc::strong_count(&a), 2); // b 作成時に a を clone したため、強参照カウントが増える
        assert_eq!(Rc::strong_count(&b), 1);
        assert_eq!(b.tail(), Some(&RefCell::new(Rc::clone(&a))));

        let link = a.tail().unwrap();
        assert_eq!(link, &a_tail);
        *link.borrow_mut() = Rc::clone(&b);

        // 上の代わりにこっちをやってもスタックオーバーフローにならない。
        // ∵ a_tail は a.tail() で取得できる RefCell が持つ Rc (これを a.tail() の Rc という)を clone したものであるため、
        //   a.tail() の Rc と同じもの(Nil)を指してはいるが、Rc 自体は別物である。
        //   そのため、a_tail の Rc を入れ替えたところで a.tail() の Rc には影響がない。
        // *a_tail.borrow_mut() = Rc::clone(&b);
        // assert_eq!(b, Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a)))));
        // assert_eq!(a, Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil)))));
        // assert_eq!(
        //     a_tail,
        //     RefCell::new(Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a)))))
        // );

        assert_eq!(Rc::strong_count(&b), 2);
        assert_eq!(Rc::strong_count(&a), 2);

        // Uncomment the next line to see that we have a cycle; it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }
}

// HashMap に Node を保持し、children フィールドで他の Node を参照しようという試み。
// 借用規則が強くて循環参照になるような Node を作ることはできない（多分）。
mod hashmap_ref {
    use std::collections::HashMap;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Node<'a, T> {
        value: T,
        parent: Option<&'a Node<'a, T>>,
        children: Vec<&'a Node<'a, T>>,
    }

    // leaf を作成し、branch は children フィールドに leaf の不変参照を使う。そのあと HashMap に insert しようとする方法。
    fn leaf_move() {
        let mut map = HashMap::new();
        let leaf = Node {
            value: 3,
            parent: None,
            children: vec![],
        };
        // children フィールドの Node のライフタイムが 'a であるため、branch 生存中は leaf も不変借用されたまま。
        let branch = Node {
            value: 5,
            parent: None,
            children: vec![&leaf],
        };

        // branch がムーブされる。
        map.insert("branch", branch);

        // branch ムーブ後に leaf のライフタイムは切れていないため使うのは問題ない（leaf の不変参照は切れてはいる）。
        assert_eq!(leaf.value, 3);

        // branch を map に insert したことで、HashMap<&'static str, Node<'_, i32>> の Node のライフタイムは branch 生存中のもの、
        // すなわち leaf の不変参照が有効な間になる。
        // ここで leaf をムーブすると leaf の寿命が切れてしまうためエラーになる。
        // map.insert("leaf", leaf);
    }

    // leaf だけ HashMap に insert して所有権を移した後、HashMap から leaf の参照を得て、それを branch の children に使おうとする方法。
    fn map_borrow() {
        let mut map = HashMap::new();
        let leaf = Node {
            value: 3,
            parent: None,
            children: vec![],
        };
        map.insert("leaf", leaf); // leaf の所有権を HashMap に移す

        // HashMap から leaf ノードの不変参照を取得。
        // ここでの map の get メソッド呼び出し時に行う不変借用は leaf が生きている間続く。
        let leaf = map.get("leaf").unwrap();

        // leaf が生きている間、map の不変参照が生きているので insert や get_mut などの可変メソッドは使用できない。
        // map.insert(
        //     "dummy",
        //     Node {
        //         value: 5,
        //         parent: None,
        //         children: vec![],
        //     },
        // );
        let _branch = Node {
            value: 5,
            parent: None,
            children: vec![&leaf],
        };
    }

    pub fn run() {
        leaf_move();
        map_borrow();
    }
}

// Node 本体は usize をキーとした HashMap で持ち、Node の parent や children は usize で持つ。
mod hashmap_key {
    use std::collections::HashMap;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Node<T> {
        value: T,
        parent: Option<usize>,
        children: Vec<usize>,
    }

    #[derive(Debug)]
    struct Graph<T> {
        nodes: HashMap<usize, Node<T>>,
        next_id: usize, // ID 発行カウンタ
    }

    impl<T> Graph<T> {
        fn add(&mut self, node: Node<T>) {
            self.nodes.insert(self.next_id, node);
            self.next_id += 1;
        }
    }

    pub fn run() {
        let mut graph = Graph {
            nodes: HashMap::new(),
            next_id: 0,
        };
        graph.add(Node {
            value: 5,
            parent: None,
            children: vec![1],
        });
        graph.add(Node {
            value: 3,
            parent: Some(0),
            children: vec![],
        });

        println!("{graph:?}");
    }
}

// 親ノードが子ノードへの強参照を持つ。子ノードから親ノードへは弱参照を持つようにすることで循環参照の問題を回避。
mod weak {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    // Weak は PartialEq を実装していないので Node の比較はできない。
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        #[allow(dead_code)]
        children: RefCell<Vec<Rc<Node>>>,
    }

    pub fn run() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(Rc::weak_count(&branch), 1); // leaf.parent が弱参照で branch を持つためカウントは1
            assert_eq!(Rc::strong_count(&leaf), 2); // branch.children が内部で leaf への強参照を持つためカウントは2
            assert_eq!(Rc::weak_count(&leaf), 0);

            // スコープを抜けるタイミングで branch の強参照カウントが0になって drop される。
            // 弱参照カウントが残っているかどうかは関係ない。
        }

        // Some(Node {
        //   value: 5,
        //   parent: RefCell { value: (Weak) },
        //   children: RefCell { value: [Node {
        //     value: 3,
        //     parent: RefCell { value: (Weak) },
        //     children: RefCell { value: [] }
        //   }]}
        // })
        println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
        assert_eq!(leaf.value, 3);
        assert_eq!(Rc::strong_count(&leaf), 1); // branch はスコープを抜けているため強参照カウントは1
        assert_eq!(Rc::weak_count(&leaf), 0);
    }
}

fn main() {
    mut_ref::run();
    mut_box::run();
    refcell::run();
    hashmap_ref::run();
    hashmap_key::run();
    weak::run();
}
