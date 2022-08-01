#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroy struct E {}", self.a);
    }
}

fn fn_once<F>(f: F)
where
    F: FnOnce(),
{
    println!("fn_once begins");
    f();
    println!("fn_once ends");
}

fn fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    println!("fn_mut begins");
    f();
    f();
    f();
    println!("fn_mut ends");
}

fn fn_immut<F>(func: F)
where
    F: Fn(),
{
    println!("fn_immut begins");
    func();
    func();
    println!("fn_immut ends");
}

fn main() {
    let mut e = E {
        a: "a_fasdf".to_string(),
    };
    let f = || {
        println!("closure calls : {:?}", e);
    };

    fn_once(f);

    {
        let f2 = || {
            println!("FnMut closure calls: {:?}", e);
            e.a = "b_fasdf".to_string();
        };
        fn_mut(f2);
    }

    {
        //let e = E { a: "fn_once".to_string() };
        let f3 = || {
            println!("Fn closure calls: {:?}", e);
        };
        fn_immut(f3);
    }

    {
        fn fn_immut2<F>(f: F)
        where
            F: Fn() -> String,
        {
            println!("calling Fn closure from fn, {}", f());
        }

        let a = "9Fn_safsdf".to_string();
        fn_immut2(|| a.clone()); // 闭包返回一个字符串
    }

    let mut x = 5;
    {
        let mut square_x = || x *= x;
        square_x();
    }
    assert_eq!(x, 25);

    {
        fn do_twice<F>(mut func: F)
        where
            F: FnMut(),
        {
            func();
            func();
        }

        let mut x: usize = 1;
        {
            let add_two_to_x = || x += 2;
            do_twice(add_two_to_x);
        }

        assert_eq!(x, 5);
    }

    println!("main ends");
}


//
// rust 中 move, copy, clone, drop 和闭包捕获
//
// https://rustcc.cn/article?id=565c926d-38f4-4b02-947f-c14ac1245ba0
//

#[test]
fn test_clone_struct() {
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum E1 {
        Text,
        Digit,
    }
    
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct S2 {
        u: usize,
        e: E1,
        s: String,
    }

    let s2 = S2 {
        u: 1,
        e: E1::Text,
        s: "hello".to_string(),
    };
    println!("{:#?}", s2);
}

#[test]
fn test_fn_i8() {
    let mut i = 1_i8;
    let f = || i + 1;

    let v = f();

    dbg!(&i);

    let v2 = f();

    i+= 10;

    assert_eq!(2, v);
    assert_eq!(2, v2);
    assert_eq!(11, i);
}

#[test]
fn test_fn_mut_i8() {
    let mut i = 1_i8;
    let mut f = || {
        i += 1;
        i
    };

    let v = f();
    //dbg!(&i);
    let v2 = f();
    i += 10;
    assert_eq!(2, v);
    assert_eq!(3, v2);
    assert_eq!(13, i);
}

#[test]
fn test_fn_mut_i8_move()
{
    let mut i = 1_i8;
    let mut f = move || {
        i += 1;
        i
    };
    let v = f();
    dbg!(&i);
    i += 10;
    let v2 = f();
    assert_eq!(2, v);
    assert_eq!(3, v2);
    assert_eq!(11, i);
}

#[test]
fn test_fn_string() {
    let mut s = "hello".to_string();
    let f = || -> String {
        dbg!(&s);
        "world".to_string()
    };
    let v = f();
    dbg!(&s);
    let v2 = f();
    s += " world";
    assert_eq!("world", v);
    assert_eq!("world", v2);
    assert_eq!("hello world", s);
}

#[test]
fn test_fn_mut_string() {
    let mut s = "hello".to_string();
    let mut f = || {
        dbg!(&s);
        s.push_str(" world");
        s.clone()
    };
    let v = f();
    // dbg!(&s);
    let v2 = f();
    dbg!(&s);
    s += " moto";
    assert_eq!("hello world", v);
    assert_eq!("hello world world", v2);
    assert_eq!("hello world world moto", s);
}

#[test]
fn test_fn_mut_string_move() {
    let mut s = "hello".to_string();
    let mut f = move || {
        dbg!(&s);
        s.push_str(" world");
        s.clone()
    };

    // s被move进f闭包中，s没有被消耗，是FnMut trait的一个实例，所以可以被move
    let v = f();

    // s被move进闭包，s不能被borrowed
    // dbg!(&s);

    // f可以多次调用
    let v2 = f();
    // dbg!(&s);

    // s被move进闭包，s不能被borrowed，但是可以绑定新实例，因为闭包是move的
    s = "moto".to_string();

    assert_eq!("hello world", v);
    assert_eq!("hello world world", v2);
    assert_eq!("moto", s);
}

#[test]
fn test_fn_once_string() {
    let mut s = "Hello".to_owned();
    let f = || {
        s.push_str(" world");
        s   // s被消耗
    };

    // s被move进f闭包中，s被消耗，是FnOnce trait
    let v = f();

    // s变量已经被move了，不能再被borrowed
    // dbg!(&s);
    
    // f只能调用一次
    // let v2 = f();

    // s被move进闭包，s不能被borrowed，但是可以绑定新实例
    s = "moto".to_owned();

    assert_eq!("Hello world", v);
    assert_eq!("moto", &s);
}

#[test]
fn test_fn_once_move_string() {
    let mut s = "Hello".to_owned();
    let f = move || s.into_boxed_str();

    // s被move进f闭包中，s被消耗，是FnOnce trait
    let v = f();

    // s变量已经被move了，不能再被borrowed
    // dbg!(&s);
    
    // f只能调用一次
    // let v2 = f();

    // s被move进闭包，s不能被borrowed，但是可以绑定新实例
    s = "moto".to_owned();

    assert_eq!("Hello", &*v);
    assert_eq!("moto", &s);
}
