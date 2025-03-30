fn main() {
    slice();
    delete();
    delete_predicate();
    add();
}

fn slice() {
    {
        // as_slice
        let buffer = vec![1, 2, 3, 4, 5];
        // slice1～3 は全て同じ
        let slice1 = buffer.as_slice();
        let slice2 = &buffer[..];
        let slice3: &[_] = &buffer; // 型注釈が必要
        assert_eq!(slice1, &[1, 2, 3, 4, 5]);
        assert_eq!(slice1, slice2);
        assert_eq!(slice2, slice3);
    }
    {
        // as_mut_slice : &mut [..] を使うのと同じ
        let mut buffer1 = vec![1, 2, 3, 4, 5];
        let mut buffer2 = vec![1, 2, 3, 4, 5];
        let mut buffer3 = vec![1, 2, 3, 4, 5];
        let slice1 = buffer1.as_mut_slice();
        let slice2 = &mut buffer2[..];
        let slice3: &mut [_] = &mut buffer3; // 型注釈が必要
        assert_eq!(slice1, &[1, 2, 3, 4, 5]);
        assert_eq!(slice1, slice2);
        assert_eq!(slice2, slice3);
    }
}

fn delete() {
    {
        // truncate : 要素数 len になるように残りを落とす。
        // clear : 要素数 0 にする
        let mut v = vec!["foo", "bar", "foo", "baz"];
        v.truncate(2);
        assert_eq!(v, ["foo", "bar"]);

        v.clear();
        assert!(v.is_empty());
    }
    {
        // pop : O(1)
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec, [1, 2]);
    }
    {
        // swap_remove : index 番目の要素を削除。順番が保たれないが O(1)
        let mut v = vec!["foo", "bar", "foo", "baz"];
        assert_eq!(v.swap_remove(0), "foo");
        assert_eq!(v, ["baz", "bar", "foo"]);
    }
    {
        // remove : index 番目の要素を削除（最悪 O(len)）
        let mut v = vec!["foo", "bar", "foo", "baz"];
        assert_eq!(v.remove(0), "foo");
        assert_eq!(v, ["bar", "foo", "baz"]);
    }
    {
        // drain : 指定した範囲を削除し、削除した要素を iterator として返す。
        let mut v = vec![1, 2, 3, 4, 5];
        let u: Vec<_> = v.drain(1..3).collect();
        assert_eq!(v, &[1, 4, 5]);
        assert_eq!(u, &[2, 3]);
    }
}

fn delete_predicate() {
    {
        // retain : predicate で true となった要素のみ残す
        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, [2, 4]);
    }
}

fn add() {
    {
        // push : capacity が十分なら O(1), メモリ再割り当てが必要ならそれに O(capacity).
        let mut vec = vec![1, 2];
        vec.push(3);
        assert_eq!(vec, [1, 2, 3]);
    }
    {
        // insert : index の位置に挿入。O(len)
        let mut vec = vec!['a', 'b', 'c'];
        vec.insert(1, 'd');
        assert_eq!(vec, ['a', 'd', 'b', 'c']);
        vec.insert(4, 'e');
        assert_eq!(vec, ['a', 'd', 'b', 'c', 'e']);
    }
    {
        // append
        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];
        vec1.append(&mut vec2);
        assert_eq!(vec1, [1, 2, 3, 4, 5, 6]);
        assert_eq!(vec2, []);
    }
}
