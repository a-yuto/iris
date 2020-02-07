extern crate csv;
use csv::*;
extern crate rand;
use rand::Rng;

use std::fs::File;
/*  corrとsを書きましょう。
 *
 *
 *
 * 
 */
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

struct Colum<T> {
    name: String,
    data: Vec<T>,
}
fn print_colum(col: &Colum<f64>) {
    println!("{}",col.name);
    println!("{:?}",col.data);
}
fn print_stringcolum(col: &Colum<String>) {
    println!("{}",col.name);
    println!("{:?}",col.data);
}
    
fn mean(col: &Colum<f64>) -> Colum<f64> {
    let  sum: f64 = col.data.iter().sum::<f64>();
    let mean: f64 = sum  / col.data.len() as f64;
    let _col = &col.name;
    Colum {
        name: _col.to_string(),
        data: vec![mean]
    }
}
fn var(col: &Colum<f64>) -> Colum<f64> {
    fn cal(colum: &Colum<f64>,itr:usize,ans:f64) -> f64{
        match itr == colum.data.len() {
            true  => ans,
            false => cal(&colum,itr + 1, ans + (colum.data[itr] - mean(&colum).data[0]) * (colum.data[itr] - mean(&colum).data[0]))
        }
    }
    Colum{
        name: col.name.to_string(),
        data: vec![cal(&col,0,0.0)]
    }
}


pub struct IrisDf {
    itr:         Colum<usize>,
    sepal_length: Colum<f64>, 
    sepal_width:  Colum<f64>, 
    petal_length:Colum<f64>,
    petal_width: Colum<f64>, 
    species:     Colum<String>,
}

fn print_irisdf(df: &IrisDf) {
    let length = df.itr.data.len();
    println!("{}:{}:{}:{}:{}:`{}",
    df.itr.name,df.sepal_length.name,df.sepal_width.name,df.petal_length.name,df.petal_width.name,df.species.name);
    for i in 0..length {
        println!("{:?}:  {:?}:          {:?}:        {:?}:        {:?}:      {:?}",
        df.itr.data[i],df.sepal_length.data[i],df.sepal_width.data[i],df.petal_length.data[i],df.petal_width.data[i],df.species.data[i]);
    }
}

fn data_size(data: &IrisDf) -> (usize,usize) {
    let tate = data.itr.data.len();
    let yoko = 6;
    (tate,yoko)
}

fn sep_tt(data: &IrisDf) -> (IrisDf,IrisDf){
    let (_tate,_yoko) = data_size(&data);
    
    let mut _test_itr: Colum<usize> = Colum {
        name: "itr".to_string(),
        data: Vec::new()
    };
    let mut _train_itr: Colum<usize> = Colum {
        name: "itr".to_string(),
        data: Vec::new()
    };
    let mut _test_sepal_length: Colum<f64> = Colum {
        name: "sepal_length".to_string(),
        data: Vec::new()
    };
    let mut _train_sepal_length: Colum<f64> = Colum {
        name: "sepal_length".to_string(),
        data: Vec::new()
    };
    
    let mut _test_sepal_width: Colum<f64> = Colum {
        name: "sepal_width".to_string(),
        data: Vec::new()
    };
    let mut _train_sepal_width: Colum<f64> = Colum {
        name: "sepal_width".to_string(),
        data: Vec::new()
    };
    
    let mut _test_petal_length: Colum<f64> = Colum {
        name: "petal_length".to_string(),
        data: Vec::new()
    };
    let mut _train_petal_length: Colum<f64> = Colum {
        name: "petal_length".to_string(),
        data: Vec::new()
    };
    
    let mut _test_petal_width: Colum<f64> = Colum {
        name: "petal_width".to_string(),
        data: Vec::new()
    };
    let mut _train_petal_width: Colum<f64> = Colum {
        name: "petal_width".to_string(),
        data: Vec::new()
    };
    
    let mut _test_species: Colum<String> = Colum {
        name: "species".to_string(),
        data: Vec::new()
    };
    let mut _train_species: Colum<String> = Colum {
        name: "species".to_string(),
        data: Vec::new()
    };
    for itr in 0.._tate {
        let mut    rng      = rand::thread_rng();
        let random_num: i32 = rng.gen();
        let         _itr = data.itr.data[itr];
        let sepal_length = data.sepal_length.data[itr];
        let sepal_width  = data.sepal_width.data[itr];
        let petal_length = data.sepal_length.data[itr];
        let petal_width  = data.sepal_width.data[itr];
        let      species = &data.species.data[itr];
        if random_num % 2 == 0 {
            _test_itr         .data.push(_itr);
            _test_sepal_length.data.push(sepal_length);
            _test_sepal_width .data.push(sepal_width);
            _test_petal_length.data.push(petal_length);
            _test_petal_width .data.push(petal_width);
            _test_species     .data.push(species.to_string());
        } else {
            _train_itr         .data.push(_itr);
            _train_sepal_length.data.push(sepal_length);
            _train_sepal_width .data.push(sepal_width);
            _train_petal_length.data.push(petal_length);
            _train_petal_width .data.push(petal_width);
            _train_species     .data.push(species.to_string());
        }
    }
    (IrisDf {
        itr:          _test_itr,
        sepal_length: _test_sepal_length,
        sepal_width:  _test_sepal_width,
        petal_length: _test_petal_length,
        petal_width:  _test_petal_width,
        species:      _test_species,
    },
    IrisDf {
        itr:          _train_itr,
        sepal_length: _train_sepal_length,
        sepal_width:  _train_sepal_width,
        petal_length: _train_petal_length,
        petal_width:  _train_petal_width,
        species:      _train_species,
    })
}

