check_csk_usb 1.0
MJ
USB状态检测工具 可以通过 --help 获得版主

USAGE:
    check_csk_usb [FLAGS] [OPTIONS]

FLAGS:
        --close-notice    关闭通知模式
    -h, --help            Prints help information
    -l, --listen          监听模式，打开的时候即便检测成功也不退出, 默认不打开
    -V, --version         Prints version information

OPTIONS:
    -p, --pid <pid>            设置 pid, 默认 5678 [default: 5678]
    -t, --timeout <timeout>    设置检查间隔时间单位(秒), 默认 3 秒 [default: 3]
    -v, --vid <vid>            设置 vid, 默认 1234 [default: 1234]