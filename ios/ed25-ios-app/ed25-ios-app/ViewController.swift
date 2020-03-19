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
        let helloStr = getHelloStr()
        let alertController = UIAlertController(title: "Welcome to My First App", message: helloStr, preferredStyle: UIAlertController.Style.alert)
        alertController.addAction(UIAlertAction(title: "OK", style: UIAlertAction.Style.default, handler: nil))
        present(alertController, animated: true, completion: nil)
    }
}

func getHelloStr() -> String {
    let result = hello("Rob")
    let sr = String(cString: result!)
    // IMPORTANT: once we get the result we have to release the pointer.
    hello_release(UnsafeMutablePointer(mutating: result))
    return sr
}

