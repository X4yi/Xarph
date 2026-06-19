import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Item {
    id: desktopObject
    width: 80
    height: 80

    property string objectId: ""
    property string objectType: ""
    property string displayName: ""
    property string iconName: ""
    property real posX: 0
    property real posY: 0
    property bool selected: false

    x: posX
    y: posY

    DesktopObjectBridge {
        id: bridge
        objectId: desktopObject.objectId
        objectType: desktopObject.objectType
        displayName: desktopObject.displayName
        iconName: desktopObject.iconName
        x: desktopObject.posX
        y: desktopObject.posY
    }

    Column {
        anchors.centerIn: parent
        spacing: 4

        // Icon
        Rectangle {
            width: 48
            height: 48
            radius: 8
            color: selected ? "#2a2a4e" : "transparent"
            border.color: selected ? "#8c6eff" : "transparent"
            border.width: selected ? 2 : 0

            Text {
                anchors.centerIn: parent
                text: getIconText()
                font.pixelSize: 32
                color: "#e0e0f0"
            }
        }

        // Label
        Text {
            width: 80
            horizontalAlignment: Text.AlignHCenter
            text: displayName
            font.pixelSize: 11
            color: "#d0d0e0"
            elide: Text.ElideRight
            maximumLineCount: 2
            wrapMode: Text.Wrap
        }
    }

    // Mouse area for selection and drag
    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.LeftButton | Qt.RightButton

        onClicked: function(mouse) {
            if (mouse.button === Qt.LeftButton) {
                desktopObject.selected = true
                bridge.selected = true
            } else if (mouse.button === Qt.RightButton) {
                contextMenu.buildMenu(objectType, displayName)
                contextMenu.x = mouse.x
                contextMenu.y = mouse.y
                contextMenu.visible = true
            }
        }

        onDoubleClicked: function(mouse) {
            if (mouse.button === Qt.LeftButton) {
                launchObject()
            }
        }

        // Drag support
        drag.target: parent
        drag.axis: Drag.XAndY
    }

    function getIconText() {
        switch (objectType) {
            case "folder": return "📁"
            case "file": return "📄"
            case "application": return "⚙️"
            case "project": return "📂"
            case "shortcut": return "🔗"
            case "widget": return "📊"
            default: return "❓"
        }
    }

    function launchObject() {
        // Launch based on object type
        if (objectType === "application") {
            // Launch .desktop file
        } else if (objectType === "file" || objectType === "folder") {
            // Open with default handler
        }
    }
}
