// Bikin 1 function yang menerima 1 parameter yaitu &str 
//lalu tugas dari function tersebut adalah 
///me-reverse string slice (&str) menjadi String struct. 
/// Lakukan dengan
// menggunakan metode for..in looping atau menggunakan built-in func.
use std::io;

//Struct String
#[derive(Debug)]
struct Container {
    name : String,
}

//stdio input string
fn read_string() -> Option <String> {
    let mut input = String::new();

    while io::stdin()
        .read_line(&mut input)
        .is_err(){
            println!("Invalid Input!")
        }
    //ubah ke &str slice string
    let input_parsed = input.trim().to_owned(); 
    
    if &input == "" {
        None
    } else {
        Some(input_parsed)
    }
}

//ubah parameter input &str ke String owned, push ke struct
//kemudian add ke struct
fn reverse_string(param : &str) -> Container {
    let name = param.to_owned();

    let container = Container{name};
    return container
}

fn main() {
    let mut vec_container: Vec<Container> = Vec::new();

    println!("");
    println!("===== Reverse String =====");
    println!("===== By Henry Hamilton Prasetya =====");
    println!("Input nothing to exit program & view all strings.");

    loop {
        println!("Please input a string >>>");
        
        let input_1 = match read_string(){
            Some(input_1) => input_1,
            None => return        
        };

        if input_1 == "" {
            break;
        }
        println!("YOUR INPUT : {:?}", input_1);

        let struct_in = reverse_string(&input_1);
        vec_container.push(struct_in);
    }

    for container in vec_container {
        println!("{:?}", container);
    }
}