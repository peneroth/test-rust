fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess_f: f64 = guess as f64;
    let x1 = 5;
    let x2 = plus_one(x1);

    println!("guess_f är: {guess_f}");
    println!("x1 är: {x1}");
    println!("x2 är: {x2}");
    
    let s = String::from("Hello");
    let s2 = append_string(s);
    println!("s2 = {}",s2);

    // Evaluate ownership
    let s1 = String::from("hello"); 
    let s2 = s1;

    println!("String = {}",&s2);

}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn append_string(mut s:String) -> String {
    s.push_str(", world!");
    s
}