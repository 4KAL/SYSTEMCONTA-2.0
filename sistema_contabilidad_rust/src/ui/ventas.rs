use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Venta, VentaDetalle};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, labeled_input_i32, form_two_columns, pick_list_field, SelectOption};

#[derive(Debug, Clone)]
pub struct VentaItemData { pub producto_id: Option<i64>, pub producto_nombre: String, pub cantidad: String, pub precio: String }
#[derive(Debug, Clone)]
pub struct VentaFormData {
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub tipo: String,
    pub notas: String,
    pub iva: String,
    pub items: Vec<VentaItemData>,
    pub nuevo_cliente: bool,
    pub nc_nombre: String,
    pub nc_rfc: String,
    pub nc_telefono: String,
    pub nc_direccion: String,
    pub nc_ciudad: String,
    pub nc_email: String,
}
impl Default for VentaFormData {
    fn default() -> Self {
        Self {
            cliente_id: None, cliente_nombre: String::new(), tipo: "contado".to_string(),
            notas: String::new(), iva: "15".to_string(), items: vec![], nuevo_cliente: false,
            nc_nombre: String::new(), nc_rfc: String::new(), nc_telefono: String::new(),
            nc_direccion: String::new(), nc_ciudad: String::new(), nc_email: String::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VentasState {
    pub ventas: Vec<Venta>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub desde: String,
    pub hasta: String,
    pub form: VentaFormData,
    pub show_detail: bool,
    pub detail_lineas: Vec<VentaDetalle>,
    pub opciones_productos: Vec<SelectOption>,
    pub opciones_clientes: Vec<SelectOption>,
    pub opciones_tipo: Vec<SelectOption>,
}
impl Default for VentasState {
    fn default() -> Self {
        Self {
            ventas: Vec::new(),
            show_form: false,
            editing_id: None,
            busqueda: String::new(),
            desde: String::new(),
            hasta: String::new(),
            form: VentaFormData::default(),
            show_detail: false,
            detail_lineas: Vec::new(),
            opciones_productos: Vec::new(),
            opciones_clientes: Vec::new(),
            opciones_tipo: vec![
                SelectOption { id: 1, label: "contado".to_string() },
                SelectOption { id: 2, label: "credito".to_string() },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum VentaFormMessage {
    Tipo(String), Notas(String), Iva(String),
    ClienteSeleccionado(i64),
    NuevoClienteNombre(String), NuevoClienteRfc(String), NuevoClienteTelefono(String),
    NuevoClienteDireccion(String), NuevoClienteCiudad(String), NuevoClienteEmail(String),
    ItemProducto(usize, i64), ItemCantidad(usize, String), ItemPrecio(usize, String),
    AgregarItem, QuitarItem(usize), Guardar, Cancelar, CerrarDetalle,
}

pub fn ventas_view<'a, Message: 'a + Clone>(
    state: &'a VentasState,
    on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_factura: impl Fn(i64) -> Message + 'a + Clone,
    on_xml: impl Fn(i64) -> Message + 'a + Clone,
    on_garantia: impl Fn(i64) -> Message + 'a + Clone,
    on_abonar: impl Fn(i64) -> Message + 'a + Clone,
    on_form_msg: impl Fn(VentaFormMessage) -> Message + 'a + Clone,
    on_desde: impl Fn(String) -> Message + 'a,
    on_hasta: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    if state.show_detail { return render_detalle_modal(state, on_form_msg); }
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&Venta> = state.ventas.iter().filter(|v| {
        if !state.busqueda.is_empty() {
            let q = state.busqueda.to_lowercase();
            if !v.cliente_nombre.to_lowercase().contains(&q) { return false; }
        }
        if !state.desde.is_empty() {
            if v.fecha.as_str() < state.desde.as_str() { return false; }
        }
        if !state.hasta.is_empty() {
            if v.fecha.as_str() > state.hasta.as_str() { return false; }
        }
        true
    }).collect();

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|v| {
        let id = v.id;
        let ed = on_editar.clone();
        let el = on_eliminar.clone();
        let det = on_ver_detalle.clone();
        let fac = on_factura.clone();
        let xml = on_xml.clone();
        let gar = on_garantia.clone();
        let ab = on_abonar.clone();
        let mut acciones: Vec<Element<'a, Message>> = Vec::new();
        if v.saldo_pendiente > 0.01 {
            acciones.push(
                button(text("Abonar").size(10).color(COLOR_TEXT_PRIMARY))
                    .style(|_, _| primary_button_style())
                    .on_press(ab(id)).padding([4, 8]).into()
            );
        }
        acciones.push(button(text("\u{2709}").size(12)).style(|_, _| ghost_button_style()).on_press(fac(id)).padding([4, 6]).into());
        acciones.push(button(text("<XML>").size(10).color(COLOR_INFO)).style(|_, _| ghost_button_style()).on_press(xml(id)).padding([4, 6]).into());
        acciones.push(button(text("\u{2605}").size(12)).style(|_, _| ghost_button_style()).on_press(gar(id)).padding([4, 6]).into());
        acciones.push(button(text("\u{1F441}").size(12)).style(|_, _| ghost_button_style()).on_press(det(id)).padding([4, 6]).into());
        acciones.push(button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(ed(id)).padding([4, 6]).into());
        acciones.push(button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(el(id)).padding([4, 6]).into());
        row![
            text(&v.folio).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&v.cliente_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&v.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.2}", v.total)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(if v.saldo_pendiente > 0.01 { format!("${:.2}", v.saldo_pendiente) } else { "pagado".to_string() })
                .size(12)
                .color(if v.saldo_pendiente > 0.01 { COLOR_DANGER } else { COLOR_SUCCESS })
                .width(Length::FillPortion(1)),
            text(&v.tipo).size(11).color(if v.tipo == "credito" { COLOR_CXC } else { COLOR_SUCCESS }).width(Length::FillPortion(1)),
            text(&v.estado).size(11).color(if v.estado == "cancelada" { COLOR_DANGER } else { COLOR_SUCCESS }).width(Length::FillPortion(1)),
            row(acciones).spacing(SPACING_XS).width(Length::FillPortion(2)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        row![
            text("Ventas").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text_input("Buscar cliente...", &state.busqueda)
                .on_input(on_buscar).style(|_, _| input_style()).width(180),
            text_input("Desde (YYYY-MM-DD)", &state.desde)
                .on_input(on_desde).style(|_, _| input_style()).width(150),
            text_input("Hasta", &state.hasta)
                .on_input(on_hasta).style(|_, _| input_style()).width(130),
            button(text("+ Nueva Venta").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style()).on_press(on_nueva).padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a VentasState,
    on_form_msg: impl Fn(VentaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Venta" } else { "Nueva Venta" };
    let tipo_id = if state.form.tipo == "credito" { 2 } else { 1 };
    let cliente_id = if state.form.nuevo_cliente { 0 } else { state.form.cliente_id.unwrap_or(0) };
    let mut fields: Vec<Element<'a, VentaFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Cliente", &state.opciones_clientes, cliente_id, move |id| VentaFormMessage::ClienteSeleccionado(id)),
            pick_list_field("Tipo", &state.opciones_tipo, tipo_id, move |id| VentaFormMessage::Tipo(if id == 2 { "credito" } else { "contado" }.to_string())),
        ),
    ];
    if state.form.nuevo_cliente {
        fields.push(
            container(column![
                text("Nuevo Cliente").size(12).color(COLOR_ACCENT),
                form_two_columns(labeled_input("Nombre", &state.form.nc_nombre, "Nombre completo", VentaFormMessage::NuevoClienteNombre), labeled_input("Cedula / RIF", &state.form.nc_rfc, "V-12345678", VentaFormMessage::NuevoClienteRfc)),
                form_two_columns(labeled_input("Telefono", &state.form.nc_telefono, "0414-0000000", VentaFormMessage::NuevoClienteTelefono), labeled_input("Email", &state.form.nc_email, "correo@mail.com", VentaFormMessage::NuevoClienteEmail)),
                form_two_columns(labeled_input("Direccion", &state.form.nc_direccion, "Direccion", VentaFormMessage::NuevoClienteDireccion), labeled_input("Ciudad", &state.form.nc_ciudad, "Ciudad", VentaFormMessage::NuevoClienteCiudad)),
            ].spacing(SPACING_SM).padding(SPACING_MD)).style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color { a: 0.08, ..COLOR_ACCENT })),
                border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
                text_color: Some(COLOR_TEXT_PRIMARY),
                snap: false,
                shadow: iced::Shadow::default(),
            }).width(Length::Fill).into(),
        );
    }
    fields.push(form_two_columns(
        labeled_input_f64("IVA (%) - por si el IVA sube o baja", &state.form.iva, "15.00", move |v| VentaFormMessage::Iva(v)),
        labeled_input("Notas", &state.form.notas, "Notas de la venta", move |v| VentaFormMessage::Notas(v)),
    ));
    fields.push(text("Productos de la venta").size(12).color(COLOR_ACCENT).into());
    for (i, item) in state.form.items.iter().enumerate() {
        let campo_producto: Element<'a, VentaFormMessage> = container(
            pick_list_field("Producto", &state.opciones_productos, item.producto_id.unwrap_or(0), move |id| VentaFormMessage::ItemProducto(i, id))
        ).width(Length::FillPortion(3)).into();
        let campo_cantidad: Element<'a, VentaFormMessage> = container(
            labeled_input_i32("Cantidad", &item.cantidad, "1", move |v| VentaFormMessage::ItemCantidad(i, v))
        ).width(Length::FillPortion(1)).into();
        let campo_precio: Element<'a, VentaFormMessage> = container(
            labeled_input_f64("Precio unitario ($)", &item.precio, "0.00", move |v| VentaFormMessage::ItemPrecio(i, v))
        ).width(Length::FillPortion(1)).into();
        let campo_quitar: Element<'a, VentaFormMessage> = container(
            column![
                Space::new().height(Length::Fixed(14.0)),
                button(text("\u{2715}").size(10).color(COLOR_DANGER)).style(|_, _| ghost_button_style()).on_press(VentaFormMessage::QuitarItem(i)).padding([4, 6]),
            ]
        ).width(Length::Fixed(40.0)).into();
        fields.push(
            row![
                text(format!("{}.", i+1)).size(11).color(COLOR_TEXT_MUTED).width(Length::Fixed(20.0)),
                campo_producto,
                campo_cantidad,
                campo_precio,
                campo_quitar,
            ].spacing(SPACING_XS).align_y(Alignment::Center).into()
        );
    }
    fields.push(button(text("+ Agregar Producto").size(12).color(COLOR_ACCENT)).style(|_, _| secondary_button_style()).on_press(VentaFormMessage::AgregarItem).padding(SPACING_SM).into());
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, VentaFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(VentaFormMessage::Guardar)), cancelar(VentaFormMessage::Cancelar), "Guardar Venta")
}

