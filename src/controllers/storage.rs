use std::fs::OpenOptions;
use std::io::Write;

pub fn agregar_linea_al_archivo(ruta_archivo: &str, linea: &str) {
    // Intentar abrir el archivo para agregar una línea.
    let resultado = OpenOptions::new()
        .create(true) // Crea el archivo si no existe
        .append(true) // Abre el archivo en modo de agregar (append)
        .open(ruta_archivo)
        .and_then(|mut archivo| {
            writeln!(archivo, "{}", linea) // Escribir la línea en el archivo
        });

    // Manejar el resultado dentro de la función
    match resultado {
        Ok(()) => println!("Línea agregada correctamente a '{}'", ruta_archivo),
        Err(e) => eprintln!("Error al escribir en el archivo: {}", e),
    }
}
