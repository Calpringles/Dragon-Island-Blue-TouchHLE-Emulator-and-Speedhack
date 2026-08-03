/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSURLConnection`.

use super::{ns_string, NSInteger};
use crate::environment::Environment;
use crate::mem::MutPtr;
use crate::objc::{autorelease, id, msg, msg_class, nil, objc_classes, release, ClassExports};
use std::borrow::Cow;

const NSURLErrorDomain: &str = "NSURLErrorDomain";

/// Our helper type, Foundation just uses ints.
type NSURLErrorCode = NSInteger;
const NSURLErrorNotConnectedToInternet: NSURLErrorCode = -1009;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSURLConnection: NSObject

+ (id)sendSynchronousRequest:(id)request // NSURLRequest *
           returningResponse:(MutPtr<id>)response // NSURLResponse **
                       error:(MutPtr<id>)out_error { // NSError **
    let url_string = url_string_from_request(env, request);
    log!(
        "TODO: [NSURLConnection sendSynchronousRequest:{:?} ('{}') response:{:?} error:{:?}] -> nil",
        request,
        url_string,
        response,
        out_error,
    );
    if url_string.contains("dideals") {
        let plist = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<array>\n<dict>\n<key>title</key><string>Golden Gem</string>\n<key>itemTitle</key><string>Golden Gem</string>\n<key>name</key><string>Golden Gem</string>\n<key>cost</key><integer>100</integer>\n<key>currency</key><string>Gems</string>\n<key>art</key><string>gemIcon.png</string>\n<key>dealArt</key><string>gemIcon.png</string>\n<key>id</key><string>Item_Gem</string>\n<key>type</key><string>Item</string>\n</dict>\n</array>\n</plist>";
        let size = plist.len().try_into().unwrap();
        let alloc = env.mem.alloc(size);
        let slice = env.mem.bytes_at_mut(alloc.cast(), size);
        slice.copy_from_slice(plist);
        let alloc_ptr: crate::mem::MutVoidPtr = alloc.cast();
        
        let ns_data_class = env.objc.get_known_class("NSData", &mut env.mem);
        let data: id = msg![env; ns_data_class alloc];
        let data: id = msg![env; data initWithBytesNoCopy:alloc_ptr length:size];
        autorelease(env, data);
        
        if !response.is_null() {
            // Some games crash if response is nil when data is not nil
            // Let's create an NSURLResponse if possible, or leave it nil
            env.mem.write(response, nil);
        }
        if !out_error.is_null() {
            env.mem.write(out_error, nil);
        }
        
        return data;
    }

    if !response.is_null() {
        env.mem.write(response, nil);
    }
    if !out_error.is_null() {
        let domain = ns_string::get_static_str(env, NSURLErrorDomain);
        let error = msg_class![env; NSError alloc];
        // TODO: fill userInfo
        let error = msg![env; error initWithDomain:domain code:NSURLErrorNotConnectedToInternet userInfo:nil];
        autorelease(env, error);
        env.mem.write(out_error, error);
    }
    nil
}

+ (id)connectionWithRequest:(id)request // NSURLRequest *
                   delegate:(id)delegate {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithRequest:request delegate:delegate];
    autorelease(env, new)
}

- (id)initWithRequest:(id)request // NSURLRequest *
             delegate:(id)delegate {
    msg![env; this initWithRequest:request delegate:delegate startImmediately:true]
}

- (id)initWithRequest:(id)request // NSURLRequest *
             delegate:(id)delegate
     startImmediately:(bool)start_immediately {
    let url_string = url_string_from_request(env, request);
    log!(
        "TODO: [(NSURLConnection *){:?} initWithRequest:{:?} ('{}') delegate:{:?} startImmediately:{}] -> nil",
        this,
        request,
        url_string,
        delegate,
        start_immediately,
    );
    
        let plist: &[u8] = if url_string.contains("dideals.php") {
            b"{}\0" as &[u8]
        } else {
            // For u_news and any other request, return a generic valid plist
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict></dict>\n</plist>\0" as &[u8]
        };
        let size = (plist.len() - 1) as u32; // minus 1 for null terminator
        let alloc = env.mem.alloc(size);
        let slice = env.mem.bytes_at_mut(alloc.cast(), size);
        slice.copy_from_slice(&plist[..size as usize]);
        
        let ns_data_class = env.objc.get_known_class("NSData", &mut env.mem);
        log!("initWithRequest: about to alloc data");
        let data: id = msg![env; ns_data_class alloc];
        log!("initWithRequest: about to initWithBytesNoCopy");
        let alloc_ptr: crate::mem::MutVoidPtr = alloc.cast();
        let data: id = msg![env; data initWithBytesNoCopy:alloc_ptr length:size];
        autorelease(env, data);

        if delegate != nil {
            let ns_array_class = env.objc.get_known_class("NSMutableArray", &mut env.mem);
            log!("initWithRequest: about to alloc user_info");
            let user_info: id = msg_class![env; NSMutableArray alloc];
            log!("initWithRequest: about to init user_info");
            let user_info: id = msg![env; user_info init];
            log!("initWithRequest: about to addObject:delegate");
            () = msg![env; user_info addObject:delegate];
            log!("initWithRequest: about to addObject:this");
            () = msg![env; user_info addObject:this];
            log!("initWithRequest: about to addObject:data");
            () = msg![env; user_info addObject:data];
            autorelease(env, user_info);
            
            let sel_fire = env.objc.register_host_selector("_fireCallbacks:".to_string(), &mut env.mem);
            let ns_timer_class = env.objc.get_known_class("NSTimer", &mut env.mem);
            
            log!("initWithRequest: about to schedule timer");
            let _timer: id = msg![env; ns_timer_class scheduledTimerWithTimeInterval:0.1 target:this selector:sel_fire userInfo:user_info repeats:false];
            // DO NOT autorelease timer here. scheduledTimerWithTimeInterval already returns an autoreleased object!
        }
        
        log!("initWithRequest: done");
        return this;
    // We mock everything to avoid internet check failures
}

- (())_fireCallbacks:(id)timer {
    log!("_fireCallbacks: getting userInfo");
    let user_info: id = msg![env; timer userInfo]; // an NSArray
    
    log!("_fireCallbacks: getting delegate");
    let delegate: id = msg![env; user_info objectAtIndex:0u32];
    log!("_fireCallbacks: getting connection");
    let connection: id = msg![env; user_info objectAtIndex:1u32];
    log!("_fireCallbacks: getting data");
    let data: id = msg![env; user_info objectAtIndex:2u32];
    
    log!("_fireCallbacks: register didReceiveResponse");
    let sel_recv_resp = env.objc.register_host_selector("connection:didReceiveResponse:".to_string(), &mut env.mem);
    log!("_fireCallbacks: respondsToSelector didReceiveResponse");
    if msg![env; delegate respondsToSelector:sel_recv_resp] {
        log!("_fireCallbacks: sending didReceiveResponse");
        () = msg![env; delegate connection:connection didReceiveResponse:nil];
    }
    
    log!("_fireCallbacks: register didReceiveData");
    let sel_recv_data = env.objc.register_host_selector("connection:didReceiveData:".to_string(), &mut env.mem);
    log!("_fireCallbacks: respondsToSelector didReceiveData");
    if msg![env; delegate respondsToSelector:sel_recv_data] {
        log!("_fireCallbacks: sending didReceiveData");
        () = msg![env; delegate connection:connection didReceiveData:data];
    }
    
    log!("_fireCallbacks: register connectionDidFinishLoading");
    let sel_did_finish = env.objc.register_host_selector("connectionDidFinishLoading:".to_string(), &mut env.mem);
    log!("_fireCallbacks: respondsToSelector connectionDidFinishLoading");
    if msg![env; delegate respondsToSelector:sel_did_finish] {
        log!("_fireCallbacks: sending connectionDidFinishLoading");
        () = msg![env; delegate connectionDidFinishLoading:connection];
    }
    log!("_fireCallbacks: finished");
}

@end

};

fn url_string_from_request(env: &mut Environment, request: id) -> Cow<'static, str> {
    if request == nil {
        Cow::from("(null)")
    } else {
        let url = msg![env; request URL];
        let ns_string = msg![env; url absoluteString];
        ns_string::to_rust_string(env, ns_string)
    }
}
