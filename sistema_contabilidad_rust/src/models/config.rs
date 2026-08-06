use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuracion {
    pub empresa_nombre: String,
    pub ruc: String,
    pub direccion: String,
    pub telefono: String,
    pub email: String,
    pub ciudad: String,
    pub iva: f64,
}

impl Default for Configuracion {
    fn default() -> Self {
        Configuracion {
            empresa_nombre: "MI NEGOCIO CIA. LTDA.".into(),
            ruc: "1700000000001".into(),
            direccion: "Av. Principal, Edif. Central, Piso 1".into(),
            telefono: "02-0000000".into(),
            email: "contacto@minegocio.com".into(),
            ciudad: "Quito".into(),
            iva: 15.0,
        }
    }
}

impl Configuracion {
    pub fn iniciales(&self) -> String {
        let mut iniciales = String::new();
        for palabra in self.empresa_nombre.split_whitespace() {
            let letra = palabra.chars().next();
            if let Some(c) = letra {
                if c.is_alphabetic() {
                    iniciales.push(c.to_uppercase().next().unwrap_or(c));
                }
            }
        }
        if iniciales.is_empty() {
            "SC".to_string()
        } else {
            iniciales.chars().take(3).collect()
        }
    }

    pub fn nombre_corto(&self) -> String {
        if self.empresa_nombre.trim().is_empty() {
            "Sistema Contabilidad".to_string()
        } else {
            format!("Sistema {}", self.empresa_nombre)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i64,
    pub nombre_usuario: String,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoResultados {
    pub ventas_total: f64,
    pub costo_ventas: f64,
    pub utilidad_bruta: f64,
    pub gastos_total: f64,
    pub utilidad_neta: f64,
    pub ingresos_otros: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MayorLinea {
    pub fecha: String,
    pub concepto: String,
    pub debe: f64,
    pub haber: f64,
    pub saldo: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaldoPendiente {
    pub nombre: String,
    pub total: f64,
    pub dias: i64,
}
