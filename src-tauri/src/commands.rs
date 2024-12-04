use sysinfo;
#[tauri::command]
pub fn get_processes(){
    let mut sys = sysinfo::System::new();
    sys.refresh_all();
    let t:Vec<_> = sys.processes().iter().map(|(pid, process)| {
        // let _pid = pid.as_u32();
        println!("pid:{},status:{:#?}",pid.as_u32(),process)
    }).collect();
    
}
#[test]
fn test_get_processes(){
    get_processes();
}