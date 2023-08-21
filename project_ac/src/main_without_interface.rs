//use std::fs;
//use std::env;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
//use std::path::Path;

// Enumeración para elegir el tipo de evento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize )]
enum Evento {
    Boda,
    Bautizo,
    FiestaEmpresa,
    ReunionFamiliar,
}

// Enumeración para elegir el código de vestimenta
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CodigoEtiqueta {
    BlackTie,
    Coctel,
    Semiformal,
    Casual,
}

// Enumeración para la Hora del Evento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum HoraEvento {
    Manana,
    Tarde,
    Noche,
}

// Enumeración para definir el tipo de cuerpo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TipoCuerpo {
    TrianguloInvertido,
    RelojDeArena,
    Pera,
    Rectangular,
}
// Enumeración para la Estación y Colorimetría
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum EstacionColorimetria {
    Otono,
    Invierno,
}
impl FromStr for Evento {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boda" => Ok(Evento::Boda),
            "Bautizo" => Ok(Evento::Bautizo),
            "Fiesta de la Empresa" => Ok(Evento::FiestaEmpresa),
            "Reunión Familiar" => Ok(Evento::ReunionFamiliar),
            _ => Err(()),
        }
    }
}
impl FromStr for CodigoEtiqueta {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "White Tie" => Ok(CodigoEtiqueta::WhiteTie),
            "Black Tie" => Ok(CodigoEtiqueta::BlackTie),
            "Coctel" => Ok(CodigoEtiqueta::Coctel),
            "Semiformal" => Ok(CodigoEtiqueta::Semiformal),
            "Casual" => Ok(CodigoEtiqueta::Casual),
            _ => Err(()),
        }
    }
}
impl FromStr for HoraEvento {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "White Tie" => Ok(CodigoEtiqueta::WhiteTie),
            "Manana" => Ok(HoraEvento::Manana),
            "Tarde" => Ok(HoraEvento::Tarde),
            "Noche" => Ok(HoraEvento::Noche),
            _ => Err(()),
        }
    }
}

impl FromStr for TipoCuerpo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Triangulo Invertido" => Ok(TipoCuerpo::TrianguloInvertido),
            "Reloj de Arena" => Ok(TipoCuerpo::RelojDeArena),
            "Pera" => Ok(TipoCuerpo::Pera),
            "Rectangular" => Ok(TipoCuerpo::Rectangular),
            _ => Err(()),
        }
    }
}
impl FromStr for EstacionColorimetria {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Otono" => Ok(EstacionColorimetria::Otono),
            "Invierno" => Ok(EstacionColorimetria::Invierno),
            _ => Err(()),
        }
    }
}


// Estructura para representar el Outfit recomendado
#[derive(Debug, Clone, Copy)]
struct Outfit {
    evento: Evento,
    codigo_etiqueta: CodigoEtiqueta,
    hora_evento: HoraEvento,
    tipo_cuerpo: TipoCuerpo,
    estacion_colorimetria: EstacionColorimetria,
}

// Función para obtener la selección del usuario para una enumeración
fn obtener_seleccion<T>(mensaje: &str, opciones: &[(&str, T)]) -> T
where
    T: std::str::FromStr + Clone, // Requerir que T implemente Clone
    T::Err: std::fmt::Debug,
{
    loop {
        println!("{}", mensaje);
        for (i, (opcion, _)) in opciones.iter().enumerate() {
            println!("{}: {}", i + 1, opcion);
        }

        let mut entrada = String::new();
        std::io::stdin().read_line(&mut entrada).expect("Error al leer la entrada.");

        // Convertir la entrada del usuario en un índice numérico
        if let Ok(indice) = entrada.trim().parse::<usize>() {
            if let Some((_, seleccion)) = opciones.get(indice - 1) {
                return seleccion.clone();
            }
        }

        println!("Selección inválida. Inténtalo nuevamente.");
    }
}

