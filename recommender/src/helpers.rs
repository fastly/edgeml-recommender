use rand::Rng;

fn transpose(matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    if matrix.is_empty() {
        return vec![];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut transposed = vec![vec![0.0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

fn partition(arr: &mut [f32], pivot_index: usize) -> usize {
    let pivot_value = arr[pivot_index];
    arr.swap(pivot_index, arr.len() - 1);
    let mut store_index = 0;
    for i in 0..arr.len() - 1 {
        if arr[i] < pivot_value {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, arr.len() - 1);
    store_index
}

fn quickselect(arr: &mut [f32], k: usize, rng: &mut impl Rng) -> f32 {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let pivot_index = left + rng.gen_range(0..(right - left));
        let pivot_new_index = partition(arr, pivot_index);
        if pivot_new_index == k {
            return arr[pivot_new_index];
        } else if pivot_new_index > k {
            right = pivot_new_index;
        } else {
            left = pivot_new_index + 1;
        }
    }
    arr[left]
}

fn median_with_quickselect(arr: &[f32], rng: &mut impl Rng) -> f32 {
    let len = arr.len();
    let mut arr = arr.to_vec();
    let mid = len / 2;
    if len % 2 == 0 {
        (quickselect(&mut arr, mid - 1, rng) + quickselect(&mut arr, mid, rng)) / 2.0
    } else {
        quickselect(&mut arr, mid, rng)
    }
}

pub fn median_vector(vectors: &[Vec<f32>]) -> Vec<f32> {
    let transposed = transpose(vectors);
    let mut rng = rand::thread_rng();
    transposed.iter().map(|column| median_with_quickselect(column, &mut rng)).collect()
}
