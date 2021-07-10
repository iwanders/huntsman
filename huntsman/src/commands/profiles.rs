use struct_helper::*;

pub type ProfileId = u8;

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
/// Retrieve the list of profiles currently active in memory.
pub struct ProfileList {
    pub length: ProfileId,
    #[inspect(dissect_additional_type = "bytes", dissection_hide = "true")]
    pub profile_ids: [ProfileId; 0x10], // only support
}

impl ProfileList {
    pub fn to_vec(&self) -> Vec<ProfileId> {
        let mut ids: Vec<ProfileId> = Vec::new();
        for i in 0..self.length as usize {
            ids.push(self.profile_ids[i])
        }
        ids
    }
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
/// Retrieve the number of profiles in memory.
pub struct ProfileCount {
    pub count: u8,
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
/// Payload to create a profile for the provided id.
pub struct ProfileCreate {
    pub profile_id: ProfileId,
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
/// Payload to delete a profile by id.
pub struct ProfileDelete {
    pub profile_id: ProfileId,
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
/// Payload for setting and retrieving current profile.
pub struct ProfileCurrent {
    pub profile_id: ProfileId,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::commands::helpers::{parse_wireshark_truncated, PAYLOAD_START};

    #[test]
    pub fn test_profiles_get_list() {
        let expected = parse_wireshark_truncated("02:1f:00:00:00:04:05:81:03:01:02:03:00", 0x83);
        let parsed = ProfileList::from_be_bytes(&expected[PAYLOAD_START..]).expect("success");
        assert_eq!(parsed.length, 3);
        assert_eq!(parsed.profile_ids[0], 0x01);
        assert_eq!(parsed.profile_ids[1], 0x02);
        assert_eq!(parsed.profile_ids[2], 0x03);
    }

    #[test]
    pub fn test_profiles_create() {
        let expected = parse_wireshark_truncated("00:1f:00:00:00:01:05:02:01:00", 0x07);
        let parsed = ProfileCreate::from_be_bytes(&expected[PAYLOAD_START..]).expect("success");
        assert_eq!(parsed.profile_id, 1);
    }

    #[test]
    pub fn test_profiles_delete() {
        let expected = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:03:00", 0x04);
        let parsed = ProfileDelete::from_be_bytes(&expected[PAYLOAD_START..]).expect("success");
        assert_eq!(parsed.profile_id, 3);
    }
}
