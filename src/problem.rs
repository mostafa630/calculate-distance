use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::str;

pub fn calculate_distance(document1: &str, document2: &str) -> f64 {
    let document_1_content: String = read_document(document1);
    let document_2_content: String = read_document(document2);

    // dictionarie to store words frequency in each document
    let mut dic1: HashMap<String, i64> = HashMap::new();
    let mut dic2: HashMap<String, i64> = HashMap::new();
    // sqrt of distance for each document
    let distance_1_sqrt = process_content(&document_1_content, &mut dic1);
    let distance_2_sqrt = process_content(&document_2_content, &mut dic2);

    let cosine_similarty = calc_cosine_similarty(&dic1, &dic2, &distance_1_sqrt, &distance_2_sqrt);
    return cosine_similarty;
}

fn read_file_as_loose_utf8(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Decode the bytes as UTF-8, ignoring invalid sequences
    let decoded_string = String::from_utf8_lossy(&buffer).into_owned();

    Ok(decoded_string)
}
fn read_document(document_path: &str) -> String {
    let content: String;
    match read_file_as_loose_utf8(document_path) {
        Ok(decoded_content) => {
            content = decoded_content.to_lowercase();
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            return "in_valid".to_owned();
        }
    }
    return content;
}
fn process_content(content: &String, dic: &mut HashMap<String, i64>) -> i64 {
    let mut distance_sqrt: i64 = 0; // value to returned from function
    let mut word = String::from(""); // keep track with each word in document
    for byte in content.bytes() {
        let char = byte as char;
        if char.is_alphabetic() || char.is_digit(10) {
            word.push(char);
        } else {
            update_dic_and_distance(&word, dic, &mut distance_sqrt);
            word.clear();
        }
    }
    if word.len() > 0 {
        // ensure that the final word in file has been counted
        update_dic_and_distance(&word, dic, &mut distance_sqrt);
    }
    return distance_sqrt;
}

fn update_dic_and_distance(word: &String, dic: &mut HashMap<String, i64>, distance_sqrt: &mut i64) {
    if word.len() <= 0 {
        return;
    }
    if dic.contains_key(word) {
        let old_freq: i64 = dic[word];
        let new_freq: i64 = old_freq + 1;
        *distance_sqrt -= old_freq * old_freq; // test this line
        dic.insert(String::from(word), new_freq);
        *distance_sqrt += new_freq * new_freq; // test this line
    } else {
        dic.insert(String::from(word), 1);
        *distance_sqrt += 1;
    }
}
fn calc_cosine_similarty(
    dic1: &HashMap<String, i64>,
    dic2: &HashMap<String, i64>,
    distance_1_sqrt: &i64,
    distance_2_sqrt: &i64,
) -> f64 {
    // println!("dic11111111111111111111111111111111111111111");
    // for val in dic1 {
    //     println!(" key = {}  , val = {}", val.0, val.1)
    // }
    // println!("dic22222222222222222222222222222222222222222");
    // for val in dic2 {
    //     println!(" key = {}  , val = {}", val.0, val.1)
    // }
    let sum = clac_sum(dic1, dic2);
    let cosine_similarty = calc_similarty(&sum, &distance_1_sqrt, &distance_2_sqrt);
    return cosine_similarty;
}
fn clac_sum(dic1: &HashMap<String, i64>, dic2: &HashMap<String, i64>) -> i64 {
    let mut sum: i64 = 0;
    if dic1.len() > dic2.len() {
        for word in dic1.keys() {
            if dic2.contains_key(word) {
                sum = sum + (dic1[word] * dic2[word]);
            }
        }
    } else {
        for word in dic2.keys() {
            if dic1.contains_key(word) {
                sum = sum + (dic1[word] * dic2[word]);
            }
        }
    }
    return sum;
}
fn calc_similarty(sum: &i64, distance_1_sqrt: &i64, distance_2_sqrt: &i64) -> f64 {
    // println!("sum = {}", sum);
    // println!("d1 = {}", distance_1_sqrt);
    // println!("d2 = {}", distance_2_sqrt);
    if (*distance_1_sqrt == 0 && *distance_2_sqrt != 0)
        || (*distance_1_sqrt != 0 && *distance_2_sqrt == 0)
    {
        return 90f64;
    } else if *distance_1_sqrt == 0 && *distance_2_sqrt == 0 {
        return 0f64;
    }

    let sqrt1: f64 = *sum as f64 / *distance_1_sqrt as f64;
    let sqrt2: f64 = *sum as f64 / *distance_2_sqrt as f64;
    let sqrt = (sqrt1 * sqrt2).sqrt();
    //println!("sqr = {}", sqrt);
    let pi = std::f64::consts::PI;
    let similarty = sqrt.acos() * (180 as f64 / pi);
    return similarty;
}
