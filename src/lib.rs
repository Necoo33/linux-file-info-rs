use std::{process::Command, fmt::Debug, io::{Error, ErrorKind}, fs};
use std::str::from_utf8;

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct LinuxEntity {
    pub entity_name: String,
    pub entity_type: String,
    pub owner: String,
    pub group: String,
    pub hardlink: u8,
    pub permission: u16,
    pub size: i32,
    pub last_change_date: String,
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct Permissions<'a> {
    pub entity_type: &'a str,
    pub permission: u16
}

#[cfg(target_os = "linux")]
fn decode_permission_string(perm_string: &str) -> Permissions {
    let file_type = perm_string.chars().next().unwrap_or(' ');
    let ft = match file_type {
        '-' => "file",
        'd' => "folder",
        'l' => "symlink",
        _ => panic!("Invalid file type character: {}", file_type),
    };

    let permissions = &perm_string[1..10]; 
    let mut permission_value = 0;

    for (i, chunk) in permissions.chars().collect::<Vec<char>>().chunks(3).enumerate() {
        let mut chunk_value = 0;
        if chunk[0] == 'r' {
            chunk_value += 4;
        }
        if chunk[1] == 'w' {
            chunk_value += 2;
        }
        if chunk[2] == 'x' {
            chunk_value += 1;
        }
        permission_value += chunk_value * 10u16.pow(2 - i as u32);
    }

    Permissions {
        entity_type: ft,
        permission: permission_value,
    }
}

#[cfg(target_os = "linux")]
pub fn current_folder_info() -> Vec<LinuxEntity> {
    let check_permission = Command::new("sudo").arg("ls").arg("-l").output().unwrap();
    let mut entities: Vec<LinuxEntity> = vec![];
    let output = from_utf8(&check_permission.stdout).unwrap();

    for (index, line) in output.lines().into_iter().enumerate() {
        if index == 0 {
            continue;
        }

        let split_the_line: Vec<&str> = line.split_whitespace().collect();

        let perm_str = decode_permission_string(split_the_line[0]);

        let new_entity = LinuxEntity {
            entity_name: split_the_line[8].to_string(),
            entity_type: perm_str.entity_type.to_string(),
            permission: perm_str.permission,
            owner: split_the_line[2].to_string(),
            group: split_the_line[3].to_string(),
            hardlink: split_the_line[1].parse().unwrap(),
            size: split_the_line[4].parse().unwrap(),
            last_change_date: format!("{} {} {}", split_the_line[5], split_the_line[6], split_the_line[7])
        };

        entities.push(new_entity);
    }

    return entities;
}

#[cfg(target_os = "linux")]
pub fn other_folder_info(path: &str) -> Result<Vec<LinuxEntity>, Error> {
    let check_permission = Command::new("sudo").arg("ls").arg("-l").arg(path).output().unwrap();
    let mut entities: Vec<LinuxEntity> = vec![];
    let output = from_utf8(&check_permission.stdout).unwrap();

    if !&output.starts_with("total ") {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid input. If you want to check a file, use 'check_file()' function instead."));
    }

    for (index, line) in output.lines().into_iter().enumerate() {
        if index == 0 {
            continue;
        }

        let split_the_line: Vec<&str> = line.split_whitespace().collect();

        let perm_str = decode_permission_string(split_the_line[0]);

        let new_entity = LinuxEntity {
            entity_name: split_the_line[8].to_string(),
            entity_type: perm_str.entity_type.to_string(),
            permission: perm_str.permission,
            owner: split_the_line[2].to_string(),
            group: split_the_line[3].to_string(),
            hardlink: split_the_line[1].parse().unwrap(),
            size: split_the_line[4].parse().unwrap(),
            last_change_date: format!("{} {} {}", split_the_line[5], split_the_line[6], split_the_line[7])
        };

        entities.push(new_entity);
    }

    return Ok(entities);
}

