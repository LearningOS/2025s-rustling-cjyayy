/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let pivot_index = partition(array);
    let (left, right) = array.split_at_mut(pivot_index);
    sort(left);
    sort(&mut right[1..]); // Skip the pivot element
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let len = array.len();
    let pivot_index = len / 2;
    array.swap(pivot_index, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if array[j] <= array[len - 1] {
            array.swap(i, j);
            i += 1;
        }
    }
    
    array.swap(i, len - 1);
    i
}