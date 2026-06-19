import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import Xarph

ApplicationWindow {
    id: settingsWindow
    title: "Xarph Settings"
    width: 800
    height: 600
    visible: true
    color: "#0c0c12"

    SettingsBridge {
        id: settingsBridge
        Component.onCompleted: loadSettings()
    }

    FileDialog {
        id: wallpaperFileDialog
        title: "Choose Wallpaper"
        nameFilters: ["Images (*.png *.jpg *.jpeg *.webp *.avif *.gif)", "Videos (*.mp4 *.webm *.mkv *.mov)"]
        onAccepted: {
            var path = selectedFile.toString()
            // Remove file:// prefix
            if (path.startsWith("file://")) {
                path = path.substring(7)
            }
            settingsBridge.chooseWallpaper(path)
            settingsBridge.saveSettings()
        }
    }

    ColorDialog {
        id: colorDialog
        title: "Choose Background Color"
        onAccepted: {
            settingsBridge.chooseColor(color.toString())
            settingsBridge.saveSettings()
        }
    }

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // Sidebar
        Rectangle {
            width: 200
            Layout.fillHeight: true
            color: "#0c0c12"
            border.color: "#2a2a3e"
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 12
                spacing: 4

                Text {
                    text: "Settings"
                    font.pixelSize: 18
                    font.bold: true
                    color: "#e0e0f0"
                    Layout.bottomMargin: 12
                }

                Repeater {
                    model: ["Wallpaper", "Theme", "Panel", "About"]

                    Rectangle {
                        Layout.fillWidth: true
                        height: 36
                        radius: 6
                        color: listView.currentIndex === index ? "#2a2a4e" : "transparent"

                        Text {
                            anchors.left: parent.left
                            anchors.leftMargin: 12
                            anchors.verticalCenter: parent.verticalCenter
                            text: modelData
                            font.pixelSize: 14
                            color: listView.currentIndex === index ? "#e0e0f0" : "#a0a0b0"
                        }

                        MouseArea {
                            anchors.fill: parent
                            onClicked: listView.currentIndex = index
                        }
                    }
                }

                Item { Layout.fillHeight: true }
            }
        }

        // Content
        ListView {
            id: listView
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: 4
            currentIndex: 0
            clip: true

            delegate: Item {
                width: listView.width
                height: listView.height

                Loader {
                    anchors.fill: parent
                    anchors.margins: 24
                    sourceComponent: {
                        switch (listView.currentIndex) {
                            case 0: return wallpaperPage
                            case 1: return themePage
                            case 2: return panelPage
                            case 3: return aboutPage
                            default: return wallpaperPage
                        }
                    }
                }
            }
        }
    }

    Component {
        id: wallpaperPage
        ColumnLayout {
            spacing: 16

            Text {
                text: "Wallpaper"
                font.pixelSize: 24
                font.bold: true
                color: "#e0e0f0"
            }

            // Current wallpaper preview
            Rectangle {
                Layout.fillWidth: true
                height: 200
                radius: 8
                color: "#1a1a2e"
                border.color: "#2a2a3e"
                border.width: 1

                Image {
                    anchors.fill: parent
                    anchors.margins: 8
                    source: settingsBridge.wallpaper_path ? "file://" + settingsBridge.wallpaper_path : ""
                    fillMode: Image.PreserveAspectCrop
                    visible: settingsBridge.wallpaper_path !== "" && settingsBridge.wallpaper_color === ""
                }

                Rectangle {
                    anchors.fill: parent
                    anchors.margins: 8
                    color: settingsBridge.wallpaper_color
                    visible: settingsBridge.wallpaper_color !== ""
                }
            }

            // Wallpaper controls
            RowLayout {
                spacing: 12

                Button {
                    text: "Choose Image"
                    onClicked: wallpaperFileDialog.open()
                }

                Button {
                    text: "Choose Color"
                    onClicked: colorDialog.open()
                }

                ComboBox {
                    id: modeCombo
                    model: ["Fill", "Fit", "Stretch", "Tile", "Center"]
                    currentIndex: {
                        switch (settingsBridge.wallpaper_mode) {
                            case "cover": return 0
                            case "contain": return 1
                            case "stretch": return 2
                            case "repeat": return 3
                            case "center": return 4
                            default: return 0
                        }
                    }
                    onCurrentIndexChanged: {
                        var modes = ["cover", "contain", "stretch", "repeat", "center"]
                        settingsBridge.changeWallpaperMode(modes[currentIndex])
                        settingsBridge.saveSettings()
                    }
                }
            }
        }
    }

    Component {
        id: themePage
        ColumnLayout {
            spacing: 16

            Text {
                text: "Theme"
                font.pixelSize: 24
                font.bold: true
                color: "#e0e0f0"
            }

            ComboBox {
                id: themeCombo
                model: settingsBridge.getThemes().split(",")
                currentIndex: 0
                onCurrentIndexChanged: {
                    settingsBridge.applyTheme(textAt(currentIndex))
                    settingsBridge.saveSettings()
                }
            }
        }
    }

    Component {
        id: panelPage
        ColumnLayout {
            spacing: 16

            Text {
                text: "Panel"
                font.pixelSize: 24
                font.bold: true
                color: "#e0e0f0"
            }

            Text {
                text: "Panel count: " + settingsBridge.panel_count
                font.pixelSize: 14
                color: "#a0a0b0"
            }
        }
    }

    Component {
        id: aboutPage
        ColumnLayout {
            spacing: 16

            Text {
                text: "About Xarph"
                font.pixelSize: 24
                font.bold: true
                color: "#e0e0f0"
            }

            Text {
                text: "Xarph Desktop Environment v0.2.0"
                font.pixelSize: 14
                color: "#a0a0b0"
            }

            Text {
                text: "A custom desktop environment built with Qt6/QML and Rust"
                font.pixelSize: 14
                color: "#666680"
            }
        }
    }
}
