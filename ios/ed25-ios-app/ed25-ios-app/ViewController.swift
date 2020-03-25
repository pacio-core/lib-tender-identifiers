//
//  ViewController.swift
//  ed25-ios-app
//
//  Created by me on 16/03/2020.
//  Copyright © 2020 me. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
    }
    
    @IBAction func showMessage(sender: UIButton) {
        let strFromRust = strPtrRet()
        let alertController = UIAlertController(title: "Welcome to My First App", message: strFromRust, preferredStyle: UIAlertController.Style.alert)
        alertController.addAction(UIAlertAction(title: "OK", style: UIAlertAction.Style.default, handler: nil))
        present(alertController, animated: true, completion: nil)
    }
}

///////////////////

func strPtrRet() -> String? {
    let keypair = keypair_from()
    print(keypair.start, keypair.len)
//    let kp_data = Data(kp().asUnsafeBufferPointer())
    print(kepa.asUnsafeBufferPointer().first!)
//    let str = rustBytes.asHexStr()

//    if let stringFromRust = str {
//        print("got a string from Rust")
//        print(stringFromRust)
//    } else {
//        print("Could not parse Rust string as UTF-8")
//    }

    return "Hello world"
}

extension RustByteSlice {
    func asUnsafeBufferPointer() -> UnsafeBufferPointer<UInt8> {
        return UnsafeBufferPointer(start: start, count: len)
    }
    func asString(encoding: String.Encoding = String.Encoding.utf8) -> String? {
        return String(bytes: asUnsafeBufferPointer(), encoding: encoding)
    }
    func asHexStr() -> String? {
        Data(asUnsafeBufferPointer()).map{ String(format:"%02x", $0) }.joined()
    }
}

//////////////////////////


//func getHelloStr() -> String {
//    let result = hello("Rob")
//    let sr = String(cString: result!)
//    // IMPORTANT: once we get the result we have to release the pointer.
//    hello_release(UnsafeMutablePointer(mutating: result))
//    return sr
//}
//
// func swiftStr() {
//     let myString = "Hello from Swift"
// //    let data: Data = myString.data(using: String.Encoding.utf8, allowLossyConversion: false)!
// //    utf8_bytes_to_rust(UnsafePointer<UInt8>(data.bytes), data.length)
//     utf8_bytes_to_rust(myString.toUnsafePointer(), myString.count) // is mystring.count correct ?
// }

extension String {
    func toUnsafePointer() -> UnsafePointer<UInt8>? {
        guard let data = self.data(using: .utf8) else {
            return nil
        }

        let buffer = UnsafeMutablePointer<UInt8>.allocate(capacity: data.count)
        let stream = OutputStream(toBuffer: buffer, capacity: data.count)
        stream.open()
        let value = data.withUnsafeBytes {
            $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
        }
        guard let val = value else {
            return nil
        }
        stream.write(val, maxLength: data.count)
        stream.close()

        return UnsafePointer<UInt8>(buffer)
    }

    func toUnsafeMutablePointer() -> UnsafeMutablePointer<Int8>? {
        return strdup(self)
    }
}
