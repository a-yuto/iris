pub struct Colum<T> {
    pub name: String,
    pub data: Vec<T>,
}
pub push_colum(col: &Colum<f64>) -> Colum<f64>{
    let new_col = col.data
}
pub fn print_colum(col: &Colum<f64>) {
    println!("{}",col.name);
    println!("{:?}",col.data);
}
pub fn print_stringcolum(col: &Colum<String>) {
    println!("{}",col.name);
    println!("{:?}",col.data);
}



