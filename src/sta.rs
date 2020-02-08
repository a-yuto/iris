use crate::colum::*; 


pub fn mean(col: &Colum<f64>) -> Colum<f64> {
    let  sum: f64 = col.data.iter().sum::<f64>();
    let mean: f64 = sum  / col.data.len() as f64;
    let _col = &col.name;
    Colum {
        name: _col.to_string(),
        data: vec![mean]
    }
}

pub fn var(col: &Colum<f64>) -> Colum<f64> {
    fn cal(colum: &Colum<f64>,itr:usize,ans:f64) -> f64{
        match itr == colum.data.len() {
            true  => ans  / itr as f64,
            false => cal(&colum,itr + 1, ans + (colum.data[itr] - mean(&colum).data[0]) * (colum.data[itr] - mean(&colum).data[0]))
        }
    }
    Colum{
        name: col.name.to_string(),
        data: vec![cal(&col,0,0.0)]
    }
}

pub fn corvar(col1: &Colum<f64>,col2: &Colum<f64>) -> Colum<f64> {
    fn cal(col1: &Colum<f64>,col2: &Colum<f64>,itr:usize,ans:f64) -> f64 {
        let _col1_mean = mean(&col1).data[0];
        let _col2_mean = mean(&col2).data[0];
        match itr == col1.data.len() {
            true  => ans,
            false => cal(&col1,&col2,itr + 1,
                         ans + (col1.data[itr] - _col1_mean) * (col2.data[itr] - _col2_mean))
        }
    }
    Colum {
        name: col1.name.to_string() + "and" + &col2.name + "corvar",
        data: vec![cal(&col1,&col2,0,0.0)]
    }
}
pub fn stdev(col: &Colum<f64>) -> Colum<f64> {
    Colum{
        name: col.name.to_string(),
        data: vec![var(&col).data[0].floor()]
    }
}


