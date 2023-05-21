use bincode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, Clone, Copy, Debug, EnumIter)]
pub enum Card {
	/// Turn Left
	Left,
	/// Turn Right
	Right,
	/// Keep doing current action
	Wait(u8),
	/// Starts driving foward
	MotorOn,
	/// Stops driving
	MotorOff
}

impl Card {
	/// return embeded sprite
	pub fn picture() {
		unimplemented!()
	}
	/// return the image path
	pub fn path() {
		unimplemented!()
	}
}

/// count of cards, witch are avaibale for the player
#[derive(Clone, Debug, Decode, Default, Encode, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AvailableCards {
	#[cfg_attr(feature = "serde", serde(default))]
	pub left: u8,
	#[cfg_attr(feature = "serde", serde(default))]
	pub right: u8,
	pub wait: u8,
	pub motor_on: u8,
	pub motor_off: u8
}

impl AvailableCards {
	///return how many cards are avaible from the requested variant `card`
	pub fn card_count(&self, card: Card) -> u8 {
		match card {
			Card::Left => self.left,
			Card::Right => self.right,
			Card::Wait(_i) => self.wait,
			Card::MotorOn => self.motor_on,
			Card::MotorOff => self.motor_off
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Card;
	#[test]
	fn as_ref_str() {
		assert_eq!("Left", Card::Left.as_ref());
	}
}
