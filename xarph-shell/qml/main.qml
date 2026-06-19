import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import Xarph

Window {
    id: root
    visible: true
    visibility: Window.FullScreen
    color: "transparent"

    WallpaperLayer {
        id: wallpaper
        anchors.fill: parent
    }

    Desktop {
        id: desktop
        anchors.fill: parent
    }

    Panel {
        id: panel
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        height: 40
    }

    // Fullscreen backdrop for StartMenu — catches clicks to close it
    MouseArea {
        id: startMenuBackdrop
        anchors.fill: parent
        visible: startMenu.visible
        z: 9
        onClicked: startMenu.visible = false
    }

    StartMenu {
        id: startMenu
        visible: false
        x: 0
        y: panel.height
        width: 400
        height: 500
        z: 10
    }

    // Fullscreen backdrop for ContextMenu — catches clicks to close it
    MouseArea {
        id: contextMenuBackdrop
        anchors.fill: parent
        visible: contextMenu.visible
        z: 11
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onClicked: contextMenu.visible = false
    }

    ContextMenu {
        id: contextMenu
        visible: false
        z: 12
    }
}
