use std::io;

fn main() {
    let mut verb: String = String::new();

    let mut num1 = String::new(); 
    let mut num2 = String::new();


    let mut num3: i64 = 0;
    let mut num4: i64 = 0;

    let mut result: i64 = 0;

    println!("First: ");
    match io::stdin().read_line(&mut num1) {
        Ok(_) => num3 = num1.trim().parse()
            .expect("please give me correct string number!"),
        Err(err) => println!("{}", err)
    };

    println!("Second: ");
    match io::stdin().read_line(&mut num2) {
        Ok(_) => num4 = num2.trim().parse()
            .expect("please give me correct string number!"),
        Err(err) => println!("{}", err)
    };

    println!("what do? (+, -, *, /): ");
    match io::stdin().read_line(&mut verb) {
        Ok(_) => match verb.trim() {
            "+" => result = num3 + num4,
            "-" => result = num3 - num4,
            "*" => result = num3 * num4,
            "/" => result = num3 / num4,
            _ => println!("WTF") //
        },
        Err(err) => println!("{}", err)
    }
;

    println!("{}", result);
}
