use std::{thread, time};
use clap::Clap;

/// USB状态检测工具
/// 可以通过 --help 获得版主
#[derive(Clap)]
#[clap(version = "1.0", author = "MJ")]
struct Opts {
    /// 设置 vid, 默认 1234
    #[clap(short, long, default_value = "1234")]
    vid: String,

    /// 设置 pid, 默认 5678
    #[clap(short, long, default_value = "5678")]
    pid: String,

    /// 设置检查间隔时间单位(秒), 默认 3 秒
    #[clap(short, long, default_value = "3")]
    timeout: u64,

    /// 监听模式，打开的时候即便检测成功也不退出, 默认不打开
    #[clap(short, long)]
    listen: bool,

    /// 关闭通知模式
    #[clap(long)]
    close_notice: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let vid = opts.vid;
    let pid = opts.pid;
    let sleep_time = time::Duration::from_millis(opts.timeout * 1000);

    'check: loop {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            let vendor_id = format!("{:04x}", device_desc.vendor_id());
            let product_id = format!("{:04x}", device_desc.product_id());

            println!("{{\"pid\":{}, \"vid\": {}}}", product_id, vendor_id);

            if !opts.close_notice && *vid == vendor_id  && *pid == product_id {
                println!("{{\"message\": \"csk connected\"}}");
            }

            if !opts.listen && *vid == vendor_id  && *pid == product_id {
                break 'check;
            }
        }

        thread::sleep(sleep_time);
    }
}
