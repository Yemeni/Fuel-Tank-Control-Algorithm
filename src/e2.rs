
fn main2 (){
    let n = 5;
    let mut output: Vec<String> = Vec::new();
    let str: String = "Fizz".to_string();
    let num: String = 2.to_string();
    output.push(str);
    for i in 1..=n {
        println!("{i}");
        println!("Is: {}", i)

    }

    let m = 4%5;
    println!("Hello a");
    if 1 == 2 {
        println!("Hello b")
    }else if 1 == 2 {
        println!("Something")
    }else{
        println!("{:?}", output)
    }

}
