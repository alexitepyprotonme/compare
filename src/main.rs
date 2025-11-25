#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use chrono;
use std::f32::consts::E;
use std::fs::File;
use std::{array, fs};
use std::env;
use std::io::{self, BufRead};
use std::path::Path;
use url::Url;
use std::{collections::BTreeSet, iter::FromIterator};
use colored::*;

fn retainings<T>(mut items: Vec<T>, to_remove: Vec<T>) -> Vec<T>
           where
               T: std::cmp::Ord,
{
    let to_remove = BTreeSet::from_iter(to_remove);
    items.retain(|e| !to_remove.contains(e));

    items
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = format!(
        "\n\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        r#"           _     _                 _       _     _           "#,
        r#"  __ _  __| | __| |      ___ _   _| |__   | |__ (_) __ _     "#,
        r#" / _` |/ _` |/ _` |     / __| | | | '_ \  | '_ \| |/ _` |    "#,
        r#"| (_| | (_| | (_| |  _  \__ \ |_| | |_) | | |_) | | (_| |    "#,
        r#" \__,_|\__,_|\__,_| ( ) |___/\__,_|_.__/  |_.__/|_|\__, |    "#,
        r#"                    |/                             |___/     "#,
        r#"             _          ____          __  __ _ _           _ "#,
        r#"  __ _  __ _| |_ __ _  | __ ) _   _  |  \/  (_) | __ _  __| |"#,
        r#" / _` |/ _` | __/ _` | |  _ \| | | | | |\/| | | |/ _` |/ _` |"#,
        r#"| (_| | (_| | || (_| | | |_) | |_| | | |  | | | | (_| | (_| |"#,
        r#" \__, |\__,_|\__\__,_| |____/ \__, | |_|  |_|_|_|\__,_|\__,_|"#,
        r#" |___/                        |___/                          "#,
        r#" Fastest files compare tool that you have ever sean."#,
    );
    println!("{}", s.bold().green());

    println!("{}", "This is a tool to compare and subtratct two files from each other and put the items in third file that are present \n in first file which does not exist in the second file");
    println!("{}", "file3 = file1 - file2");
    
    println!("{}", "Please enter the first file's path:");
    let mut file1_path: String = String::from("");
    std::io::stdin().read_line(&mut file1_path).expect("Error !!!");
    file1_path = file1_path.replace("\n", "");
    // let greeting_file_result = File::open(format!("{}",file1_path));
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    println!("{}", "Please enter the second file's path:");
    let mut file2_path: String = String::from("");
    std::io::stdin().read_line(&mut file2_path).expect("Error !!!");
    file2_path = file2_path.replace("\n", "");

    println!("{}", "Please enter the third output file's path:");
    let mut file3_path: String = String::from("");
    std::io::stdin().read_line(&mut file3_path).expect("Error !!!");
    file3_path = file3_path.replace("\n", "");

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let items1 = fs::read_to_string(format!("{}",file1_path)).expect("Error !!!");

    let items2 = fs::read_to_string(format!("{}",file2_path)).expect("Error !!!");

    let items1_ = items1.split("\n");
    let items2_ = items2.split("\n");

    println!("start :-----{}",chrono::offset::Local::now().time());
    let mut i = 0;
    for item1 in items1_ {
        vec1.push(item1);
        i = i + 1;
        println!("{}===={}===={}","file1",i,item1);
    }
    i = 0;
    for item2 in items2_ {
        vec2.push(item2);
        i = i + 1;
        println!("{}===={}===={}","file2",i,item2);
    }
    let mut some_x = "";
    i = 0;
    vec1.sort();
    vec1.dedup();

    vec2.sort();
    vec2.dedup();

    println!("{}","---------------!!! Items sorted !!!---------------");

    println!("{}","---------------!!! subtraction started !!!---------------");
    let vec_c = retainings(vec1,vec2);
    println!("{}","---------------!!! subtraction ended   !!!---------------");
    
    fs::write(format!("{}", file3_path), vec_c.join("\n")).expect("");
    println!("end :-------{}",chrono::offset::Local::now().time());
    

    Ok(())

}