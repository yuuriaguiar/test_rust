
mod tela;
mod models;

use tela::menu as menu;
use models::cliente::Cliente;

fn main() {
    let mut clientes: Vec<Cliente> = Vec::new();
    menu::mostrar_menu(&mut clientes);
}
