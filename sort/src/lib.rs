use rand::seq::SliceRandom;

pub struct VerifySort<'a> {
    pub sorted: &'a mut [i32],
    pub compare_count: usize,
    pub swap_count: usize,
}

pub fn make_random_vector(count: usize) -> Vec<i32> {
    let mut v: Vec<i32> = (0..count as i32 * 10).collect();
    v.shuffle(&mut rand::rng());
    v.truncate(count);
    v
}

/// 交換ソート
pub mod exchange_sort {
    use super::VerifySort;

    /// バブルソート (平均: O(n^2), 最悪: O(n^2))
    /// 最後の要素から順番に確定させていく。
    ///
    /// 1. 0番目と1番目を比較して、順序が逆なら交換する。
    /// 2. 1番目と2番目を比較して、順序が逆なら交換する。
    /// 3. n-2番目とn-1番目の比較まで繰り返すと、n-1番目（最後）の要素が最大値となる。
    /// 4. 0..=n-2 番目の要素について、同様に隣接する要素を比較して順序が逆なら交換する。
    /// 5. これを繰り返して、全ての要素がソートされるまで続ける。
    pub fn bubble(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;

        for i in 0..n {
            for j in 0..n - i - 1 {
                // 隣接する要素を比較して、順序が逆なら交換
                compare_count += 1;
                if src[j] > src[j + 1] {
                    src.swap(j, j + 1);
                    swap_count += 1;
                }
            }
        }
        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }

    /// シェーカーソート (平均: O(n^2), 最悪: O(n^2))
    pub fn shaker(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            let mut swapped = false;
            for i in left..right {
                compare_count += 1;
                if src[i] > src[i + 1] {
                    src.swap(i, i + 1);
                    swap_count += 1;
                    swapped = true;
                }
            }
            right -= 1;

            for i in (left..right).rev() {
                compare_count += 1;
                if src[i] > src[i + 1] {
                    src.swap(i, i + 1);
                    swap_count += 1;
                    swapped = true;
                }
            }
            left += 1;

            compare_count += 1;
            if !swapped {
                break;
            }
        }

        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }

    /// クイックソート (平均: O(n log n), 最悪: O(n^2))
    /// pivot（基準値）を選び、pivotより小さい要素と大きい要素に分けて再帰的にソートすることで、pivot の位置を順番に確定させていく。
    pub fn quick(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;

        fn quick_sort_helper(
            arr: &mut [i32],
            low: usize,
            high: usize,
            compare_count: &mut usize,
            swap_count: &mut usize,
        ) {
            if low < high {
                let pivot_index = partition(arr, low, high, compare_count, swap_count);
                if pivot_index > 0 {
                    quick_sort_helper(arr, low, pivot_index - 1, compare_count, swap_count);
                }
                quick_sort_helper(arr, pivot_index + 1, high, compare_count, swap_count);
            }
        }

        fn partition(
            arr: &mut [i32],
            low: usize,
            high: usize,
            compare_count: &mut usize,
            swap_count: &mut usize,
        ) -> usize {
            let pivot = arr[high];
            let mut i = low as isize - 1;

            for j in low..high {
                *compare_count += 1;
                if arr[j] <= pivot {
                    i += 1;
                    arr.swap(i as usize, j);
                    *swap_count += 1;
                }
            }
            arr.swap((i + 1) as usize, high);
            *swap_count += 1;
            (i + 1) as usize
        }

        quick_sort_helper(src, 0, n - 1, &mut compare_count, &mut swap_count);

        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }
}

/// 選択ソート
pub mod selection_sort {
    use super::VerifySort;

    /// 選択ソート (平均: O(n^2), 最悪: O(n^2))
    /// 0番目から順番に確定させていく。
    ///
    /// 1. 0..=n-1 番目の要素について最小値を見つけ、0番目の要素と交換する。
    /// 2. 1..=n-1 番目の要素について最小値を見つけ、1番目の要素と交換する。
    /// 3. これを繰り返す。
    pub fn selection(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;

        for i in 0..n {
            let mut min_index = i;
            for j in i + 1..n {
                compare_count += 1;
                if src[j] < src[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                src.swap(i, min_index);
                swap_count += 1;
            }
        }

        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }

    pub fn heap(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;

        // Heapify subtree rooted at index i
        fn heapify(
            arr: &mut [i32],
            n: usize,
            i: usize,
            compare_count: &mut usize,
            swap_count: &mut usize,
        ) {
            let mut largest = i;
            let l = 2 * i + 1;
            let r = 2 * i + 2;

            // println!(
            //     "{arr:?} (largest, l, r): ({largest}, {l}, {r}) ({}, {}, {})",
            //     arr[largest],
            //     if l < n { arr[l] } else { -1 },
            //     if r < n { arr[r] } else { -1 }
            // );

            if l < n {
                *compare_count += 1;
                if arr[l] > arr[largest] {
                    largest = l;
                }
            }
            if r < n {
                *compare_count += 1;
                if arr[r] > arr[largest] {
                    largest = r;
                }
            }
            if largest != i {
                arr.swap(i, largest);
                *swap_count += 1;
                heapify(arr, n, largest, compare_count, swap_count);
            }
        }

        // Build max heap
        for i in (0..n / 2).rev() {
            heapify(src, n, i, &mut compare_count, &mut swap_count);
        }

        // println!("{src:?}: Heap built");

        // Extract elements from heap one by one
        for i in (1..n).rev() {
            src.swap(0, i);
            swap_count += 1;
            heapify(src, i, 0, &mut compare_count, &mut swap_count);
        }

        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }
}

/// 挿入ソート
pub mod insertion_sort {
    use super::VerifySort;

