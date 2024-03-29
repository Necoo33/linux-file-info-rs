# Linux File Info - Linux Entity Inspector

This crate is for gathering useful infos about linux entities(files, folders or symlinks) with developer friendly way.

This crate is parallel crate of [windows-file-info](https://crates.io/crates/windows-file-info) crate, which developed for handling same tasks on windows and developed by me. If you need an entity inspector for Windows, check out that crate.

It basically makes "sudo ls -l" calls with different ways and parses output nicely.

If you like that crate, give a star that liblary on [github repo](https://github.com/Necoo33/linux-file-info-rs)

It primarily uses that struct about all entities:

```rust

#[derive(Debug)]
pub struct LinuxEntity {
    pub entity_name: String, // current name of the entity
    pub entity_type: String, // type of the entity, file, folder or symlink.
    pub owner: String, // owning user of the  entity
    pub group: String, // 
    pub hardlink: u8,
    pub permission: u16, // permission as numbers.
    pub size: i32, // as bytes.
    pub last_change_date: String, // example: Jan 12 20:49
}

```

sample uses:

Disclaimer: You can give absolute paths to all `other_folder_info()` and `file_info()` and other functions.

```rust
use linux_file_info::*;

fn main(){
    let current_folder = current_folder_info();

    // assuming you have "hello-everyone" folder near to the that project's folder:
    let other_folder = other_folder_info("../hello-everyone");

    // checking /etc/nftables.conf file with absolute path. It also gets info of symlinks:
    let check_cargo_file = file_info("/etc/nftables.conf");

    // checking if Cargo.toml is file:
    let cargo_toml_is_file = is_file("Cargo.toml");

    // checking if src is folder:
    let src_is_folder = is_folder("src");

    // checking if Cargo.lock is a symlink:
    let cargo_lock_is_symlink = is_symlink("Cargo.lock");

    // checking if sfsdfsfds is exist:
    let sfsdfsfds_is_exist = is_exist("sfsdfsfds");

    // checking the current user:
    let who_am_i = get_current_user();


}

```

Warning: `current_folder_info()` function works based on your current directory. If you run this on root directory of your computer, you'll take this kind of response:

```rust
[
    LinuxEntity {
        entity_name: "Docker",
        entity_type: "folder",
        owner: "root",
        group: "root",
        hardlink: 3,
        permission: 755,
        size: 4096,
        last_change_date: "Jul 8 2023",
    },
    LinuxEntity {
        entity_name: "bin",
        entity_type: "symlink",
        owner: "root",
        group: "root",
        hardlink: 1,
        permission: 777,
        size: 7,
        last_change_date: "May 2 2023",
    },
    LinuxEntity {
        entity_name: "boot",
        entity_type: "folder",
        owner: "root",
        group: "root",
        hardlink: 2,
        permission: 755,
        size: 4096,
        last_change_date: "Apr 18 2022",
    },
    LinuxEntity {
        entity_name: "dev",
        entity_type: "folder",
        owner: "root",
        group: "root",
        hardlink: 16,
        permission: 755,
        size: 3560,
        last_change_date: "Jan 13 15:33",
    },
    LinuxEntity {
        entity_name: "etc",
        entity_type: "folder",
        owner: "root",
        group: "root",
        hardlink: 81,
        permission: 755,
        size: 4096,
        last_change_date: "Jan 13 15:33",
    },
    LinuxEntity {
        entity_name: "home",
        entity_type: "folder",
        owner: "root",
        group: "root",
        hardlink: 3,
        permission: 755,
        size: 4096,
        last_change_date: "Jul 8 2023",
    },

    // other entities
]

```

Another Example, if you run `other_folder_info()` function on your computer's main directory with "./etc/ssh" parameter you'll take that kind of answer:

```rust

[
    LinuxEntity {
        entity_name: "ssh_config",
        entity_type: "file",
        owner: "root",
        group: "root",
        hardlink: 1,
        permission: 644,
        size: 1650,
        last_change_date: "Nov 23 2022",
    },
    LinuxEntity {
        entity_name: "ssh_config.d",
        entity_type: "folder",
        owner: "root",
        group: "root",
        hardlink: 2,
        permission: 755,
        size: 4096,
        last_change_date: "Nov 23 2022",
    },
]

```

You have 2 options for giving absolute and more reliable paths:

1 - You can give absolute path on the parameter like this: `other_folder_info("/sys/dev/block")`, `file_info("/sys/dev/block/1:0")`

2 - You can use that functions with defining some kind of bash variables which equals some paths(like your current user's home folder or /root folder) and giving other paths depending on that variable.
