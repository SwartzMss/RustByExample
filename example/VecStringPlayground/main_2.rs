fn process_optional_vec(opt_vec: Option<Vec<String>>) {
    if let Some(vec) = opt_vec {
        println!("Processing vector with {} elements.", vec.len());
        for (index, value) in vec.iter().enumerate() {
            println!("Element {}: {}", index + 1, value);
        }
    } else {
        println!("No vector was provided.");
    }
}


struct MyStruct {
    my_field: Option<Vec<String>>,
}

impl MyStruct {
    fn take_field(&mut self) -> Option<Vec<String>> {
        self.my_field.take()  // This will replace the field with `None` and return the original value
    }
}

fn main() {
    let mut my_struct = MyStruct {
        my_field: Some(vec!["hello".to_string(), "world".to_string()]),
    };

    // 使用 take_field() 方法来取得 my_field 的所有权，并将其设置为 None
    let taken_field = my_struct.take_field();  // taken_field now owns the Vec, and my_field is None

    // 使用取出的数据
    process_optional_vec(taken_field);

    // 验证 my_struct.my_field 是否为空
    assert!(my_struct.my_field.is_none());
    println!("my_struct.my_field after take: {:?}", my_struct.my_field);
}
