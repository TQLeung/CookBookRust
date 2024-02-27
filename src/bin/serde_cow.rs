/*
序列化本来就没有问题，你手欠给

办法一：
Msg加了derive(Deserialize)才报错了，删掉就好了；

办法二：
Person 添加 #[derive(Clone)]
Msg 有 #[derive(Deserialize)] // 如果一定需要Deserialize, 
Msg 的 person字段类型改成Cow<'a, Person>之类的就好了;

*/
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    id: String,
    name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("dropping {}", self.name)
    }
}

#[derive(Serialize, Deserialize)]
struct Msg<'a> {
    id: String,
    person: Cow<'a, Person>,
}

fn main() {
    let person = Person {
        id: "123".to_string(),
        name: "Alice".to_string(),
    };

    let msg = Msg {
        id: "456".to_string(),
        person: std::borrow::Cow::Borrowed(&person),
    };

    let msg_str = serde_json::to_string(&msg).unwrap();
    println!("Serialized Msg: {}", msg_str);
}