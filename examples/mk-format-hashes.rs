use pxar::format::hash_filename;

const CONSTANTS: &[(&str, &str, &str)] = &[
    ("", "PXAR_ENTRY", "__PROXMOX_FORMAT_ENTRY__"),
    ("", "PXAR_FILENAME", "__PROXMOX_FORMAT_FILENAME__"),
    ("", "PXAR_SYMLINK", "__PROXMOX_FORMAT_SYMLINK__"),
    ("", "PXAR_DEVICE", "__PROXMOX_FORMAT_DEVICE__"),
    ("", "PXAR_XATTR", "__PROXMOX_FORMAT_XATTR__"),
    ("", "PXAR_ACL_USER", "__PROXMOX_FORMAT_ACL_USER__"),
    ("", "PXAR_ACL_GROUP", "__PROXMOX_FORMAT_ACL_GROUP__"),
    ("", "PXAR_ACL_GROUP_OBJ", "__PROXMOX_FORMAT_ACL_GROUP_OBJ__"),
    ("", "PXAR_ACL_DEFAULT", "__PROXMOX_FORMAT_ACL_DEFAULT__"),
    (
        "",
        "PXAR_ACL_DEFAULT_USER",
        "__PROXMOX_FORMAT_ACL_DEFAULT_USER__",
    ),
    (
        "",
        "PXAR_ACL_DEFAULT_GROUP",
        "__PROXMOX_FORMAT_ACL_DEFAULT_GROUP__",
    ),
    ("", "PXAR_FCAPS", "__PROXMOX_FORMAT_FCAPS__"),
    ("", "PXAR_QUOTA_PROJID", "__PROXMOX_FORMAT_QUOTA_PROJID__"),
    (
        "Marks item as hardlink",
        "PXAR_HARDLINK",
        "__PROXMOX_FORMAT_HARDLINK__",
    ),
    (
        "Marks the beginnig of the payload (actual content) of regular files",
        "PXAR_PAYLOAD",
        "__PROXMOX_FORMAT_PXAR_PAYLOAD__",
    ),
    (
        "Marks item as entry of goodbye table",
        "PXAR_GOODBYE",
        "__PROXMOX_FORMAT_GOODBYE__",
    ),
    (
        "The end marker used in the GOODBYE object",
        "PXAR_GOODBYE_TAIL_MARKER",
        "__PROXMOX_FORMAT_PXAR_GOODBYE_TAIL_MARKER__",
    ),
];

fn main() {
    for constant in CONSTANTS {
        if !constant.0.is_empty() {
            println!("/// {}", constant.0);
        }
        println!(
            "pub const {}: u64 = 0x{:016x};",
            constant.1,
            hash_filename(constant.2.as_bytes()),
        )
    }
}
