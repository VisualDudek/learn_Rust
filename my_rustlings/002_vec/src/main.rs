// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;

    vec.push(88);

    vec
}

fn fill_vec_better(vec: Vec<i32>) -> Vec<i32> {
    // TODO: you do not need following line
    let vec = vec;

    vec.push(88);

    vec
}

fn vec_from_slice(arr: &[i32]) -> Vec<i32> {
    todo!()
}

fn vec_form_vec_ref(v: &Vec<i32>) -> Vec<i32> {
    todo!()
}

// mut in-place
fn vec_mut_inplace() {
    v.push(88);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t001_move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    #[test]
    fn t002_move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec_better(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    #[test]
    fn t003_test_vec_from_slice() {
        let vec0 = vec![22, 44, 66];
        let vec1 = vec_from_slice(&vec0);
        assert_eq!(vec1, vec![22, 44, 66]);
    }

    #[test]
    fn t004_test_vec_from_vec_ref() {
        let vec0 = vec![22, 44, 66];
        let vec1 = vec_form_vec_ref(&vec0);
        assert_eq!(vec1, vec![22, 44, 66]);
    }

    #[test]
    fn t005_test_vec_mut_inplace() {
        let mut vec0 = vec![22, 44, 66];
        vec_mut_inplace(&mut vec0);
        assert_eq!(vec0, vec![22, 44, 66, 88]);
    }
}
