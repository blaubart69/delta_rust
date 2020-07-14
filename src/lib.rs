use std::cmp::Ordering;

pub fn run_old_unwrap<'a, A:'a + Sized, B:'a + Sized, T1, T2, K1, K2, K3, A1, C1, C2, C3, C4>(
    list_a              : &mut T1,
    list_b              : &mut T2,
    key_comparer_a_b    : K1,
    key_comparer_a_a    : K2,
    key_comparer_b_b    : K3,
    attribute_comparer  : A1,
    only_exist_in_a     : &mut C1,
    only_exist_in_b     : &mut C2,
    modified            : &mut C3,
    samesame            : &mut C4,
)
where
 T1 : Iterator<Item=&'a A> + ?Sized
,T2 : Iterator<Item=&'a B> + ?Sized
,K1 : Fn(&A, &B) -> Ordering
,K2 : Fn(&A, &A) -> Ordering
,K3 : Fn(&B, &B) -> Ordering
,A1 : Fn(&A, &B) -> bool
,C1 : FnMut(&A)
,C2 : FnMut(&B)
,C3 : FnMut(&A,&B)
,C4 : FnMut(&A,&B)

{
    let mut last_a: Option<&A> = None;
    let mut last_b: Option<&B> = None;

    let mut a_opt = list_a.next();
    let mut b_opt = list_b.next();

    loop {
        if a_opt.is_none() && b_opt.is_none() {
            break;
        }

        if a_opt.is_some() && b_opt.is_some() {
            let a = a_opt.unwrap();
            let b = b_opt.unwrap();

            match key_comparer_a_b(&a, &b) {
                Ordering::Equal => {
                    if attribute_comparer(&a, &b) == true {
                        samesame(&a, &b);
                    } else {
                        modified(&a, &b);
                    }
                    last_a = a_opt;
                    last_b = b_opt;
                    a_opt = list_a.next();
                    b_opt = list_b.next();
                }
                Ordering::Less => {
                    only_exist_in_a(&a);
                    last_a = a_opt;
                    a_opt = list_a.next();
                }
                Ordering::Greater => {
                    only_exist_in_b(&b);
                    last_b = b_opt;
                    b_opt = list_b.next();
                }
            }
        } else if a_opt.is_some()
               && b_opt.is_none() {

            only_exist_in_a(&a_opt.unwrap());
            last_a = a_opt;
            a_opt = list_a.next();

        } else if a_opt.is_none()
               && b_opt.is_some() {

            only_exist_in_b(&b_opt.unwrap());
            last_b = b_opt;
            b_opt = list_b.next();

        };
    }
}

pub fn run<'a, A:'a + Sized, B:'a + Sized, T1, T2, K1, K2, K3, A1, C1, C2, C3, C4>(
    list_a              : &mut T1,
    list_b              : &mut T2,
    key_comparer_a_b    : K1,
    key_comparer_a_a    : K2,
    key_comparer_b_b    : K3,
    attribute_comparer  : A1,
    only_exist_in_a     : &mut C1,
    only_exist_in_b     : &mut C2,
    modified            : &mut C3,
    samesame            : &mut C4,
)
    where
        T1 : Iterator<Item=&'a A> + ?Sized
    ,T2 : Iterator<Item=&'a B> + ?Sized
    ,K1 : Fn(&A, &B) -> Ordering
    ,K2 : Fn(&A, &A) -> Ordering
    ,K3 : Fn(&B, &B) -> Ordering
    ,A1 : Fn(&A, &B) -> bool
    ,C1 : FnMut(&A)
    ,C2 : FnMut(&B)
    ,C3 : FnMut(&A,&B)
    ,C4 : FnMut(&A,&B)

