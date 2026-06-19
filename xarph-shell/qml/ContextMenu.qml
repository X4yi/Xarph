import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Rectangle {
    id: contextMenu
    width: 220
    color: "#0c0c12"
    border.color: "#2a2a3e"
    border.width: 1
    radius: 8

    property string objectType: ""
    property string objectName: ""

    ContextMenuBridge {
        id: contextMenuBridge
    }

    // Close on Escape
    Keys.onEscapePressed: contextMenu.visible = false

    onVisibleChanged: {
        if (visible) {
            contextMenu.forceActiveFocus()
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 4
        spacing: 2

        // Info label
        Text {
            text: contextMenu.objectName
            font.pixelSize: 12
            font.bold: true
            color: "#e0e0f0"
            Layout.fillWidth: true
            Layout.leftMargin: 8
            Layout.rightMargin: 8
            Layout.topMargin: 4
            elide: Text.ElideRight
        }

        // Separator
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 1
            color: "#2a2a3e"
            Layout.leftMargin: 8
            Layout.rightMargin: 8
        }

        // Menu items
        Repeater {
            model: contextMenuBridge.itemCount

            Rectangle {
                Layout.fillWidth: true
                height: isSeparator ? 1 : 32
                color: "transparent"

                property bool isSeparator: contextMenuBridge.getItemLabel(index) === "---"

                Rectangle {
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter
                    height: 1
                    color: "#2a2a3e"
                    visible: parent.isSeparator
                    anchors.leftMargin: 8
                    anchors.rightMargin: 8
                }

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 8
                    anchors.rightMargin: 8
                    spacing: 8
                    visible: !parent.isSeparator

                    Text {
                        text: contextMenuBridge.getItemIcon(index)
                        font.pixelSize: 14
                        color: "#a0a0b0"
                        visible: text !== ""
                    }

                    Text {
                        text: contextMenuBridge.getItemLabel(index)
                        font.pixelSize: 12
                        color: contextMenuBridge.isItemEnabled(index) ? "#e0e0f0" : "#666680"
                        Layout.fillWidth: true
                    }

                    Text {
                        text: contextMenuBridge.getItemShortcut(index)
                        font.pixelSize: 10
                        color: "#666680"
                        visible: text !== ""
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    visible: !parent.isSeparator && contextMenuBridge.isItemEnabled(index)
                    hoverEnabled: true
                    onEntered: parent.color = "#2a2a4e"
                    onExited: parent.color = "transparent"
                    onClicked: {
                        var actionId = contextMenuBridge.getItemId(index)
                        contextMenuBridge.executeAction(actionId)
                        contextMenu.visible = false
                    }
                }
            }
        }
    }
}
