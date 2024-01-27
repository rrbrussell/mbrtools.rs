
/// The MBR for IBM PC and compatibles has a defined size of 512 bytes.
pub const MBR_SIZE: u16 = 512;
const CYLINDER_MASK: u8 = 0xc0;
const SECTOR_MASK: u8 = 0x3f;

pub enum Variations {
    Classical,
    Modern,
    AAP,
    NEWLDR,
    ASTDOS,
    Ontrack
}

/// An partition in the MBR.
pub struct PartitionEntry {
    status: u8,
    chs_first_absolute_sector: CylinderHeadSectorAddress,
    partition_type: u8,
    chs_last_absolute_sector: CylinderHeadSectorAddress,
    lba_first_absolute_sector: LogicalBlockAddress,
    lba_length: LogicalBlockAddress,
}

/// A packed CHS structure.
///
/// This is in little endian format.
///
/// The first byte is the Head address.
/// The second byte is split between the upper bits of the Cylinder address and
/// the Sector address.
/// The third byte is the lower bits of the Cylinder address.
///
/// Access and writing methods are provided for you.
pub struct CylinderHeadSectorAddress {
    data: [u8; 3],
    }

impl CylinderHeadSectorAddress {
    /// Get the Cylinder out of a packed CHS structure.
    pub fn get_cylinder(&self) -> u16 {
        let high_cylinder: u8 = self.data[1];
        let low_cylinder: u8 = self.data[2];
        let mut return_value: u16;
        return_value = ((high_cylinder & CYLINDER_MASK) << 2) as u16
            + low_cylinder as u16;
        return return_value;
    }

    /// Get the Head out of a packed CHS structure.
    pub fn get_head(&self) -> u8 {
        return self.data[0];
    }

    /// Get the Sector out of a pack CHS structure.
    pub fn get_sector(&self) -> u8 {
        return self.data[2] & SECTOR_MASK;
    }

    /// Set the Cylinder in a packed CHS structure.
    pub fn set_cylinder(&mut self, cylinder: u16) {
        let high_cylinder: u8;
        let low_cylinder: u8 = cylinder as u8;
        high_cylinder = ((cylinder >> 2) as u8 & CYLINDER_MASK) as u8;
        self.data[1] = high_cylinder + (self.data[1] & SECTOR_MASK);
        self.data[2] = low_cylinder;
    }

    /// Set the Head in a packed CHS structure.
    pub fn set_head(&mut self, head: u8) {
        self.data[0] = head;
    }

    /// Set the Sector in a packed CHS structure.
    pub fn set_sector(&mut self, sector: u8) {
        self.data[1] = (sector & CYLINDER_MASK) + (self.data[1] & SECTOR_MASK);
    }
}

/// LBA or Logical Block Addressing is the replacement for CHS for specifying
/// the size of a partition. MBR uses a 32 bit little endian integer for this
/// task.
pub struct LogicalBlockAddress {
    data: u32,
}

impl LogicalBlockAddress {
    pub fn read_from_bytes(bytes:&mut &[u8]) -> Option<LogicalBlockAddress> {
        if bytes.len() < 4 {
            // We don't have enough input data.
            return None;
        }
        let (input_bytes, rest) = bytes.split_at(4);
        *bytes = rest;
        let mut local_bytes: [u8; 4]= [0,0,0,0];
        for i in 0..4 {
            local_bytes[i] = input_bytes[i];
        }
        return Some(LogicalBlockAddress{data: u32::from_be_bytes(local_bytes)});
    }

    /// Get a block address.
    pub fn get_address(&self) -> u32 {
        return self.data;
    }

    /// Store a block address.
    pub fn set_address(&mut self, address: u32) {
        self.data = address;
    }

    /// Convert a `LogicalBlockAddress` to a byte array
    pub fn write_to_bytes(&self) -> [u8; 4] {
       return u32::to_le_bytes(self.data);
    }
}
