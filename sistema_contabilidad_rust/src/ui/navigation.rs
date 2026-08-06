use crate::theme::*;
use crate::models::Configuracion;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length, Alignment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavItem {
    Dashboard, Clientes, Proveedores, Productos, Ventas, Gastos,
    PlanCuentas, Ubicaciones, Maquinas, Garantias, Creditos, Ahorros,
    Asientos, PagosRecibidos, PagosRealizados, DeudasEmpresa, Reportes, CobroComisiones,
    Compras, Cotizaciones, Configuracion,
    Retenciones, Nomina, Depreciacion, CierreContable, Conciliacion, CajaChica,
}

const NAV_SECTIONS: &[(&str, &[NavItem])] = &[
    ("General", &[NavItem::Dashboard]),
    ("Gestión", &[NavItem::Clientes, NavItem::Proveedores, NavItem::Productos]),
    ("Operaciones", &[NavItem::Ventas, NavItem::Cotizaciones, NavItem::Compras, NavItem::Gastos, NavItem::PlanCuentas, NavItem::Asientos]),
    ("Activos", &[NavItem::Ubicaciones, NavItem::Maquinas, NavItem::Garantias, NavItem::CobroComisiones]),
    ("Financiero", &[NavItem::DeudasEmpresa, NavItem::Creditos, NavItem::Ahorros, NavItem::PagosRecibidos, NavItem::PagosRealizados]),
    ("SRI / Impuestos", &[NavItem::Retenciones, NavItem::Nomina]),
    ("Contabilidad", &[NavItem::Depreciacion, NavItem::CierreContable, NavItem::Conciliacion, NavItem::CajaChica]),
    ("Reportes", &[NavItem::Reportes]),
    ("Sistema", &[NavItem::Configuracion]),
];

impl NavItem {
    pub fn titulo(&self) -> &'static str {
        match self {
            NavItem::Dashboard => "Dashboard", NavItem::Clientes => "Clientes",
            NavItem::Proveedores => "Proveedores", NavItem::Productos => "Productos",
            NavItem::Ventas => "Ventas", NavItem::Gastos => "Gastos",
            NavItem::PlanCuentas => "Plan de Cuentas", NavItem::Ubicaciones => "Ubicaciones",
            NavItem::Maquinas => "Máquinas", NavItem::Garantias => "Garantías",
            NavItem::Creditos => "Crédito", NavItem::Ahorros => "Ahorro",
            NavItem::Asientos => "Asientos", NavItem::PagosRecibidos => "Pagos Recibidos",
            NavItem::PagosRealizados => "Pagos Realizados",
            NavItem::DeudasEmpresa => "Deudas Empresa",
            NavItem::Reportes => "Reportes",
            NavItem::CobroComisiones => "Cobro Comisiones",
            NavItem::Compras => "Compras / Almacén",
            NavItem::Cotizaciones => "Cotizaciones",
            NavItem::Configuracion => "Configuración",
            NavItem::Retenciones => "Retenciones",
            NavItem::Nomina => "Nómina",
            NavItem::Depreciacion => "Depreciación",
            NavItem::CierreContable => "Cierre Contable",
            NavItem::Conciliacion => "Conciliación Bancaria",
            NavItem::CajaChica => "Arqueo de Caja",
        }
    }
    pub fn icono(&self) -> &'static str {
        match self {
            NavItem::Dashboard => "\u{2302}", NavItem::Clientes => "\u{263A}",
            NavItem::Proveedores => "\u{263C}", NavItem::Productos => "\u{2603}",
            NavItem::Ventas => "\u{2191}", NavItem::Gastos => "\u{2193}",
            NavItem::PlanCuentas => "\u{2261}", NavItem::Ubicaciones => "\u{25C9}",
            NavItem::Maquinas => "\u{2699}", NavItem::Garantias => "\u{2605}",
            NavItem::Creditos => "\u{25A3}", NavItem::Ahorros => "\u{2665}",
            NavItem::Asientos => "\u{270E}", NavItem::PagosRecibidos => "\u{21A3}",
            NavItem::PagosRealizados => "\u{21A2}",
            NavItem::DeudasEmpresa => "\u{25C6}",
            NavItem::Reportes => "\u{2714}",
            NavItem::CobroComisiones => "\u{2713}",
            NavItem::Compras => "\u{2190}",
            NavItem::Cotizaciones => "\u{2709}",
            NavItem::Configuracion => "\u{2699}",
            NavItem::Retenciones => "\u{25A0}",
            NavItem::Nomina => "\u{263A}",
            NavItem::Depreciacion => "\u{25BC}",
            NavItem::CierreContable => "\u{2717}",
            NavItem::Conciliacion => "\u{24C8}",
            NavItem::CajaChica => "\u{25C9}",
        }
    }
}

