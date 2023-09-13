

// mod info;
mod change;

macro_rules! exit {
    ($code:expr) => {
        unsafe{
            exit($code);
        }
    };
}




fn main() {
    
    // info::info();
    change::change_mac("enp2s0", "96d8d983f823");
    extern "C" {
        fn exit(status:i8);
        fn ioctl(r:u8,rq:u8,e:*const i8);
        fn perror(a:*const i8);
        fn printf(r : *const i8, ...);
        fn char(c:u8);
    }

    // exit!(5);

    // unsafe{
        // ioctl(1,5,"dasda".as_ptr() as *const i8);
        // perror("da".as_ptr() as *const i8 )};

}

//ioctl(3, SIOCSIFHWADDR, {ifr_name="enp2s0", ifr_hwaddr={sa_family=ARPHRD_ETHER, sa_data=96:78:d9:83:f8:21}}) = 0
//ioctl(3, SIOCSIFHWADDR, {ifr_name="enp2s0", ifr_hwaddr={sa_family=ARPHRD_ETHER, sa_data=65:6e:70:32:73:30}}) = -1