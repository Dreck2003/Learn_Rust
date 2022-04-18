use std::io;

// const corte: i32 = 100;
fn main() {
    //Excercise for a fenheight to Celsius:
    println!("Enter your farenheigth number xd: ");

    let text_input = String::new();
    fa_to_cel(text_input);

    //Exercise for a fibbonacci :
    println!("Enter your number for a function fibbonaccci!");
    let fibbo = String::new();

    let number = read_input(fibbo);
    if number == 0 {
        println!("Type other number!");
    } else {
        let fibbo = fibbonacci(number, 0);
        println!("The fibbonacci result is: {}", fibbo);
    }

    //Exercise print song Twelve Days Of Christmas using loop

    let array = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "I sent eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let mut iterator = 0;

    let mut rompe_todo = 0;
    println!("");

    println!("On the first day of Christmas \n My true love sent to me");
    println!("And a partridge in a pear tree");
    println!("---------------------------");
    while iterator < array.len() {
        println!("On the first day of Christmas \n My true love sent to me");
        if rompe_todo > 100 {
            println!("corto!!!!!!11");
            break;
        }

        let mut inicio = iterator;

        loop {
            if inicio == 0 {
                break;
            }

            println!("{}", array[inicio]);
            inicio = inicio - 1;
        }
        println!("{}", array[0]);
        println!("And a partridge in a pear tree");
        println!("---------------------------");

        iterator = iterator + 1;
        rompe_todo = rompe_todo + 1;
    }
}

fn fa_to_cel(mut input: String) {
    io::stdin()
        .read_line(&mut input)
        .expect("enter a number real");

    let result = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => 5,
    };

    println!("converting Fahrenheit to Celsius...");
    let celsius: f64 = (result - 32) as f64 * 0.5556;
    println!("The celsius grade is: {}°", celsius);
}

fn fibbonacci(enesimo: i32, mut corte: i32) -> i32 {
    if enesimo == 0 || enesimo == 1 {
        return enesimo;
    }

    if corte == 100 {
        return 3;
    }
    corte = corte + 1;

    fibbonacci(enesimo - 1, corte) + fibbonacci(enesimo - 2, corte)
}

fn read_input(mut text: String) -> i32 {
    io::stdin()
        .read_line(&mut text)
        .expect("The input is not a number");

    let result = match text.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };

    result
}
