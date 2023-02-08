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

struct Header {
    version: u16,
    byte_size: u16,
    packet_number: u16,
    packet_count: u16,
    packet_type: u16,
    packet_group: u8,
}

pub struct Packet {
    header: Header,
    data: [u8]
}

