// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

struct S1(i32);

#[derive(Clone)]
struct S2(i32);

fn main() {
    let mut vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&mut vec0);

    assert_eq!(vec0, vec![22, 44, 66, 88]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);

    let ref_i32: &i32 = &1;
    let _cloned_ref_i32 = ref_i32.clone();

    let ref_s: &S1 = &S1(1);
    let _cloned_ref_s: &S1 = ref_s.clone();

    let mut s2 = S2(1);
    let mut_ref_s2 = &mut s2;
    let _cloned_s2 = mut_ref_s2.clone();
    let _another_cloned_s2 = (*mut_ref_s2).clone();
    *mut_ref_s2 = S2(10);

    println!("{}", s2.0);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec.clone()
}