// Función para obtener las selecciones del usuario para cada característica
fn obtener_selecciones_del_usuario() -> Outfit {
    // Obtener la selección del evento
    let evento = obtener_seleccion(
        "Selecciona el evento:",
        &[
            ("Boda", Evento::Boda),
            ("Bautizo", Evento::Bautizo),
            ("Fiesta de la Empresa", Evento::FiestaEmpresa),
            ("Reunión Familiar", Evento::ReunionFamiliar),
        ],
    );

    // Obtener la selección del Código de Etiqueta
    let codigo_etiqueta = obtener_seleccion(
        "Selecciona el Código de Etiqueta:",
        &[
            //("White Tie", CodigoEtiqueta::WhiteTie),
            ("Black Tie", CodigoEtiqueta::BlackTie),
            ("Coctel", CodigoEtiqueta::Coctel),
            ("Semiformal", CodigoEtiqueta::Semiformal),
            ("Casual", CodigoEtiqueta::Casual),
        ],
    );

    // Obtener la selección de la Hora del Evento
    let hora_evento = obtener_seleccion(
        "Selecciona la Hora del Evento:",
        &[
            ("Mañana", HoraEvento::Manana),
            ("Tarde", HoraEvento::Tarde),
            ("Noche", HoraEvento::Noche),
        ],
    );

    // Obtener la selección del Tipo de Cuerpo
    let tipo_cuerpo = obtener_seleccion(
        "Selecciona el Tipo de Cuerpo:",
        &[
            ("Triángulo Invertido", TipoCuerpo::TrianguloInvertido),
            ("Reloj de Arena", TipoCuerpo::RelojDeArena),
            ("Pera", TipoCuerpo::Pera),
            ("Rectangular", TipoCuerpo::Rectangular),
            //("Ovalo", TipoCuerpo::Ovalo),
        ],
    );

    // Obtener la selección de la Estación y Colorimetría
    let estacion_colorimetria = obtener_seleccion(
        "Selecciona la Estación-Colorimetría:",
        &[
            //("Verano", EstacionColorimetria::Verano),
            ("Otoño", EstacionColorimetria::Otono),
            ("Invierno", EstacionColorimetria::Invierno),
            //("Primavera", EstacionColorimetria::Primavera),
        ],
    );

    // Devolver una instancia de la estructura Outfit con las selecciones del usuario
    Outfit {
        evento,
        codigo_etiqueta,
        hora_evento,
        tipo_cuerpo,
        estacion_colorimetria,
    }
}

// Definir la estructura para las condiciones de outfit en el archivo JSON
#[derive(Debug, Serialize, Deserialize)]
struct DecisionData {
    conditions: Vec<DecisionCondition>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DecisionCondition {
    evento: Evento,
    codigo_etiqueta: CodigoEtiqueta,
    hora_evento: HoraEvento,
    tipo_cuerpo: TipoCuerpo,
    estacion_colorimetria: EstacionColorimetria,
    imagenes_outfit: ImagenesOutfit,
}

#[derive(Debug, Clone, Copy)]
struct OutfitPair {
    outfit_1: Outfit,
    outfit_2: Outfit,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImagenesOutfit {
    outfit_1: String,
    outfit_2: String,
}

fn recomendar_outfit(outfit: &Outfit) {
    println!("\n¡Recomendación de Outfit!\n");
    println!("Evento: {:?}", outfit.evento);
    println!("Código de Etiqueta: {:?}", outfit.codigo_etiqueta);
    println!("Hora del Evento: {:?}", outfit.hora_evento);
    println!("Tipo de Cuerpo: {:?}", outfit.tipo_cuerpo);
    println!("Estación-Colorimetría: {:?}\n", outfit.estacion_colorimetria);

    // Cargar el contenido del archivo JSON
    let mut file = File::open("src/decision.json").expect("Failed to open file");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Failed to read file");

    // Parsear el contenido JSON a la estructura DecisionData
    let decision_data: DecisionData = serde_json::from_str(&json_content).expect("Failed to parse JSON");

    // Buscar las condiciones que cumplen con el outfit
    let matched_conditions: Vec<&DecisionCondition> = decision_data.conditions.iter()
        .filter(|condition| condition.evento == outfit.evento
            && condition.codigo_etiqueta == outfit.codigo_etiqueta
            && condition.hora_evento == outfit.hora_evento
            && condition.tipo_cuerpo == outfit.tipo_cuerpo
            && condition.estacion_colorimetria == outfit.estacion_colorimetria)
        .collect();

    if matched_conditions.is_empty() {
        panic!("No se encontraron condiciones coincidentes para generar outfits recomendados.");
    }

    let imagenes_outfit = &matched_conditions[0].imagenes_outfit;

    println!("¡Recomendación de Outfit 1! Imagen: {}", imagenes_outfit.outfit_1);
    println!("¡Recomendación de Outfit 2! Imagen: {}", imagenes_outfit.outfit_2);
}


fn cargar_imagenes(cantidad: usize) -> Vec<String> {
    let carpeta = "static/outfits";
    let mut images = Vec::new();

    for i in 1..=cantidad {
        let ruta = format!("{}/{}.jpg", carpeta, i);
        images.push(ruta);
    }

    images
}

fn imprimir_outfit_images(imagenes_outfit: &ImagenesOutfit) {
    println!("Outfit 1: {}", imagenes_outfit.outfit_1);
    println!("Outfit 2: {}", imagenes_outfit.outfit_2);
}

fn main() {
    println!("¡Bienvenido al Selector de Outfit!\n");
    let outfits = cargar_imagenes(29);

    loop{
    // Obtener las selecciones del usuario
    let outfit_seleccionado = obtener_selecciones_del_usuario();

    recomendar_outfit(&outfit_seleccionado);

    //println!("¡Recomendación de Outfit 1!");
    //imprimir_outfit_images(&imagenes_outfit_recomendado);

    println!("Escribe 'Siguiente' para continuar o 'Salir' para salir.");
    let mut entrada = String::new();
        std::io::stdin().read_line(&mut entrada).expect("Error al leer la entrada.");

        let entrada = entrada.trim().to_lowercase();
        if entrada == "salir" {
            break;
        }
    }
}
