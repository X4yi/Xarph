import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import Xarph

Window {
    id: root
    visible: true
    width: 800
    height: 600
    title: "Xarph Services"
    color: "#0c0c12"

    ServiceBridge {
        id: serviceBridge
        Component.onCompleted: loadServices()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: "Service Manager"
                font.pixelSize: 16
                font.bold: true
                color: "#e0e0f0"
            }

            Item { Layout.fillWidth: true }

            Button {
                text: "Refresh"
                onClicked: serviceBridge.loadServices()
            }
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ListView {
                id: serviceList
                anchors.fill: parent
                model: ListModel { id: serviceModel }

                delegate: Rectangle {
                    width: serviceList.width
                    height: 40
                    radius: 4
                    color: svcMouseArea.containsMouse ? "#2a2a4e" : "transparent"

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 8
                        anchors.rightMargin: 8
                        spacing: 12

                        Rectangle {
                            width: 8
                            height: 8
                            radius: 4
                            color: model.svcActive ? "#4caf50" : "#f44336"
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 2

                            Text {
                                text: model.svcName
                                font.pixelSize: 13
                                color: "#e0e0f0"
                                elide: Text.ElideRight
                            }

                            Text {
                                text: model.svcDescription
                                font.pixelSize: 10
                                color: "#666680"
                                elide: Text.ElideRight
                                Layout.fillWidth: true
                            }
                        }

                        Text {
                            text: model.svcStatus
                            font.pixelSize: 11
                            color: model.svcActive ? "#4caf50" : "#f44336"
                            width: 60
                        }

                        Button {
                            text: model.svcActive ? "Stop" : "Start"
                            font.pixelSize: 10
                            implicitWidth: 50
                            implicitHeight: 22
                            onClicked: {
                                if (model.svcActive) {
                                    serviceBridge.stopService(model.svcName)
                                } else {
                                    serviceBridge.startService(model.svcName)
                                }
                            }
                        }

                        Button {
                            text: "Restart"
                            font.pixelSize: 10
                            implicitWidth: 55
                            implicitHeight: 22
                            onClicked: serviceBridge.restartService(model.svcName)
                        }
                    }

                    MouseArea {
                        id: svcMouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                    }
                }
            }
        }

        Text {
            Layout.fillWidth: true
            text: serviceBridge.serviceCount + " services"
            font.pixelSize: 11
            color: "#666680"
        }
    }

    Component.onCompleted: serviceBridge.loadServices()
}
