//
//  ViewController+CoreMotion.swift
//  bevy_in_iOS
//
//  Created by 李金磊 on 2022/12/22.
//

import CoreMotion

extension ViewController {
    func startGyroUpdates() {
        if !motionManager.isGyroAvailable {
            print("gyro is not available")
            return
        }
        if motionManager.isGyroActive {
            return
        }
                
        motionManager.startGyroUpdates(to: OperationQueue.init()) { gyroData, error in
            guard let gyroData = gyroData else {
                print("startGyroUpdates error: \(error!)")
                return
            }
            self.rotationRate = gyroData.rotationRate
        }
    }
    
    func stopGyroUpdates() {
        motionManager.stopGyroUpdates()
    }
    
    func startAccelerometerUpdates() {
        if !motionManager.isAccelerometerAvailable {
            print("Accelerometer is not available")
            return
        }
        if motionManager.isAccelerometerActive {
            return
        }
        motionManager.startAccelerometerUpdates(to: OperationQueue.init()) { accelerometerData, error in
            guard let accelerometerData = accelerometerData else {
                print("startAccelerometerUpdates error: \(error!)")
                return
            }
            // 获取加速计数据
            print("\(accelerometerData)")
        }
    }
    
    func stopAccelerometerUpdates() {
        motionManager.stopAccelerometerUpdates()
    }
    
    func startDeviceMotionUpdates() {
        motionManager.startDeviceMotionUpdates(to: OperationQueue.init()) { deviceMotion, error in
            guard let deviceMotion = deviceMotion else {
                print("startDeviceMotionUpdates error: \(error!)")
                return
            }
            self.gravity = deviceMotion.gravity
        }
    }
    
    func stopDeviceMotionUpdates() {
        motionManager.stopDeviceMotionUpdates()
    }
}
