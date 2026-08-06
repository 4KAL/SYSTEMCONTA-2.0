use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::Cliente;
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, form_two_columns, texto_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ClienteFormData {
    pub codigo: String, pub nombre: String, pub rfc: String, pub email: String,
    pub telefono: String, pub direccion: String, pub ciudad: String, pub limite_credito: String,
}

impl Default for ClienteFormData {
    fn default() -> Self {
        Self { codigo: String::new(), nombre: String::new(), rfc: String::new(), email: String::new(),
            telefono: String::new(), direccion: String::new(), ciudad: String::new(), limite_credito: String::new() }
    }
}

#[derive(Debug, Clone)]
pub struct ClientesState {
    pub clientes: Vec<Cliente>, pub busqueda: String,
    pub show_form: bool, pub editing_id: Option<i64>, pub form: ClienteFormData,
    pub errores: HashMap<String, String>,
}

impl Default for ClientesState {
    fn default() -> Self {
        Self { clientes: Vec::new(), busqueda: String::new(), show_form: false, editing_id: None, form: ClienteFormData::default(), errores: HashMap::new() }
    }
}

#[derive(Debug, Clone)]
pub enum ClienteFormMessage {
    Codigo(String), Nombre(String), Rfc(String), Email(String), Telefono(String),
    Direccion(String), Ciudad(String), LimiteCredito(String), Guardar, Cancelar,
}

pub fn clientes_view<'a, Message: 'a + Clone>(
    state: &'a ClientesState,
    on_crear: Message, on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone, on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_form_msg: impl Fn(ClienteFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_cliente_form(state, on_form_msg); }

    let filtrados: Vec<&Cliente> = if state.busqueda.is_empty() {
        state.clientes.iter().filter(|c| c.activo).collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.clientes.iter().filter(|c| c.activo && (
            c.nombre.to_lowercase().contains(&q) || c.codigo.as_deref().unwrap_or("").to_lowercase().contains(&q) || c.rfc.to_lowercase().contains(&q)
        )).collect()
    };

    let header = row![
        text("Clientes").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar clientes...", &state.busqueda)
            .on_input(on_buscar)
            .style(|_, _| input_style())
            .width(220),
        button(text("+ Nuevo").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_crear)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|c| {
        let id = c.id;
        row![
            text(c.codigo.as_deref().unwrap_or("")).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            text(&c.nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&c.rfc).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&c.telefono).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&c.ciudad).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.0}", c.limite_credito)).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            row![
                button(text("\u{270E}").size(12))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ]
        .spacing(SPACING_SM)
        .align_y(Alignment::Center)
        .padding([SPACING_SM, SPACING_MD])
        .into()
    }).collect();

    let body: Element<'a, Message> = if filtrados.is_empty() {
        container(column![
            text("No hay clientes registrados").size(16).color(COLOR_TEXT_SECONDARY),
            text("Crea tu primer cliente para comenzar").size(12).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_SM).align_x(Alignment::Center))
        .center(Length::Fill).width(Length::Fill).height(300).into()
    } else {
        scrollable(column(rows).spacing(2.0).width(Length::Fill))
            .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
    };

    column![
        header, Space::new().height(Length::Fixed(SPACING_MD)), body,
    ]
    .padding(SPACING_LG)
    .spacing(SPACING_SM)
    .into()
}

fn render_cliente_form<'a, Message: 'a + Clone>(
    state: &'a ClientesState,
    on_form_msg: impl Fn(ClienteFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Cliente" } else { "Nuevo Cliente" };
    let mut fields: Vec<Element<'a, ClienteFormMessage>> = vec![
        form_two_columns(
            labeled_input("Código", &state.form.codigo, "CLI-001", ClienteFormMessage::Codigo),
            labeled_input("Nombre", &state.form.nombre, "Nombre del cliente", ClienteFormMessage::Nombre),
        ),
    ];
    if let Some(err) = texto_error("nombre", &state.errores) { fields.push(err); }
    fields.extend(vec![
        form_two_columns(
            labeled_input("RFC", &state.form.rfc, "XXXX000000XXX", ClienteFormMessage::Rfc),
            labeled_input("Email", &state.form.email, "cliente@email.com", ClienteFormMessage::Email),
        ),
        form_two_columns(
            labeled_input("Teléfono", &state.form.telefono, "555-123-4567", ClienteFormMessage::Telefono),
            labeled_input("Ciudad", &state.form.ciudad, "Ciudad", ClienteFormMessage::Ciudad),
        ),
        labeled_input("Dirección", &state.form.direccion, "Calle y número", ClienteFormMessage::Direccion),
        labeled_input_f64("Límite de Crédito", &state.form.limite_credito, "0.00", ClienteFormMessage::LimiteCredito),
    ]);

    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, ClienteFormMessage>| {
        let cb = fm_clone.clone();
        f.map(move |msg| cb(msg))
    };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn),
        Some(guardar(ClienteFormMessage::Guardar)),
        cancelar(ClienteFormMessage::Cancelar), "Guardar")
}
