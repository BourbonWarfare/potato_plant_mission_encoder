/*
    potato_plant_mission_encoder: Encodes mission data from game and compiles it into parsable history format
    Copyright (C) 2022  Bailey Danyluk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::time::Duration;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Type {
    ObjectCreated,
    ObjectKilled,
    Fired,
    MarkerCreated,
    MarkerDestroyed,
    MarkerUpdated,
    ObjectGetIn,
    ObjectGetOut,
    Custom,
    MissionStart,
    MissionEnd
}

pub struct ArmaEvent(Duration, Type);

impl ArmaEvent {
    pub fn to_bytes_le(&self) -> [u8; 8 + 1] {
        let duration_bytes = self.0.as_secs().to_le_bytes();
        let type_int = self.1 as u8;
        let type_bytes = type_int.to_le_bytes();

        [duration_bytes[0], duration_bytes[1], duration_bytes[2], duration_bytes[3], duration_bytes[4], duration_bytes[5], duration_bytes[6], duration_bytes[7], type_bytes[0]]
    }
}

#[cfg(test)]
mod arma_event_tests {
    use super::*;

    #[test]
    fn ensure_event_converts_to_bytes() {
        let test_second = 457853;
        let test_type = Type::Custom;

        let event = ArmaEvent(Duration::from_secs(test_second), test_type);

        let bytes = event.to_bytes_le();

        let test_second_bytes = test_second.to_le_bytes();
        let test_type_bytes = (test_type as u8).to_le_bytes();
        assert_eq!(bytes, [test_second_bytes[0], test_second_bytes[1], test_second_bytes[2], test_second_bytes[3], test_second_bytes[4], test_second_bytes[5], test_second_bytes[6], test_second_bytes[7], test_type_bytes[0]]);
    }
}
