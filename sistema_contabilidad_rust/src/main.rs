mod app;
mod db;
mod models;
mod pdf;
mod theme;
mod ui;
mod voz;
mod widgets;
mod logo_data;

use iced::{window, Element, Task};
use app::{App, Message};
use iced::futures::SinkExt;

fn cargar_icono() -> Option<window::Icon> {
    window::icon::from_rgba(
        logo_data::LOGO_RGBA.to_vec(),
        logo_data::LOGO_WIDTH,
        logo_data::LOGO_HEIGHT,
    )
    .ok()
}

fn main() -> iced::Result {
    let icon = cargar_icono();
    iced::application(boot, update, view)
        .title(title_fn)
        .subscription(subscription)
        .exit_on_close_request(false)
        .window(window::Settings { icon, ..Default::default() })
        .run()
}

fn subscription(_state: &App) -> iced::Subscription<Message> {
    iced::window::close_requests().map(|_id| Message::CerrarSolicitado)
}

fn title_fn(state: &App) -> String {
    match state.fase {
        app::Fase::Presentacion => String::from("Sistema de Contabilidad"),
        app::Fase::Instalacion => String::from("Instalación - Sistema de Contabilidad"),
        app::Fase::Login => format!("Iniciar sesión - {}", state.empresa.nombre_corto()),
        app::Fase::Principal => state.empresa.nombre_corto(),
    }
}

fn boot() -> (App, Task<Message>) {
    let app = App::default();
    let mut tarea = Task::none();
    if app.fase == app::Fase::Presentacion {
        voz::asegurar_motor();
        voz::hablar(
            "Bienvenidos al sistema de contabilidad. Veo que es su primera vez instalándolo. Necesitaremos de sus datos para poder ajustarnos a sus necesidades.",
        );
        tarea = Task::run(
            iced::stream::channel(1, |mut sender: iced::futures::channel::mpsc::Sender<Message>| async move {
                tokio::time::sleep(std::time::Duration::from_secs(16)).await;
                let _ = sender.send(Message::FinPresentacion).await;
            }),
            |msg| msg,
        );
    }
    (app, tarea)
}

fn update(state: &mut App, message: Message) -> Task<Message> {
    app::update(state, message)
}

fn view(state: &App) -> Element<Message> {
    app::view(state)
}