pub fn sidebar_view<Message: 'static + Clone>(
    current: &NavItem,
    empresa: &Configuracion,
    usuario: &str,
    on_navigate: impl Fn(NavItem) -> Message + 'static + Clone,
    on_celular: impl Fn() -> Message + 'static + Clone,
    on_logout: impl Fn() -> Message + 'static + Clone,
) -> Element<'static, Message> {
    let iniciales = empresa.iniciales();
    let nombre_empresa = empresa.nombre_corto();
    let ruc_empresa = format!("RUC {}", empresa.ruc);
    let usuario_own = usuario.to_string();
    let mut col: iced::widget::Column<'static, Message> = column![]
        .spacing(SPACING_XS);

    col = col.push(
        container(column![
            row![
                container(text(iniciales).size(18).color(COLOR_ACCENT))
                    .padding([SPACING_XS, SPACING_SM])
                    .style(|_| iced::widget::container::Style {
                        background: Some(iced::Background::Color(COLOR_ACCENT_GLOW)),
                        border: iced::Border { radius: RADIUS_SM.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                        text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
                    }),
                column![
                    text(nombre_empresa).size(11).color(COLOR_TEXT_PRIMARY),
                    text(ruc_empresa).size(8).color(COLOR_TEXT_MUTED),
                ].spacing(2),
            ].spacing(SPACING_SM).align_y(Alignment::Center),
        ].spacing(SPACING_XS))
        .padding([SPACING_MD, SPACING_MD])
        .width(Length::Fill),
    );

    for (section_name, items) in NAV_SECTIONS {
        col = col.push(
            container(text(*section_name).size(9).color(COLOR_TEXT_MUTED))
                .padding([SPACING_MD, SPACING_MD]),
        );

        for item in *items {
            let selected = item == current;
            let msg = on_navigate(item.clone());
            let btn = button(
                row![
                    text(item.icono()).size(15).color(if selected { COLOR_ACCENT } else { COLOR_TEXT_MUTED }),
                    text(item.titulo()).size(12).color(if selected { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_SECONDARY }),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_XS, 0.0]),
            )
            .style(move |_, _| sidebar_button_style(selected))
            .width(Length::Fill)
            .padding([SPACING_SM, SPACING_MD])
            .on_press(msg);

            col = col.push(btn);
        }
    }

    col = col.push(Space::new().height(Length::Fill));

    let on_celular = on_celular();
    col = col.push(
        button(
            row![
                text("\u{260E}").size(13).color(COLOR_ACCENT),
                text("Conectar celular").size(12).color(COLOR_TEXT_PRIMARY),
            ]
            .spacing(SPACING_SM)
            .align_y(Alignment::Center),
        )
        .style(move |_, _| sidebar_button_style(false))
        .width(Length::Fill)
        .padding([SPACING_SM, SPACING_MD])
        .on_press(on_celular),
    );

    let logout = on_logout();
    col = col.push(
        container(column![
            row![
                container(text("\u{1F464}").size(13).color(COLOR_ACCENT))
                    .padding([4, 6])
                    .style(|_| iced::widget::container::Style {
                        background: Some(iced::Background::Color(COLOR_ACCENT_GLOW)),
                        border: iced::Border { radius: RADIUS_SM.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                        text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
                    }),
                column![
                    text(usuario_own).size(11).color(COLOR_TEXT_PRIMARY),
                    text("Usuario").size(8).color(COLOR_TEXT_MUTED),
                ].spacing(2),
                Space::new().width(Length::Fill),
                button(text("\u{21AA}").size(12).color(COLOR_TEXT_MUTED))
                    .style(|_, _| ghost_button_style())
                    .on_press(logout)
                    .padding([4, 8]),
            ].spacing(SPACING_SM).align_y(Alignment::Center),
        ].spacing(SPACING_XS))
        .padding([SPACING_SM, SPACING_MD])
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .width(Length::Fill),
    );

    container(scrollable(col).style(|_, _| scrollable_style()).width(Length::Fill))
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_SIDEBAR)),
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false,
            border: iced::Border { radius: 0.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            shadow: iced::Shadow::default(),
        })
        .width(230)
        .height(Length::Fill)
        .into()
}
