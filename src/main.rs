mod funcoes;

fn main()
{
    {
        funcoes::imprime_texto();    

        let valor: i32;

        valor = funcoes::convert_to_int(&String::from("123"));
        println!("O valor em int é {}\nCom mais um é {}", valor, valor + 1);
    }
}