{
    let mut last_a: Option<&A> = None;
    let mut last_b: Option<&B> = None;

    let mut a_opt = list_a.next();
    let mut b_opt = list_b.next();

    loop {

        match a_opt {
            None => {
                match b_opt {
                    None => { break; }
                    Some(b) => {
                        only_exist_in_b(&b);
                        last_b = b_opt;
                        b_opt = list_b.next();
                    }
                }
            }
            Some(a) => {
                match b_opt {
                    None => {
                        only_exist_in_a(&a);
                        last_a = a_opt;
                        a_opt = list_a.next();
                    }
                    Some(b) => {
                        match key_comparer_a_b(&a, &b) {
                            Ordering::Equal => {
                                if attribute_comparer(&a, &b) == true {
                                    samesame(&a, &b);
                                } else {
                                    modified(&a, &b);
                                }
                                last_a = a_opt;
                                last_b = b_opt;
                                a_opt = list_a.next();
                                b_opt = list_b.next();
                            }
                            Ordering::Less => {
                                only_exist_in_a(&a);
                                last_a = a_opt;
                                a_opt = list_a.next();
                            }
                            Ordering::Greater => {
                                only_exist_in_b(&b);
                                last_b = b_opt;
                                b_opt = list_b.next();
                            }
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::run;
    use std::cmp::Ordering;

    fn int_ordering(a:&i32, b:&i32) -> Ordering {
        if a < b {
            Ordering::Less
        }
        else if a == b {
            Ordering::Equal
        }
        else {
            Ordering::Greater
        }
    }

    fn run_delta<'a>(
        list_a : &mut dyn Iterator<Item=&'a i32>,
        list_b : &mut dyn Iterator<Item=&'a i32>) -> (i32, i32, i32, i32) {

        let mut samesame = 0;
        let mut modified = 0;
        let mut onlya = 0;
        let mut onlyb = 0;

        run(list_a, list_b,
            int_ordering,
            int_ordering,
            int_ordering,
            |&a, &b| { true },
            &mut |&a| { onlya+=1; },
            &mut |&b| { onlyb+=1; },
            &mut |&a, &b| { modified+=1},
            &mut |&a, &b| { samesame+=1; });

        (samesame, modified, onlya, onlyb)
    }

    #[test]
    fn three_same_elemets() {
        let l1 = vec![1,2,3];
        let l2 = vec![1,2,3];

        let mut samesame = 0;
        let mut modified = 0;
        let mut onlya = 0;
        let mut onlyb = 0;

        run(&mut l1.iter(),&mut l2.iter(),
             //|&a, &b| { if a < b { Ordering::Less} else if a == b { Ordering::Equal} else { Ordering::Greater} },
             int_ordering,
             int_ordering,
             int_ordering,
             |&a, &b| { true },
             &mut |&a| { onlya+=1; },
             &mut |&b| { onlyb+=1; },
             &mut |&a, &b| { modified+=1},
             &mut |&a, &b| { samesame+=1; });

        assert_eq!(3,samesame);
        assert_eq!(0,onlya);
        assert_eq!(0,onlyb);
        assert_eq!(0,modified);
    }
    #[test]
    fn one_element_but_different() {
        let l1 = vec![1];
        let l2 = vec![2];

        let mut samesame = 0;
        let mut modified = 0;
        let mut onlya = 0;
        let mut onlyb = 0;

        run(&mut l1.iter(),&mut l2.iter(),
            |&a, &b| { if a < b { Ordering::Less} else if a == b { Ordering::Equal} else { Ordering::Greater} },
            |&a1, &a2| { Ordering::Greater },
            |&b1, &b2| { Ordering::Greater },
            |&a, &b| { true },
            &mut |&a| { onlya+=1; assert_eq!(1, a); },
            &mut |&b| { onlyb+=1; assert_eq!(2, b); },
            &mut |&a, &b| { modified+=1},
            &mut |&a, &b| { samesame+=1; });

        assert_eq!(0,samesame);
        assert_eq!(1,onlya);
        assert_eq!(1,onlyb);
        assert_eq!(0,modified);
    }
    #[test]
    fn one_element_missing_in_other_list() {
        let l1 = vec![1];
        let l2 = vec![1,2];

        let (samesame, modified, onlya, onlyb) = run_delta(&mut l1.iter(), &mut l2.iter());

        assert_eq!(1,samesame);
        assert_eq!(0,onlya);
        assert_eq!(1,onlyb);
        assert_eq!(0,modified);
    }

}
