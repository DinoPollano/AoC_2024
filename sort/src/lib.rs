
pub fn select_sort ( list:  &mut Vec<i32>) {
    let mut sorted_index = 0;

    while sorted_index < list.len() {
        let mut smallest_index : usize = sorted_index ;
        for n in sorted_index..list.len(){
           if  list[n] < list[smallest_index] {
               smallest_index = n;
           }
        }
        list.swap(sorted_index, smallest_index);
        sorted_index += 1;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test = Vec::new();
        test.extend([6,3,8,4,5]);
        select_sort(&mut test);
        let mut is_sorted = true;
        for n in 1..test.len() {
            is_sorted = test[n-1] < test[n]
        }

        assert_eq!(is_sorted, true);
    }
}
