#[allow(dead_code, unused_variables)]
/// 1.86現在の allocator, unsafe 及びトレイト実装を除いた大雑把な Vec 実装
mod myvec {
    use std::collections::TryReserveError;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;
    use std::ops::RangeBounds;
    use std::ptr::NonNull;
    use std::vec::{Drain, Splice};

    struct Vec<T> {
        buf: RawVec<T>,
        len: usize,
    }

    struct RawVec<T> {
        inner: RawVecInner,
        _marker: PhantomData<T>,
    }

    struct RawVecInner {
        ptr: Unique<u8>,
    }

    struct Unique<T: ?Sized> {
        pointer: NonNull<T>,
        _marker: PhantomData<T>,
    }

    impl<T> Vec<T> {
        /// Constructs a new, empty `Vec<T>`.
        pub const fn new() -> Self {
            unimplemented!()
        }

        /// Constructs a new, empty `Vec<T>` with at least the specified capacity.
        pub fn with_capacity(capacity: usize) -> Self {
            unimplemented!()
        }

        /// Returns the total number of elements the vector can hold without reallocating.
        pub fn capacity(&self) -> usize {
            unimplemented!()
        }

        /// Reserves capacity for at least `additional` more elements to be inserted in the given `Vec<T>`.
        /// The collection may reserve more space to speculatively avoid frequent reallocations.
        /// After calling `reserve`, capacity will be greater than or equal to `self.len() + additional`.
        /// Does nothing if capacity is already sufficient.
        pub fn reserve(&mut self, additional: usize) {}

        /// Reserves the minimum capacity for at least `additional` more elements to be inserted in the given `Vec<T>`.
        /// Unlike [`reserve`], this will not deliberately over-allocate to speculatively avoid frequent allocations.
        /// After calling `reserve_exact`, capacity will be greater than or equal to `self.len() + additional`.
        /// Does nothing if the capacity is already sufficient.
        pub fn reserve_exact(&mut self, additional: usize) {}

        /// Tries to reserve capacity for at least `additional` more elements to be inserted in the given `Vec<T>`.
        /// The collection may reserve more space to speculatively avoid frequent reallocations.
        /// After calling `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`.
        /// Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.
        pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            unimplemented!()
        }