fn render_detalle_modal<'a, Message: 'a + Clone>(
    state: &'a VentasState,
    on_form_msg: impl Fn(VentaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let venta = state.ventas.iter().find(|v| {
        state.detail_lineas.first().map(|d| d.venta_id == v.id).unwrap_or(false)
    });

    let mut fields: Vec<Element<'a, VentaFormMessage>> = Vec::new();

    if let Some(v) = venta {
        fields.push(
            row![
                column![
                    text("Folio").size(10).color(COLOR_TEXT_MUTED),
                    text(&v.folio).size(14).color(COLOR_ACCENT),
                ].spacing(2).width(Length::FillPortion(1)),
                column![
                    text("Cliente").size(10).color(COLOR_TEXT_MUTED),
                    text(&v.cliente_nombre).size(14).color(COLOR_TEXT_PRIMARY),
                ].spacing(2).width(Length::FillPortion(2)),
                column![
                    text("Fecha").size(10).color(COLOR_TEXT_MUTED),
                    text(&v.fecha).size(14).color(COLOR_TEXT_PRIMARY),
                ].spacing(2).width(Length::FillPortion(1)),
                column![
                    text("Total").size(10).color(COLOR_TEXT_MUTED),
                    text(format!("${:.2}", v.total)).size(14).color(COLOR_VENTAS),
                ].spacing(2).width(Length::FillPortion(1)),
            ].spacing(SPACING_MD).align_y(Alignment::Center).into()
        );

        fields.push(Space::new().height(Length::Fixed(SPACING_SM)).into());

        fields.push(
            row![
                text("Producto").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
                text("Cantidad").size(10).color(COLOR_TEXT_MUTED).width(Length::Fixed(60.0)),
                text("Precio Unitario").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                text("Descuento").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                text("Importe").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_MD]).into()
        );

        for det in &state.detail_lineas {
            fields.push(
                row![
                    text(&det.producto_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(format!("{}", det.cantidad)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::Fixed(60.0)),
                    text(format!("${:.2}", det.precio_unitario)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
                    text(format!("${:.2}", det.descuento)).size(12).color(COLOR_DANGER).width(Length::FillPortion(1)),
                    text(format!("${:.2}", det.importe)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
                ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_MD]).into()
            );
        }
    } else {
        fields.push(text("Seleccione una venta para ver el detalle").size(14).color(COLOR_TEXT_SECONDARY).into());
    }

    let cerrar = on_form_msg.clone();
    let map_fn = move |f: Element<'a, VentaFormMessage>| { let cb = cerrar.clone(); f.map(move |_| cb(VentaFormMessage::CerrarDetalle)) };
    let cancel = on_form_msg;
    form_card("Detalle de Venta", fields.into_iter().map(map_fn), None, cancel(VentaFormMessage::CerrarDetalle), "")
}
