import QtQuick 2.15
import QtQuick.Layouts 1.15
import qs.services 1.0 as Services

RowLayout {
    id: root

    spacing: 12
    Layout.alignment: Qt.AlignHCenter

    property var systemService: Services.SystemService

    Components.SystemIcon {
        icon: "network-wireless"
        active: root.systemService ? root.systemService.wifiConnected : false
        tooltip: root.systemService ? root.systemService.wifiName : "WiFi disconnected"
    }

    Components.SystemIcon {
        icon: "bluetooth"
        active: root.systemService ? root.systemService.bluetoothEnabled : false
        tooltip: root.systemService ? root.systemService.bluetoothDevices + " devices" : "Bluetooth off"
    }

    Components.SystemIcon {
        icon: root.systemService && root.systemService.batteryCharging ? "battery-charging" : "battery"
        active: true
        text: root.systemService ? Math.round(root.systemService.batteryPercent) + "%" : ""
        tooltip: root.systemService ? "Battery: " + Math.round(root.systemService.batteryPercent) + "%" : "N/A"
    }
}
