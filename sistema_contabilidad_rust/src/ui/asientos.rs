use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Asiento, AsientoLinea};
use crate::theme::*;
use super::forms::{form_card, labeled_input, pick_list_field, SelectOption, form_two_columns, texto_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AsientoLineaData { pub cuenta_id: String, pub cuenta_nombre: String, pub descripcion: String, pub debe: String, pub haber: String }
#[derive(Debug, Clone)]
pub struct AsientoFormData { pub fecha: String, pub concepto: String, pub descripcion: String, pub referencia: String, pub lineas: Vec<AsientoLineaData> }
impl Default for AsientoFormData {
    fn default() -> Self { Self { fecha: chrono::Local::now().format("%Y-%m-%d").to_string(), concepto: String::new(), descripcion: String::new(), referencia: String::new(), lineas: vec![] } }
}
#[derive(Debug, Clone)]
pub struct AsientosState {
    pub asientos: Vec<Asiento>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub desde: String,
    pub hasta: String,
    pub show_detail: bool,
    pub detail_lineas: Vec<AsientoLinea>,
    pub form: AsientoFormData,
    pub opciones_cuentas: Vec<SelectOption>,
    pub errores: HashMap<String, String>,
}
impl Default for AsientosState {
    fn default() -> Self {
        Self {
            asientos: Vec::new(), show_form: false, editing_id: None,
            busqueda: String::new(), desde: String::new(), hasta: String::new(),
            show_detail: false, detail_lineas: Vec::new(),
            form: AsientoFormData::default(), opciones_cuentas: Vec::new(), errores: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AsientoFormMessage {
    Fecha(String), Concepto(String), Descripcion(String), Referencia(String),
    LineaCuenta(usize, String), LineaDescripcion(usize, String), LineaDebe(usize, String), LineaHaber(usize, String),
    AgregarLinea, QuitarLinea(usize), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub enum AsientoMessage {
    Editar(i64), Eliminar(i64), Buscar(String), VerDetalle(i64), CerrarDetalle,
}

pub fn asientos_view<'a, Message: 'a + Clone>(
    state: &'a AsientosState,
    on_nuevo: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_cerrar_detalle: Message,
    on_form_msg: impl Fn(AsientoFormMessage) -> Message + 'a + Clone,
    on_desde: impl Fn(String) -> Message + 'a,
    on_hasta: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    if state.show_detail {
        return render_detalle(state, on_cerrar_detalle);
    }
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&Asiento> = state.asientos.iter().filter(|a| {
        if !state.busqueda.is_empty() {
            let q = state.busqueda.to_lowercase();
            if !a.concepto.to_lowercase().contains(&q) { return false; }
        }
        if !state.desde.is_empty() && a.fecha.as_str() < state.desde.as_str() { return false; }
        if !state.hasta.is_empty() && a.fecha.as_str() > state.hasta.as_str() { return false; }
        true
    }).collect();

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|a| {
        let id = a.id;
        row![
            text(a.numero.as_deref().unwrap_or("")).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&a.fecha).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(2)),
            text(&a.concepto).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.2}", a.total_debe)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(format!("${:.2}", a.total_haber)).size(12).color(COLOR_GASTOS).width(Length::FillPortion(1)),
            text(&a.estado).size(11).color(if a.estado == "cancelado" { COLOR_DANGER } else { COLOR_SUCCESS }).width(Length::FillPortion(1)),
            row![
                button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{1F441}").size(12)).style(|_, _| ghost_button_style()).on_press(on_ver_detalle(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        row![
            text("Asientos Contables").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text_input("Buscar...", &state.busqueda).on_input(on_buscar).style(|_, _| input_style()).width(140),
            text_input("Desde", &state.desde).on_input(on_desde).style(|_, _| input_style()).width(120),
            text_input("Hasta", &state.hasta).on_input(on_hasta).style(|_, _| input_style()).width(120),
            button(text("+ Nuevo Asiento").size(13).color(COLOR_TEXT_PRIMARY)).style(|_, _| primary_button_style()).on_press(on_nuevo).padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}

fn render_detalle<'a, Message: 'a + Clone>(
    state: &'a AsientosState,
    on_cerrar: Message,
) -> Element<'a, Message> {
    let asiento_id = state.detail_lineas.first().map(|l| l.asiento_id);
    let asiento = asiento_id.and_then(|id| state.asientos.iter().find(|a| a.id == id));

    let mut content: Vec<Element<'a, Message>> = vec![
        row![
            text("Detalle de Asiento").size(20).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            button(text("\u{2715}").size(14).color(COLOR_TEXT_MUTED))
                .style(|_, _| ghost_button_style())
                .on_press(on_cerrar)
                .padding([4, 8]),
        ].align_y(Alignment::Center).into(),
        Space::new().height(Length::Fixed(SPACING_MD)).into(),
    ];

    if let Some(a) = asiento {
        content.push(row![
            text("N\u{FA}mero:").size(12).color(COLOR_TEXT_MUTED),
            text(a.numero.as_deref().unwrap_or("-")).size(12).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text("Fecha:").size(12).color(COLOR_TEXT_MUTED),
            text(&a.fecha).size(12).color(COLOR_TEXT_PRIMARY),
        ].spacing(SPACING_SM).into());
        content.push(row![
            text("Concepto:").size(12).color(COLOR_TEXT_MUTED),
            text(&a.concepto).size(12).color(COLOR_TEXT_PRIMARY),
        ].spacing(SPACING_SM).into());
        content.push(Space::new().height(Length::Fixed(SPACING_SM)).into());
    }

    content.push(row![
        text("C\u{FA}digo").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Nombre").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Descripci\u{F3}n").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Debe").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Haber").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
    ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_SM]).into());

    let mut total_debe = 0.0;
    let mut total_haber = 0.0;
    for linea in &state.detail_lineas {
        total_debe += linea.debe;
        total_haber += linea.haber;
        content.push(row![
            text(&linea.cuenta_codigo).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&linea.cuenta_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(linea.descripcion.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(3)),
            text(format!("${:.2}", linea.debe)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(format!("${:.2}", linea.haber)).size(12).color(COLOR_GASTOS).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_SM]).into());
    }

    content.push(Space::new().height(Length::Fixed(SPACING_SM)).into());
    content.push(row![
        Space::new().width(Length::Fill),
        text("Total Debe:").size(12).color(COLOR_TEXT_MUTED),
        text(format!("${:.2}", total_debe)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
        text("Total Haber:").size(12).color(COLOR_TEXT_MUTED),
        text(format!("${:.2}", total_haber)).size(12).color(COLOR_GASTOS).width(Length::FillPortion(1)),
    ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_SM]).into());

    let col = column(content).spacing(SPACING_SM);
    container(
        container(col.padding(SPACING_LG))
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(COLOR_CARD)),
                border: iced::Border { radius: RADIUS_XL.into(), width: 1.0, color: COLOR_BORDER },
                text_color: Some(COLOR_TEXT_PRIMARY),
                snap: false,
                shadow: SHADOW_CARD,
            })
            .width(600)
            .max_width(700),
    )
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color { a: 0.7, ..COLOR_BG })),
        border: iced::Border::default(),
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: iced::Shadow::default(),
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .center(Length::Fill)
    .into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a AsientosState,
    on_form_msg: impl Fn(AsientoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Asiento Contable" } else { "Nuevo Asiento Contable" };
    let mut fields: Vec<Element<'a, AsientoFormMessage>> = vec![
        form_two_columns(labeled_input("Fecha", &state.form.fecha, "2024-01-01", AsientoFormMessage::Fecha), labeled_input("Concepto", &state.form.concepto, "Concepto del asiento", AsientoFormMessage::Concepto)),
    ];
    if let Some(err) = texto_error("concepto", &state.errores) { fields.push(err); }
    fields.extend(vec![
        form_two_columns(labeled_input("Descripción", &state.form.descripcion, "Descripción (opcional)", AsientoFormMessage::Descripcion), labeled_input("Referencia", &state.form.referencia, "Documento de referencia", AsientoFormMessage::Referencia)),
    ]);
    let mut total_debe = 0.0;
    let mut total_haber = 0.0;
    for (i, item) in state.form.lineas.iter().enumerate() {
        let d: f64 = item.debe.parse().unwrap_or(0.0);
        let h: f64 = item.haber.parse().unwrap_or(0.0);
        total_debe += d;
        total_haber += h;
        let cuenta_id: i64 = item.cuenta_id.parse().unwrap_or(0);
        fields.push(
            row![
                text(format!("{}.", i+1)).size(11).color(COLOR_TEXT_MUTED).width(Length::Fixed(20.0)),
                pick_list_field("", &state.opciones_cuentas, cuenta_id, move |id| AsientoFormMessage::LineaCuenta(i, id.to_string())),
                text_input("Desc", &item.descripcion).on_input(move |v| AsientoFormMessage::LineaDescripcion(i, v)).style(|_, _| input_style()).width(Length::FillPortion(2)),
                text_input("Debe", &item.debe).on_input(move |v| AsientoFormMessage::LineaDebe(i, v)).style(|_, _| input_style()).width(Length::FillPortion(1)),
                text_input("Haber", &item.haber).on_input(move |v| AsientoFormMessage::LineaHaber(i, v)).style(|_, _| input_style()).width(Length::FillPortion(1)),
                button(text("\u{2715}").size(10).color(COLOR_DANGER)).style(|_, _| ghost_button_style()).on_press(AsientoFormMessage::QuitarLinea(i)).padding([4, 6]),
            ].spacing(SPACING_XS).align_y(Alignment::Center).into()
        );
    }
    fields.push(row![
        Space::new().width(Length::Fill),
        text(if (total_debe - total_haber).abs() < 0.01 { format!("Debe: ${:.2}  Haber: ${:.2}  \u{2713}", total_debe, total_haber) } else { format!("Debe: ${:.2}  Haber: ${:.2}  \u{26A0} Diferencia: ${:.2}", total_debe, total_haber, (total_debe - total_haber).abs()) }).size(12).color(if (total_debe - total_haber).abs() < 0.01 { COLOR_SUCCESS } else { COLOR_DANGER }),
    ].into());
    fields.push(button(text("+ Agregar Línea").size(12).color(COLOR_ACCENT)).style(|_, _| secondary_button_style()).on_press(AsientoFormMessage::AgregarLinea).padding(SPACING_SM).into());
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, AsientoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(AsientoFormMessage::Guardar)), cancelar(AsientoFormMessage::Cancelar), "Guardar Asiento")
}
