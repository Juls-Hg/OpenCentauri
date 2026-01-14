use std::{
    fs::remove_file, io::{ErrorKind, Read, Write}, os::unix::fs::symlink, sync::{Arc, Mutex}, thread, time::{Duration, Instant}
};

use serialport::{SerialPort, TTYPort};

use crate::{
    communication_handler::CommunicationHandler, kbuf::{kbuf_use_new_buf}, msgbox::MsgboxEndpoint, sharespace::{sharespace_mmap, mmap_log_buffer}
};

mod error;
mod kbuf;
mod msgbox;
mod sharespace;
mod util;
mod communication_handler;

fn log_tail_thread() {
    let log_buffer = mmap_log_buffer();

    println!("Log buffer mapped at: {:p}", log_buffer.as_ptr());

    let mut read_ptr = 4;

    loop {
        let write_ptr = u32::from_le_bytes(log_buffer[0..4].try_into().unwrap()) as usize;

        if write_ptr == read_ptr {
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        let process_buffer = |buffer: &[u8]| {
            for message in buffer.split(|&b| b == 0) {
                if !message.is_empty() {
                    println!("DSP_log: {}", String::from_utf8_lossy(message).trim_end());
                }
            }
        };

        if write_ptr > read_ptr {
            process_buffer(&log_buffer[read_ptr..write_ptr]);
        } else { // write_ptr < read_ptr, wrapped around
            process_buffer(&log_buffer[read_ptr..]);
            process_buffer(&log_buffer[4..write_ptr]);
        }
        
        read_ptr = write_ptr;

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn read_dsp_thread(mut msgbox : MsgboxEndpoint, handler_mutex : Arc<Mutex<CommunicationHandler>>, mut port : TTYPort) {
    let mut arm_head_read_addr = 0;
    loop {
        if !msgbox.msgbox_wait_for_signal()
        {
            thread::sleep(Duration::from_millis(1));
            continue;
        }

        let now = Instant::now();

        let new_data_to_read = match msgbox
            .msgbox_read_signal(arm_head_read_addr) {
                Ok(n) => n,
                Err(e) => {
                    println!("Failed to read signal from msgbox: {}", e);
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
            };

        arm_head_read_addr = msgbox.msgbox_new_msg_write;

        if !new_data_to_read
        {
            println!("Got msgbox message but no data to read?");
            thread::sleep(Duration::from_millis(1));
            continue;
        }
        let data = {
            let mut handler = handler_mutex.lock().expect("Failed to aquire communicationhandler mutex (dsp->host)");
            handler.dsp_mem_read()
        };

        if data.len() <= 0
        {
            println!("No data available to read...");
            thread::sleep(Duration::from_millis(1));
            continue;
        }
        
        port.write_all(&data).unwrap(); // TODO: Error handling
        println!("Read {} bytes from the DSP in {}ms.", data.len(), now.elapsed().as_millis());
    }
}

fn write_dsp_thread(mut msgbox : MsgboxEndpoint, handler_mutex : Arc<Mutex<CommunicationHandler>>, mut port : TTYPort) {
    loop {
        let now = Instant::now();
        let mut buff = [0u8; 4096];
        let len = match port.read(&mut buff) {
            Ok(l) => l,
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                return;
            }
            Err(e) => {
                println!("Error reading from serial port: {}", e);
                panic!();
            }
        };

        {
            let mut handler = handler_mutex.lock().expect("Failed to aquire communicationhandler mutex (host->dsp)");
            handler.dsp_mem_write(&mut msgbox, &buff[..len]);
        }

        println!("Wrote {} bytes to the DSP in {}ms.", len, now.elapsed().as_millis());
    }
}

fn main() {
    println!("Hello, world!");

    std::thread::spawn(log_tail_thread);

    let mmap = sharespace_mmap();
    println!("Got sharespace mmap!");
    let kbuf = kbuf_use_new_buf(mmap.dsp_sharespace.arm_write_addr).unwrap();
    println!("Got kbuf mmap!");
    let mut handler = CommunicationHandler::new(mmap, kbuf);
    println!("Got communication handler!");
    handler.init_no_mmap();
    println!("Done init_no_mmap!");
    handler.wait_dsp_set_init();
    println!("Done DSP init!");
    let msgbox = MsgboxEndpoint::new().unwrap();
    println!("Got msgbox endpoint!");

    let (mut master, slave) = TTYPort::pair().expect("Unable to create ptty pair");
    master.set_timeout(Duration::MAX).unwrap();

    let mut link_path = std::env::temp_dir();
    link_path.push("dsp-serial");

    let _ = remove_file(&link_path);
    let name = slave.name().unwrap();
    symlink(name, &link_path).unwrap();

    println!("Created serial port at {:?}", link_path);

    let master_clone = master.try_clone_native().expect("Unable to clone ptty master");
    let msgbox_clone = msgbox.try_clone().expect("Unable to clone msgbox endpoint");
    let handler_mutex = Arc::new(Mutex::new(handler));
    let handler_mutex_clone = handler_mutex.clone();

    thread::spawn(move || write_dsp_thread(msgbox_clone, handler_mutex_clone, master_clone));
    read_dsp_thread(msgbox, handler_mutex, master);
}
