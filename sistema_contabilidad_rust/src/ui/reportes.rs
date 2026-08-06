use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{PlanCuentas, Asiento, EstadoResultados, MayorLinea, SaldoPendiente, LibroComprasLinea, LibroVentasLinea, ResumenAts};
use crate::theme::*;
use super::forms::SelectOption;

#[derive(Debug, Clone)]
pub struct ReportesState {
    pub tab: ReporteTab,
    pub libro_diario: Vec<Asiento>,
    pub balance: Vec<PlanCuentas>,
    pub balance_debe: Vec<f64>,
    pub balance_haber: Vec<f64>,
    pub balance_resumen: Option<(f64, f64, f64, f64, f64, f64, f64)>,
    pub resultado: Option<EstadoResultados>,
    pub mayor: Vec<MayorLinea>,
    pub mayor_cuenta: i64,
    pub opciones_cuentas: Vec<SelectOption>,
    pub antiguedad_cxc: Vec<SaldoPendiente>,
    pub antiguedad_cxp: Vec<SaldoPendiente>,
    pub libro_compras: Vec<LibroComprasLinea>,
    pub libro_ventas: Vec<LibroVentasLinea>,
    pub resumen_ats: ResumenAts,
    pub desde: String,
    pub hasta: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReporteTab {
    LibroDiario,
    BalanceGeneral,
    Comprobacion,
    EstadoResultados,
    LibroMayor,
    Antiguedad,
    LibroCompras,
    LibroVentas,
    Ats,
}

impl Default for ReportesState {
    fn default() -> Self {
        Self {
            tab: ReporteTab::LibroDiario, libro_diario: Vec::new(),
            balance: Vec::new(), balance_debe: Vec::new(), balance_haber: Vec::new(),
            balance_resumen: None, resultado: None, mayor: Vec::new(), mayor_cuenta: 0,
            opciones_cuentas: Vec::new(), antiguedad_cxc: Vec::new(), antiguedad_cxp: Vec::new(),
            libro_compras: Vec::new(), libro_ventas: Vec::new(), resumen_ats: ResumenAts::default(),
            desde: String::new(), hasta: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReportesMessage {
    Tab(ReporteTab),
    Desde(String),
    Hasta(String),
    Generar,
    MayorCuenta(i64),
    ExportarAts,
}

fn tab_btn<'a, Message: 'a + Clone>(
    label: &'static str,
    active: bool,
    tab: ReporteTab,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> iced::widget::Button<'a, Message> {
    let m = on_msg;
    button(text(label).size(12))
        .style(move |_, _| if active { primary_button_style() } else { ghost_button_style() })
        .on_press(m(ReportesMessage::Tab(tab)))
}

pub fn reportes_view<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let t1 = on_msg.clone();
    let t2 = on_msg.clone();
    let t3 = on_msg.clone();
    let t4 = on_msg.clone();
    let t5 = on_msg.clone();
    let t6 = on_msg.clone();
    let t7 = on_msg.clone();
    let t8 = on_msg.clone();
    let t9 = on_msg.clone();
    let tabs = row![
        tab_btn("Libro Diario", state.tab == ReporteTab::LibroDiario, ReporteTab::LibroDiario, t1),
        tab_btn("Balance General", state.tab == ReporteTab::BalanceGeneral, ReporteTab::BalanceGeneral, t2),
        tab_btn("Comprobación", state.tab == ReporteTab::Comprobacion, ReporteTab::Comprobacion, t3),
        tab_btn("Estado de Resultados", state.tab == ReporteTab::EstadoResultados, ReporteTab::EstadoResultados, t4),
        tab_btn("Libro Mayor", state.tab == ReporteTab::LibroMayor, ReporteTab::LibroMayor, t5),
        tab_btn("Antigüedad de Saldos", state.tab == ReporteTab::Antiguedad, ReporteTab::Antiguedad, t6),
        tab_btn("Libro Compras", state.tab == ReporteTab::LibroCompras, ReporteTab::LibroCompras, t7),
        tab_btn("Libro Ventas", state.tab == ReporteTab::LibroVentas, ReporteTab::LibroVentas, t8),
        tab_btn("ATS", state.tab == ReporteTab::Ats, ReporteTab::Ats, t9),
    ].spacing(SPACING_SM);

    let content: Element<'a, Message> = match state.tab {
        ReporteTab::LibroDiario => render_libro_diario(state, on_msg),
        ReporteTab::BalanceGeneral => render_balance_resumen(state, on_msg),
        ReporteTab::Comprobacion => render_comprobacion(state, on_msg),
        ReporteTab::EstadoResultados => render_resultados(state, on_msg),
        ReporteTab::LibroMayor => render_mayor(state, on_msg),
        ReporteTab::Antiguedad => render_antiguedad(state, on_msg),
        ReporteTab::LibroCompras => render_libro_compras(state, on_msg),
        ReporteTab::LibroVentas => render_libro_ventas(state, on_msg),
        ReporteTab::Ats => render_ats(state, on_msg),
    };

    column![tabs, content]
        .padding(SPACING_LG)
        .spacing(SPACING_MD)
        .into()
}

fn barra_fechas<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let on_desde = on_msg.clone();
    let on_hasta = on_msg.clone();
    let on_gen = on_msg.clone();
    row![
        text_input("Desde (YYYY-MM-DD)", &state.desde).on_input(move |v| on_desde(ReportesMessage::Desde(v))).size(12).style(|_, _| input_style()).width(Length::Fixed(160.0)),
        text_input("Hasta (YYYY-MM-DD)", &state.hasta).on_input(move |v| on_hasta(ReportesMessage::Hasta(v))).size(12).style(|_, _| input_style()).width(Length::Fixed(160.0)),
        button(text("Generar").size(12)).style(|_, _| primary_button_style()).on_press(on_gen(ReportesMessage::Generar)),
    ].spacing(SPACING_MD).align_y(Alignment::Center).into()
}

fn render_libro_diario<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let rows: Vec<Element<'a, Message>> = state.libro_diario.iter().map(|a| {
        container(
            row![
                text(&a.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                text(&a.concepto).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(4)),
                text(format!("{:>8.2}", a.total_debe)).size(11).color(COLOR_VENTAS).width(Length::FillPortion(1)),
                text(format!("{:>8.2}", a.total_haber)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).align_y(Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .into()
    }).collect();
    column![
        row![
            text("Libro Diario").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            barra_fechas(state, on_msg),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column(rows).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
    ].spacing(SPACING_MD).into()
}

fn render_comprobacion<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let tipo_color = |t: &str| -> iced::Color {
        match t {
            "activo" => COLOR_INFO, "pasivo" => COLOR_CXP,
            "capital" => COLOR_UTILIDAD, "ingreso" => COLOR_VENTAS,
            "gasto" => COLOR_GASTOS, _ => COLOR_TEXT_SECONDARY,
        }
    };
    let rows: Vec<Element<'a, Message>> = state.balance.iter().enumerate().map(|(i, c)| {
        let debe = state.balance_debe.get(i).copied().unwrap_or(0.0);
        let haber = state.balance_haber.get(i).copied().unwrap_or(0.0);
        let saldo = debe - haber;
        container(
            row![
                text(&c.codigo).size(11).color(COLOR_ACCENT).width(Length::FillPortion(2)),
                text(&c.nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                text(&c.tipo).size(11).color(tipo_color(&c.tipo)).width(Length::FillPortion(1)),
                text(format!("{:>10.2}", debe)).size(11).color(COLOR_VENTAS).width(Length::FillPortion(1)),
                text(format!("{:>10.2}", haber)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(1)),
                text(format!("{:>10.2}", saldo)).size(11).color(if saldo >= 0.0 { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).align_y(Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .into()
    }).collect();
    column![
        row![
            text("Balance de Comprobación").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            button(text("Generar").size(12)).style(|_, _| primary_button_style()).on_press(on_msg(ReportesMessage::Generar)),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column(rows).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
    ].spacing(SPACING_MD).into()
}

fn fila_titulo_balance<'a, Message: 'a + Clone>(txt: &'a str) -> Element<'a, Message> {
    container(
        container(text(txt).size(14).color(COLOR_TEXT_PRIMARY)).padding([SPACING_SM, SPACING_MD])
    )
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color { a: 0.06, ..COLOR_BG })),
        border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
        text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
    })
    .width(Length::Fill)
    .into()
}

fn fila_valor<'a, Message: 'a + Clone>(label: &'a str, valor: String, color: iced::Color, bold: bool) -> Element<'a, Message> {
    let mut txt = text(format!("{:<35}{:>15}", label, valor)).size(12).color(color);
    if bold {
        txt = txt.font(iced::font::Font { weight: iced::font::Weight::Bold, ..iced::font::Font::default() });
    }
    container(container(txt).padding([SPACING_SM, SPACING_MD]))
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_CARD)),
        border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
        text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
    })
    .width(Length::Fill)
    .into()
}

