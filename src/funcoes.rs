pub fn imprime_texto()
{
    println!("Conseguiu acessar o método da função!");
}

pub fn convert_to_int(str_valor: & String) -> i32
{
    let int_valor = str_valor.trim().parse::<i32>().unwrap();
    return int_valor;
}