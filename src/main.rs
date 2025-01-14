use controllers::generate_values::cadenas_id;
use controllers::storage::agregar_linea_al_archivo;

mod controllers;

const RUTA_ARCHIVO: &str = "./keys_rust.txt";

fn main() {
    let resultado = cadenas_id();
    println!("Generated Key: {}", resultado);

    // Store generated key on a txt file
    agregar_linea_al_archivo(RUTA_ARCHIVO, &resultado);
}