fn render_balance_resumen<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let mut items: Vec<Element<'a, Message>> = vec![fila_titulo_balance("ACTIVOS")];
    if let Some((efectivo, cxc, inventario, total_activos, cxp, total_pasivos, patrimonio)) = state.balance_resumen {
        items.push(fila_valor("Efectivo (pagos recibidos - pagados)", format!("${:.2}", efectivo), COLOR_SUCCESS, false));
        items.push(fila_valor("Cuentas por cobrar (ventas a crédito)", format!("${:.2}", cxc), COLOR_CXC, false));
        items.push(fila_valor("Inventario (stock x precio de compra)", format!("${:.2}", inventario), COLOR_INFO, false));
        items.push(fila_valor("TOTAL ACTIVOS", format!("${:.2}", total_activos), COLOR_TEXT_PRIMARY, true));
        items.push(Space::new().height(SPACING_SM).into());
        items.push(fila_titulo_balance("PASIVOS"));
        items.push(fila_valor("Deudas de la empresa", format!("${:.2}", cxp), COLOR_CXP, false));
        items.push(fila_valor("TOTAL PASIVOS", format!("${:.2}", total_pasivos), COLOR_TEXT_PRIMARY, true));
        items.push(Space::new().height(SPACING_SM).into());
        items.push(fila_titulo_balance("PATRIMONIO"));
        items.push(fila_valor("Patrimonio (activos - pasivos)", format!("${:.2}", patrimonio), COLOR_UTILIDAD, true));
        items.push(Space::new().height(SPACING_SM).into());
        items.push(fila_valor("Verificación: Activos = Pasivos + Patrimonio", format!("${:.2}", total_activos), if (total_pasivos + patrimonio - total_activos).abs() < 0.01 { COLOR_SUCCESS } else { COLOR_DANGER }, true));
    }
    column![
        row![
            text("Balance General").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            button(text("Generar").size(12)).style(|_, _| primary_button_style()).on_press(on_msg(ReportesMessage::Generar)),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column(items).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
    ].spacing(SPACING_MD).into()
}

