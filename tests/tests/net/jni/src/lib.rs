/*
 * Copyright (C) 2026 The AOSP and FrizkOS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr;

pub type jboolean = u8;
pub const JNI_TRUE: jboolean = 1;
pub const JNI_FALSE: jboolean = 0;

pub type jclass = *mut c_void;
pub type JNIEnv = *mut c_void;

#[repr(C)]
struct addrinfo {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: u32,
    ai_addr: *mut sockaddr,
    ai_canonname: *mut c_char,
    ai_next: *mut addrinfo,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct hostent {
    h_name: *mut c_char,
    h_aliases: *mut *mut c_char,
    h_addrtype: c_int,
    h_length: c_int,
    h_addr_list: *mut *mut c_char,
}

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const NI_MAXHOST: usize = 1025;
const NI_NAMEREQD: c_int = 8;

unsafe extern "C" {
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    fn freeaddrinfo(res: *mut addrinfo);
    fn getnameinfo(
        sa: *const sockaddr,
        salen: u32,
        host: *mut c_char,
        hostlen: u32,
        serv: *mut c_char,
        servlen: u32,
        flags: c_int,
    ) -> c_int;
    fn inet_pton(af: c_int, cp: *const c_char, buf: *mut c_void) -> c_int;
    fn inet_ntop(af: c_int, cp: *const c_void, buf: *mut c_char, len: u32) -> *const c_char;
    fn gethostbyname(name: *const c_char) -> *mut hostent;
    fn gethostbyaddr(addr: *const c_void, len: u32, type_: c_int) -> *mut hostent;
}

const GOOGLE_DNS_IPV4: &str = "8.8.8.8";
const GOOGLE_DNS_IPV4_2: &str = "8.8.4.4";
const GOOGLE_DNS_IPV6: &str = "2001:4860:4860::8888";
const GOOGLE_DNS_IPV6_2: &str = "2001:4860:4860::8844";

pub fn run_native_dns_checks() -> bool {
    // 1. getaddrinfo for www.google.com
    let node1 = CString::new("www.google.com").unwrap();
    let mut answer: *mut addrinfo = ptr::null_mut();

    let res = unsafe { getaddrinfo(node1.as_ptr(), ptr::null(), ptr::null(), &mut answer) };
    if res != 0 {
        return false;
    }

    let mut found_v4 = false;
    let mut found_v6 = false;
    let mut curr = answer;
    while !curr.is_null() {
        let ai = unsafe { &*curr };
        if !ai.ai_addr.is_null() {
            let sa_fam = unsafe { (*ai.ai_addr).sa_family as c_int };
            if sa_fam == AF_INET {
                found_v4 = true;
            } else if sa_fam == AF_INET6 {
                found_v6 = true;
            }
        }
        curr = ai.ai_next;
    }
    unsafe { freeaddrinfo(answer) };

    if !found_v4 && !found_v6 {
        return false;
    }

    // 2. getaddrinfo for ipv6.google.com (if resolvable)
    let node_v6 = CString::new("ipv6.google.com").unwrap();
    let mut answer_v6: *mut addrinfo = ptr::null_mut();
    let res_v6 = unsafe { getaddrinfo(node_v6.as_ptr(), ptr::null(), ptr::null(), &mut answer_v6) };
    if res_v6 == 0 {
        let mut curr_v6 = answer_v6;
        let mut only_v6 = true;
        let mut has_v6 = false;
        while !curr_v6.is_null() {
            let ai = unsafe { &*curr_v6 };
            if !ai.ai_addr.is_null() {
                let fam = unsafe { (*ai.ai_addr).sa_family as c_int };
                if fam == AF_INET6 {
                    has_v6 = true;
                } else if fam == AF_INET {
                    only_v6 = false;
                }
            }
            curr_v6 = ai.ai_next;
        }
        unsafe { freeaddrinfo(answer_v6) };
        if !has_v6 || !only_v6 {
            return false;
        }
    }

    // 3. getnameinfo on Google DNS IPv4
    let mut sa4: sockaddr_in = unsafe { std::mem::zeroed() };
    sa4.sin_family = AF_INET as u16;
    let ip4_c = CString::new(GOOGLE_DNS_IPV4).unwrap();
    unsafe {
        inet_pton(
            AF_INET,
            ip4_c.as_ptr(),
            &mut sa4.sin_addr as *mut in_addr as *mut c_void,
        );
    }

    let mut buf = [0 as c_char; NI_MAXHOST];
    let res = unsafe {
        getnameinfo(
            &sa4 as *const sockaddr_in as *const sockaddr,
            std::mem::size_of::<sockaddr_in>() as u32,
            buf.as_mut_ptr(),
            buf.len() as u32,
            ptr::null_mut(),
            0,
            NI_NAMEREQD,
        )
    };
    if res != 0 {
        return false;
    }
    let host_str = unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy() };
    if !host_str.contains("google") {
        return false;
    }

    // 4. getnameinfo on Google DNS IPv6
    let mut sa6: sockaddr_in6 = unsafe { std::mem::zeroed() };
    sa6.sin6_family = AF_INET6 as u16;
    let ip6_c = CString::new(GOOGLE_DNS_IPV6_2).unwrap();
    unsafe {
        inet_pton(
            AF_INET6,
            ip6_c.as_ptr(),
            &mut sa6.sin6_addr as *mut in6_addr as *mut c_void,
        );
    }
    let mut buf6 = [0 as c_char; NI_MAXHOST];
    let res = unsafe {
        getnameinfo(
            &sa6 as *const sockaddr_in6 as *const sockaddr,
            std::mem::size_of::<sockaddr_in6>() as u32,
            buf6.as_mut_ptr(),
            buf6.len() as u32,
            ptr::null_mut(),
            0,
            NI_NAMEREQD,
        )
    };
    if res == 0 {
        let host6_str = unsafe { CStr::from_ptr(buf6.as_ptr()).to_string_lossy() };
        if !host6_str.contains("google") {
            return false;
        }
    }

    // 5. gethostbyname
    let yt = CString::new("www.youtube.com").unwrap();
    let he = unsafe { gethostbyname(yt.as_ptr()) };
    if he.is_null() {
        return false;
    }
    let addr_list = unsafe { (*he).h_addr_list };
    if addr_list.is_null() || unsafe { (*addr_list).is_null() } {
        return false;
    }

    // 6. gethostbyaddr
    let mut addr6 = [0u8; 16];
    let _ip6_addr = CString::new(GOOGLE_DNS_IPV4).unwrap(); // Or IPv6 if available
    let res = unsafe {
        inet_pton(
            AF_INET6,
            CString::new(GOOGLE_DNS_IPV6).unwrap().as_ptr(),
            addr6.as_mut_ptr() as *mut c_void,
        )
    };
    if res > 0 {
        let he_addr = unsafe {
            gethostbyaddr(
                addr6.as_ptr() as *const c_void,
                addr6.len() as u32,
                AF_INET6,
            )
        };
        if !he_addr.is_null() {
            let name_ptr = unsafe { (*he_addr).h_name };
            if !name_ptr.is_null() {
                let name = unsafe { CStr::from_ptr(name_ptr).to_string_lossy() };
                if !name.contains("google") {
                    return false;
                }
            }
        }
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_net_cts_DnsTest_testNativeDns(
    _env: JNIEnv,
    _class: jclass,
) -> jboolean {
    if run_native_dns_checks() {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_resolution() {
        assert!(run_native_dns_checks());
    }
}
