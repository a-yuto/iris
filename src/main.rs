extern crate csv;
use csv::*;
extern crate rand;
use rand::Rng;

use std::fs::File;
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


pub struct IrisDf {
    itr:         Colum<usize>,
    sepal_length: Colum<f64>, 
    sepal_width:  Colum<f64>, 
    petal_length:Colum<f64>,
    petal_width: Colum<f64>, 
    species:     Colum<String>,
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

fn sep_tt(data: &IrisDf) -> (IrisDf,IrisDf){
    let (_tate,_yoko) = data_size(&data);
    
    let mut _test_itr: Colum<usize> = Colum {
        name: "sepal_length".to_string(),
        data: Vec::new()
    };
    let mut _train_itr: Colum<usize> = Colum {
        name: "sepal_length".to_string(),
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
    ;
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
    let (test,train) = sep_tt(&data);
    print_irisdf(&test);
    println!("");
    print_irisdf(&train);
    
    let (test_tate,test_yoko) = data_size(&test);
    let mut _mean = IrisSta {
        sentosa:    0.0,
        versicolor: 0.0,
        virginica:  0.0
    };
    for i in (0..test_tate) {
        if &test.species.data[i] == "setosa" {
            _mean.sentosa   += test.sepal_width.data[i];
        } else if &test.species.data[i] == "versicolor"{
            _mean.versicolor += test.sepal_width.data[i];
        } else {
            _mean.virginica  += test.sepal_width.data[i];
        }
    }
    println!("sntosa's sepal width is {}"    ,_mean.sentosa);
    println!("versicolor's sepal width is {}",_mean.versicolor);
    println!("viginica's sepal width is {}"  ,_mean.virginica);
}
