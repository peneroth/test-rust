fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess_f: f64 = guess as f64;
    let x1 = 5;
    let x2 = plus_one(x1);

    println!("guess_f är: {guess_f}");
    println!("x1 är: {x1}");
    println!("x2 är: {x2}");

    // Evaluate ownership

    // Default function call
    {
        let s1 = String::from("Hello");
        let s2 = append_string(s1);  
        // println!("s1 = {}",s1); // s have ceased to exist
        println!("s2 = {}",s2);
    }
    // Try to reasign
    {
        let mut s1 = String::from("Hello");
        s1 = append_string(s1);  
        println!("s1 = {}",s1);
    }

    // Try to mutable references
    {
        let mut s1 = String::from("Hello");
        append_string_ref(&mut s1);  
        println!("s1 = {}",s1);
    }


}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn append_string(mut s:String) -> String {
    s.push_str(", world!");
    s
}

fn append_string_ref(s: &mut String) {
    s.push_str(", world!");
}