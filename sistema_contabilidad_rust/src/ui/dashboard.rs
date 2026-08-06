use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length, Alignment};
use crate::theme::*;
use crate::widgets::{kpi_card_view, bar_chart_view, KpiCard};

#[derive(Debug, Clone)]
pub struct DashboardData {
    pub ventas_hoy: f64,
    pub gastos_hoy: f64,
    pub cxc: f64,
    pub cxp: f64,
    pub utilidad_mes: f64,
    pub total_clientes: i64,
    pub ventas_mes: f64,
    pub gastos_mes: f64,
    pub ventas_anio: f64,
    pub gastos_anio: f64,
    pub utilidad_anio: f64,
    pub ventas_mensuales: Vec<(String, f64)>,
    pub gastos_categorias: Vec<(String, f64)>,
    pub alertas_stock: Vec<(String, i32)>,
    pub alertas_creditos: Vec<(String, f64)>,
    pub alertas_cobros: Vec<(String, String, i32, f64)>,
    pub actividad: Vec<(String, String, String)>,
}

impl Default for DashboardData {
    fn default() -> Self {
        Self {
            ventas_hoy: 0.0, gastos_hoy: 0.0, cxc: 0.0, cxp: 0.0,
            utilidad_mes: 0.0, total_clientes: 0,
            ventas_mes: 0.0, gastos_mes: 0.0, ventas_anio: 0.0, gastos_anio: 0.0, utilidad_anio: 0.0,
            ventas_mensuales: Vec::new(), gastos_categorias: Vec::new(),
            alertas_stock: Vec::new(), alertas_creditos: Vec::new(), alertas_cobros: Vec::new(), actividad: Vec::new(),
        }
    }
}