fn render_resultados<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let mut items: Vec<Element<'a, Message>> = vec![];
    if let Some(r) = &state.resultado {
        items.push(fila_valor("Ventas del periodo", format!("${:.2}", r.ventas_total), COLOR_VENTAS, false));
        items.push(fila_valor("(-) Costo de ventas", format!("${:.2}", r.costo_ventas), COLOR_GASTOS, false));
        items.push(fila_valor("UTILIDAD BRUTA", format!("${:.2}", r.utilidad_bruta), COLOR_UTILIDAD, true));
        items.push(Space::new().height(SPACING_SM).into());
        items.push(fila_valor("(-) Gastos del periodo", format!("${:.2}", r.gastos_total), COLOR_GASTOS, false));
        items.push(Space::new().height(SPACING_SM).into());
        items.push(fila_valor("UTILIDAD NETA", format!("${:.2}", r.utilidad_neta), if r.utilidad_neta >= 0.0 { COLOR_SUCCESS } else { COLOR_DANGER }, true));
        items.push(Space::new().height(SPACING_SM).into());
        let margen = if r.ventas_total > 0.0 { r.utilidad_neta / r.ventas_total * 100.0 } else { 0.0 };
        items.push(fila_valor("Margen neto", format!("{:.1}%", margen), COLOR_TEXT_SECONDARY, false));
    }
    column![
        row![
            text("Estado de Resultados").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            barra_fechas(state, on_msg),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column(items).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
    ].spacing(SPACING_MD).into()
}

