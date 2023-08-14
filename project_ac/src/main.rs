/*
Interfaz para Seleccionar Vestimenta de acuerdo con un código de etiqueta
by Alisson Constantine M.
*/

// Enumeración para elegir el tipo de evento
#[derive(Debug, Clone, Copy)]
enum Evento {
    Boda,
    Bautizo,
    FiestaEmpresa,
    ReunionFamiliar,
}

// Enumeración para elegir el código de vestimenta
#[derive(Debug, Clone, Copy)]
enum CodigoEtiqueta {
    WhiteTie,
    BlackTie,
    Coctel,
    Semiformal,
    Casual,
}

// Enumeración para la Hora del Evento
#[derive(Debug, Clone, Copy)]
enum HoraEvento {
    Manana,
    Tarde,
    Noche,
}

// Enumeración para definir el tipo de cuerpo
#[derive(Debug, Clone, Copy)]
enum TipoCuerpo {
    TrianguloInvertido,
    RelojDeArena,
    Pera,
    Rectangular,
    Ovalo,
}


// Enumeración para la Estación y Colorimetría
#[derive(Debug, Clone, Copy)]
enum EstacionColorimetria {
    Verano,
    Otono,
    Invierno,
    Primavera,
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

fn main () {
    let mut x = 10; //mut (mutable) para que se pueda moficiar luego // let forma de definir la funcion
    println!("Hello, world!");
    println!("x is {}", x);
}
