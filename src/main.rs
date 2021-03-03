// std and main are not available for bare metal software
#![no_std]
#![no_main]
#[warn(non_snake_case)]

//use cortex_m_rt::entry; // The runtime
//use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
//use stm32f1xx_hal::{delay::Delay, pac, prelude::*}; // STM32F1 specific functions
//use stm32f0::stm32f0x1;



extern crate cortex_m;

extern crate panic_halt; //extern crate panic_semihosting;
extern crate cortex_m_rt;
use cortex_m_rt::entry;

//#[allow(unused_imports)]
extern crate stm32f0;
use stm32f0::stm32f0x1;

mod fifo;

//use fifo::make_fifo!;

//struct Fifo<T: ?Sized>{
//	max_size: usize,
//	data: T,
//}
//
//
//
//macro_rules! make_fifo {
//    ($size:literal) => {{
//		Fifo{
//			max_size: $size,
//			data: [0; $size +1],
//		}        
//    }}
//}
//
//const fn ff(ss: usize) -> Fifo{
//	make_fifo!(ss)
//}

//#[macro_use] crate fifo make_fifo!;

//extern crate stm32-rs;
//use panic_halt; // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {

	
    //let dp = pac::Peripherals::take().unwrap();
	//let cp = cortex_m::Peripherals::take().unwrap();
	let periph = stm32f0x1::Peripherals::take().unwrap();

	let gpioc = &periph.GPIOC;
//	let rcc = &periph.RCC;
//	rcc.apb2enr.write(|w| w.iopcen().set_bit());
//    gpioc.crh.write(|w| unsafe{
//        w.mode13().bits(0b11);
//        w.cnf13().bits(0b00)
//    });
	
//	let aaa: Fifo<[u8; 30]> = Fifo{
//		max_size: 30,
//		data: [0; 30],
//	};
	
	//let ff = make_fifo!(20);

	
	let mut buf1 = [0_u8; 20];
	let mut ff = fifo::Fifo::new(&mut buf1);
	match ff.put(7){
		Err(_e) => (),
		_	=> ()		
	};
		
	//let z1 = fifo::mf!(10);
	//let mut z2 = make_fifo!(10);
	let mut buf1 = [0_u8, 15];
	let mut z2 = fifo::Fifo::new(&mut buf1);
	match z2.put(66){
		Err(_e) => (),
		_	=> ()		
	};
	
	let _cc = z2.get();
	let mut buf1 = [0_u8, 10];
	let mut z3 = fifo::Fifo::new(&mut buf1);
//	let mut z3 = make_fifo!(10);
	match z3.put_all(&mut z2){
		Err(_e) => (),
		_	=> ()		
	};
	
    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit());
        //cortex_m::asm::delay(2000000);
        gpioc.brr.write(|w| w.br12().set_bit());
        //cortex_m::asm::delay(2000000);
    }
}




