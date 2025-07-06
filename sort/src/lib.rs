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
    /// バブルソート (平均: O(n^2), 最悪: O(n^2))
    /// 最後の要素から順番に確定させていく。
    ///
    /// 1. 0番目と1番目を比較して、順序が逆なら交換する。
    /// 2. 1番目と2番目を比較して、順序が逆なら交換する。
    /// 3. n-2番目とn-1番目の比較まで繰り返すと、n-1番目（最後）の要素が最大値となり確定。
    /// 4. 0..=n-2 番目の要素について、同様に隣接する要素を比較して順序が逆なら交換する。
    /// 5. これを繰り返して、全ての要素がソートされるまで続ける。
    pub fn bubble(arr: &mut [impl PartialOrd]) {
        let n = arr.len();

        for i in 0..n {
            for j in 0..n - i - 1 {
                // 隣接する要素を比較して、順序が逆なら交換
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    /// シェーカーソート (平均: O(n^2), 最悪: O(n^2))
    pub fn shaker(arr: &mut [impl PartialOrd]) {
        let n = arr.len();
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            let mut swapped = false;
            for i in left..right {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }
            right -= 1;

            for i in (left..right).rev() {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }
            left += 1;

            if !swapped {
                break;
            }
        }
    }

    /// クイックソート (平均: O(n log n), 最悪: O(n^2))
    ///
    /// 1. pivot（基準値）を適当（ここでは最後の要素）に選び、pivotより小さい要素群, pivot, pivotより大きい要素群の順になるように再配置する。
    ///     - 先頭要素から順番に見ていき、pivot より大きい要素があれば、それより後に位置する pivot より小さい要素と交換する。
    ///     - ここで pivot の最終位置が決まる。
    /// 2. pivot より小さい要素群、pivot より大きい要素群に対して、再帰的にクイックソートを適用する。
    pub fn quick(arr: &mut [impl PartialOrd]) {
        if arr.len() <= 1 {
            return;
        }

        let pivot_index = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_index);

        quick(left);
        quick(&mut right[1..]); // right[0] is the pivot, so we skip it
    }

    fn partition(arr: &mut [impl PartialOrd]) -> usize {
        let len = arr.len();
        let mut i = 0;

        for j in 0..len - 1 {
            // arr[len - 1] を pivot として、pivot より小さい要素を左側に集める
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }
}

/// 選択ソート
pub mod selection_sort {
    /// 選択ソート (平均: O(n^2), 最悪: O(n^2))
    /// 0番目から順番に確定させていく。
    ///
    /// 1. 0..=n-1 番目の要素について最小値を見つけ、0番目の要素と交換する。
    /// 2. 1..=n-1 番目の要素について最小値を見つけ、1番目の要素と交換する。
    /// 3. これを繰り返す。
    pub fn selection(arr: &mut [impl Ord]) {
        for i in 0..arr.len() {
            if let Some((min_index, _)) =
                arr[i..].iter().enumerate().min_by_key(|&(_, value)| value)
            {
                arr.swap(i, i + min_index);
            }
        }
    }

    /// ヒープソート (平均: O(n log n), 最悪: O(n log n))
    pub fn heap(arr: &mut [impl PartialOrd]) {
        let len = arr.len();

        // Build a max heap
        for i in (0..len / 2).rev() {
            sift_down(arr, i, len);
        }

        // Extract elements from heap one by one
        for end in (1..len).rev() {
            arr.swap(0, end);
            sift_down(arr, 0, end);
        }
    }

    /// ヒープの再構築
    fn sift_down(arr: &mut [impl PartialOrd], mut root: usize, end: usize) {
        loop {
            let left = 2 * root + 1;
            let right = 2 * root + 2;
            let mut largest = root;

            if left < end && arr[left] > arr[largest] {
                largest = left;
            }
            if right < end && arr[right] > arr[largest] {
                largest = right;
            }
            if largest == root {
                break;
            }

            arr.swap(root, largest);
            root = largest;
        }
    }
}

/// 挿入ソート
pub mod insertion_sort {
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
    pub fn insertion(src: &mut [impl PartialOrd + Copy]) {
        let n = src.len();

        for i in 1..n {
            let key = src[i];
            let mut j = i as isize - 1;

            while j >= 0 && src[j as usize] > key {
                src[(j + 1) as usize] = src[j as usize];
                j -= 1;
            }
            src[(j + 1) as usize] = key;
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
        exchange_sort::bubble(&mut v);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_shaker_sort() {
        let mut v = make_random_vector(1000);
        exchange_sort::shaker(&mut v);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_quick_sort() {
        let mut v = make_random_vector(1000);
        exchange_sort::quick(&mut v);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_selection_sort() {
        let mut v = make_random_vector(1000);
        selection_sort::selection(&mut v);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_heap_sort() {
        let mut v = make_random_vector(1000);
        selection_sort::heap(&mut v);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = make_random_vector(1000);
        insertion_sort::insertion(&mut v);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_merge_sort() {
        let mut v = make_random_vector(1000);
        let sort = merge_sort::merge(&mut v);
        assert_eq!(sort.sorted.len(), 1000);
        assert!(sort.sorted.windows(2).all(|w| w[0] <= w[1]));
    }
}
