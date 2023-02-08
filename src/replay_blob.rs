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
use std::path::Path;

trait Serialize {
    fn to_file(&self, path: &Path) -> std::io::Result<()>; 
}

trait Deserialize {
    fn from_file(path: &Path) -> std::io::Result<Self> where Self: Sized;
}

pub mod v1 {
    use uuid::Uuid;
    use crate::arma_event;
    use crate::replay_blob::{Serialize, Deserialize};
    use std::path::Path;
    use std::fs;
    
    use std::io::Write;

    pub struct ReplayBlob {
        version: u8,
        replay_uuid: Uuid,
        mission_uuid: Uuid,
        history_uuid: Uuid,
        events: Vec<arma_event::ArmaEvent>
    }

    impl ReplayBlob {
        const VERSION: u8 = 1;
        pub fn new() -> ReplayBlob {
            ReplayBlob {
                version: ReplayBlob::VERSION,
                replay_uuid: Uuid::new_v4(),
                mission_uuid: Uuid::new_v4(),
                history_uuid: Uuid::new_v4(),
                events: Vec::new()
            }
        }
    }

    impl Serialize for ReplayBlob {
        fn to_file(&self, _path: &Path) -> std::io::Result<()> {
            let mut file = fs::File::create(self.replay_uuid.to_string())?;

            file.write_all(&[self.version])?;
            file.write_all(&self.replay_uuid.to_bytes_le())?;
            file.write_all(&self.mission_uuid.to_bytes_le())?;
            file.write_all(&self.history_uuid.to_bytes_le())?;

            for event in &self.events {
                file.write_all(&event.to_bytes_le())?;
            }

            Ok(())
        }
    }

    impl Deserialize for ReplayBlob {
        fn from_file(_path: &Path) -> std::io::Result<ReplayBlob> {
            Ok(ReplayBlob::new())
        }
    }
}

#[cfg(test)]
mod replay_blob_tests {
    

    
}

