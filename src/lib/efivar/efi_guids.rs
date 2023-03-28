use crate::types::EfiGuid;
use crate::types::EfiGuidListEntry;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    pub static ref EFI_WELL_KNOWN_GUIDS: HashMap<&'static str, EfiGuidListEntry> = HashMap::from([
        (
            "zero",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                name: "zero",
                description: "zeroed sentinel guid",
            }
        ),
        (
            "redhat",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("0223eddb-9079-4388-af77-2d65b1c35d3b").unwrap(),
                name: "redhat",
                description: "Red Hat",
            }
        ),
        (
            "sha512",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("093e0fae-a6c4-4f50-9f1b-d41e2b89c19a").unwrap(),
                name: "sha512",
                description: "SHA-512 hash",
            }
        ),
        (
            "fwupdate",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("0abba7dc-e516-4167-bbf5-4d9d1c739416").unwrap(),
                name: "fwupdate",
                description: "Linux Firmware Update Tool",
            }
        ),
        (
            "sha224",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("0b6e5233-a65c-44c9-9407-d9ab83bfc8bd").unwrap(),
                name: "sha224",
                description: "SHA-224 hash",
            }
        ),
        (
            "lenovo_boot_menu",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("126a762d-5758-4fca-8531-201a7f57f850").unwrap(),
                name: "lenovo_boot_menu",
                description: "Lenovo Boot Menu",
            }
        ),
        (
            "supermicro",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("26dc4851-195f-4ae1-9a19-fbf883bbb35e").unwrap(),
                name: "supermicro",
                description: "Super Micro",
            }
        ),
        (
            "asus",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("3b053091-6c9f-04cc-b1ac-e2a51e3be5f5").unwrap(),
                name: "asus",
                description: "Asus",
            }
        ),
        (
            "ux_capsule",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("3b8c8162-188c-46a4-aec9-be43f1d65697").unwrap(),
                name: "ux_capsule",
                description: "Firmware update localized text image",
            }
        ),
        (
            "x509_sha256",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("3bd2a492-96c0-4079-b420-fcf98ef103ed").unwrap(),
                name: "x509_sha256",
                description: "SHA-256 hash of X.509 Certificate",
            }
        ),
        (
            "rsa2048",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("3c5766e8-269c-4e34-aa14-ed776e85b3b6").unwrap(),
                name: "rsa2048",
                description: "RSA 2048 pubkey",
            }
        ),
        (
            "lenovo",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("3cc24e96-22c7-41d8-8863-8e39dcdcc2cf").unwrap(),
                name: "lenovo",
                description: "Lenovo",
            }
        ),
        (
            "lenovo_diag",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("3f7e615b-0d45-4f80-88dc-26b234958560").unwrap(),
                name: "lenovo_diag",
                description: "Lenovo Diagnostics",
            }
        ),
        (
            "x509_sha512",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("446dbf63-2502-4cda-bcfa-2465d2b0fe9d").unwrap(),
                name: "x509_sha512",
                description: "SHA-512 hash of X.509 Certificate",
            }
        ),
        (
            "external_management",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("452e8ced-dfff-4b8c-ae01-5118862e682c").unwrap(),
                name: "external_management",
                description: "External Management Mechanism",
            }
        ),
        (
            "pkcs7_cert",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("4aafd29d-68df-49ee-8aa9-347d375665a7").unwrap(),
                name: "pkcs7_cert",
                description: "PKCS7 Certificate",
            }
        ),
        (
            "fives",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("55555555-5555-5555-5555-555555555555").unwrap(),
                name: "fives",
                description: "All Fives Test Data",
            }
        ),
        (
            "shim",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("605dab50-e046-4300-abb6-3dd810dd8b23").unwrap(),
                name: "shim",
                description: "shim",
            }
        ),
        (
            "lenovo_rescue",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("665d3f60-ad3e-4cad-8e26-db46eee9f1b5").unwrap(),
                name: "lenovo_rescue",
                description: "Lenovo Rescue and Recovery",
            }
        ),
        (
            "rsa2048_sha1",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("67f8444f-8743-48f1-a328-1eaab8736080").unwrap(),
                name: "rsa2048_sha1",
                description: "RSA-2048 signature of a SHA-1 hash",
            }
        ),
        (
            "canonical",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("6dc40ae4-2ee8-9c4c-a314-0fc7b2008710").unwrap(),
                name: "canonical",
                description: "Canonical",
            }
        ),
        (
            "dell",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("70564dce-9afc-4ee3-85fc-949649d7e45c").unwrap(),
                name: "dell",
                description: "Dell",
            }
        ),
        (
            "x509_sha384",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("7076876e-80c2-4ee6-aad2-28b349a6865b").unwrap(),
                name: "x509_sha384",
                description: "SHA-384 hash of X.509 Certificate",
            }
        ),
        (
            "lenovo_setup",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("721c8b66-426c-4e86-8e99-3457c46ab0b9").unwrap(),
                name: "lenovo_setup",
                description: "Lenovo Firmware Setup",
            }
        ),
        (
            "microsoft",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("77fa9abd-0359-4d32-bd60-28f4e78f784b").unwrap(),
                name: "microsoft",
                description: "Microsoft",
            }
        ),
        (
            "lenovo_2",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("7facc7b6-127f-4e9c-9c5d-080f98994345").unwrap(),
                name: "lenovo_2",
                description: "Lenovo",
            }
        ),
        (
            "auto_created_boot_option",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("8108ac4e-9f11-4d59-850e-e21a522c59b2").unwrap(),
                name: "auto_created_boot_option",
                description: "Automatically Created Boot Option",
            }
        ),
        (
            "sha1",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("826ca512-cf10-4ac9-b187-be01496631bd").unwrap(),
                name: "sha1",
                description: "SHA-1",
            }
        ),
        (
            "lenovo_me_config",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("82988420-7467-4490-9059-feb448dd1963").unwrap(),
                name: "lenovo_me_config",
                description: "Lenovo ME Configuration Menu",
            }
        ),
        (
            "global",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("8be4df61-93ca-11d2-aa0d-00e098032b8c").unwrap(),
                name: "global",
                description: "EFI Global Variable",
            }
        ),
        (
            "grub",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("91376aff-cba6-42be-949d-06fde81128e8").unwrap(),
                name: "grub",
                description: "GRUB",
            }
        ),
        (
            "x509_cert",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("a5c059a1-94e4-4aa7-87b5-ab155c2bf072").unwrap(),
                name: "x509_cert",
                description: "X.509 Certificate",
            }
        ),
        (
            "rsa2048_sha256_cert",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("a7717414-c616-4977-9420-844712a735bf").unwrap(),
                name: "rsa2048_sha256_cert",
                description: "RSA-2048 key with SHA-256 Certificate",
            }
        ),
        (
            "lenovo_diag_splash",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("a7d8d9a6-6ab0-4aeb-ad9d-163e59a7a380").unwrap(),
                name: "lenovo_diag_splash",
                description: "Lenovo Diagnostic Splash Screen",
            }
        ),
        (
            "lenovo_msg",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("bc7838d2-0f82-4d60-8316-c068ee79d25b").unwrap(),
                name: "lenovo_msg",
                description: "Lenovo Vendor Message Device",
            }
        ),
        (
            "sha256",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("c1c41626-504c-4092-aca9-41f936934328").unwrap(),
                name: "sha256",
                description: "SHA-256",
            }
        ),
        (
            "shell",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("c57ad6b7-0515-40a8-9d21-551652854e37").unwrap(),
                name: "shell",
                description: "EFI Shell",
            }
        ),
        (
            "security",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("d719b2cb-3d3a-4596-a3bc-dad00e67656f").unwrap(),
                name: "security",
                description: "EFI Security Database",
            }
        ),
        (
            "rsa2048_sha256",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("e2b36190-879b-4a3d-ad8d-f2e7bba32784").unwrap(),
                name: "rsa2048_sha256",
                description: "RSA-2048 signature of a SHA-256 hash",
            }
        ),
        (
            "lenovo_startup_interrupt",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("f46ee6f4-4785-43a3-923d-7f786c3c8479").unwrap(),
                name: "lenovo_startup_interrupt",
                description: "Lenovo Startup Interrupt Menu",
            }
        ),
        (
            "sha384",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("ff3e5307-9fd0-48c9-85f1-8ad56c701e01").unwrap(),
                name: "sha384",
                description: "SHA-384",
            }
        ),
        (
            "zzignore-this-guid",
            EfiGuidListEntry {
                guid: EfiGuid::from_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap(),
                name: "zzignore-this-guid",
                description: "zzignore-this-guid",
            }
        ),
    ]);
}

impl EFI_WELL_KNOWN_GUIDS {
    pub fn sort_by_guid(&self) -> Vec<&EfiGuidListEntry> {
        let mut sorted_entries = self.values().collect::<Vec<_>>();
        sorted_entries.sort_unstable_by(|e1, e2| e1.guid.cmp(&e2.guid));
        return sorted_entries;
    }

    pub fn sort_by_name(&self) -> Vec<&EfiGuidListEntry> {
        let mut sorted_entries = self.values().collect::<Vec<_>>();
        sorted_entries.sort_unstable_by(|e1, e2| e1.name.cmp(&e2.name));
        return sorted_entries;
    }
}
