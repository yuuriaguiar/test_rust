use crate::tela::ler;

pub fn mostrar_menu() {
  loop {
    println!("\
        ================= Menu =================\n\
        Escolha uma das opções abaixo:\n\n\
        1 - Cadastrar cliente\n\
        2 - Alterar cliente\n\
        3 - Excluir cliente\n\
        4 - Listar cliente\n\
        0 - Sair do programa\n\
    ");

    let opcao: i32 = ler::ler_dados_int();
    match opcao {
      1 => println!("Opção 1"),
      2 => println!("Opção 2"),
      3 => println!("Opção 3"),
      4 => println!("Opção 4"),
      0 => {
        println!("Saindo do programa...");
        return;
      },
      _ => println!("Opção inválida"),
      }

      println!("Digite enter para continuar...");
      ler::ler_dados();
    
  }
}