pub use crate::chsa::CylinderHeadSectorAddress;

#[path = "chsa.rs"]
pub mod chsa;

/// The MBR for IBM PC and compatibles has a defined size of 512 bytes.
pub const MBR_SIZE: u16 = 512;
const CYLINDER_MASK: u8 = 0xc0;
const SECTOR_MASK: u8 = 0x3f;

/// The different variations of MBR that exist.
/// Most of these are historical from my research.
pub enum Variations {
    Classical,
    Modern,
    AAP,
    NEWLDR,
    ASTDOS,
    Ontrack
}

/// The bootable or not bootable status
///
/// TODO! add extra documentation.
#[repr(u8)]
pub enum PartitionStatus {
    NotBootable = 0x00,
    Invalid = 0x7F,
    Bootable = 0x80,
}

impl From<u8> for PartitionStatus {
    fn from(value: u8) -> Self {
        if value == 0 {
            return PartitionStatus::NotBootable;
        } else if value == 128 {
            return PartitionStatus::Bootable;
        }
        return PartitionStatus::Invalid;
    }
}

/// An partition in the MBR.
pub struct PartitionEntry {
    status: PartitionStatus,
    chs_first_absolute_sector: CylinderHeadSectorAddress,
    partition_type: u8,
    chs_last_absolute_sector: CylinderHeadSectorAddress,
    lba_first_absolute_sector: LogicalBlockAddress,
    lba_length: LogicalBlockAddress,
}

impl PartitionEntry {
    pub fn empty() -> PartitionEntry {
        return PartitionEntry{ status: PartitionStatus::NotBootable,
            chs_first_absolute_sector: CylinderHeadSectorAddress::empty(),
            partition_type: 0,
            chs_last_absolute_sector: CylinderHeadSectorAddress::empty(),
            lba_first_absolute_sector: LogicalBlockAddress::empty(),
            lba_length: LogicalBlockAddress::empty()
        };
    }
}


/// LBA or Logical Block Addressing is the replacement for CHS for specifying
/// the size of a partition. MBR uses a 32 bit little endian integer for this
/// task. The accessory functions already account for to and from little endian
/// conversion.
pub struct LogicalBlockAddress {
    data: u32,
}

impl LogicalBlockAddress {
    /// Returns an empty LBA
    pub fn empty() -> LogicalBlockAddress {
        return LogicalBlockAddress { data: 0 };
    }
    
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

    /// Convert a `LogicalBlockAddress` to a byte array.
    pub fn write_to_bytes(&self) -> [u8; 4] {
       return u32::to_le_bytes(self.data);
    }
}

#[repr(u8)]
pub enum PartitionType {
    /// Designates that the table entry is empty.
    Empty = 0x00,
    /// Obsolete as of MS-DOS 3.3
    DosFat12,
    /// Obsolete -- Old Unix V7 Port 
    XenixRoot,
    /// Obsolete -- Old Unix V7 Port
    XenixUsr,
    /// Obsolete -- Sub 32MebiByte partitions from DOS 3.0
    Dos3Fat16,
    /// Obsolete -- DOS 3.3+ Extended Partition
    Dos3Extended,
    /// DOS 3.31 FAT 16
    Dos331Fat16,
    /// NTFS and exFAT use the same id
    NtfsExfat,
    /// AIX Boot Partition
    AixBoot,
    /// AIX Data Partition
    AixData,
    /// OS/2 Boot Manager
    OS2BootManger,
    Win95Fat32,
    /// LBA version of Win95Fat32
    Win95Fat32LBA,
    /// Avoid if MSDOS is also used.
    Win95Fat16LBA = 0x0e,
    /// Avoid if MSDOS is also used.
    Win95ExtendedLBA,
    /// Hidden by OS/2
    HiddenDosFat12 = 0x11,
}
