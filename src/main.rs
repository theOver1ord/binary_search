use std::env;
use std::fs::File;
use std::io::prelude::*;

//
fn vhoditli (el: String, stroka: String) -> usize { //функция, которая проверяет, входит ли элемент (el) в строку (stroka)
    let mut func = 0;
    for i in 0..stroka.len(){
        if &el == &stroka[i..i+1] {
            func += 1;
        } 
    }
    func
}

// 
fn read_file() -> (Vec<i32>, Vec<i32>) {
    let letters = String::from("-0123456789"); 

    // Read file contents to string
    let mut f = File::open("rosalind_bins (3).txt").expect("file not found");
    let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
    let c = contents.len();

    let a = 9822;
    let b = 9616;
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    
    //переписываем данные из строки в первый вектор //надо бы написать функцию, т.к. делаем это 2 раза
    let mut curs1 = 0; //курсор в строке
    for i in 0..a { //далее нужно написать кусок, который читает символ, находящийся под курсором, и, если символ входит в алфавит (piu) - сохранит его в отдельный элемент (chislo) массива (v1)
        let mut n = 0;
        let mut chislo = String::new();
        while n < 10 {
            if vhoditli(contents[curs1..curs1+1].to_string(), letters.to_string()) == 1 {
                chislo.push_str(&contents[curs1..curs1+1]);
            } else {
                n += 100;
            }
            curs1 += 1;
            n += 1;
        }
        let v11 = chislo.parse::<i32>().unwrap(); // переводим string - > i32
        v1.push(v11);
        }

    curs1 += 1;

    for i in 0..b { 
        let mut n = 0;
        let mut chislo = String::new();
        while n < 10 {
            if vhoditli(contents[curs1..curs1+1].to_string(), letters.to_string()) == 1 {
            chislo.push_str(&contents[curs1..curs1+1]);
            } else {
                n += 100;
            }
            curs1 += 1;
            n += 1;
        }
        let mut v11 = chislo.parse::<i32>().unwrap(); 
        v2.push(v11);
    }

    //Return tuple
    (v1,v2)
}

// Entry point
fn main() {
    let a = 9822;
    let b = 9616;
    let (v1, v2) = read_file();
    let mut v3: Vec<i32> = Vec::new(); //вектор с ответами

    for i in 0..b { // сравним 2 вектора с числами, каждый элемент с каждым
        let mut flag: i32 = 0; //введем флаг, который изменяется (+1) в случаях, когда сравниваемые элементы веткторов не равны друг другу, это поволит нам понять, когда нам проставить в массив "-1" (нет совпадений) - "flag = a" на конце строки     
        for j in 0..a {
            if v2 [i] == v1 [j] {
                let c = j as i32 + 1;
                v3.push(c);
            } else {
                flag += 1;

            }
            if flag == a as i32 {
                let c = -1;
                v3.push(c);
            }
        }
    }

    let mut otvet = String::new(); //хорошо бы еще в файлик записать ответ
    for i in 0..b {
        println!("massiv {}", v3[i]);
        otvet.push_str(&v3[i].to_string());
        otvet.push(' ');
    }
    
    println!("otvet: {}", &otvet[..]);

}
