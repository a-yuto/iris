
use iris::df::*;

fn main() {
    let data = read_iris_csv("test_data.csv");
    print_irisdf(&data);
    
}
