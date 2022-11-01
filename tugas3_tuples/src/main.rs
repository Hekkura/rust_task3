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

#[derive(Debug)]
struct Colours {
    name: String,
    rgba: (u8,u8,u8,f32),
}

//initialize warna warna & Rgba value
fn init_colours() -> Vec<Colours> {
    let mut vec_colours:Vec<Colours> = Vec::new();

    vec_colours.push(Colours{name:"Black".to_owned(),rgba:(0,0,0,1.0)});
    vec_colours.push(Colours{name:"Red".to_owned(),rgba:(255,0,0,1.0)});
    vec_colours.push(Colours{name:"Green".to_owned(),rgba:(0,255,0,1.0)});
    vec_colours.push(Colours{name:"Blue".to_owned(),rgba:(0,0,255,1.0)});
    vec_colours.push(Colours{name:"Navy Blue".to_owned(),rgba:(25,116,210,1.0)});
    vec_colours.push(Colours{name:"Burgundy".to_owned(),rgba:(159,29,53,1.0)});
    vec_colours.push(Colours{name:"Tosca".to_owned(),rgba:(158,66,66,1.0)});
    vec_colours.push(Colours{name:"Brown".to_owned(),rgba:(234,221,202,1.0)});
    vec_colours.push(Colours{name:"Gray".to_owned(),rgba:(128,128,128,1.0)});
    vec_colours.push(Colours{name:"Orange".to_owned(),rgba:(255,165,0,1.0)});
    vec_colours.push(Colours{name:"Aqua".to_owned(),rgba:(0,255,255,1.0)});
    vec_colours.push(Colours{name:"Lime".to_owned(),rgba:(50,205,50,1.0)});
    vec_colours.push(Colours{name:"White".to_owned(),rgba:(0,0,0,0.0)});

    return vec_colours;
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

//function buat read input &str, output rgba value tuple
// fn read_color(input:&str) -> (u8,u8,u8,f32) { //ganti ke result <tuple, String> {
//     let vec_colours = init_colours();

//     for colours in &vec_colours {
//         return colours.rgba
//     }

//     // for colours in &vec_colours {
//     //     if &colours.name == input {
//     //         return colours.rgba
//     //     }
//     //     println!("Colour not found");
//     // }
// } 

//helper function : print colours ke format output dengan destructuring.
fn print_output(input:(u8,u8,u8,f32)) {
    println!("r:{:?}, g:{:?}, b:{:?}, a:{:?}", input.0,input.1,input.2,input.3)
}


fn main() {
    println!("");
    println!("===== Tuple RGBA =====");
    println!("===== By Henry Hamilton Prasetya =====");
    println!("Input nothing to exit.");

    println!("DEBUG");
    println!("ALL COLORS & RGBA DATA");
    let vec_colours_test = init_colours() ;
    for colours in &vec_colours_test {
        println!("{:?}", colours);
    }
    //================================================================= 
    //  LOOP MENU
    loop {
        println!("Input the color you want to see the RGBA value of >>");
        let input_1 = match read_string(){
            Some(input_1) => input_1,
            None => return
        };
        if input_1 == "" {
            break;
        }
        
        //ini harusnya di function tapi entah kenapa error terus. Jadi di main aja.
        let vec_colours = init_colours() ;
        for colours in &vec_colours {
            if &colours.name == &input_1 {
                print_output(colours.rgba);
                break;
            }

        }
    }
    
}