use barcoders::sym::code128::Code128;
use subprocess::{Exec, Redirection};
use image::{GrayImage, Luma};
use std::thread;
use std::time::Duration;

fn main() {

    // 測試用
    // let output = match Exec::cmd("cmd")
    //     .args(&["/C", "type C:\\Users\\abclf\\OneDrive\\桌面\\1.txt"])
    //     .stdout(Redirection::Pipe)
    //     .capture() {
    //         Ok(output) => output.stdout_str(),
    //         Err(e) => {
    //             eprintln!("Failed to capture output: {}", e);
    //             return;
    //         }
    //     };


    // Execute the command to get the deviceid number
    //"netsh mbn show interface | findstr /R \"[0-9][0-9]*$\""
    // 從文件讀取指令
    let output = Exec::cmd("cmd")
        .args(&["/C", "command.bat"])
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    if let Some(deviceid) = output.lines().filter(|line| line.trim().chars().all(char::is_numeric)).last() {
        println!("deviceid: {}", deviceid.trim());
    
        // 添加字符集B的開頭字符Ɓ 不加會報錯 Code128必須加入開頭字符Ɓ
        let full_deviceid = format!("Ɓ{}", deviceid.trim());
    
        let code = match Code128::new(&full_deviceid) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Failed to create barcode: {}", e);
                return;
            }
        };
    
        let encoded = code.encode();
        let width = encoded.len() as u32 * 2; // 調整條形碼寬度
        let height = 100; // 條形碼高度
        let mut image = GrayImage::new(width, height);
    
        // 繪制條形碼
        for (x, barcode_pixel) in encoded.iter().enumerate() {
            let pixel_value = if *barcode_pixel == 0 { 255 } else { 0 };
            for y in 0..height {
                image.put_pixel(x as u32 * 2, y, Luma([pixel_value])); // 單像素寬度
                image.put_pixel(x as u32 * 2 + 1, y, Luma([pixel_value]));
            }
        }

        if let Err(e) = image.save("barcode.png") {
            eprintln!("Failed to save barcode image: {}", e);
        } else {
            println!("Barcode saved as barcode.png");
            // 使用"系統默認"的圖片打開BARCODE圖片
            if std::process::Command::new("cmd")
                .args(&["/C", "start", "barcode.png"])
                .status()
                .is_ok()
            {
                println!("Opened barcode.png");
            } else {
                eprintln!("Failed to open barcode.png");
            }            
        }
    } else {
        println!("No valid deviceid found.");
    }
    // 暫停3秒
    thread::sleep(Duration::from_secs(3));
}