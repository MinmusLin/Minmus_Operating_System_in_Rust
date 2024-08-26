// Project Name:  MinmusOS
// File Name:     sort.rs
// File Function: Sort utils
// Author:        Jishen Lin
// License:       MIT License

pub fn bubble_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn selection_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    for i in 0..n {
        let mut min_index: usize = i;
        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

pub fn insertion_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    for i in 1..n {
        let key: i32 = arr[i];
        let mut j: isize = i as isize - 1;
        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

pub fn merge_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    if n > 1 {
        let mid: usize = n / 2;
        let mut temp: [i32; 256] = [0; 256];
        merge_sort(&mut arr[0..mid]);
        merge_sort(&mut arr[mid..n]);
        merge(arr, mid, &mut temp);
    }
}

fn merge(arr: &mut [i32], mid: usize, temp: &mut [i32]) {
    let (mut i, mut j, mut k): (usize, usize, usize) = (0, mid, 0);
    let n: usize = arr.len();
    while i < mid && j < n {
        if arr[i] <= arr[j] {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }
    while i < mid {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }
    while j < n {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }
    for i in 0..n {
        arr[i] = temp[i];
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    if n <= 1 {
        return;
    }
    let pivot_index: usize = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..n]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot: i32 = arr[arr.len() - 1];
    let mut i: usize = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

pub fn heap_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest: usize = i;
    let left: usize = 2 * i + 1;
    let right: usize = 2 * i + 2;
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

pub fn shell_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    let mut gap: usize = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp: i32 = arr[i];
            let mut j: usize = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}

pub fn counting_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    if n == 0 {
        return;
    }
    let mut max: i32 = arr[0];
    let mut min: i32 = arr[0];
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
    }
    let range: usize = (max - min + 1) as usize;
    let mut count: [usize; 256] = [0; 256];
    let mut output: [i32; 256] = [0; 256];
    for &num in arr.iter() {
        count[(num - min) as usize] += 1;
    }
    for i in 1..range {
        count[i] += count[i - 1];
    }
    for &num in arr.iter().rev() {
        let index: usize = (num - min) as usize;
        output[count[index] - 1] = num;
        count[index] -= 1;
    }
    for i in 0..n {
        arr[i] = output[i];
    }
}

pub fn bucket_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    if n == 0 {
        return;
    }
    let mut max: i32 = arr[0];
    let mut min: i32 = arr[0];
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
    }
    let bucket_count: usize = (max - min) as usize + 1;
    let mut buckets: [[i32; 256]; 256] = [[0; 256]; 256];
    let mut bucket_sizes: [usize; 256] = [0; 256];
    for &num in arr.iter() {
        let bucket_index: usize = (num - min) as usize;
        buckets[bucket_index][bucket_sizes[bucket_index]] = num;
        bucket_sizes[bucket_index] += 1;
    }
    let mut index: usize = 0;
    for i in 0..bucket_count {
        let size: usize = bucket_sizes[i];
        if size > 0 {
            for j in 1..size {
                let key: i32 = buckets[i][j];
                let mut k: isize = j as isize - 1;
                while k >= 0 && buckets[i][k as usize] > key {
                    buckets[i][(k + 1) as usize] = buckets[i][k as usize];
                    k -= 1;
                }
                buckets[i][(k + 1) as usize] = key;
            }
            for &num in buckets[i][..size].iter() {
                arr[index] = num;
                index += 1;
            }
        }
    }
}

pub fn radix_sort(arr: &mut [i32]) {
    let n: usize = arr.len();
    if n == 0 {
        return;
    }
    let mut max: i32 = arr[0];
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }
    let mut exp: i32 = 1;
    let mut output: [i32; 256] = [0; 256];
    while max / exp > 0 {
        let mut count: [usize; 10] = [0; 10];
        for &num in arr.iter() {
            count[(num / exp % 10) as usize] += 1;
        }
        for i in 1..10 {
            count[i] += count[i - 1];
        }
        for &num in arr.iter().rev() {
            let index: usize = (num / exp % 10) as usize;
            output[count[index] - 1] = num;
            count[index] -= 1;
        }
        for i in 0..n {
            arr[i] = output[i];
        }
        exp *= 10;
    }
}