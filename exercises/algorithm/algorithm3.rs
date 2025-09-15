/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]){
	// 实现快速排序算法
    if array.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(array);
    
    // 递归排序两个子数组
    sort(&mut array[0..pivot_index]);
    sort(&mut array[pivot_index + 1..]);
}

// 分区函数，返回pivot最终位置
fn partition<T: Ord>(array: &mut [T]) -> usize {
    let len = array.len();
    let pivot_index = len - 1; // 选择最后一个元素作为pivot
    
    let mut i = 0; // 小于pivot的元素的最后位置
    
    // 遍历数组，将小于pivot的元素移到左侧
    for j in 0..len - 1 {
        if array[j] <= array[pivot_index] {
            array.swap(i, j);
            i += 1;
        }
    }
    
    // 将pivot放到正确位置
    array.swap(i, pivot_index);
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}