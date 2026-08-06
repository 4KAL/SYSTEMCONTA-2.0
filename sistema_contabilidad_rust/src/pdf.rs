use printpdf::{BuiltinFont, Color, IndirectFontRef, Mm, PdfDocumentReference, PdfLayerReference, Rect, Rgb};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use crate::models::{Cliente, Configuracion, Venta, VentaDetalle};

const PAGE_W: f32 = 215.9;
const PAGE_H: f32 = 279.4;
const MARGEN: f32 = 20.0;

fn cargar_fuentes(doc: &PdfDocumentReference) -> Result<(IndirectFontRef, IndirectFontRef), String> {
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Error cargando fuente Helvetica: {}", e))?;
    let bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Error cargando fuente Helvetica Bold: {}", e))?;
    Ok((font, bold))
}

fn t(layer: &PdfLayerReference, font: &IndirectFontRef, x: f32, y: f32, size: f32, texto: &str) {
    layer.use_text(texto, size, Mm(x), Mm(y), font);
}

fn t_center(layer: &PdfLayerReference, font: &IndirectFontRef, y: f32, size: f32, texto: &str) {
    let w = texto.chars().count() as f32 * size * 0.3528 * 0.52;
    layer.use_text(texto, size, Mm(PAGE_W / 2.0 - w / 2.0), Mm(y), font);
}

fn hline(layer: &PdfLayerReference, x: f32, y: f32, w: f32, thickness: f32) {
    layer.set_fill_color(Color::Rgb(Rgb { r: 0.25, g: 0.25, b: 0.3, icc_profile: None }));
    layer.add_rect(Rect::new(Mm(x), Mm(y), Mm(x + w), Mm(y + thickness)));
}

fn encabezado_empresa(layer: &PdfLayerReference, font: &IndirectFontRef, bold: &IndirectFontRef, empresa: &Configuracion) {
    t(layer, bold, MARGEN, PAGE_H - 22.0, 18.0, &empresa.empresa_nombre);
    t(layer, font, MARGEN, PAGE_H - 32.0, 10.0, &format!("RUC: {}", empresa.ruc));
    t(layer, font, MARGEN, PAGE_H - 40.0, 10.0, &format!("{}, {}", empresa.direccion, empresa.ciudad));
    t(layer, font, MARGEN, PAGE_H - 48.0, 10.0, &format!("Tel: {}   |   Email: {}", empresa.telefono, empresa.email));
    hline(layer, MARGEN, PAGE_H - 52.0, PAGE_W - 2.0 * MARGEN, 0.6);
}

fn pie_empresa(layer: &PdfLayerReference, font: &IndirectFontRef, empresa: &Configuracion) {
    hline(layer, MARGEN, 18.0, PAGE_W - 2.0 * MARGEN, 0.3);
    t(layer, font, MARGEN, 12.0, 8.0, &format!("{} | RUC: {} | Tel: {} | Email: {}", empresa.empresa_nombre, empresa.ruc, empresa.telefono, empresa.email));
}

fn datos_contacto(cliente: Option<&Cliente>) -> (String, String, String) {
    let c = cliente.map(|c| (
        c.rfc.clone(),
        c.telefono.clone(),
        format!("{} {}", c.direccion.clone(), c.ciudad.clone()).trim().to_string(),
    )).unwrap_or_default();
    c
}

