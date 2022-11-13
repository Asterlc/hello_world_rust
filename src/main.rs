static GLOBAL: u8 = 1;
static TEXTO_GLOBAL: &str = "Texto";
static mut NAO_SEGURA: &str = "Marcar como unsafe";

fn sombra() {
    let mut a: i32 = 123;
    {
        a = 321;
    }
    println!("a: {}", a);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn condicional(idade: i32) {
    let condicao: &str = if idade >= 18 { "Maior que 18" } else { "Menor que 18" };
    println!("Condicao: {}", condicao);
}

fn escopo() {
    println!("Hello, world!");
    println!("Variável global: {}", &GLOBAL);
    println!("Variável global2: {}", &TEXTO_GLOBAL);
    unsafe {
        println!("Não é seguro: {}", &NAO_SEGURA);
    }


    let inteiro: i64 = 32;
    let flutuante: f32 = 32.32;
    const PI: f32 = 3.1415;

    println!("Inteiro: {}", &inteiro);
    println!("Memória Inteiro: {}", std::mem::size_of_val(&inteiro));
    println!("Flutuante: {}", &flutuante);
    println!("Memória Flutuante: {}", std::mem::size_of_val(&flutuante));
    println!("Valor de PI: {}", &PI);
    println!("Memória PI: {}", std::mem::size_of_val(&PI));
}

fn linguagem(nome: &str) -> &str {
    let proposito = match nome.to_uppercase().as_str() {
        "PHP" => "Web",
        "JAVA" => "Backend",
        "HTML" => "Páginas web",
        _ => "Desconhecido"
    };
    println!("Propósito de {} é {}", &nome.to_uppercase().as_str(), &proposito);
    proposito
}

fn ownerShip() {
    let mut string = String::from("Lucas");
    rouba(&mut string);
    println!("OwnerShip: {}", &string)
}

fn rouba(string: &mut String) -> &String {
    println!("Nome: {}", string);
    string
}

// fn erros() {
//     // panic!("Erro blabla")
//     match resultado() {
//         Ok(s) => println!("Sucesso: {}", s),
//         _ => Err(42)
//     };
// }
//
// fn resultado() -> Result<String, u8> {
//     // Ok(String::from("Tudo certo"))
//     Err(42)
// }

fn taboada(number: i32, table: i32, method: char) {
    let mut contador: i32 = 1;
    if method == 'f' {
        for contador in 1..=table {
            println!("{} x {} = {}", number, contador, number * contador);
        }
    }
    if method == 'w' {
        while contador <= table {
            println!("{} x {} = {}", number, contador, number * contador);
            contador += 1;
        }
    }

    if method == 'l' {
        loop {
            println!("{} x {} = {}", number, contador, number * contador);
            contador += 1;
            if contador == table { break; };
        }
    }
}

fn main() {
    //escopo();
    //sombra();
    // let x: i32 = soma(10, 10);
    //println!("soma de x é : {}", x);
    //condicional(18);
    // taboada(9, 10, 'f');
    // linguagem("HtMl");
    ownerShip();
    // erros();
}
