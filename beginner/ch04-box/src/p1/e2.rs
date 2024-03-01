use std::cell::Cell;

fn retain_even(nums: &mut Vec<i32>) {
    // .. 覆盖了 nums 所有的切片，是全范围的一个缩写
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    /* ---------- ---------- ---------- ---------- */

    // // 创建一个新的 Vec<Cell<i32>> 来存储 nums 中的每个元素的 Cell 包装
    // let mut cells: Vec<Cell<i32>> = Vec::with_capacity(nums.len());

    // // 遍历 nums，将每个元素转换为 Cell<i32>，然后添加到 cells 中
    // // 迭代器解引用，得到的就是普通的元素
    // for num in nums.iter() {
    //     cells.push(Cell::new(*num));
    // }

    // // 获取 cells 的不可变引用切片
    // let slice: &[Cell<i32>] = &cells[..];

    /* ---------- ---------- ---------- ---------- */

    let mut i = 0;
    for num in slice.iter().filter(|num| num.get() % 2 == 0) {
        slice[i].set(num.get());
        i = i + 1;
    }

    // 这段代码就是：将 Vec 只留下偶数
    nums.truncate(i);
}
