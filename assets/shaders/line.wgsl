//// BorderDashed.wgsl
//
//// 输入结构体，包含矩形的位置和尺寸
//[[block]]
//struct BorderDashedInput {
//    rect_position: vec2<f32>;
//    rect_size: vec2<f32>;
//};
//
//// 输出结构体，包含像素颜色
//[[block]]
//struct BorderDashedOutput {
//    color: vec4<f32>;
//};
//
//// 着色器入口函数
//[[stage(fragment)]]
//fn main(input: BorderDashedInput) -> BorderDashedOutput {
//    // 边框宽度
//    let border_width = 0.02; // 调整边框宽度
//
//    // 虚线边框颜色
//    let border_color = vec4<f32>(1.0, 0.0, 0.0, 1.0); // 红色边框
//
//    // 计算矩形中心
//    let rect_center = input.rect_position + input.rect_size * 0.5;
//
//    // 计算矩形的边界
//    let left_edge = input.rect_position.x;
//    let right_edge = input.rect_position.x + input.rect_size.x;
//    let top_edge = input.rect_position.y;
//    let bottom_edge = input.rect_position.y + input.rect_size.y;
//
//    // 计算虚线边框
//    let x_in_border = mod(rect_center.x - left_edge, border_width * 2.0) < border_width;
//    let y_in_border = mod(rect_center.y - top_edge, border_width * 2.0) < border_width;
//
//    // 如果点在虚线边框上，设置为边框颜色，否则设置为透明
//    let border_color_or_transparent = (x_in_border || y_in_border) ? border_color : vec4<f32>(0.0, 0.0, 0.0, 0.0);
//
//    // 输出颜色
//    return BorderDashedOutput { color: border_color_or_transparent };
//}
