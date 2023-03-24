
// use crate::lesson4::traffic_signlal::{GetTime, TrafficSignal};
// use crate::lesson4::sum::sum;
use crate::lesson4::area::{area, Rec};

pub mod lesson4;

fn main() {
    // let red = TrafficSignal::Red;
    // let green = TrafficSignal::Green;
    // let yellow = TrafficSignal::Yellow;
    //
    // println!("{:?}", red.time());
    // println!("{:?}", green.time());
    // println!("{:?}", yellow.time());

    // println!("{:?}",sum(&vec![1,  u32::MAX]))
    let r = Rec { l: 11.1, w: 12.1 };
    println!("{:?}", area(&r));
}


pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let size = array.len();
    // 单个元素集合或者空集合 直接返回 不需要排序
    if size <= 1 {
        return;
    }

    // 对所有元素进行重复，除了最后一个
    for i in 0..(size - 1) {
        // 记录是否发生过交换
        let mut swapped = false;

        for j in 1..(size - i) {
            // 比较相邻的元素 如果第一个比第二个大，就交换他们两个。
            if array[j - 1] > array[j] {
                // 切片交换两个元素
                array.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果未发生交换，说明已经排好序，跳出后续的冒泡
        if !swapped {
            break;
        }
    }
}





