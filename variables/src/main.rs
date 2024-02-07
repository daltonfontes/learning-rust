fn main() {
    // Variaveis e Mutabilidade
    let mut x: i32 = 5;

    println!("value {x}");
    x = 6;
    println!("value {x}");
    x = 1991;
    println!("value {x}");
    // constantes
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("hora : {THREE_HOURS_IN_SECOND}");

    //Shadowing
    let x: i32 = 5;
    let x: i32 = x + 15;
    {
        let x: i32 = x * 30;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //Tipo de dados

    //numeros flutuantes
    let _x: f64 = 2.0;
    let _y: f32 = 3.0;
    println!("values : {_x}, {_y}");

    // operadores matematicos

    //addition
    let sum: i32 = 10 + 10;
    println!("resultado :{sum}");
    //multiplication
    let product: i32 = 3 * 3;
    println!("resultado :{product}");
    //subtraction
    let difference: i32 = 1000 - 900;
    println!("resultado : {difference}");
    //division
    let quotient = 60.0 / 53.7;
    let truncated = -5 / 3;
    println!("resultado : {quotient}");
    println!("resultado : {truncated}");

    // tipo boleano

    let a: bool = false;
    let b:bool = true;
    println!("A: {a}");
    println!("B: {b}");


}
