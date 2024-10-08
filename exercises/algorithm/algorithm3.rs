/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


// 归并排序，暂时放弃
// 
fn sort<T: std::cmp::PartialOrd + Clone>(array: &mut [T]){
	//TODO
    if array.len() == 1{
        return;
    }
    for i in 0..array.len()-1{ // 比较 n-1 次, 由于是左闭右开，需要写到 len-1
        for j in i+1..array.len(){
            if array[i] > array[j]{
                let t = array[i].clone();
                array[i] = array[j].clone();
                array[j] = t.clone(); 
            }
        }
    }
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