// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a =
        "\
    当你走进这欢乐场
    背上所有的梦与想
    各色的脸上各色的妆
    没人记得你的模样
    
    三巡酒过你在角落
    固执的唱着苦涩的歌
    听它在喧嚣里被淹没
    你拿起酒杯对自己说
    
    一杯敬朝阳 一杯敬月光
    唤醒我的向往 温柔了寒窗
    于是可以不回头地逆风飞翔
    不怕心头有雨 眼底有霜
    
    一杯敬故乡 一杯敬远方
    守着我的善良 催着我成长
    所以南北的路从此不再漫长
    灵魂不再无处安放
    
    躁动不安的座上客
    自以为是地表演着
    伪装着 舞蹈着 疲惫着
    你拿起酒杯对自己说
    一杯敬朝阳 一杯敬月光
    唤醒我的向往 温柔了寒窗
    于是可以不回头地逆风飞翔
    不怕心头有雨 眼底有霜
    
    一杯敬故乡 一杯敬远方
    守着我的善良 催着我成长
    所以南北的路从此不再漫长
    灵魂不再无处安放
    
    一杯敬明天 一杯敬过往
    支撑我的身体 厚重了肩膀
    虽然从不相信所谓山高水长
    人生苦短何必念念不忘
    
    一杯敬自由 一杯敬死亡
    宽恕我的平凡 驱散了迷惘
    好吧天亮之后总是潦草离场
    清醒的人最荒唐
    清醒的人最荒唐
    
    ";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