pub struct IrisSta {
    sentosa   : f64,
    versicolor: f64,
    virginica : f64
}

fn mean_predict(df: &IrisDf) -> Box<Fn(f64) -> String> {
    let (_tate,_yoko) = data_size(&df);
    println!("test_tate = {}",_tate);
    let mut _sepal_width_mean = IrisSta {
        sentosa:    0.0,
        versicolor: 0.0,
        virginica:  0.0
    };
    let mut _species_num = IrisSta {
        sentosa:    0.0,
        versicolor: 0.0,
        virginica:  0.0
    };
    for i in 0.._tate {
        if &df.species.data[i] == "setosa" {
            _sepal_width_mean.sentosa += df.sepal_width.data[i];
            _species_num.sentosa      += 1.0;
        } else if &df.species.data[i] == "versicolor"{
            _sepal_width_mean.versicolor += df.sepal_width.data[i];
            _species_num.versicolor      += 1.0;
        } else {
            _sepal_width_mean.virginica  += df.sepal_width.data[i];
            _species_num.virginica       += 1.0;
        }
    }
    _sepal_width_mean.sentosa    /= _species_num.sentosa;
    _sepal_width_mean.versicolor  /= _species_num.versicolor;
    let bound_sen_ver = (_sepal_width_mean.sentosa    + _sepal_width_mean.versicolor ) / 2.0;    
    Box::new(move |_sepal_width|
        if _sepal_width < bound_sen_ver {
            "versicolor".to_string()
        } else {
            "setosa".to_string()
        }
    )
}
fn single_liner_regression(df: &IrisDf) {
    let x1 = &df.sepal_length;
    let x2 = &df.sepal_length;
    let x1_mean = mean(&x1);
    let x2_mean = mean(&x2);
}
fn correct_ans_rate(test: &IrisDf, predict: Box<Fn(f64) -> String>) -> f64 {
    let mut correct_num = 0.0;
    let (_tate,_yoko) = data_size(&test);
    for i in 0.._tate {
        let pre_result = predict(test.sepal_width.data[i]);
        let answer     = &test.species.data[i];
        println!("predict result is {}, answer is {}",pre_result,answer);
        if &pre_result == answer {correct_num += 1.0;}
    }
    100.0 * (correct_num / _tate as f64)
}


fn main() {
    let path = "test_data.csv";
    let mut _table = csv::Reader::from_path(path).unwrap();   
    
    let mut _itr: Colum<usize> = Colum {
        name: "itr".to_string(),
        data: Vec::new()
    };
    let mut _sepal_length: Colum<f64> = Colum {
        name: "sepal_length".to_string(),
        data: Vec::new()
    };
    let mut _sepal_width: Colum<f64> = Colum {
        name: "sepal_width".to_string(),
        data: Vec::new()
    };
    let mut _petal_length: Colum<f64> = Colum {
        name: "petal_lengt".to_string(),
        data: Vec::new()
    };
    let mut _petal_width: Colum<f64> = Colum {
        name: "petal_width".to_string(),
        data: Vec::new()
    };
    let mut _species: Colum<String> = Colum {
        name: "species".to_string(),
        data: Vec::new()
    };
    for _line in _table.records() {
        let mut cnt_yoko = 0;
        for _val in _line.unwrap().iter() {
            match cnt_yoko {
                0 =>          _itr.data.push(_val.parse::<usize>().unwrap()),
                1 => _sepal_length.data.push(_val.parse::<f64>().unwrap()),
                2 =>  _sepal_width.data.push(_val.parse::<f64>().unwrap()),
                3 => _petal_length.data.push(_val.parse::<f64>().unwrap()),
                4 =>  _petal_width.data.push(_val.parse::<f64>().unwrap()),
                5 =>      _species.data.push(_val.to_string()),
                _ => panic!("ぷえーーーーーー"),
            }
            cnt_yoko += 1;
        }
    }
    let data = IrisDf {
        itr: _itr,
        sepal_length: _sepal_length,
        sepal_width:  _sepal_width,
        petal_length: _petal_length,
        petal_width:  _petal_width,
        species:      _species,    
    };
    let (test,train) = sep_tt(&data);
    let predict = mean_predict(&train);
    println!("predict result is {}",correct_ans_rate(&test,predict));
}