fn render_mayor<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let on_cuenta = on_msg.clone();
    let on_desde = on_msg.clone();
    let on_hasta = on_msg.clone();
    let on_gen = on_msg.clone();
    let header = row![
        pick_list(&state.opciones_cuentas[..], state.opciones_cuentas.iter().find(|o| o.id == state.mayor_cuenta), move |o: SelectOption| on_cuenta(ReportesMessage::MayorCuenta(o.id)))
            .style(|_, _| pick_list_style())
            .menu_style(|_| menu_style())
            .padding([8, 12])
            .width(Length::Fixed(260.0)),
        text_input("Desde (YYYY-MM-DD)", &state.desde).on_input(move |v| on_desde(ReportesMessage::Desde(v))).size(12).style(|_, _| input_style()).width(Length::Fixed(160.0)),
        text_input("Hasta (YYYY-MM-DD)", &state.hasta).on_input(move |v| on_hasta(ReportesMessage::Hasta(v))).size(12).style(|_, _| input_style()).width(Length::Fixed(160.0)),
        button(text("Generar").size(12)).style(|_, _| primary_button_style()).on_press(on_gen(ReportesMessage::Generar)),
    ].spacing(SPACING_MD).align_y(Alignment::Center);

    let rows: Vec<Element<'a, Message>> = state.mayor.iter().map(|l| {
        container(
            row![
                text(&l.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                text(&l.concepto).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(4)),
                text(format!("{:>10.2}", l.debe)).size(11).color(COLOR_VENTAS).width(Length::FillPortion(1)),
                text(format!("{:>10.2}", l.haber)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(1)),
                text(format!("{:>10.2}", l.saldo)).size(11).color(if l.saldo >= 0.0 { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).align_y(Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .into()
    }).collect();

    let saldo_final: f64 = state.mayor.last().map(|l| l.saldo).unwrap_or(0.0);
    let vacio: Element<'a, Message> = container(
        container(text("Seleccione una cuenta y genere el reporte").size(13).color(COLOR_TEXT_MUTED)).padding(SPACING_MD)
    ).center(Length::Fill).width(Length::Fill).height(200).into();
    let cuerpo: Element<'a, Message> = if state.mayor.is_empty() {
        vacio
    } else {
        column![
            row![
                text(format!("{} movimiento(s) · Saldo final: ${:.2}", state.mayor.len(), saldo_final))
                    .size(12).color(if saldo_final >= 0.0 { COLOR_SUCCESS } else { COLOR_DANGER }),
                Space::new().width(Length::Fill),
            ].padding([SPACING_SM, SPACING_MD]),
            scrollable(column(rows).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
        ].spacing(SPACING_XS).into()
    };
    column![
        row![
            text("Libro Mayor").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            header,
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        cuerpo,
    ].spacing(SPACING_MD).into()
}

fn buckets_saldos(saldos: &[SaldoPendiente]) -> (f64, f64, f64, f64) {
    let mut b = (0.0, 0.0, 0.0, 0.0);
    for s in saldos {
        if s.dias <= 30 { b.0 += s.total; }
        else if s.dias <= 60 { b.1 += s.total; }
        else if s.dias <= 90 { b.2 += s.total; }
        else { b.3 += s.total; }
    }
    b
}

fn tabla_antiguedad<'a, Message: 'a + Clone>(titulo: &'a str, saldos: &'a [SaldoPendiente]) -> Element<'a, Message> {
    let (b30, b60, b90, b90p) = buckets_saldos(saldos);
    let total: f64 = saldos.iter().map(|s| s.total).sum();
    let filas: Vec<Element<'a, Message>> = saldos.iter().map(|s| {
        container(
            row![
                text(&s.nombre).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                text(format!("${:.2}", s.total)).size(11).color(COLOR_DANGER).width(Length::FillPortion(1)),
                text(format!("{} día(s)", s.dias)).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).align_y(Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .into()
    }).collect();
    column![
        row![
            text(titulo).size(16).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            text(format!("Total: ${:.2}", total)).size(13).color(COLOR_DANGER),
        ].align_y(Alignment::Center),
        Space::new().height(SPACING_XS),
        row![
            crate::widgets::kpi_card_view(crate::widgets::KpiCard { titulo: "0-30 días".into(), valor: format!("${:.2}", b30), subtitulo: "al corriente".into(), color: COLOR_SUCCESS, icono: '\u{2714}' }),
            crate::widgets::kpi_card_view(crate::widgets::KpiCard { titulo: "31-60 días".into(), valor: format!("${:.2}", b60), subtitulo: "atraso leve".into(), color: COLOR_CXP, icono: '\u{23F0}' }),
            crate::widgets::kpi_card_view(crate::widgets::KpiCard { titulo: "61-90 días".into(), valor: format!("${:.2}", b90), subtitulo: "atraso medio".into(), color: COLOR_DANGER, icono: '\u{26A0}' }),
            crate::widgets::kpi_card_view(crate::widgets::KpiCard { titulo: "Más de 90 días".into(), valor: format!("${:.2}", b90p), subtitulo: "crítico".into(), color: COLOR_DANGER, icono: '\u{2715}' }),
        ].spacing(SPACING_SM).width(Length::Fill),
        Space::new().height(SPACING_XS),
        scrollable(column(filas).spacing(SPACING_XS)).style(|_, _| scrollable_style()).height(200),
    ].spacing(SPACING_SM).into()
}