    /// 挿入ソート (平均: O(n^2), 最悪: O(n^2))
    /// 0番目から基準までの要素がソート済みになる。基準を1番目から順番にしていく。
    ///
    /// 1. 1番目の要素を基準にして、0番目の要素と比較し、順序が逆なら交換する。
    /// 2. 2番目の要素を基準にする。
    ///     1. 基準と1番目の要素と比較し、順序が逆なら1番目の要素を2番目に代入する。順序が正しいなら基準の要素を2番目に代入（実質そのまま）し、2番目基準終了。
    ///     2. 基準と0番目の要素と比較し、順序が逆なら0番目の要素を1番目に代入する。順序が正しいなら基準の要素を1番目に代入し、2番目基準終了。
    ///     3. 基準の要素を0番目に代入し、2番目基準終了。
    /// 3. 3番目の要素を基準にする。
    /// 4. これを繰り返して、全ての要素がソートされるまで続ける。
    pub fn insertion(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;

        for i in 1..n {
            let key = src[i];
            let mut j = i as isize - 1;

            while j >= 0 && src[j as usize] > key {
                compare_count += 1;
                src[(j + 1) as usize] = src[j as usize];
                j -= 1;
                swap_count += 1;
            }
            src[(j + 1) as usize] = key;
            if j >= 0 {
                compare_count += 1;
            }
        }

        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }
}

pub mod merge_sort {
    use super::VerifySort;

    /// マージソート (平均: O(n log n), 最悪: O(n log n))
    /// 分割統治法を用いて、配列を再帰的に分割し、ソートされた部分配列をマージする。
    pub fn merge(src: &mut [i32]) -> VerifySort {
        let n = src.len();
        let mut compare_count = 0;
        let mut swap_count = 0;

        if n <= 1 {
            return VerifySort {
                sorted: src,
                compare_count,
                swap_count,
            };
        }

        let mid = n / 2;
        let mut left = src[0..mid].to_vec();
        let mut right = src[mid..n].to_vec();

        let left_sort = merge(&mut left);
        let right_sort = merge(&mut right);

        compare_count += left_sort.compare_count + right_sort.compare_count;
        swap_count += left_sort.swap_count + right_sort.swap_count;

        let mut i = 0;
        let mut j = 0;

        for elem in src[..(left.len() + right.len())].iter_mut() {
            if i < left.len() && (j >= right.len() || left[i] <= right[j]) {
                *elem = left[i];
                compare_count += if j >= right.len() { 2 } else { 3 };
                i += 1;
            } else {
                *elem = right[j];
                compare_count += if i < left.len() {
                    if j >= right.len() { 3 } else { 2 }
                } else {
                    1
                };
                j += 1;
            }
            swap_count += 1;
        }

        VerifySort {
            sorted: src,
            compare_count,
            swap_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_random_vector() {
        let v = make_random_vector(10);
        assert_eq!(v.len(), 10);
        assert!(v.windows(2).any(|w| w[0] != w[1]));
        println!("Generated vector: {:?}", v);
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = make_random_vector(1000);
        let sort = exchange_sort::bubble(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(sort.compare_count, 499500);
    }

    #[test]
    fn test_shaker_sort() {
        let mut v = make_random_vector(1000);
        let sort = exchange_sort::shaker(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_quick_sort() {
        let mut v = make_random_vector(1000);
        let sort = exchange_sort::quick(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_selection_sort() {
        let mut v = make_random_vector(1000);
        let sort = selection_sort::selection(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(sort.compare_count, 499500);
    }

    #[test]
    fn test_heap_sort() {
        let mut v = make_random_vector(1000);
        let sort = selection_sort::heap(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = make_random_vector(1000);
        let sort = insertion_sort::insertion(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_merge_sort() {
        let mut v = make_random_vector(1000);
        let sort = merge_sort::merge(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
    }
}
