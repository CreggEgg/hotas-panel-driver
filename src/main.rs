use std::{time::{self, Duration}, thread, io::{Read, BufReader, BufRead}, result, fs::read};


use serialport::*;
use vigem_client::TargetId;

fn main() {
	
    let id = TargetId::XBOX360_WIRED;
    println!("Hello, world!");
    let client = vigem_client::Client::connect().unwrap();
    let mut target = vigem_client::Xbox360Wired::new(client,id);
	let mut port: COMPort = serial_setup().expect("uh oh");
	port.set_timeout(Duration::from_millis(1000)).expect("couldnt set timeout");
	

//Plugin the virtual controller
    target.plugin().unwrap();

    target.wait_ready().unwrap();

	// The input state of the virtual controller
	let mut gamepad = vigem_client::XGamepad {
		buttons: vigem_client::XButtons!(UP | DOWN | RIGHT | LEFT | START | BACK | LTHUMB | LB | RB | GUIDE | A | B | X | Y),
		..Default::default()
	};

	let start = time::Instant::now();
    let mut reader = BufReader::new(port);
	let mut last_value = String::new();
	loop {
        let mut serial_buf = String::new();//.read(serial_buf.as_mut_slice()).expect("Found no data!");
		reader.read_line(&mut serial_buf);
		
		// match result {
		// 	Ok(bytes) => {
		// 		println!("{:?}, {}", serial_buf, bytes);
		// 	},
		// 	Err(error) => {
		// 		println!("Went wrong {}", error);
		// 	}
		// }
		

		let elapsed = start.elapsed().as_secs_f64();

		// Play for 10 seconds
		
		if(serial_buf.len() > 0 && serial_buf != last_value) {
			
			if((serial_buf.as_bytes()[0]) == 49) {
				println!("{:?}", serial_buf.as_bytes()[0]);
				gamepad.left_trigger = 255;
			} else {
				gamepad.left_trigger = 0;
			}
		}
		let _ = target.update(&gamepad);
		last_value = serial_buf;
		

		thread::sleep(time::Duration::from_millis(10));
		
	}

    
}

fn serial_setup() -> Result<COMPort> {
	serialport::new("COM3", 9600).open_native()
	
}


