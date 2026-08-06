use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::Proveedor;
use crate::theme::*;
use super::forms::{form_card, labeled_input, form_two_columns, texto_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProveedorFormData {
    pub codigo: String, pub nombre: String, pub contacto: String, pub rfc: String,
    pub email: String, pub telefono: String, pub direccion: String,
}

impl Default for ProveedorFormData {
    fn default() -> Self {
        Self { codigo: String::new(), nombre: String::new(), contacto: String::new(),
            rfc: String::new(), email: String::new(), telefono: String::new(), direccion: String::new() }
    }
}

#[derive(Debug, Clone)]
pub struct ProveedoresState {
    pub proveedores: Vec<Proveedor>, pub busqueda: String,
    pub show_form: bool, pub editing_id: Option<i64>, pub form: ProveedorFormData,
    pub errores: HashMap<String, String>,
}

impl Default for ProveedoresState {
    fn default() -> Self {
        Self { proveedores: Vec::new(), busqueda: String::new(), show_form: false, editing_id: None, form: ProveedorFormData::default(), errores: HashMap::new() }
    }
}

#[derive(Debug, Clone)]
pub enum ProveedorFormMessage {
    Codigo(String), Nombre(String), Contacto(String), Rfc(String),
    Email(String), Telefono(String), Direccion(String), Guardar, Cancelar,
}

pub fn proveedores_view<'a, Message: 'a + Clone>(
    state: &'a ProveedoresState,
    on_crear: Message, on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone, on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_form_msg: impl Fn(ProveedorFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&Proveedor> = if state.busqueda.is_empty() {
        state.proveedores.iter().filter(|p| p.activo).collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.proveedores.iter().filter(|p| p.activo && (
            p.nombre.to_lowercase().contains(&q) || p.codigo.as_deref().unwrap_or("").to_lowercase().contains(&q)
        )).collect()
    };

    let header = row![
        text("Proveedores").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar proveedores...", &state.busqueda)
            .on_input(on_buscar).style(|_, _| input_style()).width(220),
        button(text("+ Nuevo").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style()).on_press(on_crear).padding([SPACING_SM, SPACING_MD]),
    ].spacing(SPACING_MD).align_y(Alignment::Center);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|p| {
        let id = p.id;
        row![
            text(p.codigo.as_deref().unwrap_or("")).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            text(&p.nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&p.contacto).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&p.telefono).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.0}", p.saldo_pendiente)).size(12).color(COLOR_CXP).width(Length::FillPortion(1)),
            row![
                button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        header, Space::new().height(Length::Fixed(SPACING_MD)),
        scrollable(column(rows).spacing(2.0).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].padding(SPACING_LG).spacing(SPACING_SM).into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a ProveedoresState,
    on_form_msg: impl Fn(ProveedorFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Proveedor" } else { "Nuevo Proveedor" };
    let mut fields: Vec<Element<'a, ProveedorFormMessage>> = vec![
        form_two_columns(labeled_input("Código", &state.form.codigo, "PROV-001", ProveedorFormMessage::Codigo), labeled_input("Nombre", &state.form.nombre, "Nombre del proveedor", ProveedorFormMessage::Nombre)),
    ];
    if let Some(err) = texto_error("nombre", &state.errores) { fields.push(err); }
    fields.extend(vec![
        form_two_columns(labeled_input("Contacto", &state.form.contacto, "Nombre del contacto", ProveedorFormMessage::Contacto), labeled_input("RFC", &state.form.rfc, "XXXX000000XXX", ProveedorFormMessage::Rfc)),
        form_two_columns(labeled_input("Email", &state.form.email, "proveedor@email.com", ProveedorFormMessage::Email), labeled_input("Teléfono", &state.form.telefono, "555-123-4567", ProveedorFormMessage::Telefono)),
        labeled_input("Dirección", &state.form.direccion, "Calle y número", ProveedorFormMessage::Direccion),
    ]);
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, ProveedorFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(ProveedorFormMessage::Guardar)), cancelar(ProveedorFormMessage::Cancelar), "Guardar")
}
