import QtQuick 2.15
import QtDBus 1.15

Item {
    id: root

    property var daemonInterface: DBusInterface {
        service: "org.x4yi.X4Shell"
        path: "/org/x4yi/X4Shell/v1"
        iface: "org.x4yi.X4Shell.v1"
    }

    property bool wifiConnected: false
    property string wifiName: ""

    property bool bluetoothEnabled: false
    property int bluetoothDevices: 0

    property real batteryPercent: 0
    property bool batteryCharging: false

    Component.onCompleted: {
        console.log("SystemService initialized")
    }

    function onPowerChanged(powerState) {
        root.batteryPercent = powerState.percent || 0
        root.batteryCharging = powerState.charging || false
    }

    function onBluetoothChanged(state) {
        root.bluetoothEnabled = state.enabled || false
        root.bluetoothDevices = state.deviceCount || 0
    }

    function onWiFiChanged(state) {
        root.wifiConnected = state.connected || false
        root.wifiName = state.name || ""
    }
}
