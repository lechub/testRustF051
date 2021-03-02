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
		ConversionError,
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
	data: &'a mut [u8],
}


impl<'a> Fifo<'a>{
	pub fn new(data: &'a mut [u8])->Self{
		Fifo{
			head: 0,
			tail: 0,
			data,
		}
	}
	
//	pub fn new(size: usize)->Self{
//		let 
//		Fifo{
//			head: 0,
//			tail: 0,
//			data: |size| {[0; 6]},
//		}
//	}
	
//	macro_rules! make_fifo {
//    ($size:literal) => {{
//		Fifo::new(&mut [0_u8; $size]);  
//    }}
//}

	
	fn increment_index(&self, index: usize)-> usize{
		let new_index = index + 1;
		if new_index >= self.data.len() { return 0 };
		new_index	 
	}
	
	pub fn put(&mut self, znak: u8) -> Result<bool, FifoError> {
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
	
	pub fn get(&mut self) -> Option<u8> {
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
	pub const fn max_capacity(&self) -> usize { 
		self.data.len() -1
	}
	
	pub fn used_bytes(&self) -> usize { 
		let mut head_var = self.head;
		let tail_var = self.tail;
		if head_var < tail_var {
			head_var += self.data.len();
		}
		head_var - tail_var
	}

	pub fn available_bytes(&self) -> usize {
		self.max_capacity() - self.used_bytes()
	}

/*  ***************************************************************
	 * @brief  Pobiera 1 slowo 16-bitowe z bufora.
	 * Czyli dwa kolejne bajty - pierwszy jako starszy bajt i drugi jako mlodszy
	 * @return Zwraca slowo (16-bitowe) lub BUFFER_EMPTY_FLAG_U32, gdy pusty bufor
	 */
	pub fn get_u16(&mut self) -> Option<u16>{
		let r1: u16 = self.get()?.into();
		let r2: u16 = self.get()?.into();
		let result = (r1 << 8)|r2;
		Some(result)
	}


/* ***************************************************************
	 * @brief  Pobiera 1 slowo 32-bitowe z bufora.
	 * Czyli 4 kolejne bajty - pierwszy jako najstarszy bajt
	 * @param  result	Wskaznik do rezultatu operacji
	 * @return Zwraca slowo (32-bitowe). Poprawna operacja zwraca w *result true, niepoprawna false
	 */
	pub fn get_u32(&mut self) -> Option<u32>{	
		let r1: u32 = self.get()?.into();
		let r2: u32 = self.get()?.into();
		let result  = (r1 << 16)|r2;
		Some(result)
	}

	pub fn put_all(&mut self, source: &mut Fifo) -> Result<bool, FifoError>{	
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


//#[macro_export] 
//macro_rules! make_fifo {
//    ($size:literal) => {{
//		$crate::fifo::Fifo::new(&mut [0_u8; $size]) 
//    }}
//}

#[macro_export]
macro_rules! make_fifo {
    ($size:literal) => {{
		let mut newFifoData = [0; $size];
        crate::fifo::Fifo::new(&mut newFifoData)
    }}
}


fn aa(){
	let _zz = make_fifo!(10);
}





/*

pub struct Fifo<'a>{
	head: usize,	// indeks zapisu - wskazuje wolny bajt do zapisu kolejnego bajtu - o ile jest miejsce
	tail: usize,	// indeks odczytu - wskazuje bajt gotowy do odczytu, albo pusty bufor, gdy jest rowny head 
	data: &'a mut[u8],
}

impl<'a> Fifo<'a>{
	pub fn new(tab: &'a mut [u8])->Self{
		Fifo{
			head: 0,
			tail: 0,
			data: tab,
		}
	}
	
	pub fn new2(size: usize)->Self{
		Fifo{
			head: 0,
			tail: 0,
			data: [0; size],
		}
	}
	
//	macro_rules! make_fifo {
//    ($size:literal) => {{
//		Fifo::new(&mut [0_u8; $size]);  
//    }}
//}

	
	fn increment_index(&self, index: usize)-> usize{
		let new_index = index + 1;
		if new_index >= self.data.len() { return 0 };
		new_index	 
	}
	
	pub fn put(&mut self, znak: u8) -> Result<bool, FifoError> {
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
	
	pub fn get(&mut self) -> Option<u8> {
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
	pub const fn max_capacity(&self) -> usize { 
		self.data.len() -1
	}
	
	pub fn used_bytes(&self) -> usize { 
		let mut head_var = self.head;
		let tail_var = self.tail;
		if head_var < tail_var {
			head_var += self.data.len();
		}
		head_var - tail_var
	}

	pub fn available_bytes(&self) -> usize {
		self.max_capacity() - self.used_bytes()
	}

/*  ***************************************************************
	 * @brief  Pobiera 1 slowo 16-bitowe z bufora.
	 * Czyli dwa kolejne bajty - pierwszy jako starszy bajt i drugi jako mlodszy
	 * @return Zwraca slowo (16-bitowe) lub BUFFER_EMPTY_FLAG_U32, gdy pusty bufor
	 */
	pub fn get_u16(&mut self) -> Option<u16>{
		let r1: u16 = self.get()?.into();
		let r2: u16 = self.get()?.into();
		let result = (r1 << 8)|r2;
		Some(result)
	}


/* ***************************************************************
	 * @brief  Pobiera 1 slowo 32-bitowe z bufora.
	 * Czyli 4 kolejne bajty - pierwszy jako najstarszy bajt
	 * @param  result	Wskaznik do rezultatu operacji
	 * @return Zwraca slowo (32-bitowe). Poprawna operacja zwraca w *result true, niepoprawna false
	 */
	pub fn get_u32(&mut self) -> Option<u32>{	
		let r1: u32 = self.get()?.into();
		let r2: u32 = self.get()?.into();
		let result  = (r1 << 16)|r2;
		Some(result)
	}

	pub fn put_all(&mut self, source: &mut Fifo) -> Result<bool, FifoError>{	
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


#[macro_export] 
macro_rules! make_fifo {
    ($size:literal) => {{
		$crate::fifo::Fifo::new(&mut [0_u8; $size]) 
    }}
}



//fn aa(){
//	let _zz = make_fifo!(10);
//}

*/


