use std::collections::HashMap;
fn main() {
    let numbers = [1, 1, 2, 10 ,3 ,3 ,3 ,66, 0, 2, 3, 23, 2, 3];
    let mut num = HashMap::new();
    for n in numbers{
        let mut k = 0;
        for m in numbers{
            if n == m {
                k += 1;
            }
        }
        if !num.contains_key(&n.to_string()){
            num.insert(n.to_string(), k);
            println!("{} - {}", n.to_string(), k);
        }
    }
}
