use crate::models::cliente::Cliente;
use super::ler::{ler_dados, ler_dados_int};
use super::operationsbasics::{limpar_tela, esperar};


pub fn incluir_cliente(clientes: &mut Vec<Cliente>){
limpar_tela();

  let mut cliente: Cliente = Cliente::default();

  cliente.id = clientes.len() + 1;

  digitar_dados_do_cliente(&mut cliente);
  
  clientes.push(cliente);

  limpar_tela();
  println!("Cliente cadastrado com sucesso!");
  esperar(1);
}

fn digitar_dados_do_cliente(cliente: &mut Cliente) {
  println!("Digite o nome do cliente");
  cliente.nome = ler_dados();

  println!("Digite o CPF do cliente");
  cliente.cpf = ler_dados();

  println!("Digite o endereço do cliente");
  cliente.endereco = ler_dados();
}

pub fn alterar_clientes(clientes: &mut Vec<Cliente>) {
  limpar_tela();  
  if nao_tem_clientes(clientes) {
    return;
  }

  let id = capitura_id();
  if let Some(indice) = buscar_cliente_por_id(clientes, id) { 
    println!("{}", "-".to_string().repeat(40));
    println!("Alterando o cliente");
    println!("{}", "-".to_string().repeat(40));
    mostrar_cliente(&clientes[indice]);
    println!("{}", "-".to_string().repeat(40));
    digitar_dados_do_cliente(&mut clientes[indice]);
    limpar_tela();
    println!("Cliente alterado com sucesso!");

  } else {
    limpar_tela();
    println!("Cliente não encontrado!");
  }
  esperar(1);
}

fn buscar_cliente_por_id(clientes: &[Cliente], id: usize) -> Option<usize> {
    clientes.iter().position(|cliente| cliente.id == id)
}

fn capitura_id() -> usize {
  limpar_tela();
  println!("Digite o ID do cliente");
  ler_dados_int()
}

fn mostrar_cliente(cliente: &Cliente) {
    println!("\
    ID: {}\n\
    Nome: {}\n\
    CPF: {}\n\
    Endereço: {}
    ", cliente.id, cliente.nome, cliente.cpf, cliente.endereco);
  }

pub fn listar_clientes(clientes: &[Cliente]) {
  limpar_tela();

  if nao_tem_clientes(clientes) {
    return;
  }

  println!("{}", "-".to_string().repeat(40));
  for cliente in clientes {
    mostrar_cliente(cliente);
    println!("{}", "-".to_string().repeat(40));
  }

  println!("Digite enter para continuar...");
  ler_dados();
}

  fn nao_tem_clientes(clientes: &[Cliente]) -> bool {
    if clientes.len() == 0 {
      println!("Não há clientes cadastrados!");
      esperar(1);
      return true;
    }
    return false;
  }