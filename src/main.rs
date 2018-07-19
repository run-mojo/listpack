extern crate listpack;

use listpack::*;

fn main() {
    test_replace();
//    test_cmp();
    for _ in 0..100 {
//        test_cmp();
//        test_eq();
    }
}

//fn print_cmp(lp: &mut Listpack, ele_1: Element, ele_2: Element) {
//    let v1 = lp.get(ele_1);
//    let v2 = lp.get(ele_2);
//
//    match &v1.partial_cmp(&v2).unwrap() {
//        std::cmp::Ordering::Less => {
//            println!("{} < {}", lp.get_str(ele_1), lp.get_str(ele_2));
//        }
//        std::cmp::Ordering::Equal => {
//            println!("{} == {}", lp.get_str(ele_1), lp.get_str(ele_2));
//        }
//        std::cmp::Ordering::Greater => {
//            println!("{} > {}", lp.get_str(ele_1), lp.get_str(ele_2));
//        }
//    }
//}
//
//fn print_eq(lp: &mut Listpack, ele_1: Element, ele_2: Element) {
//    let v1 = lp.get(ele_1);
//    let v2 = lp.get(ele_2);
//    if v1 == v2 {
//        println!("{} == {}", lp.get_str(ele_1), lp.get_str(ele_2));
//    } else {
//        println!("{} != {}", lp.get_str(ele_1), lp.get_str(ele_2));
//    }
//}

//fn test_adapter() {
//    let mut lp = Listpack::new();
//
//}

fn test_replace() {
    let mut lp = Listpack::new();
    lp.append_int(1);
    let mut ele = lp.first().unwrap();
    println!("{:p}", ele);

    for _ in 0..100 {
        lp.append_int(500);
    }

    ele = lp.first().unwrap();
    println!("{:p}", ele);

    ele = lp.replace_int(ele, 2).unwrap();
    println!("{:p}", ele);

    ele = lp.replace_int(ele, 200).unwrap();
    println!("{:p}", ele);
    println!("{:p}", lp.first().unwrap());

    println!("{}", lp.get_int(ele));

//    println!("Iterate forward...");
//    let mut ele = lp.start();
//    while let Some(v) = lp.first_or_next(ele) {
//        ele = v;
//        let val = lp.get(ele);
//        match val {
//            Value::Int(v) => {
//                println!("Int     -> {}", v);
//            }
//            Value::Str(_v, _len) => {
//                println!("String  -> {}", val.as_str());
//            }
//        }
//    }
}

//fn test_cmp() {
//    let mut lp = Listpack::new();
//    lp.append_str("hello");
//    lp.append_str("bye");
//
//    let mut ele_1 = lp.first().unwrap();
//    let mut ele_2 = lp.seek(1).unwrap();
//    print_cmp(&mut lp, ele_1, ele_2);
//
//    {
//        lp.append_str("Bye");
//        lp.append_str("bye");
//        ele_1 = lp.next(ele_2).unwrap();
//        ele_2 = lp.next(ele_1).unwrap();
//        print_cmp(&mut lp, ele_1, ele_2);
//    }
//
//    {
//        lp.append_str("by");
//        lp.append_str("bye");
//        ele_1 = lp.next(ele_2).unwrap();
//        ele_2 = lp.next(ele_1).unwrap();
//        print_cmp(&mut lp, ele_1, ele_2);
//    }
//
//    {
//        lp.append_str("bya");
//        lp.append_str("bye");
//        ele_1 = lp.next(ele_2).unwrap();
//        ele_2 = lp.next(ele_1).unwrap();
//        print_cmp(&mut lp, ele_1, ele_2);
//    }
//}
//
//fn test_eq() {
//    let mut lp = Listpack::new();
//
//    {
//        lp.append_str("hello");
//        lp.append_str("bye");
//        let ele_1 = lp.first().unwrap();
//        let ele_2 = lp.next(lp.first().unwrap()).unwrap();
//        print_eq(&mut lp, ele_1, ele_2);
//    }
//
//    {
//        lp.append_str("Bye");
//        lp.append_str("bye");
////        println!("len -> {}", lp.len());
//        let ele_1 = lp.seek(2).unwrap();
////        let ele_1 = lp.next(lp.next(lp.first().unwrap()).unwrap()).unwrap();
//        let ele_2 = lp.next(ele_1).unwrap();
//        print_eq(&mut lp, ele_1, ele_2);
//    }
//
//    {
//        lp.append_str("by");
//        lp.append_str("bye");
//        let ele_1 = lp.seek(4).unwrap();
//        let ele_2 = lp.next(ele_1).unwrap();
//        print_eq(&mut lp, ele_1, ele_2);
//    }
//
//    {
//        lp.append_str("byea");
//        lp.append_str("bye");
//        let ele_1 = lp.seek(6).unwrap();
//        let ele_2 = lp.next(ele_1).unwrap();
//        print_eq(&mut lp, ele_1, ele_2);
//    }
//
//    {
//        lp.append_str("bye");
//        lp.append_str("bye");
//        let ele_1 = lp.seek(8).unwrap();
//        let ele_2 = lp.next(ele_1).unwrap();
//        print_eq(&mut lp, ele_1, ele_2);
//    }
//}
