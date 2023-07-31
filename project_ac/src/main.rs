/*
Rust Project
by Alisson Constantine M.
*/

//main function always to top
/*
fn main () {
    let mut x = 10; //mut (mutable) para que se pueda moficiar luego // let forma de definir la funcion
    println!("Hello, world!");
    println!("x is {}", x);
    x=20;
    println!("x is {}", x);
}*/

use iced::{button, Align, Application, Button, Column, Command, Element, Settings, Text};

#[derive(Debug, Clone, Copy)]
enum Evento {
    Boda,
    Bautizo,
    FiestaEmpresa,
    ReunionFamiliar,
}

// Agregar otras enumeraciones para el resto de las características a elegir.

#[derive(Debug, Clone, Copy)]
enum Message {
    EventoSeleccionado(Evento),
    // Agregar otras variantes de mensajes para el resto de las características.
}

#[derive(Default)]
struct OutfitApp {
    // Aquí agregar campos para almacenar las elecciones del usuario.
}

impl Application for OutfitApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (OutfitApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Outfit Selector")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut iced::Clipboard) -> Command<Self::Message> {
        match message {
            Message::EventoSeleccionado(evento) => {
                // Aquí implementarías la lógica para manejar la selección del evento
            }
            // Implementar las otras variantes de mensajes para el resto de las características.
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let evento_selector = {
            let eventos = [
                (Evento::Boda, "Boda"),
                (Evento::Bautizo, "Bautizo"),
                (Evento::FiestaEmpresa, "Fiesta de la Empresa"),
                (Evento::ReunionFamiliar, "Reunión Familiar"),
            ];

            let options = eventos
                .iter()
                .map(|(evento, label)| iced::widget::PickList::option(*evento, label));

            iced::widget::pick_list::PickList::new(
                &mut self.evento_selector,
                &options.collect::<Vec<_>>(),
                Some(self.evento),
                Message::EventoSeleccionado,
            )
            .text_size(20)
        };

        // Agregar los demás componentes (pick_lists) para el resto de las características a elegir.

        let content = Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Selecciona las características de tu outfit:").size(24))
            .push(evento_selector);

        // Agregar el resto de componentes a la columna (content).

        // Agregar el estilo y diseño para la interfaz gráfica
        content.into()
    }
}

fn main() {
    OutfitApp::run(Settings::default());
}
