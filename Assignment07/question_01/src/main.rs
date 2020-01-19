//parent module with name of car.
mod car {
	//Punblic sub module with name of car_props.
	pub mod car_props { 
		//public funtion model() inside car_props module.
		pub fn model() -> String {
			String::from("Mercedes Benz S-class 2019")
		}

	}
}

use car::car_props;

fn main() {

   let what_car = car_props::model();

   println!("Car model: {}",what_car );

}
