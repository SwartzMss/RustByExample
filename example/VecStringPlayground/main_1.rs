fn process_vec(mut vec: Vec<String>) {
    // 处理向量中的每个字符串：例如，转换为大写
    vec.iter_mut().for_each(|item| *item = item.to_uppercase());

    // 打印处理后的向量
    println!("Processed vector: {:?}", vec);
}

struct MyStruct {
    my_field: Vec<String>,
}

fn main() {
    let mut my_struct = MyStruct {
        my_field: vec!["hello".to_string(), "world".to_string()],
    };

    // 这边的话 你是不能直接暴力转移所有权的，因为对于结构体里面的字段 通常会有多个地方在使用，直接转移的话 其他地方就会出现问题
    // process_vec(my_struct.my_field);

    // 使用 std::mem::take 来取得 my_field 的所有权
    let taken_field = std::mem::take(&mut my_struct.my_field);  // taken_field now owns the Vec, and my_field is an empty Vec

    // 使用取出的数据
    process_vec(taken_field);

    // 验证 my_struct.my_field 是否已被清空
    assert!(my_struct.my_field.is_empty());
    println!("my_struct.my_field after take: {:?}", my_struct.my_field);
}
