
const CYLINDER_MASK: u8 = 0xc0;
const SECTOR_MASK: u8 = 0x3f;

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
    pub fn empty() -> CylinderHeadSectorAddress {
        return CylinderHeadSectorAddress{ data: [0,0,0] }
    }
    
    /// Get the Cylinder out of a packed CHS structure.
    pub fn get_cylinder(&self) -> u16 {
        let high_cylinder: u8 = self.data[1];
        let low_cylinder: u8 = self.data[2];
        return ((high_cylinder & CYLINDER_MASK) << 2) as u16
            + low_cylinder as u16;
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