fn render_antiguedad<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let g = on_msg.clone();
    column![
        row![
            text("Antigüedad de Saldos").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            button(text("Generar").size(12)).style(|_, _| primary_button_style()).on_press(g(ReportesMessage::Generar)),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column![
            tabla_antiguedad("Cuentas por cobrar (clientes)", &state.antiguedad_cxc),
            Space::new().height(SPACING_MD),
            tabla_antiguedad("Cuentas por pagar (proveedores)", &state.antiguedad_cxp),
        ].spacing(0)).style(|_, _| scrollable_style()),
    ].spacing(SPACING_MD).into()
}

fn tabla_lineas<'a, Message: 'a + Clone>(
    filas: impl IntoIterator<Item = (String, String, String, String, String, String)>,
    cabeceras: [&'static str; 6],
) -> Vec<Element<'a, Message>> {
    let mut out: Vec<Element<'a, Message>> = vec![];
    out.push(
        container(
            row![
                text(cabeceras[0]).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                text(cabeceras[1]).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
                text(cabeceras[2]).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                text(cabeceras[3]).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                text(cabeceras[4]).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                text(cabeceras[5]).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).align_y(Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_SURFACE)),
            border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .into(),
    );
    for (c1, c2, c3, c4, c5, c6) in filas {
        out.push(
            container(
                row![
                    text(c1).size(10).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                    text(c2).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(c3).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                    text(c4).size(10).color(COLOR_VENTAS).width(Length::FillPortion(1)),
                    text(c5).size(10).color(COLOR_GASTOS).width(Length::FillPortion(1)),
                    text(c6).size(10).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
                ].spacing(SPACING_SM).align_y(Alignment::Center)
                .padding([SPACING_SM, SPACING_MD])
            )
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(COLOR_CARD)),
                border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
                text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
            })
            .into(),
        );
    }
    out
}

