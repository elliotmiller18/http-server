use crate::utils;

const MAX_REQUEST_SIZE: usize = 4096;
const CARRIAGE_RETURN: u8 = 0x0D;
const LINE_FEED: u8 = 0x0A;
//TODO: increase this as we add support for more methods
const MAX_METHOD_NAME_SIZE: usize = utils::max(&[GET_METHOD_NAME_SIZE], 1);
const GET_METHOD_NAME_SIZE: usize = 3;
const INTERNAL_SERVER_ERROR: &'static str = "Internal Server Error";


enum HttpMethod {
    GET
    //TODO: add more
}

impl HttpMethod {
    pub fn safe_new(input: &str) -> Result<Self, ()> {
        return match input {
            | "GET" => Ok(HttpMethod::GET),
            | _ => Err(())
        }
    }
}

struct ServerResponse(pub u32, pub String);

//TODO: have this return a result, specifically ip for tracking people viewing my site
fn accept(accepting_sockfd: i32) -> Result<(), u32> {
    // just keep recv'ing until the connection is done
    loop {
        // we probably need to do this:
        let mut response: Vec<u8> = Vec::with_capacity(MAX_REQUEST_SIZE);
        
        // recv
        //TODO: distinguish between 413 and 414 for URI too long and Payload too long
        unsafe  {
            let void_buf = response.as_mut_ptr() as *mut std::os::raw::c_void;
            let bufferlen = libc::recv(accepting_sockfd, void_buf, MAX_REQUEST_SIZE, 0);

            //TODO: have this respond with various HTTP status codes based on the error 
            if bufferlen < 0 {
                return Err(500);
            }
            // connection closed
            else if bufferlen == 0 { return Ok(()) }

            response.set_len(bufferlen as usize);
        }
        
        // get request header (if any), we're just doing GET for now
        let mut method_buf: String = String::new();
        for byte in &response {
            if *byte < b'A' || *byte > b'Z' { return Err(400); }
            method_buf.push(*byte as char);
            if method_buf.len() > MAX_METHOD_NAME_SIZE { break; }
        }

        let method: HttpMethod = match HttpMethod::safe_new(method_buf.as_str()) {
            Ok(res) => res,
            Err(_) => { return Err(400); }
        };
        // do whatever's necessary for request
        // respond
        // repeat 
    }
}

fn get(request: &Vec<u8>) -> ServerResponse {
    // check if the character after the GET method is a space for sanity
    if request[GET_METHOD_NAME_SIZE] != b' ' { return ServerResponse(500, INTERNAL_SERVER_ERROR.to_string())} 

    for byte in &request[GET_METHOD_NAME_SIZE + 1..] {

    }

    todo!("finish");
}

