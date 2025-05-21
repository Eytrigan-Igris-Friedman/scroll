use  libc::{TCSANOW, ICANON, ECHO, tcgetattr, tcsetattr, termios};
use std::io;
use std::os::unix::io::AsRawFd;

static stdin_fd:i32 = io::stdin().as_raw_fd();
 
static mut t = unsafe {
    static mut term_ios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 19],        // Control character array
        ..std::mem::zeroed()  // Ensuring safety while initializing fields
    };
    tcgetattr(stdin_fd, &mut term_ios);
    term_ios
};


pub fn enable_raw_mode(){
   
    t.c_lflag &= !(ICANON | ECHO); // Disable canonical mode and echo
    
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &t);
    }
}


pub fn disable_raw_mode(){
     // Restore original settings before exit
    t.c_lflag |= ICANON | ECHO;
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &t);
    }
}
