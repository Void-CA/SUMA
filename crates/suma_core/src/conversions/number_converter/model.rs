#[derive(Debug, Clone)]
pub struct NumberConverter {
    pub value: i64,
    pub history: Vec<String>,
}

impl NumberConverter {
    pub fn new(value: i64) -> Self {
        NumberConverter {
            value,
            history: vec![format!("Inicial: decimal={}", value)],
        }
    }

    // Métodos que no modifican el estado deberían usar &self
    pub fn as_binary(&self) -> String {
        format!("{:b}", self.value)
    }
    
    pub fn as_hex(&self) -> String {
        format!("{:X}", self.value)
    }
    
    pub fn as_letters(&self) -> String {
        let mut letters = String::new();
        let mut num = self.value;
        
        while num > 0 {
            let rem = ((num - 1) % 26) as u8;
            letters.insert(0, (b'A' + rem) as char);
            num = (num - 1) / 26;
        }
        letters
    }

    // Métodos que agregan al historial (modifican estado)
    pub fn to_binary(&mut self) -> String {
        let binary = self.as_binary();
        self.history.push(format!("Convertido a binario: {}", binary));
        binary
    }

    pub fn to_hex(&mut self) -> String {
        let hex = self.as_hex();
        self.history.push(format!("Convertido a hexadecimal: {}", hex));
        hex
    }

    pub fn to_letters(&mut self) -> String {
        let letters = self.as_letters();
        self.history.push(format!("Convertido a letras: {}", letters));
        letters
    }

    // Mejor rendimiento: evitar clone del Vec
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
    
    // Para obtener una copia si es necesario
    pub fn history_cloned(&self) -> Vec<String> {
        self.history.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let conv = NumberConverter::new(42);
        assert_eq!(conv.value, 42);
        assert_eq!(conv.history.len(), 1);
        assert!(conv.history[0].contains("42"));
    }

    #[test]
    fn test_as_binary() {
        assert_eq!(NumberConverter::new(0).as_binary(), "0");
        assert_eq!(NumberConverter::new(1).as_binary(), "1");
        assert_eq!(NumberConverter::new(42).as_binary(), "101010");
        assert_eq!(NumberConverter::new(255).as_binary(), "11111111");
    }

    #[test]
    fn test_as_hex() {
        assert_eq!(NumberConverter::new(0).as_hex(), "0");
        assert_eq!(NumberConverter::new(10).as_hex(), "A");
        assert_eq!(NumberConverter::new(255).as_hex(), "FF");
        assert_eq!(NumberConverter::new(3735928559).as_hex(), "DEADBEEF");
    }

    #[test]
    fn test_as_letters() {
        assert_eq!(NumberConverter::new(1).as_letters(), "A");
        assert_eq!(NumberConverter::new(26).as_letters(), "Z");
        assert_eq!(NumberConverter::new(27).as_letters(), "AA");
        assert_eq!(NumberConverter::new(52).as_letters(), "AZ");
        assert_eq!(NumberConverter::new(53).as_letters(), "BA");
    }

    #[test]
    fn test_to_binary_updates_history() {
        let mut conv = NumberConverter::new(42);
        assert_eq!(conv.to_binary(), "101010");
        assert_eq!(conv.history.len(), 2);
        assert!(conv.history[1].contains("101010"));
    }

    #[test]
    fn test_to_hex_updates_history() {
        let mut conv = NumberConverter::new(255);
        assert_eq!(conv.to_hex(), "FF");
        assert_eq!(conv.history.len(), 2);
    }

    #[test]
    fn test_to_letters_updates_history() {
        let mut conv = NumberConverter::new(1);
        assert_eq!(conv.to_letters(), "A");
        assert_eq!(conv.history.len(), 2);
    }

    #[test]
    fn test_negative_numbers() {
        let conv = NumberConverter::new(-42);
        assert_eq!(conv.as_binary(), format!("{:b}", -42i64));
        assert_eq!(conv.as_hex(), format!("{:X}", -42i64));
        assert_eq!(conv.as_letters(), "");
    }

    #[test]
    fn test_zero_letters() {
        assert_eq!(NumberConverter::new(0).as_letters(), "");
    }

    #[test]
    fn test_get_history() {
        let mut conv = NumberConverter::new(10);
        conv.to_binary();
        conv.to_hex();
        assert_eq!(conv.get_history().len(), 3);
    }
}