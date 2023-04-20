
/*std and main are not available for bare metal software*/

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

//mod fifo;
mod fifo;
use fifo::Fifo;

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
	

	let mut ff: Fifo<8> = Fifo::new();
	match ff.put(7){
		Err(_e) => (),
		_	=> ()		
	};
		
	let mut z2: Fifo<10> = Fifo::new();
	match z2.put(66){
		Err(_e) => (),
		_	=> ()		
	};

	z2.put_bytes(&[5_u8, 6_u8, 7_u8, 8_u8]);
	
	ff.put_all(&mut z2);

	
    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit());
        //cortex_m::asm::delay(2000000);
        gpioc.brr.write(|w| w.br12().set_bit());
        //cortex_m::asm::delay(2000000);
    }
}




