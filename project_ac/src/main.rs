//use std::fs;
//use std::env;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
//use std::path::Path;
use image::{DynamicImage, open, GenericImageView};

use glfw::{Context, Key};
use gl::types::*;
use std::ptr;
use std::ffi::CString;

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
    Casual, // Considerar que no hay bodas casuales
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

fn recomendar_outfit(outfit: &Outfit, images: &Vec<DynamicImage>) {
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
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let imagenes_outfit = &matched_conditions[0].imagenes_outfit;
    let index_outfit_1 = imagenes_outfit.outfit_1.parse::<usize>().expect("Invalid image index");
    let index_outfit_2 = imagenes_outfit.outfit_2.parse::<usize>().expect("Invalid image index");

    let image_outfit_1_path = format!("static/outfits/{}.jpg", index_outfit_1);
    let image_outfit_2_path = format!("static/outfits/{}.jpg", index_outfit_2);
    
    let image_outfit_1 = open(image_outfit_1_path).expect("Failed to open image 1");
    let image_outfit_2 = open(image_outfit_2_path).expect("Failed to open image 2");
    
    // Inicializar GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

    // Crear ventana GLFW
    let (mut window, events) = glfw.create_window(1280, 720, "Outfit Recomendado", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // Hacer que el contexto de OpenGL sea actual
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Bucle principal
    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        // Limpiar el búfer de color
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Dibujar imagen 1
        draw_image(&image_outfit_1, -0.5);

        // Dibujar imagen 2
        draw_image(&image_outfit_2, 0.5);

        // Intercambiar el búfer de la ventana
        window.swap_buffers();

        // Esperar eventos
        glfw.poll_events();
    }
}

fn draw_image(image: &DynamicImage, x_offset: f32) {
    unsafe {
        gl::Enable(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        let mut texture_id: GLuint = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let (width, height) = image.dimensions();
        let data = image.to_rgba8().into_raw();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const std::ffi::c_void,
        );

        gl::BindTexture(gl::TEXTURE_2D, 0);

        let vertex_shader = CString::new(
            r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec2 aTexCoord;

            out vec2 TexCoord;

            void main()
            {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                TexCoord = vec2(aTexCoord.x, aTexCoord.y);
            }
            "#,
        )
        .unwrap();

        let fragment_shader = CString::new(
            r#"
            #version 330 core
            out vec4 FragColor;

            in vec2 TexCoord;

            uniform sampler2D ourTexture;

            void main()
            {
                FragColor = texture(ourTexture, TexCoord);
            }
            "#,
        )
        .unwrap();

        let shader_program = gl::CreateProgram();
        let vertex_shader = compile_shader(&vertex_shader, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(&fragment_shader, gl::FRAGMENT_SHADER);

        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);

        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        let vertices: [f32; 20] = [
            -0.5 + x_offset,  0.5, 0.0, 0.0, 1.0,  // top left
             0.5 + x_offset,  0.5, 0.0, 1.0, 1.0,  // top right
             0.5 + x_offset, -0.5, 0.0, 1.0, 0.0,  // bottom right
            -0.5 + x_offset, -0.5, 0.0, 0.0, 0.0   // bottom left 
        ];

        let mut vbo: GLuint = 0;
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

        let mut vao: GLuint = 0;
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as GLsizei, (3 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);

        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

fn compile_shader(source: &CString, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success: GLint = 1;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let error: CString = CString::new(vec![b' '; len as usize]).unwrap();
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", String::from_utf8_lossy(error.to_bytes()));
        }

        shader
    }
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

    //recomendar_outfit(&outfit_seleccionado);
    let images: Vec<DynamicImage> = cargar_imagenes(29).iter().map(|path| open(path).unwrap()).collect();
    recomendar_outfit(&outfit_seleccionado, &images);

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
