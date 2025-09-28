use crate::hspice::*;

#[test]
fn test() {
    let mut txt = String::new();
    let mut seed = 0;
    let bits = 31;
    for i in 0..bits {
        txt += &line_source_random(&format!("A{i:02}PP"), seed, "0", "avdd");
        seed += 1;
    }
    for i in 0..bits {
        txt += &line_source_random(&format!("B{i:02}PP"), seed, "0", "avdd");
        seed += 1;
    }
    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}