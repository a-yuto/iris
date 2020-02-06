extern crate csv;
use csv::*;
extern crate rand;
use rand::Rng;

use std::fs::File;
/*colu に対してpushとか色々関数を定義するべき*/
trait ColumFunction<T> {
    fn print_colum(col:&Colum<T>);
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


pub struct IrisDf {
    itr: Colum<usize>,
    sepal_length: Colum<f64>, 
    sepal_width:  Colum<f64>, 
    petal_length: Colum<f64>,
    petal_width:  Colum<f64>, 
    species:      Colum<String>,
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
fn print_irisdf(df: &IrisDf) {
    let length = df.itr.data.len();;
    println!("{}:{}:{}:{}:{}:`{}",
    df.itr.name,df.sepal_length.name,df.sepal_width.name,df.petal_length.name,df.petal_width.name,df.species.name);
    for i in (0..length) {
        println!("{:?}:  {:?}:          {:?}:        {:?}:        {:?}:      {:?}",
        df.itr.data[i],df.sepal_length.data[i],df.sepal_width.data[i],df.petal_length.data[i],df.petal_width.data[i],df.species.data[i]);
    }
}

fn data_size(data: &IrisDf) -> (usize,usize) {
    let tate = data.itr.data.len();
    let yoko = 6;
    (tate,yoko)
}

fn sep_tt(data: &IrisDf) {
    let (_tate,_yoko) = data_size(&data);
    let mut _test_sepal_length: Colum<f64> = Colum {
        name: "sepal_length".to_string(),
        data: Vec::new()
    };
    let mut _train_sepal_length: Colum<f64> = Colum {
        name: "sepal_length".to_string(),
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
        let sepal_length = data.sepal_length.data[itr];
        let      species = &data.species.data[itr];
        if random_num % 2 == 0 {
            _test_sepal_length.data.push(sepal_length);
            _test_species.data.push(species.to_string());
        } else {
            _train_sepal_length.data.push(sepal_length);
            _train_species.data.push(species.to_string());
        }
    }
    print_colum(&_test_sepal_length);
    print_stringcolum(&_test_species);
    print_colum(&_train_sepal_length);
    print_stringcolum(&_train_species);
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
    println!("{:?}",_table.headers());
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
    sep_tt(&data);
}
