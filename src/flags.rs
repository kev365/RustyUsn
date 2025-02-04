use std::fmt;
use serde::ser;


bitflags! {
    pub struct FileAttributes: u32 {
        const ARCHIVE                = 0x0000_0020;
        const COMPRESSED             = 0x0000_0800;
        const DEVICE                 = 0x0000_0040;
        const DIRECTORY              = 0x0000_0010;
        const ENCRYPTED              = 0x0000_4000;
        const HIDDEN                 = 0x0000_0002;
        const INTEGRITY_STREAM       = 0x0000_8000;
        const NORMAL                 = 0x0000_0080;
        const NOT_CONTENT_INDEXED    = 0x0000_2000;
        const NO_SCRUB_DATA          = 0x0002_0000;
        const OFFLINE                = 0x0000_1000;
        const READONLY               = 0x0000_0001;
        const RECALL_ON_DATA_ACCESS  = 0x0040_0000;
        const RECALL_ON_OPEN         = 0x0004_0000;
        const REPARSE_POINT          = 0x0000_0400;
        const SPARSE_FILE            = 0x0000_0200;
        const SYSTEM                 = 0x0000_0004;
        const TEMPORARY              = 0x0000_0100;
        const VIRTUAL                = 0x0000_1000;
    }
}
bitflags! {
    pub struct Reason: u32 {
        const BASIC_INFO_CHANGE      = 0x0000_8000;
        const CLOSE                  = 0x8000_0000;
        const COMPRESSION_CHANGE     = 0x0002_0000;
        const DATA_EXTEND            = 0x0000_0002;
        const DATA_OVERWRITE         = 0x0000_0001;
        const DATA_TRUNCATION        = 0x0000_0004;
        const EA_CHANGE              = 0x0000_0400;
        const ENCRYPTION_CHANGE      = 0x0004_0000;
        const FILE_CREATE            = 0x0000_0100;
        const FILE_DELETE            = 0x0000_0200;
        const HARD_LINK_CHANGE       = 0x0001_0000;
        const INDEXABLE_CHANGE       = 0x0000_4000;
        const INTEGRITY_CHANGE       = 0x0080_0000;
        const NAMED_DATA_EXTEND      = 0x0000_0020;
        const NAMED_DATA_OVERWRITE   = 0x0000_0010;
        const NAMED_DATA_TRUNCATION  = 0x0000_0040;
        const OBJECT_ID_CHANGE       = 0x0008_0000;
        const RENAME_NEW_NAME        = 0x0000_2000;
        const RENAME_OLD_NAME        = 0x0000_1000;
        const REPARSE_POINT_CHANGE   = 0x0010_0000;
        const SECURITY_CHANGE        = 0x0000_0800;
        const STREAM_CHANGE          = 0x0020_0000;
        const TRANSACTED_CHANGE      = 0x0040_0000;
    }
}
bitflags! {
    pub struct SourceInfo: u32 {
        const AUXILIARY_DATA                 = 0x0000_0002;
        const DATA_MANAGEMENT                = 0x0000_0001;
        const REPLICATION_MANAGEMENT         = 0x0000_0004;
        const CLIENT_REPLICATION_MANAGEMENT  = 0x0000_0008;
    }
}

impl fmt::Display for FileAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}

impl ser::Serialize for FileAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}

impl ser::Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

impl fmt::Display for SourceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}

impl ser::Serialize for SourceInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}