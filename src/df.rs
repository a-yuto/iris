extern crate csv;
use csv::*;
extern crate rand;
use rand::Rng;


use crate::colum::*;
pub struct IrisDf {
    pub itr:         Colum<usize>,
    pub sepal_length: Colum<f64>,
    pub sepal_width:  Colum<f64>,
    pub petal_length:Colum<f64>,
    pub petal_width: Colum<f64>,
    pub species:     Colum<String>,
}

struct single_data {
    pub itr:         usize,
    pub sepal_length:f64,
    pub sepal_width: f64,
    pub petal_length:f64,
    pub petal_width: f64,
    pub species:     String,
}
pub fn print_irisdf(df: &IrisDf) {
    let length = df.itr.data.len();
    println!("{}:{}:{}:{}:{}:`{}",
    df.itr.name,df.sepal_length.name,df.sepal_width.name,df.petal_length.name,df.petal_width.name,df.species.name);
    for i in 0..length {
        println!("{:?}:  {:?}:          {:?}:        {:?}:        {:?}:      {:?}",
        df.itr.data[i],df.sepal_length.data[i],df.sepal_width.data[i],df.petal_length.data[i],df.petal_width.data[i],df.species.data[i]);
    }
}

pub fn data_size(data: &IrisDf) -> (usize,usize) {
    let tate = data.itr.data.len();
    let yoko = 6;
    (tate,yoko)
}
pub fn sep_tt(data: &IrisDf) -> (IrisDf,IrisDf){
    let (_tate,_yoko) = data_size(&data);

    let mut _test_itr :Colum<usize> = Colum {
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
    pub sentosa   : f64,
    pub versicolor: f64,
    pub virginica : f64
}

pub fn mean_predict(df: &IrisDf) -> Box<Fn(f64) -> String> {
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

pub fn correct_ans_rate(test: &IrisDf, predict: Box<Fn(f64) -> String>) -> f64 {
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

pub fn read_iris_csv(path: &str) -> IrisDf {
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
    IrisDf {
        itr: _itr,
        sepal_length: _sepal_length,
        sepal_width:  _sepal_width,
        petal_length: _petal_length,
        petal_width:  _petal_width,
        species:      _species,
    }
}
/*
pub fn select_target(df: &IrisDf,target: &str) -> IrisDf {
    fn select(df: &IrisDf,ans: &IrisDf,target,itr: usize) -> IrisDf {
        let (tate,yoko) = data_size(&df);
        if tate == itr {
            ans
        }
        else if &df.species.data[itr] == target {
            IrisDf {
                itr: df.itr,
                sepal_length: _sepal_length,
                sepal_width:  _sepal_width,
                petal_length: _petal_length,
                petal_width:  _petal_width,
                species:      _species,
            }
        }
    }
}*/