        /// Tries to reserve the minimum capacity for at least `additional` elements to be inserted in the given `Vec<T>`.
        /// Unlike [`try_reserve`], this will not deliberately over-allocate to speculatively avoid frequent allocations.
        /// After calling `try_reserve_exact`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`.
        /// Does nothing if the capacity is already sufficient.
        pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
            unimplemented!()
        }

        /// Shrinks the capacity of the vector as much as possible.
        pub fn shrink_to_fit(&mut self) {}

        /// Shrinks the capacity of the vector with a lower bound.
        pub fn shrink_to(&mut self, min_capacity: usize) {}

        /// Converts the vector into [`Box<[T]>`][owned slice].
        pub fn into_boxed_slice(self) -> Box<[T]> {
            unimplemented!()
        }

        /// Shortens the vector, keeping the first `len` elements and dropping the rest.
        pub fn truncate(&mut self, len: usize) {}

        /// Extracts a slice containing the entire vector.
        pub fn as_slice(&self) -> &[T] {
            unimplemented!()
        }

        /// Extracts a mutable slice of the entire vector.
        pub fn as_mut_slice(&mut self) -> &mut [T] {
            unimplemented!()
        }

        /// Returns a raw pointer to the vector's buffer,
        /// or a dangling raw pointer valid for zero sized reads if the vector didn't allocate.
        pub fn as_ptr(&self) -> *const T {
            unimplemented!()
        }

        /// Returns a raw mutable pointer to the vector's buffer,
        /// or a dangling raw pointer valid for zero sized reads if the vector didn't allocate.
        pub fn as_mut_ptr(&mut self) -> *mut T {
            unimplemented!()
        }

        /// Removes an element from the vector and returns it.
        pub fn swap_remove(&mut self, index: usize) -> T {
            unimplemented!()
        }

        /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
        pub fn insert(&mut self, index: usize, element: T) {}

        /// Removes and returns the element at position `index` within the vector,
        /// shifting all elements after it to the left.
        pub fn remove(&mut self, index: usize) -> T {
            unimplemented!()
        }

        /// Retains only the elements specified by the predicate.
        pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {}

        /// Retains only the elements specified by the predicate, passing a mutable reference to it.
        pub fn retain_mut(&mut self, f: impl FnMut(&mut T) -> bool) {}

        /// Removes all but the first of consecutive elements in the vector that resolve to the same key.
        pub fn dedup_by_key<K: PartialEq>(&mut self, key: impl FnMut(&mut T) -> K) {}

        /// Removes all but the first of consecutive elements in the vector satisfying a given equality relation.
        pub fn dedup_by(&mut self, same_bucket: impl FnMut(&mut T, &mut T) -> bool) {}

        /// Appends an element to the back of a collection.
        pub fn push(&mut self, value: T) {}

        /// Removes the last element from a vector and returns it, or [`None`] if it is empty.
        pub fn pop(&mut self) -> Option<T> {
            unimplemented!()
        }

        /// Removes and returns the last element from a vector if the predicate returns `true`,
        /// or [`None`] if the predicate returns false or the vector is empty (the predicate will not be called in that case).
        pub fn pop_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
            unimplemented!()
        }

        /// Moves all the elements of `other` into `self`, leaving `other` empty.
        pub fn append(&mut self, other: &mut Self) {}

        /// Removes the subslice indicated by the given range from the vector,
        /// returning a double-ended iterator over the removed subslice.
        pub fn drain(&mut self, range: impl RangeBounds<usize>) -> Drain<'_, T> {
            unimplemented!()
        }

        /// Clears the vector, removing all values.
        pub fn clear(&mut self) {}

        /// Returns the number of elements in the vector, also referred to as its 'length'.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the vector contains no elements.
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Splits the collection into two at the given index.
        pub fn split_off(&mut self, at: usize) -> Self {
            unimplemented!()
        }

        /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
        pub fn resize_with(&mut self, new_len: usize, f: impl FnMut() -> T) {}

        /// Consumes and leaks the `Vec`, returning a mutable reference to the contents, `&'a mut [T]`.
        pub fn leak<'a>(self) -> &'a mut [T] {
            unimplemented!()
        }

        /// Returns the remaining spare capacity of the vector as a slice of `MaybeUninit<T>`.
        pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
            unimplemented!()
        }

        /// Creates a splicing iterator that replaces the specified range in the vector
        /// with the given `replace_with` iterator and yields the removed items.
        /// `replace_with` does not need to be the same length as `range`.
        pub fn splice<I: IntoIterator<Item = T>>(
            &mut self,
            range: impl RangeBounds<usize>,
            replace_with: I,
        ) -> Splice<'_, I::IntoIter> {
            unimplemented!()
        }
    }

    impl<T: Clone> Vec<T> {
        /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
        pub fn resize(&mut self, new_len: usize, value: T) {}

        /// Clones and appends all elements in a slice to the `Vec`.
        pub fn extend_from_slice(&mut self, other: &[T]) {}

        /// Given a range `src`, clones a slice of elements in that range and appends it to the end.
        pub fn extend_from_within(&mut self, src: impl RangeBounds<usize>) {}
    }

    impl<T, const N: usize> Vec<[T; N]> {
        /// Takes a `Vec<[T; N]>` and flattens it into a `Vec<T>`.
        pub fn into_flattened(self) -> Vec<T> {
            unimplemented!()
        }
    }

    impl<T: PartialEq> Vec<T> {
        /// Removes consecutive repeated elements in the vector according to the [`PartialEq`] trait implementation.
        pub fn dedup(&mut self) {}
    }
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
        let mut buffer2 = [1, 2, 3, 4, 5];
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
fn main() {
    slice();
    delete();
    delete_predicate();
    add();
}
