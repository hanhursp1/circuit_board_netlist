pub mod parser_ipc356a;
pub mod netlist;

#[cfg(test)]
mod tests {
	use crate::netlist::Netlist;

use super::*;

	#[test]
	fn it_works() {
		const TEST_STR: &str = "P  JOB 
P  CODE  UTF-8
P  UNITS  CUST0
P  TITLE  
P  NUM   1.0
P  REV   A
P  VER   IPC-D-356A317GND              U30   -2    D0472PA00X 041600Y 015589X0709Y         S0
317+5V              U30   -3    D0472PA00X 042600Y 015585X0709Y         S0
317+12V             U30   -1    D0472PA00X 040600Y 015589X0709Y1024R000 S0
317GND              C1    -1    D0315PA00X 045322Y 016900X0551Y         S0
317+5V              C1    -2    D0315PA00X 044278Y 014900X0551Y         S0
317GND              C2    -1    D0315PA00X 038400Y 016900X0630Y         S0
317+12V             C2    -2    D0315PA00X 038400Y 014900X0630Y         S0
317+12V             U31   -1    D0276PA00X 036150Y 014906X0433Y         S0
317GND              U31   -2    D0276PA00X 036150Y 015694X0433Y         S0
317+5V              U32   -1    D0276PA00X 046850Y 015056X0433Y         S0
317GND              U32   -2    D0276PA00X 046850Y 015844X0433Y         S0
317GND              RESET1-4    D0472PA00X 040674Y 007432X0787Y         S0
317GND              RESET1-3    D0472PA00X 043233Y 007432X0787Y         S0
317A_DEC            RESET1-2    D0472PA00X 043233Y 005660X0787Y         S0
317A_DEC            RESET1-1    D0472PA00X 040674Y 005660X0787Y         S0
317GND              RESET2-4    D0472PA00X 045055Y 007432X0787Y         S0
317GND              RESET2-3    D0472PA00X 047614Y 007432X0787Y         S0
317A_INC            RESET2-2    D0472PA00X 047614Y 005660X0787Y         S0
317A_INC            RESET2-1    D0472PA00X 045055Y 005660X0787Y         S0";
		
		let net = parser_ipc356a::parse_netlist(&String::from(TEST_STR));
		println!("Units: {:?}", net.units);
		let net: Netlist = net.into();
		println!("{}", net);
		let x: Box<[i32]> = Box::new([1, 2, 3]);
		println!("{:?}", x);
		println!("{}", x.len());
	}
}
