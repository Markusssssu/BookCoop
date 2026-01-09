use rust_decimal::Decimal; 
pub struct Inventory {
    pub book_id: i32,        
    pub price: Decimal,      
    pub stock_count: i32,    
    pub cover_url: Option<String>,
}