pub fn dashboard_view<'a, Message: 'a + Clone>(
    data: &DashboardData,
    _on_refresh: Message,
) -> Element<'a, Message> {
    let kpi_ventas = KpiCard {
        titulo: "Ventas Hoy".into(), valor: format!("${:.2}", data.ventas_hoy),
        subtitulo: "Ingresos del día".into(), color: COLOR_VENTAS, icono: '\u{2191}',
    };
    let kpi_gastos = KpiCard {
        titulo: "Gastos Hoy".into(), valor: format!("${:.2}", data.gastos_hoy),
        subtitulo: "Egresos del día".into(), color: COLOR_GASTOS, icono: '\u{2193}',
    };
    let kpi_cxc = KpiCard {
        titulo: "Cuentas por Cobrar".into(), valor: format!("${:.2}", data.cxc),
        subtitulo: "Ventas a crédito".into(), color: COLOR_CXC, icono: '\u{25A3}',
    };
    let kpi_cxp = KpiCard {
        titulo: "Cuentas por Pagar".into(), valor: format!("${:.2}", data.cxp),
        subtitulo: "Saldo a proveedores".into(), color: COLOR_CXP, icono: '\u{25A4}',
    };
    let kpi_utilidad = KpiCard {
        titulo: "Utilidad del Mes".into(), valor: format!("${:.2}", data.utilidad_mes),
        subtitulo: "Ventas - Gastos".into(), color: COLOR_UTILIDAD, icono: '\u{25B2}',
    };
    let kpi_clientes = KpiCard {
        titulo: "Clientes".into(), valor: format!("{}", data.total_clientes),
        subtitulo: "Registrados activos".into(), color: COLOR_INFO, icono: '\u{263A}',
    };

    let kpi_row = row![
        kpi_card_view(kpi_ventas), kpi_card_view(kpi_gastos),
        kpi_card_view(kpi_cxc), kpi_card_view(kpi_cxp),
    ]
    .spacing(SPACING_MD)
    .width(Length::Fill);

    let kpi_row2 = row![
        kpi_card_view(kpi_utilidad), kpi_card_view(kpi_clientes),
    ]
    .spacing(SPACING_MD)
    .width(Length::Fill);

    let chart_row = row![
        container(
            bar_chart_view("Ventas por Mes", &data.ventas_mensuales, COLOR_VENTAS)
        )
        .style(|_| card_style())
        .width(Length::FillPortion(1)),
        container(
            bar_chart_view("Gastos por Categoría (este mes)", &data.gastos_categorias, COLOR_GASTOS)
        )
        .style(|_| card_style())
        .width(Length::FillPortion(1)),
    ]
    .spacing(SPACING_MD)
    .width(Length::Fill);

    let alert_items: Vec<Element<'a, Message>> = {
        let mut items: Vec<Element<'a, Message>> = Vec::new();
        for (nom, stock) in &data.alertas_stock {
            items.push(
                row![
                    text("\u{26A0}").size(14).color(COLOR_WARNING),
                    text(nom.clone()).size(12).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
                    text(format!("Stock: {}", stock)).size(11).color(COLOR_DANGER),
                ].spacing(SPACING_SM).align_y(Alignment::Center).into()
            );
        }
        for (nom, saldo) in &data.alertas_creditos {
            items.push(
                row![
                    text("\u{26A0}").size(14).color(COLOR_DANGER),
                    text(nom.clone()).size(12).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
                    text(format!("${:.2}", saldo)).size(11).color(COLOR_DANGER),
                ].spacing(SPACING_SM).align_y(Alignment::Center).into()
            );
        }
        if items.is_empty() {
            items.push(text("Sin alertas pendientes").size(13).color(COLOR_TEXT_MUTED).into());
        }
        items
    };

    let cobro_items: Vec<Element<'a, Message>> = {
        let mut items: Vec<Element<'a, Message>> = Vec::new();
        for (maquina, ubicacion, dia, monto) in &data.alertas_cobros {
            items.push(
                row![
                    text("\u{24B6}").size(14).color(COLOR_VENTAS),
                    text(maquina.clone()).size(12).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
                    text(ubicacion.clone()).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                    text(if *monto > 0.0 { format!("${:.2}", monto) } else { String::new() }).size(11).color(COLOR_VENTAS),
                    text(format!("Día {}", dia)).size(11).color(COLOR_WARNING),
                ].spacing(SPACING_SM).align_y(Alignment::Center).into()
            );
        }
        if items.is_empty() {
            items.push(text("Sin cobros pendientes este mes").size(13).color(COLOR_TEXT_MUTED).into());
        }
        items
    };

    let actividad_items: Vec<Element<'a, Message>> = if data.actividad.is_empty() {
        vec![text("Sin actividad reciente").size(13).color(COLOR_TEXT_MUTED).into()]
    } else {
        data.actividad.iter().take(10).map(|(tipo, desc, fecha)| {
            let color = if tipo == "venta" { COLOR_VENTAS } else { COLOR_GASTOS };
            let icon = if tipo == "venta" { "\u{2191}" } else { "\u{2193}" };
            row![
                text(icon).size(14).color(color),
                text(desc.clone()).size(12).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
                text(fecha.clone()).size(10).color(COLOR_TEXT_MUTED),
            ]
            .spacing(SPACING_SM)
            .align_y(Alignment::Center)
            .into()
        }).collect()
    };

    column![
        text("Dashboard").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().height(Length::Fixed(SPACING_SM)),
        column![kpi_row, kpi_row2].spacing(SPACING_MD),
        column![
            kpi_card_view(KpiCard { titulo: "Ventas del Mes".into(), valor: format!("${:.2}", data.ventas_mes), subtitulo: "Ingresos del mes".into(), color: COLOR_VENTAS, icono: '\u{2191}' }),
            kpi_card_view(KpiCard { titulo: "Gastos del Mes".into(), valor: format!("${:.2}", data.gastos_mes), subtitulo: "Egresos del mes".into(), color: COLOR_GASTOS, icono: '\u{2193}' }),
        ]
        .spacing(SPACING_MD),
        column![
            kpi_card_view(KpiCard { titulo: "Ventas del Año".into(), valor: format!("${:.2}", data.ventas_anio), subtitulo: "Ingresos YTD".into(), color: COLOR_VENTAS, icono: '\u{2191}' }),
            kpi_card_view(KpiCard { titulo: "Gastos del Año".into(), valor: format!("${:.2}", data.gastos_anio), subtitulo: "Egresos YTD".into(), color: COLOR_GASTOS, icono: '\u{2193}' }),
            kpi_card_view(KpiCard { titulo: "Utilidad del Año".into(), valor: format!("${:.2}", data.utilidad_anio), subtitulo: "YTD".into(), color: COLOR_UTILIDAD, icono: '\u{25B2}' }),
        ]
        .spacing(SPACING_MD),
        container(
            column![
                text("Alertas").size(14).color(COLOR_DANGER),
                Space::new().height(Length::Fixed(SPACING_SM)),
                column(alert_items).spacing(SPACING_SM),
            ].padding(SPACING_MD)
        )
        .style(|_| card_style())
        .width(Length::Fill),
        Space::new().height(Length::Fixed(SPACING_SM)),
        container(
            column![
                row![
                    text("Cobros de Comisiones del Mes").size(14).color(COLOR_VENTAS),
                    Space::new().width(Length::Fill),
                    text("Máquinas por cobrar").size(10).color(COLOR_TEXT_MUTED),
                ]
                .align_y(Alignment::Center),
                Space::new().height(Length::Fixed(SPACING_SM)),
                column(cobro_items).spacing(SPACING_SM),
            ].padding(SPACING_MD)
        )
        .style(|_| card_style())
        .width(Length::Fill),
        Space::new().height(Length::Fixed(SPACING_SM)),
        chart_row,
        container(
            column![
                row![
                    text("Actividad Reciente").size(14).color(COLOR_TEXT_PRIMARY),
                    Space::new().width(Length::Fill),
                    text("Últimos 10 movimientos").size(10).color(COLOR_TEXT_MUTED),
                ]
                .align_y(Alignment::Center),
                Space::new().height(Length::Fixed(SPACING_SM)),
                column(actividad_items).spacing(SPACING_SM),
            ]
            .padding(SPACING_MD)
        )
        .style(|_| card_style())
        .width(Length::Fill),
    ]
    .padding(SPACING_LG)
    .spacing(SPACING_MD)
    .into()
}