#[cfg(target_os = "linux")]
pub fn file_info(path: &str) -> Result<LinuxEntity, Error> {
    let run_command = Command::new("sudo").arg("ls").arg("-l").arg(path).output().unwrap();
    let output = from_utf8(&run_command.stdout).unwrap();

    if output.starts_with("total ") {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid input. If you want to check a folder, use 'check_other_folder()' function instead."))
    }

    let split_the_output: Vec<&str> = output.split_whitespace().collect();

    let perm_str = decode_permission_string(split_the_output[0]);

    let file_name;
    
    if split_the_output[8].contains("/") {
        let split_the_file_name: Vec<&str> = split_the_output[8].split("/").collect();
        
        file_name = split_the_file_name.last().unwrap().to_string();
    } else {
        file_name = split_the_output[8].to_string()
    }

    Ok(LinuxEntity {
        entity_name: file_name,
        entity_type: perm_str.entity_type.to_string(),
        permission: perm_str.permission,
        owner: split_the_output[2].to_string(),
        group: split_the_output[3].to_string(),
        hardlink: split_the_output[1].parse().unwrap(),
        size: split_the_output[4].parse().unwrap(),
        last_change_date: format!("{} {} {}", split_the_output[5], split_the_output[6], split_the_output[7])
    })
}

#[cfg(target_os = "linux")]
pub fn is_file(path: &str) -> bool {
    let run_command = Command::new("sudo").arg("ls").arg("-l").arg(path).output().unwrap();
    let output = from_utf8(&run_command.stdout).unwrap();

    if output.starts_with("-") {
        return true;
    } else {
        return false;
    }
} 

#[cfg(target_os = "linux")]
pub fn is_folder(path: &str) -> bool {
    let run_command = Command::new("sudo").arg("ls").arg("-l").arg(path).output().unwrap();
    let output = from_utf8(&run_command.stdout).unwrap();

    if output.starts_with("total ") {
        return true;
    } else {
        return false;
    }
}

#[cfg(target_os = "linux")]
pub fn is_symlink(path: &str) -> bool {
    let run_command = Command::new("sudo").arg("ls").arg("-l").arg(path).output().unwrap();
    let output = from_utf8(&run_command.stdout).unwrap();

    if output.starts_with("l") {
        return true;
    } else {
        return false;
    }
}

pub fn is_exist(path: &str) -> bool {
    if fs::metadata(path).is_ok() {
        true
    } else {
        false
    }
}


#[cfg(target_os = "linux")]
pub fn get_current_user() -> String {
    let find_user_command = Command::new("whoami").output();

    return match find_user_command {
        Ok(user) => from_utf8(&user.stdout).unwrap().trim().to_string(),
        Err(error) => {
            eprintln!("cannot check the user because of that: {}", error);

            "".to_string()
        }
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_permission_string(){
        let perm_str = decode_permission_string("-rw-r--r--");

        assert_eq!(perm_str.permission, 644 as u16);
        assert_eq!(perm_str.entity_type, "file");
    }

    #[test]
    fn test_current_folder_info() {
        println!("Current Folder's Entities: {:#?}", current_folder_info())
    }

    #[test]
    fn test_other_folder_info(){
        println!("/sys/dev/block folder's entities: {:#?}", other_folder_info("/sys/dev/block"))
    }

    #[test]
    fn test_file_info(){
        println!("Check 1:0 file: {:#?}", file_info("/sys/dev/block/1:0"))
    }

    #[test]
    fn test_is_file(){
        assert_eq!(true, is_file("Cargo.toml"))
    }

    #[test]
    fn test_is_folder() {
        assert_eq!(false, is_folder("Cargo.toml"))
    }

    #[test]
    fn test_is_symlink(){
        assert_eq!(true, is_symlink("/sys/dev/block/1:0"))
    }

    #[test]
    fn test_is_exist(){
        assert_eq!(false, is_exist("dfsgdfsgd"))
    }

    #[test]
    fn test_get_current_user(){
        assert_eq!(true, get_current_user().is_ok())
    }

}
