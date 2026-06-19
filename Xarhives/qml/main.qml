import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import Xarph

Window {
    id: root
    visible: true
    width: 900
    height: 600
    title: "Xarhives"
    color: "#0c0c12"

    FileBrowserBridge {
        id: browser
        Component.onCompleted: goHome()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8

        // Toolbar
        RowLayout {
            Layout.fillWidth: true
            spacing: 4

            Button {
                text: "↑"
                enabled: browser.canGoUp
                onClicked: browser.goUp()
            }

            Button {
                text: "~"
                onClicked: browser.goHome()
            }

            TextField {
                id: pathField
                Layout.fillWidth: true
                text: browser.currentPath
                color: "#e0e0f0"
                background: Rectangle {
                    color: "#1a1a2e"
                    border.color: "#2a2a3e"
                    border.width: 1
                    radius: 4
                }
                onAccepted: browser.navigate(text)
            }
        }

        // File list
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ListView {
                id: fileList
                anchors.fill: parent
                model: ListModel { id: fileModel }

                delegate: Rectangle {
                    width: fileList.width
                    height: 36
                    radius: 4
                    color: fileMouseArea.containsMouse ? "#2a2a4e" : "transparent"

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 8
                        anchors.rightMargin: 8
                        spacing: 8

                        Text {
                            text: model.fileType === "dir" ? "📁" : "📄"
                            font.pixelSize: 16
                        }

                        Text {
                            text: model.fileName
                            font.pixelSize: 13
                            color: "#e0e0f0"
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }

                        Text {
                            text: model.fileSize
                            font.pixelSize: 11
                            color: "#666680"
                        }
                    }

                    MouseArea {
                        id: fileMouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                        onDoubleClicked: {
                            if (model.fileType === "dir") {
                                browser.navigate(browser.currentPath + "/" + model.fileName)
                            } else {
                                browser.openFile(browser.currentPath + "/" + model.fileName)
                            }
                        }
                    }
                }
            }
        }

        // Status bar
        Text {
            Layout.fillWidth: true
            text: browser.currentPath
            font.pixelSize: 11
            color: "#666680"
        }
    }

    Connections {
        target: browser
        function onCurrentPathChanged() {
            fileModel.clear()
            var files = browser.getFiles()
            if (files === "") return
            var lines = files.split("\n")
            for (var i = 0; i < lines.length; i++) {
                var parts = lines[i].split("|")
                if (parts.length >= 3) {
                    fileModel.append({
                        fileName: parts[0],
                        fileType: parts[1],
                        fileSize: parts[2]
                    })
                }
            }
        }
    }
}