fn carpeta_documentos() -> PathBuf {
    let mut dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    dir.push("Documentos");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn nombre_archivo(prefijo: &str, folio: &str) -> PathBuf {
    let limpio: String = folio.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
    carpeta_documentos().join(format!("{}_{}.pdf", prefijo, limpio))
}

fn cabecera_tabla_venta(layer: &PdfLayerReference, font: &IndirectFontRef, y: f32) {
    t(layer, font, 20.0, y, 9.0, "CANT");
    t(layer, font, 40.0, y, 9.0, "PRODUCTO / DESCRIPCION");
    t(layer, font, 150.0, y, 9.0, "P. UNIT");
    t(layer, font, 174.0, y, 9.0, "DESC.");
    t(layer, font, 196.0, y, 9.0, "IMPORTE");
}

fn fila_venta(layer: &PdfLayerReference, font: &IndirectFontRef, y: f32, det: &VentaDetalle) {
    t(layer, font, 20.0, y, 9.0, &format!("{}", det.cantidad));
    t(layer, font, 40.0, y, 9.0, &det.producto_nombre);
    t(layer, font, 150.0, y, 9.0, &format!("{:.2}", det.precio_unitario));
    t(layer, font, 174.0, y, 9.0, &format!("{:.2}", det.descuento));
    t(layer, font, 196.0, y, 9.0, &format!("{:.2}", det.importe));
}

pub fn generar_factura(
    venta: &Venta,
    lineas: &[VentaDetalle],
    cliente: Option<&Cliente>,
    empresa: &Configuracion,
) -> Result<PathBuf, String> {
    let (doc, page1, layer1) = printpdf::PdfDocument::new("Factura", Mm(PAGE_W), Mm(PAGE_H), "Factura");
    let (font, bold) = cargar_fuentes(&doc)?;

    let layer = doc.get_page(page1).get_layer(layer1);

    encabezado_empresa(&layer, &font, &bold, empresa);

    // Titulo
    t_center(&layer, &bold, PAGE_H - 72.0, 20.0, "FACTURA");

    // Folio y fecha
    t(&layer, &bold, MARGEN, PAGE_H - 84.0, 11.0, "No. de Factura:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 84.0, 11.0, &venta.folio);
    t(&layer, &bold, MARGEN, PAGE_H - 94.0, 11.0, "Fecha:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 94.0, 11.0, &venta.fecha);
    t(&layer, &bold, MARGEN, PAGE_H - 104.0, 11.0, "Tipo:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 104.0, 11.0, &venta.tipo);

    // Cliente
    let (cedula, telefono, direccion) = datos_contacto(cliente);
    t(&layer, &bold, MARGEN, PAGE_H - 116.0, 11.0, "Cliente:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 116.0, 11.0, &venta.cliente_nombre);
    if !cedula.is_empty() {
        t(&layer, &font, MARGEN, PAGE_H - 126.0, 10.0, &format!("Cedula: {}", cedula));
    }
    if !telefono.is_empty() {
        t(&layer, &font, MARGEN + 60.0, PAGE_H - 126.0, 10.0, &format!("Tel: {}", telefono));
    }
    if !direccion.is_empty() {
        t(&layer, &font, MARGEN, PAGE_H - 136.0, 10.0, &format!("Direccion: {}", direccion));
    }

    hline(&layer, MARGEN, PAGE_H - 142.0, PAGE_W - 2.0 * MARGEN, 0.6);
    cabecera_tabla_venta(&layer, &bold, PAGE_H - 152.0);

    let mut y = PAGE_H - 162.0;
    for det in lineas {
        if y < 60.0 { break; }
        fila_venta(&layer, &font, y, det);
        y -= 10.0;
    }

    hline(&layer, MARGEN, y - 4.0, PAGE_W - 2.0 * MARGEN, 0.4);

    // Totales
    let ty = y - 18.0;
    t(&layer, &font, 130.0, ty, 10.0, "Subtotal:");
    t(&layer, &font, 190.0, ty, 10.0, &format!("{:.2}", venta.subtotal));
    t(&layer, &font, 130.0, ty - 12.0, 10.0, &format!("Impuesto ({}%):", empresa.iva));
    t(&layer, &font, 190.0, ty - 12.0, 10.0, &format!("{:.2}", venta.impuesto));
    t(&layer, &font, 130.0, ty - 24.0, 10.0, "Descuento:");
    t(&layer, &font, 190.0, ty - 24.0, 10.0, &format!("{:.2}", venta.descuento));
    t(&layer, &bold, 130.0, ty - 40.0, 13.0, "TOTAL:");
    t(&layer, &bold, 190.0, ty - 40.0, 13.0, &format!("{:.2}", venta.total));

    if !venta.notas.is_empty() {
        t(&layer, &font, MARGEN, 90.0, 9.0, "Notas:");
        t(&layer, &font, MARGEN, 80.0, 9.0, &venta.notas);
    }

    // Firma
    hline(&layer, MARGEN, 34.0, 60.0, 0.4);
    t(&layer, &font, MARGEN, 28.0, 9.0, "Firma del cliente");
    hline(&layer, PAGE_W - MARGEN - 60.0, 34.0, 60.0, 0.4);
    t(&layer, &font, PAGE_W - MARGEN - 60.0, 28.0, 9.0, "Firma autorizada");

    t_center(&layer, &font, 52.0, 9.0, "Gracias por su compra");

    pie_empresa(&layer, &font, empresa);

    let ruta = nombre_archivo("Factura", &venta.folio);
    let file = File::create(&ruta).map_err(|e| format!("No se pudo crear el archivo: {}", e))?;
    doc.save(&mut BufWriter::new(file)).map_err(|e| format!("Error al guardar PDF: {}", e))?;
    Ok(ruta)
}

const CONDICIONES: [&str; 4] = [
    "1. Esta garantia cubre defectos de fabricacion del producto por el periodo indicado.",
    "2. No cubre danos causados por mal uso, accidentes, alteraciones o reparaciones no autorizadas.",
    "3. Para hacer efectiva la garantia, presente este certificado junto con la factura de compra.",
    "4. Los gastos de transporte para el servicio de garantia corren por cuenta del cliente.",
];

pub fn generar_garantia(
    venta: &Venta,
    lineas: &[VentaDetalle],
    fecha_inicio: &str,
    fecha_fin: &str,
    cliente: Option<&Cliente>,
    empresa: &Configuracion,
) -> Result<PathBuf, String> {
    let (doc, page1, layer1) = printpdf::PdfDocument::new("Garantia", Mm(PAGE_W), Mm(PAGE_H), "Garantia");
    let (font, bold) = cargar_fuentes(&doc)?;

    let layer = doc.get_page(page1).get_layer(layer1);

    encabezado_empresa(&layer, &font, &bold, empresa);

    // Titulo
    t_center(&layer, &bold, PAGE_H - 72.0, 20.0, "CERTIFICADO DE GARANTIA");

    // Datos de la venta
    t(&layer, &bold, MARGEN, PAGE_H - 86.0, 11.0, "Venta No.:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 86.0, 11.0, &venta.folio);
    t(&layer, &bold, MARGEN, PAGE_H - 96.0, 11.0, "Fecha de venta:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 96.0, 11.0, &venta.fecha);

    // Cliente
    let (cedula, telefono, direccion) = datos_contacto(cliente);
    t(&layer, &bold, MARGEN, PAGE_H - 110.0, 11.0, "Cliente:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 110.0, 11.0, &venta.cliente_nombre);
    let mut cy = PAGE_H - 120.0;
    if !cedula.is_empty() { t(&layer, &font, MARGEN, cy, 10.0, &format!("Cedula: {}", cedula)); cy -= 10.0; }
    if !telefono.is_empty() { t(&layer, &font, MARGEN, cy, 10.0, &format!("Telefono: {}", telefono)); cy -= 10.0; }
    if !direccion.is_empty() { t(&layer, &font, MARGEN, cy, 10.0, &format!("Direccion: {}", direccion)); cy -= 10.0; }

    hline(&layer, MARGEN, cy - 4.0, PAGE_W - 2.0 * MARGEN, 0.6);

    // Productos cubiertos
    cabecera_tabla_venta(&layer, &bold, cy - 14.0);
    let mut y = cy - 24.0;
    for det in lineas {
        if y < 130.0 { break; }
        fila_venta(&layer, &font, y, det);
        y -= 10.0;
    }
    hline(&layer, MARGEN, y - 4.0, PAGE_W - 2.0 * MARGEN, 0.4);

    // Periodo
    t(&layer, &bold, MARGEN, y - 20.0, 11.0, "Periodo de cobertura:");
    t(&layer, &font, MARGEN + 60.0, y - 20.0, 11.0, &format!("Desde {}  hasta  {}", fecha_inicio, fecha_fin));

    // Condiciones
    let mut cy2 = y - 44.0;
    t(&layer, &bold, MARGEN, cy2, 11.0, "Condiciones:");
    cy2 -= 12.0;
    for c in CONDICIONES {
        t(&layer, &font, MARGEN, cy2, 10.0, c);
        cy2 -= 12.0;
    }

    // Firma
    hline(&layer, MARGEN, 16.0, 60.0, 0.4);
    t(&layer, &font, MARGEN, 10.0, 9.0, "Firma del cliente");

    t_center(&layer, &font, 24.0, 9.0, "Este certificado es valido solo junto con la factura de compra");

    let ruta = nombre_archivo("Garantia", &venta.folio);
    let file = File::create(&ruta).map_err(|e| format!("No se pudo crear el archivo: {}", e))?;
    doc.save(&mut BufWriter::new(file)).map_err(|e| format!("Error al guardar PDF: {}", e))?;
    Ok(ruta)
}

pub fn generar_retencion_pdf(
    ret: &crate::models::Retencion,
    empresa: &Configuracion,
) -> Result<PathBuf, String> {
    let (doc, page1, layer1) = printpdf::PdfDocument::new("ComprobanteRetencion", Mm(PAGE_W), Mm(PAGE_H), "ComprobanteRetencion");
    let (font, bold) = cargar_fuentes(&doc)?;

    let layer = doc.get_page(page1).get_layer(layer1);

    encabezado_empresa(&layer, &font, &bold, empresa);

    // Titulo
    t_center(&layer, &bold, PAGE_H - 72.0, 20.0, "COMPROBANTE DE RETENCION");

    // Numero y fecha
    t(&layer, &bold, MARGEN, PAGE_H - 86.0, 11.0, "No. de Retencion:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 86.0, 11.0, &ret.numero);
    t(&layer, &bold, MARGEN, PAGE_H - 96.0, 11.0, "Fecha de emision:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 96.0, 11.0, &ret.fecha);
    t(&layer, &bold, MARGEN, PAGE_H - 106.0, 11.0, "Comprobante que se retiene:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 106.0, 11.0, &format!("{} No. {}", ret.tipo_comprobante, ret.numero_comprobante));

    // Sujeto pasivo
    hline(&layer, MARGEN, PAGE_H - 112.0, PAGE_W - 2.0 * MARGEN, 0.6);
    t(&layer, &bold, MARGEN, PAGE_H - 122.0, 11.0, "Sujeto Pasivo / Proveedor:");
    t(&layer, &font, MARGEN + 60.0, PAGE_H - 122.0, 11.0, &ret.proveedor_nombre);
    t(&layer, &font, MARGEN, PAGE_H - 132.0, 10.0, &format!("RUC / Cedula: {}", ret.cedula));
    if !ret.referencia.is_empty() {
        t(&layer, &font, MARGEN, PAGE_H - 142.0, 10.0, &format!("Referencia: {}", ret.referencia));
    }

    // Tabla de retenciones
    hline(&layer, MARGEN, PAGE_H - 148.0, PAGE_W - 2.0 * MARGEN, 0.6);
    t(&layer, &bold, 20.0, PAGE_H - 158.0, 9.0, "IMPUESTO");
    t(&layer, &bold, 80.0, PAGE_H - 158.0, 9.0, "BASE IMPONIBLE");
    t(&layer, &bold, 150.0, PAGE_H - 158.0, 9.0, "%");
    t(&layer, &bold, 190.0, PAGE_H - 158.0, 9.0, "VALOR");

    let mut y = PAGE_H - 168.0;
    if ret.valor_renta > 0.0 {
        t(&layer, &font, 20.0, y, 9.0, "Renta");
        t(&layer, &font, 80.0, y, 9.0, &format!("{:.2}", ret.base_imp_renta));
        t(&layer, &font, 150.0, y, 9.0, &format!("{:.2}", ret.porcentaje_renta));
        t(&layer, &font, 190.0, y, 9.0, &format!("{:.2}", ret.valor_renta));
        y -= 10.0;
    }
    if ret.valor_iva > 0.0 {
        t(&layer, &font, 20.0, y, 9.0, "IVA");
        t(&layer, &font, 80.0, y, 9.0, &format!("{:.2}", ret.base_imp_iva));
        t(&layer, &font, 150.0, y, 9.0, &format!("{:.2}", ret.porcentaje_iva));
        t(&layer, &font, 190.0, y, 9.0, &format!("{:.2}", ret.valor_iva));
        y -= 10.0;
    }

    hline(&layer, MARGEN, y - 4.0, PAGE_W - 2.0 * MARGEN, 0.4);
    let ty = y - 20.0;
    t(&layer, &bold, 130.0, ty, 13.0, "TOTAL RETENIDO:");
    t(&layer, &bold, 190.0, ty, 13.0, &format!("{:.2}", ret.valor_renta + ret.valor_iva));

    t(&layer, &font, MARGEN, 60.0, 9.0, "Nota: Este comprobante debe ser declarado en el formulario 103 del SRI");

    // Firmas
    hline(&layer, MARGEN, 34.0, 60.0, 0.4);
    t(&layer, &font, MARGEN, 28.0, 9.0, "Firma del agente de retencion");

    let ruta = nombre_archivo("Retencion", &ret.numero);
    let file = File::create(&ruta).map_err(|e| format!("No se pudo crear el archivo: {}", e))?;
    doc.save(&mut BufWriter::new(file)).map_err(|e| format!("Error al guardar PDF: {}", e))?;
    Ok(ruta)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace('"', "&quot;").replace('\'', "&apos;")
}

pub fn generar_xml_factura(
    venta: &Venta,
    lineas: &[VentaDetalle],
    cliente: Option<&Cliente>,
    empresa: &Configuracion,
) -> Result<PathBuf, String> {
    let (cedula, telefono, direccion) = datos_contacto(cliente);
    let tipo_id = if cedula.trim().len() >= 13 { "04" } else if !cedula.trim().is_empty() { "05" } else { "07" };
    let id_comprador = if cedula.trim().is_empty() { "9999999999999" } else { &cedula };

    let secuencial: String = venta.folio.chars().filter(|c| c.is_ascii_digit()).collect();
    let secuencial = if secuencial.len() >= 9 {
        secuencial[secuencial.len() - 9..].to_string()
    } else {
        format!("{:0>9}", secuencial)
    };

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<factura id=\"comprobante\" version=\"1.0.0\">\n");
    xml.push_str("  <infoTributaria>\n");
    xml.push_str("    <ambiente>1</ambiente>\n");
    xml.push_str("    <tipoEmision>1</tipoEmision>\n");
    xml.push_str(&format!("    <razonSocial>{}</razonSocial>\n", xml_escape(&empresa.empresa_nombre)));
    xml.push_str(&format!("    <ruc>{}</ruc>\n", xml_escape(&empresa.ruc)));
    xml.push_str("    <codDoc>01</codDoc>\n");
    xml.push_str("    <estab>001</estab>\n");
    xml.push_str("    <ptoEmi>001</ptoEmi>\n");
    xml.push_str(&format!("    <secuencial>{}</secuencial>\n", secuencial));
    xml.push_str(&format!("    <dirMatriz>{}</dirMatriz>\n", xml_escape(&empresa.direccion)));
    xml.push_str("  </infoTributaria>\n");
    xml.push_str("  <infoFactura>\n");
    xml.push_str(&format!("    <fechaEmision>{}</fechaEmision>\n", venta.fecha));
    xml.push_str("    <obligadoContabilidad>SI</obligadoContabilidad>\n");
    xml.push_str(&format!("    <tipoIdentificacionComprador>{}</tipoIdentificacionComprador>\n", tipo_id));
    xml.push_str(&format!("    <razonSocialComprador>{}</razonSocialComprador>\n", xml_escape(&venta.cliente_nombre)));
    xml.push_str(&format!("    <identificacionComprador>{}</identificacionComprador>\n", xml_escape(id_comprador)));
    if !direccion.is_empty() {
        xml.push_str(&format!("    <direccionComprador>{}</direccionComprador>\n", xml_escape(&direccion)));
    }
    xml.push_str(&format!("    <totalSinImpuestos>{:.2}</totalSinImpuestos>\n", venta.subtotal));
    xml.push_str(&format!("    <totalDescuento>{:.2}</totalDescuento>\n", venta.descuento));
    xml.push_str("    <totalConImpuestos>\n");
    if venta.impuesto > 0.0 {
        xml.push_str("      <totalImpuesto>\n");
        xml.push_str("        <codigo>2</codigo>\n");
        xml.push_str(&format!("        <codigoPorcentaje>{}</codigoPorcentaje>\n", if empresa.iva > 12.0 { "3" } else { "2" }));
        xml.push_str(&format!("        <baseImponible>{:.2}</baseImponible>\n", venta.subtotal));
        xml.push_str(&format!("        <valor>{:.2}</valor>\n", venta.impuesto));
        xml.push_str("      </totalImpuesto>\n");
    }
    xml.push_str("    </totalConImpuestos>\n");
    xml.push_str("    <propina>0.00</propina>\n");
    xml.push_str(&format!("    <importeTotal>{:.2}</importeTotal>\n", venta.total));
    xml.push_str("    <moneda>DOLAR</moneda>\n");
    xml.push_str("  </infoFactura>\n");
    xml.push_str("  <detalles>\n");
    for det in lineas {
        xml.push_str("    <detalle>\n");
        let codigo = det.producto_id.map(|i| format!("P{:05}", i)).unwrap_or_default();
        xml.push_str(&format!("      <codigoPrincipal>{}</codigoPrincipal>\n", xml_escape(&codigo)));
        xml.push_str(&format!("      <descripcion>{}</descripcion>\n", xml_escape(&det.producto_nombre)));
        xml.push_str(&format!("      <cantidad>{}</cantidad>\n", det.cantidad));
        xml.push_str(&format!("      <precioUnitario>{:.2}</precioUnitario>\n", det.precio_unitario));
        xml.push_str(&format!("      <descuento>{:.2}</descuento>\n", det.descuento));
        xml.push_str(&format!("      <precioTotalSinImpuesto>{:.2}</precioTotalSinImpuesto>\n", det.importe));
        xml.push_str("    </detalle>\n");
    }
    xml.push_str("  </detalles>\n");
    xml.push_str("  <infoAdicional>\n");
    if !telefono.is_empty() {
        xml.push_str(&format!("    <campoAdicional nombre=\"Telefono\">{}</campoAdicional>\n", xml_escape(&telefono)));
    }
    xml.push_str("  </infoAdicional>\n");
    xml.push_str("</factura>\n");

    let ruta = nombre_archivo("XML_Factura", &venta.folio).with_extension("xml");
    std::fs::write(&ruta, xml).map_err(|e| format!("Error al guardar XML: {}", e))?;
    Ok(ruta)
}

pub fn abrir_pdf(ruta: &Path) {
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", ""])
        .arg(ruta)
        .spawn();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generar_documentos_prueba() {
        let v = Venta {
            id: 1, folio: "V-0001".into(), cliente_id: None,
            cliente_nombre: "Juan Perez".into(), fecha: "2026-07-31".into(),
            subtotal: 100.0, impuesto: 12.0, descuento: 5.0, total: 107.0,
            saldo_pendiente: 0.0, tipo: "contado".into(), estado: "completada".into(),
            metodo_pago: None, notas: "Nota de prueba".into(),
            fecha_vencimiento: None, fecha_pago: None,
        };
        let det = vec![VentaDetalle {
            id: 1, venta_id: 1, producto_id: None, descripcion: None,
            producto_nombre: "Producto de prueba".into(), cantidad: 2,
            precio_unitario: 50.0, descuento: 0.0, importe: 100.0,
        }];
        let r = generar_factura(&v, &det, None, &Configuracion::default()).unwrap();
        assert!(r.exists());
        let r2 = generar_garantia(&v, &det, "2026-07-31", "2027-07-31", None, &Configuracion::default()).unwrap();
        assert!(r2.exists());
    }
}
