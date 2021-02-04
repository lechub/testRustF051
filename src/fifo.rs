#![no_std]

//struct Fifo<T: ?Sized>{
//	max_size: const usize,
//	data: T,
//	head: usize,
//	tail: usize,
//}


pub enum FifoError{
		BufferEmpty,
		BufferFull,
	}
	
//pub trait Fifo{
//	fn new(tab: & mut [u8]) ->Self;
//	fn put(&self, znak :u8) -> Result<bool, FifoError>;
//	fn get(&self) -> Option<u8>;
//	fn max_capacity(&self)-> usize;
//	fn used_bytes(&self)-> usize;
//	fn available_bytes(&self)-> usize;
//}

pub struct Fifo<'a>{
	head: usize,	// indeks zapisu - wskazuje wolny bajt do zapisu kolejnego bajtu - o ile jest miejsce
	tail: usize,	// indeks odczytu - wskazuje bajt gotowy do odczytu, albo pusty bufor, gdy jest rowny head 
	data: &'a mut[u8]
}



impl<'a> Fifo<'a>{
	pub fn new(tab: &'a mut [u8])->Self{
		Fifo{
			head: 0,
			tail: 0,
			data: tab,
		}
	}
	
	fn increment_index(&self, index: usize)-> usize{
		index +=1;
		if index >= self.data.len() { return 0 };
		index	 
	}
	
	pub fn put(&self, znak: u8) -> Result<bool, FifoError> {
		let var_out = self.tail;
		let var_in = self.head;
		let var_inc = self.increment_index(var_in);
		if var_inc == var_out {
			return Err(FifoError::BufferFull);
		}
		self.data[var_in] = znak;	// zapisanie bajtu
		self.head = var_inc;	// oraz przesunięcie indeksu
		Ok(true)
	}
	
	fn get(&self) -> Option<u8> {
		let var_in = self.head;
		let var_out = self.tail;
		if var_in == var_out {
			return None;
		}
		let nowy = self.data[var_out];
		self.tail = self.increment_index(var_out);
		Some(nowy)
	}
	
	/* maksumalna pojemność jest o 1 mniejsza niż rozmiar tablicy
		 */
	fn max_capacity(&self) -> usize { 
		self.data.len() -1
	}
	
	fn used_bytes(&self) -> usize { 
		let head_var = self.head;
		let tail_var = self.tail;
		if head_var < tail_var {
			head_var += self.data.len();
		}
		head_var - tail_var
	}

	fn available_bytes(&self) -> usize {
		self.max_capacity() - self.used_bytes()
	}

/*  ***************************************************************
	 * @brief  Pobiera 1 slowo 16-bitowe z bufora.
	 * Czyli dwa kolejne bajty - pierwszy jako starszy bajt i drugi jako mlodszy
	 * @return Zwraca slowo (16-bitowe) lub BUFFER_EMPTY_FLAG_U32, gdy pusty bufor
	 */
	fn get_u16(&self) -> Option<u16>{
		let r1: u16 = self.get()?.into();
		let r2: u16 = self.get()?.into();
		let result = (r1 << 8)|r2;
		Some(result)
	}


/** ***************************************************************
	 * @brief  Pobiera 1 slowo 32-bitowe z bufora.
	 * Czyli 4 kolejne bajty - pierwszy jako najstarszy bajt
	 * @param  result	Wskaznik do rezultatu operacji
	 * @return Zwraca slowo (32-bitowe). Poprawna operacja zwraca w *result true, niepoprawna false
	 */
	fn get_u32(&self) -> Option<u32>{	
		let r1: u32 = self.get()?.into();
		let r2: u32 = self.get()?.into();
		let result  = (r1 << 16)|r2;
		Some(result)
	}

	fn put_all(&self, source: &Fifo) -> Result<bool, FifoError>{	
		loop{
			let data_byte = source.get();
			match data_byte{
				None => return Ok(true),
				Some(a) => {
					 if let Err(err) = self.put(a) {
						return Err(err);
				    }
				}
			};
		}
	}
	


}

//macro_rules! make_fifo {
//    ($size:literal) => {
//	let mut buf1 = [0_u8; $size];
//	Fifo::new(&mut buf1);
//	
//	}
//}
//
//fn mf(aa: <'a> u8)->Fifo<'a>{
//	make_fifo!(aa)
//}

//struct Fifo2<T: ?Sized>{
//	head: usize,
//	tail: usize,
//	data: T,
//}
//
//macro_rules! make_fifo {
//    ($size:literal) => {{
//		Fifo{
//			max_size: $size,
//			data: [0; $size],
//		}        
//    }}
//}


//pub struct FF<T>{}
//
//impl<usize> FF<usize>{
//	pub const fn new(size: usize)->Self{
//		make_fifo!(size)
//	} 
//}
//
//
//pub fn mfifo(size: usize)->Result<bool, (FifoHead, [u8; size])>{
//	
//}

//impl<u8> Fifo2<u8>{
//	pub const fn new(aa: u8) ->Self{
//		Fifo2{
//			head: 0,
//			tail: 0,
//			data: aa,
//		}
//	}
//}


//impl Fifo2<[u8; usize]>{
//	pub const fn new(size: usize) -> Self{
//		Fifo2{
//			head: 0,
//			tail: 0,
//			data: [0; usize],
//		}
//	}
//}







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


