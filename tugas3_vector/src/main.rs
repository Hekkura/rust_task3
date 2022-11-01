// Bikin 2 function yang menerima 1 parameter yaitu Vector of integer32 
//lalu tugas dari function ini adalah mencari dari 
//beberapa element didalam parameter yang merupakan
// bilangan prima.
// Hasil dari pemrosesan tersebut berbentuk data Vector of boolean.
// Terapkan metode dengan for..in looping dan juga metode built-in function

//function 1 --> Input vector int32, sorting dulu, push ke struct 
//function 2 --> Input vector int32, output vec boolean, tentuin mana yang prima
//helper function : Input string, Input int32, 

//struct isi 2 vector, satu vec i32, satunya vec bool

use std::io;

//helper function buat read string
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

//helper function read integer
fn read_integer() -> Option <i32> {
    loop {
        let input = match read_string(){
            Some(input) => input,
            None => return None
        };

        if &input =="" {
            return None;
        }

        let input_parsed: Result<i32,_> = input.parse();
        match input_parsed{
            Ok(float) => return Some(float),
            Err(_) => println!("Invalid Input!")
        }
    }
}

//function hitung bil prima
fn prime_count (input:i32) -> bool {

    if input <= 1 {
        return false;
    } else if input == 2 {
        return true;
    } else if input % 2 == 0{
        return false;
    } 

    let mut i = 3; //iterator 1
    while i < input {
        if input % i == 0 {
            return false;
        }
        i = i+2;
    }
    return true;
}
//function tentuin bilangan prima & push vec bool
fn prime_vector (input:Vec<i32>) -> Vec<bool> {
    //if angka elemen vector <=1 return false
    //cek dari 2 ke value elemen (n) -1, if n%i == 0 return false
    //return true.

    //buat vector baru
    //iterate di vector awal
    //cek rumus angka prima
    //if true, push true ke vector baru
    //if false, push false ke vector baru.
    let mut vec_after:Vec<bool> = Vec::new();
    
    for num in input {
        let bool_result = prime_count(num);
        vec_after.push(bool_result);
    }
    return vec_after


}

fn main() {
    let mut vec_container: Vec<i32> = Vec::new();

    println!("");
    println!("===== Vector(s) =====");
    println!("===== By Henry Hamilton Prasetya =====");
    println!("Input Vector length >>>");

    let input_1 = match read_integer(){
        Some(input_1) => input_1,
        None => return
    };

    let mut i:i32 = 0; //Iterator variable

    while i < input_1 {
        println!("Input Entry for Index {:?} >>>", i);
        let input_2 = match read_integer() {
            Some(input_2) => input_2,
            None => return
        };
        vec_container.push(input_2);

        i = i + 1;
    }


    println!("NUMBERS INSERTED :");
    for num in &vec_container {
        println!("{:?}", num);
    }


    let vec_after = prime_vector(vec_container);

    println!("PRIME RESULT :");
    for bool in &vec_after {
        println!("{:?}", bool);
    }

}
