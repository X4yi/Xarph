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
    title: "Xarph Admin"
    color: "#0c0c12"

    ProcessBridge {
        id: processBridge
        Component.onCompleted: loadProcesses()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8

        // Header
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: "Process Manager"
                font.pixelSize: 16
                font.bold: true
                color: "#e0e0f0"
            }

            Item { Layout.fillWidth: true }

            Text {
                id: statsText
                text: ""
                font.pixelSize: 11
                color: "#8c6eff"
            }

            Button {
                text: "Refresh"
                onClicked: {
                    processBridge.loadProcesses()
                    statsText.text = processBridge.getSystemStats()
                }
            }
        }

        // System stats
        Rectangle {
            Layout.fillWidth: true
            height: 28
            color: "#1a1a2e"
            radius: 4

            Text {
                anchors.centerIn: parent
                text: statsText.text
                font.pixelSize: 11
                color: "#c0c0d0"
            }
        }

        // Process list
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ListView {
                id: processList
                anchors.fill: parent
                model: ListModel { id: processModel }

                delegate: Rectangle {
                    width: processList.width
                    height: 32
                    radius: 4
                    color: procMouseArea.containsMouse ? "#2a2a4e" : "transparent"

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 8
                        anchors.rightMargin: 8
                        spacing: 12

                        Text {
                            text: model.pid
                            font.pixelSize: 11
                            color: "#666680"
                            width: 60
                        }

                        Text {
                            text: model.pname
                            font.pixelSize: 12
                            color: "#e0e0f0"
                            Layout.fillWidth: true
                            elide: Text.ElideRight
                        }

                        Text {
                            text: model.pstatus
                            font.pixelSize: 11
                            color: model.pstatus === "running" ? "#4caf50" : "#f44336"
                            width: 60
                        }

                        Text {
                            text: model.cpu + "%"
                            font.pixelSize: 11
                            color: "#c0c0d0"
                            width: 50
                        }

                        Text {
                            text: model.mem + " KB"
                            font.pixelSize: 11
                            color: "#c0c0d0"
                            width: 80
                        }

                        Button {
                            text: "Kill"
                            font.pixelSize: 10
                            implicitWidth: 40
                            implicitHeight: 22
                            onClicked: processBridge.killProcess(model.pid)
                        }
                    }

                    MouseArea {
                        id: procMouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                    }
                }
            }
        }

        // Status
        Text {
            Layout.fillWidth: true
            text: processBridge.processCount + " processes"
            font.pixelSize: 11
            color: "#666680"
        }
    }

    Connections {
        target: processBridge
        function onProcessCountChanged() {
            processModel.clear()
            // Reload is triggered by loadProcesses() which returns data
        }
    }

    Component.onCompleted: {
        processBridge.loadProcesses()
        statsText.text = processBridge.getSystemStats()
    }
}
