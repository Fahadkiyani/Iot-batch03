// public module with name of car.  
pub mod car {
	//Public sub module with name of car_props.
	pub mod car_props { 
		//public funtion model() inside car_props module.
		pub fn model() -> String {
			String::from("BMW M5 Sedan 2020")
		}

	}
}