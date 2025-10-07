use std::ptr;
use std::ffi::CString;

mod handle_connection; 

use libc::accept;

const BACKLOG: i32 = 10;

fn main() {
    let sockfd = match setup_listener() {
        Ok(fd) => fd,
        Err(err_message) => panic!("{}", err_message)
    };

    loop {
        let mut client_addr: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let mut addrlen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;

        let new_fd;
        unsafe {
            new_fd = accept(
                sockfd,
                &mut client_addr as *mut _ as *mut libc::sockaddr,
                &mut addrlen
            ); 
        }

        //TODO: don't break on recoverable errors
        if new_fd == -1 { break; } 
        todo!("spawn worker thread");
    }
    todo!("graceful shutdown");
}

fn setup_listener() -> Result<i32, String> {
    let port = CString::new("8000").expect("CString::new failed for port");
    
    unsafe {
        let mut hints: libc::addrinfo = std::mem::zeroed();
        // dgaf about ipv4 or ipv6
        hints.ai_family = libc::AF_UNSPEC;
        // TCP stream
        hints.ai_socktype = libc::SOCK_STREAM;
        // fill in my ip for me
        hints.ai_flags = libc::AI_PASSIVE;

        let mut addrinfo: *mut libc::addrinfo = ptr::null_mut();

        let getaddr_status = libc::getaddrinfo(ptr::null(), port.as_ptr(), &hints, &mut addrinfo);
        if getaddr_status != 0 { return Err("getaddrinfo syscall failed".to_string());}

        let mut p = addrinfo;
        let mut sockfd= -1;

        //TODO: add sockopt with reusable address 
        while !p.is_null() {
            sockfd = libc::socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
            if sockfd == -1 {
                p = (*p).ai_next;
                continue;
            }
            // if bind succeeds we break
            if libc::bind(sockfd, (*p).ai_addr, (*p).ai_addrlen) == 0 { 
                break;
            }

            // if bind fails, close
            libc::close(sockfd);
            p = (*p).ai_next;
        }

        libc::freeaddrinfo(addrinfo); 

        if p.is_null() {
            return Err("Failed to bind to any available address".to_string());
        }

        if libc::listen(sockfd, BACKLOG) == -1 { 
            libc::close(sockfd);
            return Err("listen syscall failed".to_string()); 
        }     
        //finally return the port that we're now successfully listening on
        Ok(sockfd)   
    }
}