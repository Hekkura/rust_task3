// - Bikin 1 function yang menerima 1 parameter warna berupa &str lalu tugas dari
// function tersebut mengembalikan format value RGBA dari parameter tersebut
// berbentuk tuples. Minimal harus ada 10 warna yang bisa dihandle dari function
// ini, untuk warna yang diperlukan dibebaskan.

// Bikin 1 function untuk menampilkan value RGBA tuples hasil function diatas
// dengan metode destructuring,

//10 warna --> Struct of tuples ?
// function input &str, for in liat name dari struct tuple,
// return value dengan destructuring

use std::io;
use std::collections::HashMap;
#[derive(Debug)]
struct Color (i32,i32,i32,i32);

#[derive(Debug)]
struct Colors {
    inner: HashMap<String, Color>
}

//initialize warna warna & Rgba value
fn init_colors() {
    Colors::inner.insert("Black",(0,0,0,1));
    Colors::inner.insert("Red",(255,0,0,1));
    Colors::inner.insert("Green",(0,255,0,1));
    Colors::inner.insert("Blue",(0,0,255,1));
    Colors::inner.insert("Navy Blue",(25,116,210,1));
    Colors::inner.insert("Burgundy",(159,29,53,1));
    Colors::inner.insert("Tosca",(158,66,66,1));
    Colors::inner.insert("Brown",(234,221,202,1));
    Colors::inner.insert("Gray",(128,128,128,1));
    Colors::inner.insert("Orange",(255,165,0,1));
    Colors::inner.insert("Aqua",(0,255,255,1));
    Colors::inner.insert("Lime",(50,205,50,1));
    Colors::inner.insert("White",(0,0,0,0));
}

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

//function buat read input &str, output rgba value (destructuring)
fn read_color(input:&str) -> Option<Color> {
    if input ==
}


fn main() {
    println!("");
    println!("===== Tuple RGBA =====");
    println!("===== By Henry Hamilton Prasetya =====");
    println!("Input nothing to exit.");

    loop {
        println!("Input the color you want to see the RGBA value of >>");

        let input_1 = match read_string(){
            Some(input_1) => input_1,
            None => return
        };

    }
    
}