fn render_libro_compras<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let filas: Vec<Element<'a, Message>> = tabla_lineas(
        state.libro_compras.iter().map(|c| (
            c.numero.clone(), c.proveedor_nombre.clone(), c.fecha.clone(),
            format!("{:.2}", c.subtotal), format!("{:.2}", c.iva), format!("{:.2}", c.total),
        )),
        ["No. Compra", "Proveedor", "Fecha", "Subtotal", "IVA", "Total"],
    );
    let total: f64 = state.libro_compras.iter().map(|c| c.total).sum();
    column![
        row![
            text("Libro de Compras").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            barra_fechas(state, on_msg),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column(filas).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
        row![
            text(format!("{} compras · Total: ${:.2}", state.libro_compras.len(), total)).size(12)
                .color(if total > 0.0 { COLOR_SUCCESS } else { COLOR_TEXT_MUTED }),
        ].padding([SPACING_SM, SPACING_MD]),
    ].spacing(SPACING_MD).into()
}

fn render_libro_ventas<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let filas: Vec<Element<'a, Message>> = tabla_lineas(
        state.libro_ventas.iter().map(|v| (
            v.folio.clone(), v.cliente_nombre.clone(), v.fecha.clone(),
            format!("{:.2}", v.subtotal), format!("{:.2}", v.iva), format!("{:.2}", v.total),
        )),
        ["No. Factura", "Cliente", "Fecha", "Subtotal", "IVA", "Total"],
    );
    let total: f64 = state.libro_ventas.iter().map(|v| v.total).sum();
    column![
        row![
            text("Libro de Ventas").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            barra_fechas(state, on_msg),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column(filas).spacing(SPACING_XS)).style(|_, _| scrollable_style()),
        row![
            text(format!("{} facturas · Total: ${:.2}", state.libro_ventas.len(), total)).size(12)
                .color(if total > 0.0 { COLOR_SUCCESS } else { COLOR_TEXT_MUTED }),
        ].padding([SPACING_SM, SPACING_MD]),
    ].spacing(SPACING_MD).into()
}

fn render_ats<'a, Message: 'a + Clone>(
    state: &'a ReportesState,
    on_msg: impl Fn(ReportesMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let e = on_msg.clone();
    let r = &state.resumen_ats;
    let iva_neto = r.iva_ventas - r.iva_compras;
    column![
        row![
            text("ATS (Anexo Transaccional Simplificado)").size(18).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
            barra_fechas(state, on_msg),
        ].spacing(SPACING_MD).align_y(Alignment::Center),
        scrollable(column![
            fila_valor("Ventas del periodo (total)", format!("${:.2}", r.ventas), COLOR_VENTAS, false),
            fila_valor("IVA cobrado en ventas", format!("${:.2}", r.iva_ventas), COLOR_VENTAS, false),
            fila_valor("Ventas exentas de IVA", format!("${:.2}", r.ventas_exentas), COLOR_TEXT_SECONDARY, false),
            Space::new().height(SPACING_SM),
            fila_valor("Compras del periodo (total)", format!("${:.2}", r.compras), COLOR_GASTOS, false),
            fila_valor("IVA pagado en compras", format!("${:.2}", r.iva_compras), COLOR_GASTOS, false),
            Space::new().height(SPACING_SM),
            fila_valor("IVA a favor / a pagar (neto)", format!("${:.2}", iva_neto), if iva_neto >= 0.0 { COLOR_CXP } else { COLOR_SUCCESS }, true),
        ].spacing(SPACING_XS)).style(|_, _| scrollable_style()),
        row![
            button(text("Exportar ATS (.csv)").size(12)).style(|_, _| primary_button_style()).on_press(e(ReportesMessage::ExportarAts)),
            text("Se genera el libro de compras y ventas en la carpeta Documentos").size(10).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]),
        row![
            text("El exportar requiere haber generado primero los libros.").size(10).color(COLOR_TEXT_MUTED),
        ].padding([0.0, SPACING_MD]),
    ].spacing(SPACING_MD).into()
}
