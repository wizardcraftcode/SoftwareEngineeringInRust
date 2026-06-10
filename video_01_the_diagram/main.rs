struct Player {
    name: String,
}
impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
        }
    }
}
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);


    let mut x =
        Player::new("fred");
    let y = &mut x;
    println!("{}", x.name);



    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str(", world");
    println!("{}", s2);
    println!("{}", s1);





    // let s1 = String::from("hello");
    // addToMyStrings(s1);
    // let x = s1.len();














    let x = String::from("hello");
    let s2 = &x;
    println!("{}", s2);

    let mut x = String::from("hello");
    let s3 = &mut x;
    println!("{}", s3);
    println!("{}", s2);










    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}", s2);
    let s3 = &s1;
    println!("{}", s3);
    println!("{}", s2);
    println!("{}", s1);

    let x = 5;
    let y = x;
    println!("{}", x);


    let str_a = String::from("a");
    let mut refer = &str_a;
    if refer == "a" {
        let mut str_b = String::from("b");
        str_b.push_str(refer);
        // refer = &str_b;
    }
    println!("{}", refer);


}


fn addToMyStrings1(p0: String) {
    todo!()
}

fn addToMyStrings2(p0: &String) {

}
fn addToMyStrings(p0: &mut String) {

}
