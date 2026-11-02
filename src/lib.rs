use windows::{Win32::System::WindowsProgramming::{GetComputerNameA, GetUserNameW}, core::PWSTR};

#[derive(Debug,PartialEq)]
pub struct System {
    pc_name: String,
    uptime: u64,
    owner: String,
}

impl System {
    pub async fn get_system_data() -> anyhow::Result<Self> {
        unsafe {
            let mut comp_buffer = [0u16; 256];
            let mut comp_size = comp_buffer.len() as u32;
            let computer_name = if GetComputerNameW(PWSTR(comp_buffer.as_mut_ptr()), &mut comp_size).is_ok() {
                String::from_utf16_lossy(&comp_buffer[..comp_size as usize]);
            } else {
                "Can't get the name".into()
            };

            // now, username
            let mut user_buffer = [0u16; 256];
            let mut user_size = user_buffer.len() as u32;

            let username = if GetUserNameW(PWSTR(user_buffer.as_mut_ptr()), &mut user_size).is_ok() {
                let actual_size = if user_size > 0 {user_size as usize -1};
                String::from_utf16_lossy(&user_buffer[..actual_size]);
            } else {
                "Can not be retrieved".into()
            };

            // i skip the uptime just now
        }

        Ok(Self{
            // fill this out
            pc_name: computer_name.into(),
            uptime: 20000,
            owner: "ME".into(),
        })
    }
}