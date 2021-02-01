

//struct Fifo<T: ?Sized>{
//	max_size: const usize,
//	data: T,
//	head: usize,
//	tail: usize,
//}

pub struct Fifo2<T>{
	head: usize,
	tail: usize,
	data: T,
}

macro_rules! make_fifo {
    ($size:literal) => {{
		Fifo{
			max_size: $size,
			data: [0; $size +1],
		}        
    }}
}



impl<u8> Fifo2<u8>{
	pub const fn new(aa:u8) ->Self{
		Fifo2{
			head: 0,
			tail: 0,
			data: aa,
		}
	}
}

//impl Fifo2<[u8; usize]>{
//	pub const fn new(size: usize) -> Self{
//		Fifo2{
//			head: 0,
//			tail: 0,
//			data: [0; usize],
//		}
//	}
//}



pub enum FifoError{
		BufferEmpty,
		BufferFull,
	}
	
pub trait Fifo{	
	fn put(znak :u8) -> Result<bool, FifoError>;
	fn get() -> Option<u8>;
	fn capacity()-> usize;
	fn used_bytes()-> usize;
	fn available_bytes()-> usize;
}

struct FifoStr{
	head: usize,
	tail: usize,
	data: [u8],
}

//macro_rules! make_fifo {
//    ($size:literal) => {{
//		Fifo{
//			max_size: $size,
//			data: [0; $size +1],
//			head: 0,
//			tail: 0,
//		}        
//    }}
//}

//impl Default for Fifo<T: ?Sized> {
//    fn default(size: usize) -> Fifo {
//        make_fifo!(size)
//    }
//}

//impl FifoStr  {
//	
//    pub fn new(size: usize) -> FifoStr {
//       //Self::default()
//		//let tab =  [0; size];
//		
//		FifoStr{
//			head: 0,
//			tail: 0,
//			data: [0;size],
//		}
//
//    }
//
//}


