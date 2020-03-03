use std::collections::LinkedList;
use std::io;

fn main() {

  let mut lista_total = LinkedList::new();

  loop {
  
    println!("\n[X]Olá, bem vindo ao Crud de Concessionaria!");
    println!("[1]Cadastrar.");
    println!("[2]Apresentar.");
    println!("[3]Alterar.");
    println!("[4]Excluir.");
    println!("[5]Sair.");
    println!("[X]Escolha uma opção: ");

    let mut verificador_menu = String::new();
    io::stdin().read_line(&mut verificador_menu).expect("Falhou a gravação");

    if verificador_menu.trim() == "1" {
      println!("\n[X]Opção de Cadastrar");

      println!("\n[X]Digite o codigo do carro: ");
      let mut codigo = String::new();
      io::stdin().read_line(&mut codigo).expect("Falhou a gravação");
      
      println!("\n[X]Digite o nome do carro: ");
      let mut nome = String::new();
      io::stdin().read_line(&mut nome).expect("Falhou a gravação");
      
      println!("\n[X]Digite o ano do carro: ");
      let mut ano = String::new();
      io::stdin().read_line(&mut ano).expect("Falhou a gravação");
      
      println!("\n[X]Digite o tipo de combustivel do carro: ");
      let mut combustivel = String::new();
      io::stdin().read_line(&mut combustivel).expect("Falhou a gravação");
      
      println!("\n[X]Digite a configuração do carro: ");
      let mut configuracao = String::new();
      io::stdin().read_line(&mut configuracao).expect("Falhou a gravação");
      
      println!("\n[X]Digite a quantidade de lugares do carro: ");
      let mut lugares = String::new();
      io::stdin().read_line(&mut lugares).expect("Falhou a gravação");
      
      println!("\n[X]Digite o tipo de tração do carro: ");
      let mut tracao = String::new();
      io::stdin().read_line(&mut tracao).expect("Falhou a gravação");

      let dados_cadastro = (codigo, nome, ano, combustivel, configuracao, lugares, tracao);
      lista_total.push_back(dados_cadastro);

    } 
    
    else if verificador_menu.trim() == "2" {
      println!("\n[X]Opção de Apresentar");
      
      if lista_total.len() == 0 {
        println!("[X]Nenhum usuário registrado!");
      }

      else{
        
        for i in &lista_total {
          println!("{:?}", i);
        }
      }
    } 
    
    else if verificador_menu.trim() == "3" {
      println!("\n[X]Opção de Alterar");

      if lista_total.len() == 0 {
        println!("[X]Nenhum usuário registrado!");
      }

      else{
      
      }
    } 
    
    else if verificador_menu.trim() == "4" {
      println!("\n[X]Opção de Excluir");

      if lista_total.len() == 0 {
        println!("[X]Nenhum usuário registrado!");
      }
      
      else{
      
      }
    } 
    
    else if verificador_menu.trim() == "5" {
      println!("\n[X]Opção de Sair. Obrigado!");
      break;
    } 
    
    else {
      println!("\n[X]Escolha uma opção válida!")
    }
  }
}