import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Rectangle {
    id: startMenu
    width: 400
    height: 500
    color: "#0c0c12"
    border.color: "#2a2a3e"
    border.width: 1
    radius: 8

    property string searchQuery: ""

    StartMenuBridge {
        id: startMenuBridge
        Component.onCompleted: loadApps()
    }

    // Poll for Super key toggle from compositor (every 100ms)
    Timer {
        interval: 100
        running: true
        repeat: true
        onTriggered: {
            if (startMenuBridge.checkToggle()) {
                startMenu.visible = !startMenu.visible
                if (startMenu.visible) {
                    startMenu.loadApps()
                }
            }
        }
    }

    // Close on Escape
    Keys.onEscapePressed: startMenu.visible = false

    onVisibleChanged: {
        if (visible) {
            startMenu.forceActiveFocus()
            loadApps()
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 12
        spacing: 12

        TextField {
            id: searchField
            Layout.fillWidth: true
            placeholderText: "Buscar aplicaciones..."
            color: "#e0e0f0"
            placeholderTextColor: "#666680"
            background: Rectangle {
                color: "#1a1a2e"
                border.color: searchField.activeFocus ? "#8c6eff" : "#2a2a3e"
                border.width: 1
                radius: 6
            }
            onTextChanged: {
                startMenu.searchQuery = text
                filterApps(text)
            }
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ListView {
                id: appsList
                anchors.fill: parent
                model: ListModel {
                    id: appsModel
                }

                delegate: Rectangle {
                    width: appsList.width
                    height: 48
                    radius: 6
                    color: delegateMouseArea.containsMouse ? "#2a2a4e" : "transparent"

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 8
                        anchors.rightMargin: 8
                        spacing: 8

                        Rectangle {
                            width: 36
                            height: 36
                            radius: 6
                            color: "#1a1a2e"

                            Text {
                                anchors.centerIn: parent
                                text: "●"
                                font.pixelSize: 18
                                color: "#8c6eff"
                            }
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 2

                            Text {
                                text: model.appName
                                font.pixelSize: 13
                                color: "#e0e0f0"
                                elide: Text.ElideRight
                                Layout.fillWidth: true
                            }

                            Text {
                                text: model.categories
                                font.pixelSize: 10
                                color: "#666680"
                                elide: Text.ElideRight
                                Layout.fillWidth: true
                                visible: text !== ""
                            }
                        }
                    }

                    MouseArea {
                        id: delegateMouseArea
                        anchors.fill: parent
                        hoverEnabled: true
                        onClicked: {
                            launchApp(model.desktopFile)
                        }
                    }
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Settings"
                Layout.fillWidth: true
                onClicked: {
                    startMenu.visible = false
                    startMenuBridge.launchBinary("xarph-settings")
                }
            }

            Button {
                text: "Lock"
                onClicked: {
                    startMenu.visible = false
                    Qt.openUrlExternally("xarph-lock")
                }
            }

            Button {
                text: "Power"
                onClicked: {
                    startMenu.visible = false
                }
            }
        }
    }

    function loadApps() {
        startMenuBridge.loadApps()
        appsModel.clear()

        var appsData = startMenuBridge.getAllApps()
        if (appsData === "") return

        var lines = appsData.split("\n")
        for (var i = 0; i < lines.length; i++) {
            var parts = lines[i].split("|")
            if (parts.length >= 2) {
                appsModel.append({
                    appId: parts[0],
                    appName: parts[1],
                    exec: parts.length > 2 ? parts[2] : "",
                    categories: parts.length > 3 ? parts[3] : "",
                    appIcon: "",
                    desktopFile: parts.length > 2 ? parts[2] : ""
                })
            }
        }
    }

    function filterApps(query) {
        if (query === "") {
            loadApps()
            return
        }

        appsModel.clear()
        var appsData = startMenuBridge.search(query)
        if (appsData === "") return

        var lines = appsData.split("\n")
        for (var i = 0; i < lines.length; i++) {
            var parts = lines[i].split("|")
            if (parts.length >= 2) {
                appsModel.append({
                    appId: parts[0],
                    appName: parts[1],
                    exec: parts.length > 2 ? parts[2] : "",
                    categories: parts.length > 3 ? parts[3] : "",
                    appIcon: "",
                    desktopFile: parts.length > 2 ? parts[2] : ""
                })
            }
        }
    }

    function launchApp(desktopFile) {
        if (desktopFile) {
            startMenuBridge.launchApp(desktopFile)
        }
        startMenu.visible = false
    }
}
