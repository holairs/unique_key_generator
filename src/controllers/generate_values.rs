use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn valores_random() -> (Vec<i8>, Vec<&'static str>) {
    let mut numeros_random: Vec<i8> = Vec::new();
    let mut simbolos_random: Vec<&'static str> = Vec::new();
    let numeros: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let simbolos: [&str; 13] = ["!", "@", "#", "$", "&", "^", "%", "?", "*", "b", "q", "w", "z"];

    let mut rng = thread_rng();

    // Seleccionar 4 números aleatorios
    for _ in 0..3 {
        if let Some(&numero_aleatorio) = numeros.choose(&mut rng) {
            numeros_random.push(numero_aleatorio);
        }
    }

    // Seleccionar 4 símbolos aleatorios
    for _ in 0..3 {
        if let Some(&simbolo_aleatorio) = simbolos.choose(&mut rng) {
            simbolos_random.push(simbolo_aleatorio);
        }
    }

    // Retornar los vectores generados
    (numeros_random, simbolos_random)
}

pub fn generar_bloque() -> String {
    let mut rng = thread_rng();

    // Obtener los números y símbolos aleatorios
    let (mut numeros_random, mut simbolos_random) = valores_random();

    // todo en una constante
    let mut mezcla: Vec<String> = Vec::new();

    // Agregar los números a la mezcla
    for numero in numeros_random.drain(..) {
        mezcla.push(numero.to_string());
    }
    // Agregar los símbolos a la mezcla
    for simbolo in simbolos_random.drain(..) {
        mezcla.push(simbolo.to_string());
    }

    // Randomizar la mezcla
    mezcla.shuffle(&mut rng);

    // Concatenar y devolver el bloque
    mezcla.join("")
}

pub fn cadenas_id() -> String {
    let mut resultado = String::new();

    // Generar 4 bloques distintos
    for i in 0..5 {
        let bloque = generar_bloque();
        resultado.push_str(&bloque);
        if i < 4 {
            resultado.push('-'); // Agregar el guión solo entre bloques
        }
    }

    resultado
}
