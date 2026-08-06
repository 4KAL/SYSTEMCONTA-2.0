use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::Garantia;
use crate::theme::*;
use super::forms::{form_card, labeled_input, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct GarantiaFormData { pub producto_id: String, pub venta_id: String, pub fecha_inicio: String, pub fecha_fin: String, pub descripcion: String }
impl Default for GarantiaFormData { fn default() -> Self { Self { producto_id: String::new(), venta_id: String::new(), fecha_inicio: String::new(), fecha_fin: String::new(), descripcion: String::new() } } }
#[derive(Debug, Clone)]
pub struct GarantiasState { pub garantias: Vec<Garantia>, pub busqueda: String, pub show_form: bool, pub editing_id: Option<i64>, pub form: GarantiaFormData, pub opciones_productos: Vec<SelectOption> }
impl Default for GarantiasState { fn default() -> Self { Self { garantias: Vec::new(), busqueda: String::new(), show_form: false, editing_id: None, form: GarantiaFormData::default(), opciones_productos: Vec::new() } } }

#[derive(Debug, Clone)]
pub enum GarantiaFormMessage { ProductoId(String), VentaId(String), FechaInicio(String), FechaFin(String), Descripcion(String), Guardar, Cancelar }

pub fn garantias_view<'a, Message: 'a + Clone>(
    state: &'a GarantiasState,
    on_nueva: Message, on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone, on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_form_msg: impl Fn(GarantiaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&Garantia> = if state.busqueda.is_empty() {
        state.garantias.iter().collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.garantias.iter().filter(|g|
            g.cliente_nombre.to_lowercase().contains(&q) || g.producto_nombre.to_lowercase().contains(&q)
        ).collect()
    };

    let header = row![
        text("Garantías").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar garantías...", &state.busqueda)
            .on_input(on_buscar)
            .style(|_, _| input_style())
            .width(220),
        button(text("+ Nueva").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nueva)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|g| {
        let id = g.id;
        row![
            text(&g.producto_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&g.cliente_nombre).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&g.fecha_inicio).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text(&g.fecha_fin).size(10).color(if g.activa { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(2)),
            text(if g.activa { "Activa" } else { "Vencida" }).size(11).color(if g.activa { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(1)),
            row![
                button(text("\u{270E}").size(12))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    let body: Element<'a, Message> = if filtrados.is_empty() {
        container(column![
            text("No hay garantías registradas").size(16).color(COLOR_TEXT_SECONDARY),
            text("Crea una nueva garantía para comenzar").size(12).color(COLOR_TEXT_MUTED),
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

fn render_form<'a, Message: 'a + Clone>(
    state: &'a GarantiasState, on_form_msg: impl Fn(GarantiaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Garantía" } else { "Nueva Garantía" };
    let prod_id: i64 = state.form.producto_id.parse().unwrap_or(0);
    let fields: Vec<Element<'a, GarantiaFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Producto", &state.opciones_productos, prod_id, |id| GarantiaFormMessage::ProductoId(id.to_string())),
            labeled_input("Venta ID", &state.form.venta_id, "1", GarantiaFormMessage::VentaId),
        ),
        form_two_columns(labeled_input("Fecha Inicio", &state.form.fecha_inicio, "2024-01-01", GarantiaFormMessage::FechaInicio), labeled_input("Fecha Fin", &state.form.fecha_fin, "2025-01-01", GarantiaFormMessage::FechaFin)),
        labeled_input("Descripción", &state.form.descripcion, "Descripción", GarantiaFormMessage::Descripcion),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, GarantiaFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone(); let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(GarantiaFormMessage::Guardar)), cancelar(GarantiaFormMessage::Cancelar), "Guardar")
}
