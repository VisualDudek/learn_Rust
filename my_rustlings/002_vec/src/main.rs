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
    Vec::from(arr)
}

fn vec_form_vec_ref(v: &Vec<i32>) -> Vec<i32> {
    // Vec::from(v)
    v.to_vec()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec_better(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
