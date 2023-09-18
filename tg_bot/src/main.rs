extern crate rand;

use std::collections::HashMap;
use reqwest::Client;
use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;
// use tokio_compat::*;
use async_recursion::async_recursion;
// use futures::executor::block_on;
use tokio::runtime::Runtime;


// #[tokio::main]
fn main() {
    println!("main");
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(logic());
}


// #[async_recursion]
async fn logic() {
    println!("logic");
    let client = Client::new();
    let api_token = "6605998046:AAG-R7q6Y5LGyGmsWmkDwYvF8NwwPFDdk90";
    let mut offset = 0;
    let mut count = 0;
    let mut score = 0;
    let mut d = fs::read_to_string(r"C:\Users\User\RustProject\rust_project\tg_bot\src\questions.txt").expect("Unable to read file");
    let mut data: Vec<_> = vec![];
    loop {
        let mut params = HashMap::new();
        let mut ps = HashMap::new();
        let of = &offset.to_string();
        params.insert("offset", of);
        let n = &"803290642".to_string();
        ps.insert("chat_id", n);
        let timeout = &"1".to_string();
        params.insert("timeout", timeout);
        let sm = &"send message".to_string();
        ps.insert("text", sm);
        let re_sponse = send_request(&client, &api_token, "sendMessage", &ps);
        let response = send_request(&client, &api_token, "getUpdates", &params);
        if let Some(updates) = response.await["result"].as_array() {
            if updates.len() > 0 {
                let update = &updates[updates.len() - 1];
                println!("update: {:#?}", update);
                // for update in updates {
                offset = update["update_id"].as_u64().unwrap() + 1;
                if let Some(message) = update["message"].as_object() {
                    let chat_id = message["chat"]["id"].as_i64().unwrap();
                    let text = message["text"].as_str().unwrap();
                    println!("text: {}", text);
                    if text.starts_with("/") {
                        println!("/");
                        let command = text[1..].split_whitespace().next().unwrap();
                        if command == "start" {
                            // println!("start");
                            instruction(&client, chat_id);
                            count = 0;
                            score = 0;
                        } else if command == "start_test" {
                            if (score == 0) & (count == 0) {
                                let d2: Vec<_> = d.split("\n").collect();
                                let d3: Vec<_> = d2.choose_multiple(&mut thread_rng(), 5).collect::<Vec<_>>();
                                for elem in d3 {
                                    let elem_split = elem.split("\t").collect::<Vec<_>>();
                                    data.push(elem_split);};
                                let mut params = HashMap::new();
                                let ci = &chat_id.to_string();
                                params.insert("chat_id", ci);
                                let params_text = &format!("{:#?}", &data[0][..6]);
                                params.insert("text", params_text);
                                let _response = send_request(&client, &api_token, "sendMessage", &params);
                            } else {
                                let mut params = HashMap::new();
                                let ci = &chat_id.to_string();
                                params.insert("chat_id", ci);
                                let test_is_running = &"Тест уже идёт".to_string();
                                params.insert("text", test_is_running);
                                let _response = send_request(&client, &api_token, "sendMessage", &params);
                            };
                        } else if command == "help" {
                            instruction(&client, chat_id);
                        } else if command == "stop" {
                            let mut params = HashMap::new();
                            let ci = &chat_id.to_string();
                            params.insert("chat_id", ci);
                            let test_ended = &"Тест завершён".to_string();
                            params.insert("text", test_ended);
                            let _response = send_request(&client, &api_token, "sendMessage", &params);
                            count = 0;
                            score = 0;
                            offset = 0;
                            d = fs::read_to_string(r"C:\Users\User\RustProject\rust_project\tg_bot\src\questions.txt").expect("Unable to read file");
                            data = vec![];
                        } else {
                            let mut params = HashMap::new();
                            let ci = &chat_id.to_string();
                            params.insert("chat_id", ci);
                            let no_command = &"Такой команды не существует".to_string();
                            params.insert("text", no_command);
                            let _response = send_request(&client, &api_token, "sendMessage", &params);
                        }
                    } else {
                        println!("not /");
                        if data.len() <= count {
                            continue;
                        }
                        else if text == data[count][6].trim() {
                            println!("{:#?}", data[count]);
                            println!("{}", data[count][6].trim());
                            println!("right answer");
                            count += 1;
                            score += 1;
                            if count < 5 {
                                let mut params = HashMap::new();
                                let ci = &chat_id.to_string();
                                params.insert("chat_id", ci);
                                let params_text = &format!("{:#?}", &data[count][..6]);
                                params.insert("text", params_text);
                                let _response = send_request(&client, &api_token, "sendMessage", &params);
                            }
                            else {
                                let mut params = HashMap::new();
                                let ci = &chat_id.to_string();
                                params.insert("chat_id", ci);
                                let s = format!("Тест завершён. Ваш результат: {}/5.\nПравильные ответы:\n{:#?}", score, data);
                                let s_string = s.to_string();
                                params.insert("text", &s_string);
                                let _response = send_request(&client, &api_token, "sendMessage", &params);
                                count = 0;
                                score = 0;
                                offset = 0;
                                d = fs::read_to_string(r"C:\Users\User\RustProject\rust_project\tg_bot\src\questions.txt").expect("Unable to read file");
                                data = vec![];
                            }
                        } else if (text == "1") | (text == "2") | (text == "3") | (text == "4") {
                            println!("{:#?}", data[count]);
                            println!("{}", data[count][6].trim());
                            println!("wrong answer");
                            count += 1;
                            if count < 5 {
                                let mut params = HashMap::new();
                                let ci = &chat_id.to_string();
                                params.insert("chat_id", ci);
                                let params_text = &format!("{:#?}", &data[count][..6]);
                                params.insert("text", params_text);
                                let _response = send_request(&client, &api_token, "sendMessage", &params);
                            } else {
                                let mut params = HashMap::new();
                                let ci = &chat_id.to_string();
                                params.insert("chat_id", ci);
                                let s = format!("Тест завершён. Ваш результат: {}/5.\nПравильные ответы:\n{:#?}", score, data);
                                let s_string = s.to_string();
                                params.insert("text", &s_string);
                                let _response = send_request(&client, &api_token, "sendMessage", &params);
                                count = 0;
                                score = 0;
                                offset = 0;
                                d = fs::read_to_string(r"C:\Users\User\RustProject\rust_project\tg_bot\src\questions.txt").expect("Unable to read file");
                                data = vec![];
                            }
                        } else {
                            println!("wtf");
                            let mut params = HashMap::new();
                            let ci = &chat_id.to_string();
                            params.insert("chat_id", ci);
                            let no_answer = &"Нет ответа под таким номером. Введите число от 1 до 4".to_string();
                            params.insert("text", no_answer);
                            let _response = send_request(&client, &api_token, "sendMessage", &params);
                        }
                    }
                }
            }
            else {
                continue;
            }
        }
    }
}

async fn send_request(
    client: &Client,
    api_token: &str,
    method: &str,
    params: &HashMap<&str, &std::string::String>) -> serde_json::Value{
    println!("send request, {}", method);
    let mut url = String::new();
    url.push_str("https://api.telegram.org/bot");
    url.push_str(api_token);
    url.push_str("/");
    url.push_str(method);

    let mut _response_ = client.get(&url).query(params).send();
    let json_: serde_json::Value = _response_.await.unwrap().json().await.unwrap();
    println!("json: {}", json_);
    json_
}

fn instruction(client: &Client, chat_id: i64) -> () {
    println!("instruction");
    let api_token = "6605998046:AAG-R7q6Y5LGyGmsWmkDwYvF8NwwPFDdk90";
    let mut params = HashMap::new();
    let ci = chat_id.to_string();
    let instr = "Инструкция".to_string();
    params.insert("chat_id", &ci);
    params.insert("text", &instr);
    let _response = send_request(&client, &api_token, "sendMessage", &params);
    println!("instruction sent");
}
