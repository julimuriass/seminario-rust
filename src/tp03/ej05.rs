struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: u32,
}

impl Producto {
    fn new(nombre: String, precio_bruto: f64, id: u32) -> Producto {
        Producto {
            nombre,
            precio_bruto,
            id,
        }
    }

    fn calcular_impuestos(&self, tax_percentage: f64) -> f64 {
        (tax_percentage/100.0)/self.precio_bruto
    }

    fn aplicar_descuento(&self, discount_percentage: f64) -> f64 {
        (discount_percentage/100.0)/self.precio_bruto
    }

    fn calcular_precio_total(&self, tax_percentage: Option<f64>, discount_percentage: Option<f64>) -> f64 {
        let tax = tax_percentage.map(|t| t/100.0).unwrap_or(0.0); 
        let discount= discount_percentage.map(|d| d/100.0).unwrap_or(0.0);
        self.precio_bruto+tax-discount
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_calculate_taxes() {
        let product= Producto::new(String::from("Agua."), 500.0, 123);
        assert_eq!(product.calcular_impuestos(100.0), 0.002);
    }

    #[test]
    fn test_apply_discount() {
        let product= Producto::new(String::from("Agua."), 500.0, 123);
        assert_eq!(product.aplicar_descuento(0.0), 0.0);
    }

    #[test]
    fn test_total_price() {
        let product= Producto::new(String::from("Agua."), 500.0, 123);
        assert_eq!(product.calcular_precio_total(None, None), 500.0);
        assert_eq!(product.calcular_precio_total(Some(500.0), Some(250.0)),502.5);
    }

}



