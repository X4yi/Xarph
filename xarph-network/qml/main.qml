import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import Xarph

Window {
    id: root
    visible: true
    width: 700
    height: 500
    title: "Xarph Network"
    color: "#0c0c12"

    NetworkBridge {
        id: networkBridge
        Component.onCompleted: loadInterfaces()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: "Network Monitor"
                font.pixelSize: 16
                font.bold: true
                color: "#e0e0f0"
            }

            Item { Layout.fillWidth: true }

            Rectangle {
                width: 10
                height: 10
                radius: 5
                color: networkBridge.isOnline ? "#4caf50" : "#f44336"
            }

            Text {
                text: networkBridge.isOnline ? "Online" : "Offline"
                font.pixelSize: 12
                color: networkBridge.isOnline ? "#4caf50" : "#f44336"
            }

            Button {
                text: "Refresh"
                onClicked: networkBridge.loadInterfaces()
            }
        }

        // Stats bar
        Rectangle {
            Layout.fillWidth: true
            height: 28
            color: "#1a1a2e"
            radius: 4

            Text {
                anchors.centerIn: parent
                text: networkBridge.getNetworkStats()
                font.pixelSize: 11
                color: "#c0c0d0"
            }
        }

        // Interface list
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ListView {
                id: ifaceList
                anchors.fill: parent
                model: ListModel { id: ifaceModel }

                delegate: Rectangle {
                    width: ifaceList.width
                    height: 48
                    radius: 4
                    color: ifaceMouseArea.containsMouse ? "#2a2a4e" : "transparent"

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 8
                        anchors.rightMargin: 8
                        spacing: 12

                        Rectangle {
                            width: 8
                            height: 8
                            radius: 4
                            color: model.isConnected ? "#4caf50" : "#f44336"
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 2

                            Text {
                                text: model.ifaceName
                                font.pixelSize: 13
                                color: "#e0e0f0"
                            }

                            Text {
                                text: model.ifaceType + " — " + model.ifaceIp
                                font.pixelSize: 10
                                color: "#666680"
                                Layout.fillWidth: true
                                elide: Text.ElideRight
                            }
                        }
                    }

                    MouseArea {
                        id: ifaceMouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                    }
                }
            }
        }

        Text {
            Layout.fillWidth: true
            text: networkBridge.interfaceCount + " interfaces"
            font.pixelSize: 11
            color: "#666680"
        }
    }

    Component.onCompleted: networkBridge.loadInterfaces()
}
