use svg::Document;
use svg::node::element::{Circle, Line, Rectangle, Text};

#[test]
fn test_1() {
    // 绘制一个填充满的红色的圆形
    // let circle = Circle::new()
    //     .set("cx", 50) // 圆心的x坐标
    //     .set("cy", 50)  // 圆心的y坐标
    //     .set("r", 40)   // 半径
    //     .set("fill", "red"); // 填充颜色

    let circle = Circle::new()
        .set("cx", 50)
        .set("cy", 50)
        .set("r", 40)
        .set("fill", "#ff0000")
        .set("stroke", "#0000ff")
        .set("stroke-width", 2)
        .set("opacity", 0.7)
        .set("fill-opacity", 0.5);
    
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100)) // 视口大小
        .add(circle);
    
    svg::save("image.svg", &document).unwrap();
}

#[test]
fn test_2() {
    // 绘制一个黑色边框，蓝色填充的矩形
    let rect = Rectangle::new()
        .set("x", 10)
        .set("y", 10)
        .set("width", 80)
        .set("height", 60)
        .set("fill", "blue")
        .set("stroke", "black")
        .set("stroke-width", 2);
    
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100)) // 视口大小
        .add(rect);
    
    svg::save("image.svg", &document).unwrap();
}

#[test]
fn test_3() {
    // 绘制一个绿色线条
    let line = Line::new()
        .set("x1", 10)
        .set("y1", 10)
        .set("x2", 90)
        .set("y2", 90)
        .set("stroke", "green")
        .set("stroke-width", 3);
    
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100)) // 视口大小
        .add(line);
    
    svg::save("image.svg", &document).unwrap();
}

#[test]
fn test_4() {
    // 绘制文本
    // 创建白色背景矩形
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "white");

    let circle = Circle::new()
        .set("cx", 50) // 圆心的x坐标
        .set("cy", 50)  // 圆心的y坐标
        .set("r", 2)   // 半径
        .set("fill", "red"); // 填充颜色


    let text = Text::new("hello")
        .set("x", 50)
        .set("y", 50)
        .set("text-anchor", "middle") // 水平居中
        .set("dominant-baseline", "middle")   // 垂直居中
        .set("font-family", "Arial")
        .set("font-size", 20);
    
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100)) // 视口大小
        .add(background)
        .add(circle)
        .add(text);
    
    svg::save("image.svg", &document).unwrap();
}

#[test]
fn test_text_witdh_auto() {
    fn create_adaptive_text_box(
        text: &str, 
        width: f32, 
        height: f32, 
        bg_color: &str, 
        text_color: &str
    ) -> Document {
        // 简单估算字体大小
        let char_count = text.chars().count() as f32;
        let font_size = (width / char_count.max(1.0) * 1.8)
            .min(height * 0.8)
            .max(10.0);
        
        Document::new()
            .set("viewBox", format!("0 0 {} {}", width, height))
            .set("preserveAspectRatio", "xMidYMid meet")
            .add(
                Rectangle::new()
                    .set("width", "100%")
                    .set("height", "100%")
                    .set("fill", bg_color)
                    .set("rx", 8) // 圆角
                    .set("ry", 8)
            )
            .add(
                Text::new(text)
                    .set("x", "50%")
                    .set("y", "50%")
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "middle")
                    .set("font-size", font_size)
                    .set("fill", text_color)
                    .set("font-family", "Arial, sans-serif")
            )
    }

    // 创建不同宽度的文本框
    let box1 = create_adaptive_text_box("短文本", 150.0, 50.0, "#e3f2fd", "#0d47a1");
    let box2 = create_adaptive_text_box("中等长度的文本示例", 250.0, 50.0, "#f1f8e9", "#33691e");
    let box3 = create_adaptive_text_box("这是一个非常长的文本需要自适应宽度", 350.0, 50.0, "#ffebee", "#b71c1c");
    
    // 组合成单个SVG
    let document = Document::new()
        .set("width", 400)
        .set("height", 200)
        .set("viewBox", (0, 0, 400, 200))
        .add(box1.set("transform", "translate(20, 20)"))
        .add(box2.set("transform", "translate(20, 80)"))
        .add(box3.set("transform", "translate(20, 140)"));
    
    svg::save("image.svg", &document).unwrap();
}