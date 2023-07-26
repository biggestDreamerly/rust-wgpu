/*
 * @Author: lvy lvy
 * @Date: 2023-07-17 18:01:10
 * @LastEditors: lvy lvy
 * @LastEditTime: 2023-07-24 18:17:09
 * @FilePath: /hello_cargo/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

pub struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    run();
    let instances: wgpu::Instance = wgpu::Instance::new(wgpu::Backends::all());
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info())
    }
    let width = 32;
    let height = 32;
    println!("this  is area {}", rust01::area(width, height));
    let rect_range = Rectangle {
        width: 32,
        height: 32,
    };
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    //在计算面积时将宽和高弄混倒无关紧要，不过当在屏幕上绘制长方形时就有问题了！
    //我们必须牢记 width 的元组索引是 0，height 的元组索引是 1
    //如果其他人要使用这些代码，他们必须要搞清楚这一点，并也要牢记于心。
    //很容易忘记或者混淆这些值而造成错误，因为我们没有在代码中传达数据的意图。

    println!("this  is areaTuple {}", rust01::area_tuple((width, height))); //元组
    println!("this  is areaStruct {}", rust01::area_struct(&rect_range)); //结构体
    println!("this  is impl {}", rect_range.area()); //块 todo::这个没见过 但是好神奇 第一次
}

mod rust01 {
    pub use crate::Rectangle;
    pub fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    pub fn area_tuple(can: (u32, u32)) -> u32 {
        can.0 * can.1
    }
    pub fn area_struct(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
