use chrono::Local;
use std::{collections::{BTreeMap, HashMap}};

use sysinfo::{
    Disks,  System
};

fn main() {
    
    println!("\n\n\n\n=========================================");
    println!("     СТАТИСТИКА ПРОИЗВОДИТЕЛЬНОСТИ ПК");
    println!("=========================================");
    println!("Generated on: {}", Local::now().format("%Y-%m-%d %H:%M:%S\n\n\n\n"));



    let mut sys = System::new_all();
    sys.refresh_all();

    cpu_usage(&mut sys);
    memory_usage(&mut sys);
    disk_usage();
    top_5_process_cpu(&mut sys);
    top_5_process_memory(&mut sys);


    println!("\n\n\n\n=========================================");
    println!("             КОНЕЦ ОТЧЕТА");
    println!("=========================================");
}

fn cpu_usage(sys: &mut System){


    let mut global_cpu_usage: f32 = 0.0;
    let mut cpu_frequency = 0;
    

    for cpu in sys.cpus(){
        global_cpu_usage += cpu.cpu_usage();
        cpu_frequency = cpu.frequency();
    }
    global_cpu_usage /= 4.0;

    println!("СОСТОЯНИЕ ПРОЦЕССОРА:\n");
    println!("НАГРУЗКА ПРОЦЕССОРА {}% ЧАСТОТА: {} Mhz\n\n", global_cpu_usage.round(), cpu_frequency);
}

fn memory_usage(sys: &mut System) {
    
    let total_memory = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let  memory_usage = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0; 
    
    println!("СОСТОЯНИЕ ОПЕРАТИВНОЙ ПАМЯТИ:\n");
    println!("ОБЩИЙ ОБЪЕМ ПАМЯТИ: {:.1} ГБ ИСПОЛЬЗУЕТСЯ {:.1} ГБ\n\n", total_memory, memory_usage)

}

fn disk_usage() {

    let mut disk_map = HashMap::new();

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let disk_name = disk.mount_point().to_string_lossy();
        let disk_letter = disk_name.chars().next().unwrap_or('?');
        
        
        let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0 ;
        let occupied_space = total_space - available_space;
        let percent_occupied_space = (occupied_space / total_space) * 100.0;
        


        disk_map.insert(disk_letter, (total_space, available_space, occupied_space, percent_occupied_space));        

    }
    

    let sorted_disk_map: BTreeMap<_, _> = disk_map.into_iter().collect();
    
    println!("СОСТОЯНИЕ ДИСКОВ:\n");

    for (letter, (total, available, occupied, percent)) in sorted_disk_map {
        println!("{}: ОБЩИЙ ОБЪЕМ: {:.1} ГБ СВОБОДНО: {:.1} ГБ ЗАНЯТО: {:.1} ГБ - {:.1}%", letter, total, available, occupied, percent);
    }
}

fn top_5_process_cpu(sys: &mut System) {

    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
        
    println!("\n\nТОП-5 ПРОЦЕССОВ ПО ИСПОЛЬЗОВАНИЮ ПРОЦЕССОРА:\n");
    
    for process in processes.iter().take(5) {
        println!("{:?}", process.name());
    }
}



fn top_5_process_memory(sys: &mut System) {

    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().partial_cmp(&a.memory()).unwrap());
        
    println!("\n\nТОП-5 ПРОЦЕССОВ ПО ИСПОЛЬЗОВАНИЮ ОПЕРАТИВНОЙ ПАМЯТИ:\n");
    
    for process in processes.iter().take(5) {
        println!("{:?}", process.name());
    }
}