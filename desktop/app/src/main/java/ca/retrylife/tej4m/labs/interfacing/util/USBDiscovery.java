package ca.retrylife.tej4m.labs.interfacing.util;

import java.util.Enumeration;

import javax.usb.UsbHub;

import org.usb4java.Context;
import org.usb4java.DeviceDescriptor;
import org.usb4java.DeviceList;
import org.usb4java.LibUsb;

public class USBDiscovery {

    public static Object listConnectedUSBDevices(Context usbContext) {

        DeviceList usbDevList = new DeviceList();
        int status = LibUsb.getDeviceList(usbContext, usbDevList);

        // Handle lib error
        if (status < 0) {
            return null;
        }


        DeviceDescriptor
        // Build a list of devices
        // List<> devices = new ArrayList<>();

    }

}
