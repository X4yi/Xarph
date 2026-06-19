import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Item {
    id: desktop

    DesktopModelBridge {
        id: desktopModel
        Component.onCompleted: loadObjects()
    }

    Item {
        id: objectContainer
        anchors.fill: parent

        Repeater {
            id: objectRepeater
            model: desktopModel.objectCount

            DesktopObject {
                objectId: "obj_" + index
                objectType: "file"
                displayName: "Object " + index
                posX: 100 + (index % 5) * 100
                posY: 100 + Math.floor(index / 5) * 100
            }
        }
    }

    Item {
        id: widgetContainer
        anchors.fill: parent
    }

    // Single MouseArea for both right-click and double-click
    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onReleased: function(mouse) {
            if (mouse.button === Qt.RightButton && !contextMenu.visible) {
                contextMenu.buildMenu("desktop", "")
                contextMenu.x = mouse.x
                contextMenu.y = mouse.y
                contextMenu.visible = true
            }
        }
        onDoubleClicked: function(mouse) {
            if (mouse.button === Qt.LeftButton) {
                Qt.openUrlExternally("x-terminal-emulator")
            }
        }
    }
}
