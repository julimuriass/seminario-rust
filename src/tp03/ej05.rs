struct Product {
    name: String,
    gross_price: f64,
    id: u32,
}

impl Product {
    fn new(name: String, gross_price: f64, id: u32) -> Product {
        Product {
            name,
            gross_price,
            id,
        }
    }

    fn calculate_taxes(&self, tax_percentage: f64) -> f64 {
        (tax_percentage/100.0)/self.gross_price
    }

    fn apply_discount(&self, discount_percentage: f64) -> f64 {
        (discount_percentage/100.0)/self.gross_price
    }

    fn calculate_total_price(&self, tax_percentage: Option<f64>, discount_percentage: Option<f64>) -> f64 {
        let tax = tax_percentage.map(|t| t/100.0).unwrap_or(0.0); //preg if okay???
        let discount= discount_percentage.map(|d| d/100.0).unwrap_or(0.0);
        self.gross_price+tax-discount
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_calculate_taxes() {
        let product= Product::new(String::from("Agua."), 500.0, 123);
        assert_eq!(product.calculate_taxes(100.0), 0.002);
    }

    #[test]
    fn test_apply_discount() {
        let product= Product::new(String::from("Agua."), 500.0, 123);
        assert_eq!(product.apply_discount(0.0), 0.0);
    }

    #[test]
    fn test_total_price() {
        let product= Product::new(String::from("Agua."), 500.0, 123);
        assert_eq!(product.calculate_total_price(None, None), 500.0);
        assert_eq!(product.calculate_total_price(Some(500.0), Some(250.0)),502.5);
    }

}



